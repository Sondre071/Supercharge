function New-StreamReader {
    [OutputType([System.IO.StreamReader])]
    param (
        [Parameter(Mandatory)]
        [hashtable[]]$Messages
    )

    $client = [System.Net.Http.HttpClient]::new()


    Set-Clipboard ($Config | ConvertTo-Json)

    $body = @{
        input              = $Messages 

        model              = $Config.Model
        stream             = $true

        temperature        = $Config.Parameters.Temperature
        top_p              = $Config.Parameters.Top_P
        top_k              = $Config.Parameters.Top_K
        frequency_penalty  = $Config.Parameters.Frequency_Penalty
        presence_penalty   = $Config.Parameters.Presence_Penalty
        repetition_penalty = $Config.Parameters.Repetition_Penalty
        min_p              = $Config.Parameters.Min_P
        top_a              = $Config.Parameters.Top_A
    } | ConvertTo-Json -Depth 7

    $request = [System.Net.Http.HttpRequestMessage]::new()

    $request.Headers.Add('Accept', 'application/json')
    $request.Headers.Add('Authorization', "Bearer $($Config.ApiKey)")

    $request.Content = [System.Net.Http.StringContent]::new(
        $body,
        [System.Text.Encoding]::UTF8, 'application/json'
    )

    $request.Method = 'POST'
    $request.RequestUri = 'https://openrouter.ai/api/v1/responses'

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