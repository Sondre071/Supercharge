param (
    [string]$ProjectRoot
)

$helpersPath = Join-Path $ProjectRoot 'Scripts' 'Helpers' 'OpenRouter'

# General
. (Join-Path $helpersPath 'New-Chat.ps1')
. (Join-Path $helpersPath 'Select-Prompt.ps1')
. (Join-Path $helpersPath 'Open-Settings.ps1')
. (Join-Path $helpersPath 'Select-Model.ps1')
. (Join-Path $ProjectRoot 'Scripts' 'Helpers' 'Shared' 'Use-Config.ps1')

# New chat
. (Join-Path $helpersPath 'Format-Message.ps1')
. (Join-Path $helpersPath 'New-StreamReader.ps1')
. (Join-Path $helpersPath 'Read-StreamLine.ps1')

# Settings

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

$config = Use-Config `
    -Path (Join-Path $dataPath 'openrouter.json') `
    -InitialContent $initialContent

###

while ($true) {
    $choice = Read-Menu `
        -Header 'OpenRouter' `
        -Options 'New chat', 'Settings' `
        -ExitOption 'Back'

    switch ($choice) {
        'New chat' {
            if (-not $config.ApiKey) {
                throw 'Config missing api-key or model.'
            }

            if (-not $config.Model) {
                throw 'Config missing model.'
            }

            $prompt, $cancel = Select-Prompt `
                -Path $config.Paths.Prompts
            
            if ($cancel) { continue }

            New-Chat `
                -Config $config `
                -SystemPrompt $prompt
        }

        'Settings' {
            Open-Settings -Config $config
        }

        'Back' {
            return
        }
    }
}