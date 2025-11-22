param (
    [string]$ProjectRoot
)

$dataPath = Join-Path $env:UserProfile '.supercharge'
$helpersPath = Join-Path $ProjectRoot 'Scripts' 'Helpers' 'OpenRouter'

# General
. (Join-Path $helpersPath 'New-Chat.ps1')
. (Join-Path $helpersPath 'Open-Prompts.ps1')
. (Join-Path $helpersPath 'Open-Settings.ps1')
. (Join-Path $helpersPath 'Get-Models.ps1')
. (Join-Path $ProjectRoot 'Scripts' 'Helpers' 'Shared' 'Get-Config.ps1')

# New chat
. (Join-Path $helpersPath 'Format-Message.ps1')
. (Join-Path $helpersPath 'New-StreamReader.ps1')
. (Join-Path $helpersPath 'Read-StreamLine.ps1')

$initialContent = @{
    ApiKey = ""
    Model  = ""
    Models = @()
    Url    = "https://openrouter.ai/api/v1/responses"
    Paths  = @{
        Prompts = (Join-Path $dataPath 'prompts')
    }
}

$config = Get-Config `
    -Path (Join-Path $dataPath 'openrouter.json') `
    -InitialContent $initialContent

###

while ($true) {
    $choice = Read-Menu `
        -Header 'OpenRouter' `
        -Options ('New chat', 'Settings') `
        -ExitOption 'Back'

    switch ($choice) {
        'New chat' {
            if ((-not $config.ApiKey) -or (-not $config.Model)) {
                throw 'Config missing api-key or model.'
            }

            $prompt = Open-Prompts `
                -Path $config.Paths.Prompts

            if ($null -eq $prompt) { continue }

            New-Chat `
                -Config $config `
                -Prompt $prompt
        }

        'Settings' {
            Open-Settings -Config $config
        }

        'Back' {
            return
        }
    }
}