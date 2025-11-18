param (
    [Parameter(Mandatory)]

    [hashtable]$Config,

    [Parameter(Mandatory)]
    [string]$HelpersPath
)

$createStreamRequestScript = Join-Path $HelpersPath 'CreateStreamRequest.ps1'
$addToMessageHistoryScript = Join-Path $HelpersPath 'AddToMessageHistory.ps1'
$parseStreamLineScript = Join-Path $HelpersPath 'ParseStreamLine.ps1'

$client = [System.Net.Http.HttpClient]::new()

[hashtable[]]$messageHistory = @()

Write-MenuHeader -Header 'New chat' -Subheaders ($config.CurrentModel)

while ($true) {
    $userInput = Read-Input
    Write-Host "You: $userInput`n"

    $messageHistory += (& $addToMessageHistoryScript -Text $userInput -Role 'user')

    $stream = (& $createStreamRequestScript `
            -HttpClient $client `
            -Messages $messageHistory `
            -Model $Config.CurrentModel `
            -ApiKey $Config.ApiKey `
            -Url $Config.Url)


    $reader = [System.IO.StreamReader]::new($stream)

    $modelResponse = ''

    while (-not $reader.EndOfStream) {
        $modelResponse += & $parseStreamLineScript `
            -Reader $reader `
            -Color 'Cyan'
    }

    $messageHistory += & $addToMessageHistoryScript -Text $modelResponse -Role 'assistant'

    Write-Host `n
}
