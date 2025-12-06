package main

import (
	"bufio"
	"encoding/json"
	"flag"
	"fmt"
	"io"
	"log"
	"net/http"
	"strings"
	. "supercharge/internal/openrouter"
)

func main() {
	apiKey := flag.String("api-key", "", "OpenRouter API key")
	model := flag.String("model", "", "Model to use")
	messagesJSON := flag.String("messages", "", "JSON array of messages")
	flag.Parse()

	if *apiKey == "" {
		log.Fatalf("Missing api key.")
	}
	if *model == "" {
		log.Fatalf("Missing model.")
	}
	if *messagesJSON == "" {
		log.Fatalf("Missing message history")
	}

	var messages []InputMessage
	if err := json.Unmarshal([]byte(*messagesJSON), &messages); err != nil {
		log.Fatalf("Failed to parse message history: %v\n", err)
	}

	if err := streamChat(*apiKey, *model, messages); err != nil {
		log.Fatalf("Failed to run chat stream: %v\n", err)
	}
}

func streamChat(apiKey, model string, messages []InputMessage) error {
	body := MessageRequestBody{
		Model:  model,
		Input:  messages,
		Stream: true,
	}

	bodyBytes, err := json.Marshal(body)
	if err != nil {
		return fmt.Errorf("Failed to marshal request body: %w", err)
	}

	req, err := http.NewRequest("POST", "https://openrouter.ai/api/v1/responses", strings.NewReader(string(bodyBytes)))
	if err != nil {
		return fmt.Errorf("Failed to create request: %w", err)
	}

	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer "+apiKey)

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return fmt.Errorf("Failed to execute request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		bodyBytes, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("Non-success HTTP status: %d, %s", resp.StatusCode, string(bodyBytes))
	}

	reader := bufio.NewReader(resp.Body)
	for {
		line, err := reader.ReadString('\n')
		if err != nil {
			if err == io.EOF {
				break
			}
			return fmt.Errorf("Failed to read stream: %w", err)
		}

		line = strings.TrimSpace(line)
		if line == "" || line == ": OPENROUTER PROCESSING" {
			continue
		}

		if strings.HasPrefix(line, "data: ") {
			jsonStr := strings.TrimPrefix(line, "data: ")
			if jsonStr == "[DONE]" {
				break
			}

			var event MessageResponseStreamEvent
			if err := json.Unmarshal([]byte(jsonStr), &event); err != nil {
				continue
			}

			if event.Type == "response.output_text.delta" && event.Delta != "" {
				fmt.Print(event.Delta)
			}
		}
	}

	return nil
}
