param (
    [string]$ProjectRoot
)

$HelpersPath = Join-Path $ProjectRoot 'Scripts' 'Helpers' 'OpenRouter'

. (Join-Path $HelpersPath 'New-Chat.ps1') -HelpersPath $HelpersPath
. (Join-Path $HelpersPath 'Open-Settings.ps1')
. (Join-Path $ProjectRoot 'Scripts' 'Helpers' 'Shared' 'Get-Config.ps1')

$configPath = Join-Path $env:UserProfile '.supercharge' 'openrouter.json'
$config = Get-Config `
    -Path $configPath `
    -InitialJSONContent '{"ApiKey":"","Model":"","Url":"https://openrouter.ai/api/v1/responses"}'

###

while ($true) {
    $choice = Read-Menu -Header 'OpenRouter' -Options ('New chat', 'Settings')

    switch ($choice) {
        'New chat' {
            if ((-not $config.ApiKey) -or (-not $config.Model)) {
                throw 'Config missing api-key or model.'
            }

            New-Chat `
                -Config $config
        }

        'Settings' {
            Open-Settings -Config $config
        }
    }
}