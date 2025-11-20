param (
    [string]$ProjectRoot
)

$dataPath = Join-Path $env:UserProfile '.supercharge'

$HelpersPath = Join-Path $ProjectRoot 'Scripts' 'Helpers' 'OpenRouter'

. (Join-Path $HelpersPath 'New-Chat.ps1') -HelpersPath $HelpersPath
. (Join-Path $HelpersPath 'Open-Prompts.ps1')
. (Join-Path $HelpersPath 'Open-Settings.ps1')
. (Join-Path $ProjectRoot 'Scripts' 'Helpers' 'Shared' 'Get-Config.ps1')

$initialContent = @{
    ApiKey = ""
    Model  = ""
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