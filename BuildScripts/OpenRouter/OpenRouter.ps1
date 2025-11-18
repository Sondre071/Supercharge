param (
    [string]$ProjectRoot
)

$newChatScript = Join-Path $ProjectRoot 'Scripts' 'Helpers' 'OpenRouter' 'NewChat.ps1'

$configPath = Join-Path $env:UserProfile '.supercharge' 'openrouter2.json'
$config = PSModuleManager -FilePath $configPath -InitialJSONContent '{"ApiKey":"","Url":"https://openrouter.ai/api/v1/chat/completions","CurrentModel":"",}'

$choice = Read-Menu -Header 'OpenRouter' -Options ('New chat')

switch ($choice) {
    'New chat' {
        & $newChatScript -config $config     
    }
}