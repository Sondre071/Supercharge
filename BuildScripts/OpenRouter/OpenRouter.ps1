param (
    [string]$ProjectRoot
)

#
$HelpersPath = Join-Path $ProjectRoot 'Scripts' 'Helpers' 'OpenRouter'

$newChatScript = Join-Path $HelpersPath 'NewChat.ps1'
#

$configPath = Join-Path $env:UserProfile '.supercharge' 'openrouter.json'
$config = PSModuleManager `
    -FilePath $configPath `
    -InitialJSONContent '{"ApiKey":"","Url":"https://openrouter.ai/api/v1/responses","CurrentModel":"",}'

$choice = Read-Menu -Header 'OpenRouter' -Options ('New chat')

switch ($choice) {
    'New chat' {
        & $newChatScript `
            -Config $config `
            -HelpersPath $HelpersPath
    }
}