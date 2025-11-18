param (
    [Parameter(Mandatory)]
    [hashtable[]]$Messages,

    [Parameter(Mandatory)]
    [string]$Model,

    [Parameter(Mandatory)]
    [string]$ApiKey,
    
    [Parameter(Mandatory)]
    [string]$Url,

    [Parameter(Mandatory)]
    [System.Net.Http.HttpClient]$HttpClient
)

$body = @{
    model  = $Model
    input  = $Messages 
    stream = $true
} | ConvertTo-Json -Depth 7

$request = [System.Net.Http.HttpRequestMessage]::new()

$request.Headers.Add('Accept', 'application/json')
$request.Headers.Add('Authorization', "Bearer $ApiKey")

$request.Content = [System.Net.Http.StringContent]::new(
    $body,
    [System.Text.Encoding]::UTF8, 'application/json')

$request.Method = 'POST'
$request.RequestUri = $Url

$response = $HttpClient.SendAsync(
    $request, [System.Net.Http.HttpCompletionOption]::ResponseHeadersRead
).GetAwaiter().GetResult()

if (-not $response.IsSuccessStatusCode) {
    throw "Stream request failed: `'$($response.ReasonPhrase)`'."
}

return $response.`
    Content.`
    ReadAsStreamAsync().`
    GetAwaiter().`
    GetResult()
    