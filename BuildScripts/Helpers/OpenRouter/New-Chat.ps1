param (
    [Parameter(Mandatory)]
    [string]$HelpersPath
)

. (Join-Path $HelpersPath 'New-StreamRequest.ps1')
. (Join-Path $HelpersPath 'Format-Message.ps1')
. (Join-Path $HelpersPath 'Read-StreamLine.ps1')

function New-Chat {
    param (
        [Parameter(Mandatory)]
        [hashtable]$Config
    )

    $client = [System.Net.Http.HttpClient]::new()

    [hashtable[]]$messageHistory = @()

    Write-MenuHeader -Header 'New chat' -Subheaders ($config.CurrentModel)

    while ($true) {
        $userInput = Read-Input

        if ($userInput -eq '') {
            continue
        }

        Write-Host "You: $userInput`n"

        $messageHistory += Format-Message -Text $userInput -Role 'user'

        $stream = New-StreamRequest `
            -HttpClient $client `
            -Messages $messageHistory `
            -Model $Config.CurrentModel `
            -ApiKey $Config.ApiKey `
            -Url $Config.Url

        $reader = [System.IO.StreamReader]::new($stream)

        $modelResponse = ''

        while (-not $reader.EndOfStream) {

            $modelResponse += Read-StreamLine `
                -Reader $reader `
                -Color 'Cyan'
        }

        $messageHistory += Format-Message -Text $modelResponse -Role 'assistant'

        Write-Host `n
    }
}