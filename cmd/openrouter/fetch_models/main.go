package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"io"
	"log"
	"net/http"
	. "supercharge/internal/openrouter"
)

func main() {
	apiKey := flag.String("api-key", "", "OpenRouter API-key")
	flag.Parse()

	if *apiKey == "" {
		log.Fatalf("Missing api key.")
	}

	if err := fetchModels(*apiKey); err != nil {
		log.Fatalf("Failed to fetch models: %v\n", err)
	}
}

func fetchModels(apiKey string) error {
	req, err := http.NewRequest("GET", "https://openrouter.ai/api/v1/models", nil)
	if err != nil {
		return fmt.Errorf("Failed to prepare request: %v\n", err)
	}

	req.Header.Set("Authorization", "Bearer "+apiKey)

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return fmt.Errorf("HTTP request failed: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		bodyBytes, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("Non-successful HTTP status: %d, %s", resp.StatusCode, string(bodyBytes))
	}

	var modelsResp FetchModelsResponse
	if err := json.NewDecoder(resp.Body).Decode(&modelsResp); err != nil {
		return fmt.Errorf("Failed to decode response: %w", err)
	}

	for _, model := range modelsResp.Data {
		fmt.Println(model.Id)
	}

	return nil
}
