param (
    [string]$ProjectRoot
)

$helpersPath = Join-Path $ProjectRoot 'Scripts' 'Helpers' 'OpenRouter'
$dataPath = Join-Path $env:UserProfile '.supercharge'

# General
. (Join-Path $helpersPath 'Select-Prompt.ps1')
. (Join-Path $helpersPath 'New-Chat.ps1')
. (Join-Path $helpersPath 'Open-Settings.ps1')
. (Join-Path $helpersPath 'Select-Model.ps1')
. (Join-Path $ProjectRoot 'Scripts' 'Helpers' 'Shared' 'Use-Config.ps1')

# New chat
. (Join-Path $helpersPath 'New-StreamReader.ps1')
. (Join-Path $helpersPath 'Format-Message.ps1')
. (Join-Path $helpersPath 'Read-StreamLine.ps1')

$Config = Use-Config `
    -Path (Join-Path $dataPath 'openrouter.json')

###

while ($true) {
    $choice = Read-Menu `
        -Header 'OpenRouter' `
        -Options 'New chat', 'Settings' `
        -ExitOption 'Back'

    switch ($choice) {
        'New chat' {
            if (-not $Config.ApiKey) {
                throw 'Config missing api-key or model.'
            }

            if (-not $Config.Model) {
                throw 'Config missing model.'
            }

            $prompt, $cancel = Select-Prompt `
                -Path $Config.Paths.Prompts
            
            if ($cancel) { continue }

            New-Chat -SystemPrompt $prompt
        }

        'Settings' {
            Open-Settings
        }

        'Back' {
            return
        }
    }
}