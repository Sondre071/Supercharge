package main

import (
	"encoding/xml"
	"flag"
	"fmt"
	"io"
	"os"
	"log"
	"net/http"
	"strings"
	"errors"
	. "supercharge/internal/blobstorage"
)

var ErrContainerNotFound = errors.New("Container not found.")

func main() {
	connection_string := flag.String("connectionstring", "", "Storage account connection string")
	container := flag.String("container", "", "Storage account name")
	flag.Parse()

	if *connection_string == "" {
		log.Fatalf("Missing connection string.")
	}

	if *container == "" {
		log.Fatalf("Missing storage account name.")
	}

	if err := fetch_blobs(*connection_string, *container); err != nil {
		if errors.Is(err, ErrContainerNotFound) {
			os.Exit(3)
		}
		log.Fatalf("Failed to fetch blobs: %v\n", err)
	}
}

func fetch_blobs(con_str string, container string) error {
	kvs := strings.Split(con_str, ";")

	url := strings.TrimPrefix(kvs[0], "BlobEndpoint=")
	sas := strings.TrimPrefix(kvs[4], "SharedAccessSignature=")

	url = fmt.Sprintf("%s%s?restype=container&comp=list&%s", url, container, sas)

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
		if resp.StatusCode == http.StatusNotFound {
			return ErrContainerNotFound
		}

		bodyBytes, _ := io.ReadAll(resp.Body)

		return fmt.Errorf("Non-successful HTTP status: %d, %s", resp.StatusCode, string(bodyBytes))
	}

	bodyBytes, err := io.ReadAll(resp.Body)
	if err != nil {
		return fmt.Errorf("Failed to read response body: %w", err)
	}

	var result BlobEnumerationResults
	if err := xml.Unmarshal(bodyBytes, &result); err != nil {
		return fmt.Errorf("Failed to unmarshal XML: %w", err)
	}

	for _, c := range result.Blobs {
		fmt.Println(c.Name)
	}

	return nil
}
