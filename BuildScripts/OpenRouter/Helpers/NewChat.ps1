param (
    [Parameter(Mandatory)]
    [hashtable]$Config,

    [Parameter(Mandatory)]
    [string]$HelpersPath
)

$createRequestScript = Join-Path $HelpersPath 'CreateRequest.ps1'
$addToMessageHistoryScript = Join-Path $HelpersPath 'AddToMessageHistory.ps1'

#$client = [System.Net.Http.HttpClient]::new()

[hashtable[]]$messageHistory = @()

Write-Host

while ($true) {
    $userInput = Read-Input
    Write-Host "You: $userInput`n"

    $messageHistory += (& $addToMessageHistoryScript -Text $userInput -Role 'user')

    $request = (& $createRequestScript `
            -Messages $messageHistory `
            -Model $Config.CurrentModel `
            -ApiKey $Config.ApiKey `
            -Url $Config.Url)

    $response = Invoke-WebRequest @request

    $output = ($response | ConvertFrom-Json -Depth 7).output[0].content.text

    $messageHistory += (& $addToMessageHistoryScript -Text $output -Role 'assistant')

    Write-Host "$output`n" -ForegroundColor Cyan
}
