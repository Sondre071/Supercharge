package chat

import (
	"bufio"
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"
	"time"

	//"github.com/charmbracelet/bubbles/spinner"
	//"github.com/charmbracelet/bubbles/textarea"
	//"github.com/charmbracelet/bubbles/viewport"
	//"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
	//"github.com/charmbracelet/lipgloss"
)

type Streamer struct {
	Client *http.Client
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

// OpenRouter-like streaming response (SSE, JSON per line)
type ResponseData struct {
	Choices []Choice `json:"choices"`
}

type Choice struct {
	Delta Delta `json:"delta"`
}

type Delta struct {
	Content string `json:"content"`
}

func (s *Streamer) NewStream(messageHistory []Message) (io.ReadCloser, error) {
	config := GetConfig()

	body := StreamRequest{
		Model:    config.CurrentModel,
		Messages: messageHistory,
		Stream:   true,
	}

	jsonBytes, err := json.Marshal(body)
	if err != nil {
		return nil, err
	}

	req, err := http.NewRequest("POST", config.ApiUrl, bytes.NewReader(jsonBytes))
	if err != nil {
		return nil, err
	}

	req.Header.Set("Authorization", "Bearer "+config.ApiKey)
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

/* --------------------- Bubble Tea application --------------------- */

/* Msg (your structs) */

type streamStartedMsg struct {
	scanner *bufio.Scanner
	closer  io.Closer
}
type streamChunkMsg struct{ text string }
type streamDoneMsg struct{}
type streamErrMsg struct{ err error }

/* Model */

type Model struct {
	// UI
	input    string
	messages []Message

	// streaming state
	streaming bool
	scanner   *bufio.Scanner
	closer    io.Closer

	// deps
	streamer Streamer
}

func New() Model {
	return Model{
		streamer: Streamer{
			Client: &http.Client{
				Timeout: 5 * time.Minute,
				Transport: &http.Transport{
					MaxIdleConns:       5,
					IdleConnTimeout:    90 * time.Second,
					DisableCompression: true,
				},
			},
		},
		messages:  []Message{},
		input:     "",
		streaming: false,
		closer:    nil,
		scanner:   nil,
	}
}

func (m Model) Init() tea.Cmd {
	return nil
}

func startStreamCmd(s Streamer, history []Message) tea.Cmd {
	return func() tea.Msg {
		rc, err := s.NewStream(history)
		if err != nil {
			return streamErrMsg{err}
		}

		sc := bufio.NewScanner(rc)
		sc.Buffer(make([]byte, 0, 64*1024), 1024*1024)

		return streamStartedMsg{scanner: sc, closer: rc}
	}
}

func nextChunkCmd(scanner *bufio.Scanner) tea.Cmd {
	return func() tea.Msg {
		for {
			ok := scanner.Scan()
			if !ok {
				// Could be EOF or scanner error
				if err := scanner.Err(); err != nil {
					return streamErrMsg{err}
				}
				return streamDoneMsg{}
			}
			line := strings.TrimSpace(scanner.Text())
			// SSE comment keepalives like ":" → ignore
			if line == "" || strings.HasPrefix(line, ":") {
				continue
			}
			// Expect "data: ...."
			if !strings.HasPrefix(line, "data: ") {
				continue
			}
			data := strings.TrimPrefix(line, "data: ")
			if data == "[DONE]" || data == "\"[DONE]\"" {
				return streamDoneMsg{}
			}

			var rd ResponseData
			if err := json.Unmarshal([]byte(data), &rd); err != nil {
				// Skip malformed line but don’t kill the stream
				continue
			}
			var text strings.Builder
			for _, ch := range rd.Choices {
				text.WriteString(ch.Delta.Content)
			}
			t := text.String()
			if t == "" {
				// No visible delta; keep scanning
				continue
			}
			return streamChunkMsg{text: t}
		}
	}
}

func (m Model) Update(msg tea.Msg) (Model, tea.Cmd) {
	switch msg := msg.(type) {

	case tea.KeyMsg:

		if m.streaming {
			return m, nil
		}

		switch msg.String() {

		case "enter":
			prompt := strings.TrimSpace(m.input)
			if prompt == "" || m.streaming {
				return m, nil
			}

			m.messages = append(m.messages, Message{Role: "user", Content: prompt})
			m.streaming = true

			m.input = ""

			return m, startStreamCmd(m.streamer, m.messages)

		case "ctrl+c":
			if m.closer != nil {
				m.closer.Close()
			}

			return m, tea.Quit

		default:
			m.input += msg.String()
			return m, nil
		}

	case streamStartedMsg:
		m.scanner = msg.scanner
		m.closer = msg.closer

		m.messages = append(m.messages, Message{Role: "assistant", Content: ""})

		return m, nextChunkCmd(m.scanner)

	case streamChunkMsg:
		lastMessage := &m.messages[len(m.messages)-1]
		lastMessage.Content += msg.text

		return m, nextChunkCmd(m.scanner)

	case streamDoneMsg:
		m.streaming = false

		if m.closer != nil {
			m.closer.Close()
			m.closer = nil
		}

		m.scanner = nil

		return m, nil

	case streamErrMsg:
		m.streaming = false
		if m.closer != nil {
			_ = m.closer.Close()
			m.closer = nil
		}
		m.scanner = nil
		m.messages = append(m.messages, Message{
			Role:    "program",
			Content: "Stream error: " + msg.err.Error(),
		})
		return m, nil
	}

	return m, nil
}

func (m Model) View() string {
	var b strings.Builder

	for _, msg := range m.messages {
		b.WriteString(msg.Role)
		b.WriteString(":\n")
		b.WriteString(msg.Content)
		b.WriteString("\n\n")
	}

	if m.streaming {
		b.WriteString("(streaming...)\n\n")
	} else {
		b.WriteString("user:\n")
		b.WriteString(m.input)
		b.WriteString("\n")
	}

	return b.String()
}

func GetConfig() Config {
	path := filepath.Join("config.json")

	var config Config

	file, err := os.Open(path)
	if err != nil {
		panic(fmt.Errorf("failed to open config: %w", err))
	}
	defer file.Close()

	if err := json.NewDecoder(file).Decode(&config); err != nil {
		panic(fmt.Errorf("failed to decode config: %w", err))
	}

	return config
}

type Config struct {
	ApiKey       string
	ApiUrl       string
	CurrentModel string
}
