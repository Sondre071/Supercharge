package files

import (
	"encoding/json"
	"fmt"
	"os"
)

type Config struct {
	OpenRouter struct {
		ApiKey        string
		ApiUrl        string
		CurrentModel  string
		CurrentPreset string
	}
}

func GetFile(path string) Config {
	fmt.Println("Hello, world!")

	content, err := getConfig(path)
	if err != nil {
		fmt.Printf("Error getting config: %vå\n", err)
		os.Exit(1)
	}	

	return *content
}

func getConfig(path string) (*Config, error) {
	byteValue, err := os.ReadFile(path)
	if err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		return nil, err
	}

	content := Config{}

	err = json.Unmarshal([]byte(byteValue), &content)
	if err != nil {
		fmt.Printf("Error unmarshalling JSON: %v\n", err)
		return nil, err
	}

	return &content, nil
}
