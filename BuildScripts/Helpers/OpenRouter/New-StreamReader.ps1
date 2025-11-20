function New-StreamReader {
    param (
        [Parameter(Mandatory)]
        [hashtable[]]$Messages,

        [Parameter(Mandatory)]
        [string]$ApiKey,

        [Parameter(Mandatory)]
        [string]$Model,
    
        [Parameter(Mandatory)]
        [string]$Url
    )

    $client = [System.Net.Http.HttpClient]::new()

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
        [System.Text.Encoding]::UTF8, 'application/json'
    )

    $request.Method = 'POST'
    $request.RequestUri = $Url

    $response = $client.SendAsync(
        $request, [System.Net.Http.HttpCompletionOption]::ResponseHeadersRead
    ).GetAwaiter().GetResult()

    if (-not $response.IsSuccessStatusCode) {
        throw "Request failed: `'$($response.ReasonPhrase)`'."
    }

    $stream = $response.`
        Content.`
        ReadAsStreamAsync().`
        GetAwaiter().`
        GetResult()

    return [System.IO.StreamReader]::new($stream)
}