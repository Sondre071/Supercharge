param (
    [string]$ProjectRoot
)

$HelpersPath = Join-Path $ProjectRoot 'Scripts' 'Helpers' 'OpenRouter'

. (Join-Path $HelpersPath 'New-Chat.ps1') -HelpersPath $HelpersPath

$configPath = Join-Path $env:UserProfile '.supercharge' 'openrouter.json'
$config = PSModuleManager `
    -FilePath $configPath `
    -InitialJSONContent '{"ApiKey":"","Url":"https://openrouter.ai/api/v1/responses","CurrentModel":"",}'

###

$choice = Read-Menu -Header 'OpenRouter' -Options ('New chat')

switch ($choice) {
    'New chat' {
        New-Chat `
        -Config $config `
    }
}