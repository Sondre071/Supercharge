package chat

// A simple program demonstrating the text input component from the Bubbles
// component library.

import (
	"bufio"
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"strings"

	"github.com/charmbracelet/bubbles/textinput"
	tea "github.com/charmbracelet/bubbletea"
)

const Reset = "\033[0m"
const Yellow = "\033[33m"
const Gray = "\033[37m"
const Cyan = "\033[36m"

type model struct {
	userInput       textinput.Model
	currentResponse string
	history         []Message
	streaming       bool
	stream          io.ReadCloser
	scanner         *bufio.Scanner
	err             error
}

type streamChunkMsg string
type streamDoneMsg struct{}
type errMsg error

func initialModel() model {
	ti := textinput.New()
	ti.Placeholder = "You"
	ti.Focus()
	ti.CharLimit = 120
	ti.Width = 40

	return model{userInput: ti}
}

type Config struct {
	ApiKey       string
	ApiUrl       string
	CurrentModel string
}

var configPath = "config.json"

var apiKey = ""
var apiUrl = ""
var currentModel = ""

func init() {
	file, err := os.Open(configPath)
	if err != nil {
		log.Fatalf("could not load config: %v", err)
	}

	defer file.Close()

	var config Config

	decoder := json.NewDecoder(file)
	if err := decoder.Decode(&config); err != nil {
		log.Fatalf("could not decode config: %v", err)
	}

	apiKey = config.ApiKey
	apiUrl = config.ApiUrl
	currentModel = config.CurrentModel
}

func Run() error {
	p := tea.NewProgram(initialModel())

	_, err := p.Run()
	return err
}

func (m model) Init() tea.Cmd {
	return textinput.Blink
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	var cmd tea.Cmd

	switch msg := msg.(type) {

	case tea.KeyMsg:
		switch msg.Type {
		case tea.KeyEnter:

			if m.streaming {
				return m, nil
			}

			userInput := m.userInput.Value()
			if strings.TrimSpace(userInput) == "" {
				return m, nil
			}

			m.userInput.SetValue("")
			m.userInput.Blur()
			m.currentResponse = ""
			m.history = append(m.history, Message{"user", userInput})
			m.streaming = true

			stream, err := NewStreamer().NewStream(userInput, "", nil)
			if err != nil {
				m.err = err
				m.streaming = false
				m.userInput.Focus()
				return m, nil
			}

			m.stream = stream
			m.scanner = bufio.NewScanner(stream)
			return m, readStreamChunkCmd(m.scanner)

		case tea.KeyCtrlC, tea.KeyEsc:
			return m, tea.Quit
		}

	case streamChunkMsg:
		m.currentResponse += string(msg)
		return m, readStreamChunkCmd(m.scanner)

	case streamDoneMsg:
		m.streaming = false
		m.userInput.Focus()
		if m.stream != nil {
			m.stream.Close()
		}

		m.history = append(m.history, Message{"assistant", m.currentResponse})
		m.currentResponse = ""

		return m, nil

	case errMsg:
		m.err = msg
		m.streaming = false
		m.userInput.Focus()
		if m.stream != nil {
			m.stream.Close()
		}
		return m, nil
	}

	if !m.streaming {
		m.userInput, cmd = m.userInput.Update(msg)
	}

	return m, cmd
}

func (m model) View() string {

	var out strings.Builder

	for _, msg := range m.history {
		color := Gray

		if msg.Role == "assistant" {
			color = Cyan
		}

		coloredLines := colorLines(color, msg.Content)

		out.WriteString(fmt.Sprintf("\n%s %s\n", ">", coloredLines))

	}

	if m.streaming {
		out.WriteString(colorLines(Cyan, fmt.Sprintf("\n> %s\n", m.currentResponse)))
	}

	out.WriteString("\n" + m.userInput.View())

	return out.String()
}

func colorLines(color, text string) string {
	lines := strings.Split(text, "\n")

	for i, line := range lines {
		lines[i] = color + line + Reset
	}
	return strings.Join(lines, "\n")
}

func readStreamChunkCmd(scanner *bufio.Scanner) tea.Cmd {
	return func() tea.Msg {
		for scanner.Scan() {
			line := scanner.Text()
			line = strings.TrimSpace(line)
			if strings.HasPrefix(line, ":") {
				continue
			}
			if line == "data: [DONE]" {
				return streamDoneMsg{}
			}
			if !strings.HasPrefix(line, "data: ") {
				continue
			}
			data := strings.TrimPrefix(line, "data: ")

			var chunk OpenRouterChunk
			if err := json.Unmarshal([]byte(data), &chunk); err != nil {
				continue
			}

			for _, choice := range chunk.Choices {
				chunkText := choice.Delta.Content
				return streamChunkMsg(chunkText)
			}
		}
		// If the scanner ends, finish
		if err := scanner.Err(); err != nil {
			return errMsg(err)
		}
		return streamDoneMsg{}
	}
}

type Message struct {
	Role    string `json:"role"`
	Content string `json:"content"`
}

type StreamRequest struct {
	Model    string    `json:"model"`
	Messages []Message `json:"messages"`
	Stream   bool      `json:"stream"`
}

type Streamer struct {
	ApiKey       string
	ApiUrl       string
	CurrentModel string
	Client       *http.Client
}

type OpenRouterChunk struct {
	Choices []struct {
		Delta struct {
			Content string `json:"content"`
		} `json:"delta"`
	} `json:"choices"`
}

func NewStreamer() *Streamer {
	return &Streamer{
		ApiKey:       apiKey,
		ApiUrl:       apiUrl,
		CurrentModel: currentModel,
		Client:       &http.Client{Timeout: 0},
	}
}

func (s *Streamer) NewStream(userInput, systemPrompt string, currentMessageHistory []Message) (io.ReadCloser, error) {
	var messages []Message

	if systemPrompt != "" {
		messages = append(messages, Message{Role: "system", Content: systemPrompt})
	}

	if len(currentMessageHistory) > 0 {
		messages = append(messages, currentMessageHistory...)
	}

	messages = append(messages, Message{Role: "user", Content: userInput})

	body := StreamRequest{
		Model:    s.CurrentModel,
		Messages: messages,
		Stream:   true,
	}

	jsonBytes, err := json.Marshal(body)
	if err != nil {
		return nil, err
	}

	req, err := http.NewRequest("POST", s.ApiUrl, bytes.NewReader(jsonBytes))
	if err != nil {
		return nil, err
	}

	req.Header.Set("Authorization", "Bearer "+s.ApiKey)
	req.Header.Set("Content-Type", "application/json")

	resp, err := s.Client.Do(req)
	if err != nil {
		return nil, err
	}

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		b, _ := io.ReadAll(resp.Body)
		resp.Body.Close()
		return nil, fmt.Errorf("request failed: %s\n%s", resp.Status, string(b))
	}

	return resp.Body, nil
}

func NewSession(userInput string) tea.Cmd {
	return func() tea.Msg {

		streamer := NewStreamer()

		stream, err := streamer.NewStream(userInput, "", nil)
		if err != nil {
			fmt.Println("error:", err)
		}

		defer func() {
			print("\n\n")
			stream.Close()
		}()

		return parseStream(stream)
	}
}

func parseStream(r io.Reader) tea.Cmd {
	return func() tea.Msg {
		scanner := bufio.NewScanner(r)
		for scanner.Scan() {
			line := scanner.Text()

			line = strings.TrimSpace(line)

			if line == "" || strings.HasPrefix(line, ":") {
				continue
			}
			if line == "data: [DONE]" {
				break
			}
			if !strings.HasPrefix(line, "data: ") {
				continue
			}

			data := strings.TrimPrefix(line, "data: ")

			var chunk OpenRouterChunk
			if err := json.Unmarshal([]byte(data), &chunk); err != nil {
				continue
			}

			for _, choice := range chunk.Choices {
				chunk := choice.Delta.Content
				if chunk != "" {
					return chunk
				}
			}
		}

		return scanner.Err()
	}
}
