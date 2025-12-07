$dataPath = Join-Path $env:UserProfile '.supercharge' 'data'
$jsonPath = Join-Path $env:UserProfile '.supercharge' 'data' 'openrouter.json'
$promptsPath = Join-Path $env:UserProfile '.supercharge' 'data' 'prompts'

if (-not (Test-Path $dataPath)) {
    New-Item -ItemType Directory -Path $dataPath
}

if (-not (Test-Path $jsonPath)) {
    $initialContent = @{
        apiKey     = ""
        model      = ""
        models     = @()
        parameters = @{
            temperature        = 1.0
            top_p              = 1.0
            top_k              = 0
            frequency_penalty  = 0.0
            presence_penalty   = 0.0
            repetition_penalty = 0.0
            min_p              = 0.0
            top_a              = 0.0
        }
        prompts    = $promptsPath
    }

    $json = $initialContent | ConvertTo-Json `
        -Depth 7 `
        -Compress

    New-Item `
        -Path $jsonPath
    -Value $json
}