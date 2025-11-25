param (
    [switch]$Force
)

$dataPath = Join-Path $env:UserProfile '.supercharge'

$initialContent = @{
    ApiKey     = ""
    Model      = ""
    Models     = @()
    Parameters = @{
        Temperature        = 1.0
        Top_P              = 1.0
        Top_K              = 0
        Frequency_Penalty  = 0.0
        Presence_Penalty   = 0.0
        Repetition_Penalty = 0.0
        Min_P              = 0.0
        Top_A              = 0.0
    }
    Paths      = @{
        Prompts = (Join-Path $dataPath 'prompts')
    }
}

$json = $initialContent | ConvertTo-Json `
    -Depth 7 `
    -Compress

New-Item `
    -Path (Join-Path $dataPath 'openrouter.json') `
    -Value $json `
    -Force:$Force