package main

import (
	"flag"
	"fmt"
	"io"
	"log"
	"net/http"
	"encoding/xml"
	"strings"
	. "supercharge/internal/blobstorage"
)

func main() {
	connection_string := flag.String("connectionstring", "", "Storage account connection string")
	flag.Parse()

	if *connection_string == "" {
		log.Fatalf("Missing connection string.")
	}

	if err := fetch_containers(*connection_string); err != nil {
		log.Fatalf("Failed to fetch containers: %v\n", err)
	}
}

func fetch_containers(con_str string) error {

	kvs := strings.Split(con_str, ";")

	url := strings.TrimPrefix(kvs[0], "BlobEndpoint=")
	sas := strings.TrimPrefix(kvs[4], "SharedAccessSignature=")

	url = fmt.Sprintf("%s?comp=list&%s", url, sas)

	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		return fmt.Errorf("Failed to prepare request: %v\n", err)
	}

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return fmt.Errorf("Failed to execute HTTP request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		bodyBytes, _ := io.ReadAll(resp.Body)

		return fmt.Errorf("Non-successful HTTP status: %d, %s", resp.StatusCode, string(bodyBytes))
	}

	bodyBytes, err := io.ReadAll(resp.Body)
	if err != nil {
		return fmt.Errorf("Failed to read response body: %w", err)
	}

	var result EnumerationResults
	if err := xml.Unmarshal(bodyBytes, &result); err != nil {
		return fmt.Errorf("Failed to unmarshal XML: %w", err)
	}

	for _, c := range result.Containers {
		fmt.Println(c.Name)
	}

	return nil
}
