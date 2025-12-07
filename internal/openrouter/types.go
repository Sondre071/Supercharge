package openrouter_types

type InputMessage struct {
	Role    string `json:"role"`
	Content string `json:"content"`
}

type MessageRequestBody struct {
	Model  string    `json:"model"`
	Input  []InputMessage `json:"input"`
	Stream bool      `json:"stream"`
}

type MessageResponseStreamEvent struct {
	Type           string                   `json:"type"`
	Logprobs       []map[string]interface{} `json:"logprobs,omitempty"`
	OutputIndex    int                      `json:"output_index,omitempty"`
	ItemId         string                   `json:"item_id,omitempty"`
	ContentIndex   int                      `json:"content_index,omitempty"`
	Delta          string                   `json:"delta,omitempty"`
	SequenceNumber int                      `json:"sequence_number,omitempty"`
}
