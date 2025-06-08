package openrouter

import (
	"Supercharge071/internal/menu"
	"bufio"
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"

	tea "github.com/charmbracelet/bubbletea"
)

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
		ApiKey:       "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",
		ApiUrl:       "https://openrouter.ai/api/v1/chat/completions",
		CurrentModel: "openai/gpt-4.1",
		Client:       &http.Client{Timeout: 0},
	}
}

func (s *Streamer) NewStream(userInput, systemPrompt string, currentMessageHistory []Message) (io.ReadCloser, error) {
	var messages []Message

	if systemPrompt != "" {
		messages = append(messages, Message{Role: "system", Content: systemPrompt})
	}

	if currentMessageHistory != nil && len(currentMessageHistory) > 0 {
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

		parseStream(stream, func(chunk string) {
			fmt.Print(menu.Cyan + chunk + menu.Reset)
		})

		return nil
	}
}

func parseStream(r io.Reader, onChunk func(string)) error {
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
				onChunk(chunk)
			}
		}
	}

	return scanner.Err()
}

func settings() *menu.Menu {

	return nil
}
