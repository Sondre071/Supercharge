package main

import (
	"bufio"
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"strings"
	"time"
)

type Config struct {
	ApiKey       string
	ApiUrl       string
	CurrentModel string
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

type Message struct {
	Role    string `json:"role"`
	Content string `json:"content"`
}

type Choice struct {
	Delta Delta `json:"delta"`
}

type Delta struct { Content string `json:"content"` }

type ResponseData struct {
	Choices []Choice `json:"choices"`
}

func main() {
	const configPath = "config.json"

	reader := bufio.NewReader(os.Stdin)
	argument, err := reader.ReadString('\n')
	if err != nil {
		fmt.Fprintln(os.Stderr, "failed to capture message history: ", err)
		return
	}

	var messageHistory []Message
	if err := json.Unmarshal([]byte(argument), &messageHistory); err != nil {
		fmt.Fprintln(os.Stderr, "failed to unmarshal message history: ", err)
		return
	}

	config, _ := loadConfig(configPath)

	streamer := Streamer{
		ApiKey:       config.ApiKey,
		ApiUrl:       config.ApiUrl,
		CurrentModel: config.CurrentModel,
		Client: &http.Client{
			Timeout: 5 * time.Minute,
			Transport: &http.Transport{
				MaxIdleConns:       5,
				IdleConnTimeout:    90 * time.Second,
				DisableCompression: true,
			},
		},
	}

	stream, err := streamer.NewStream(messageHistory)
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to create stream: %v\n", err)
		os.Exit(1)
	}

	scanner := bufio.NewScanner(stream)

	for scanner.Scan() {
		line := scanner.Text()
		line = strings.TrimSpace(line)
		if strings.HasPrefix(line, ":") {
			continue
		}
		if line == "data: [DONE]" {
			break
		}
		if !strings.HasPrefix(line, "data: ") {
			continue
		}
		data := strings.TrimPrefix(line, "data: ")

		var responseData ResponseData
		if err := json.Unmarshal([]byte(data), &responseData); err != nil {
			fmt.Fprintf(os.Stderr, "warning: failed to unmarshal response: %v\n", err)
			continue
		}

		for _, choice := range responseData.Choices {
			fmt.Printf("%s\n", choice.Delta.Content)
			os.Stdout.Sync()
		}
	}
}

func (s *Streamer) NewStream(messageHistory []Message) (io.ReadCloser, error) {
	body := StreamRequest{
		Model:    s.CurrentModel,
		Messages: messageHistory,
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

func loadConfig(path string) (*Config, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("failed to open config file")
	}

	defer file.Close()

	var config Config
	if err := json.NewDecoder(file).Decode(&config); err != nil {
		return nil, fmt.Errorf("could not decode config: %v", err)
	}

	return &config, nil
}
