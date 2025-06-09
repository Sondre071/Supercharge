package main

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
)

const Reset = "\033[0m"
const Yellow = "\033[33m"
const Gray = "\033[37m"
const Cyan = "\033[36m"

var configPath = "config.json"
var apiKey = ""
var apiUrl = ""
var currentModel = ""

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

type ResponseData struct {
	Choices []struct {
		Delta struct {
			Content string `json:"content"`
		} `json:"delta"`
	} `json:"choices"`
}

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

func main() {

	reader := bufio.NewReader(os.Stdin)
	argument, err := reader.ReadString('\n')
	if err != nil {
		fmt.Fprintln(os.Stderr, "failed to capture message history: ", err)
		return
	}

	var messageHistory []Message
	if err := json.Unmarshal([]byte(argument), &messageHistory); err != nil {
		fmt.Fprintln(os.Stderr, "failed to unmarshal message history: ", err)
	}

	streamer := Streamer{
		ApiKey:       apiKey,
		ApiUrl:       apiUrl,
		CurrentModel: currentModel,
		Client:       &http.Client{Timeout: 0},
	}

	stream, _ := streamer.NewStream(messageHistory)
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
