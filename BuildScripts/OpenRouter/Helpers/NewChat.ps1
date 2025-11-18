param (
    [Parameter(Mandatory)]
    [hashtable]$Config,

    [Parameter(Mandatory)]
    [string]$HelpersPath
)

$createRequestScript = Join-Path $HelpersPath 'CreateRequest.ps1'
$addToMessageHistoryScript = Join-Path $HelpersPath 'AddToMessageHistory.ps1'

$client = [System.Net.Http.HttpClient]::new()

[hashtable[]]$messageHistory = @()

Write-MenuHeader -Header 'New chat' -Subheaders ($config.CurrentModel)

while ($true) {
    $userInput = Read-Input
    Write-Host "You: $userInput`n"

    $messageHistory += (& $addToMessageHistoryScript -Text $userInput -Role 'user')

    $request = (& $createRequestScript `
            -Messages $messageHistory `
            -Model $Config.CurrentModel `
            -ApiKey $Config.ApiKey `
            -Url $Config.Url)

    $response = $client.SendAsync($request).
    GetAwaiter().
    GetResult()

    if ($response.IsSuccessStatusCode -ne $true) {
        throw "Request failed: `'$($response.ReasonPhrase)`'."
    }

    $result = $response.
    Content.
    ReadAsStringAsync().
    GetAwaiter().
    GetResult() | ConvertFrom-Json

    $message = $result.output[0].content.text

    $messageHistory += (& $addToMessageHistoryScript -Text $message -Role 'assistant')

    Write-Host "$message`n" -ForegroundColor Cyan
}
