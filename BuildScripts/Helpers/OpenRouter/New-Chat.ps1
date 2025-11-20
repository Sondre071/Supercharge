function New-Chat {
    param (
        [Parameter(Mandatory)]
        [hashtable]$Config,

        [Parameter(Mandatory)]
        [AllowEmptyString()]
        [string]$Prompt
    )

    [hashtable[]]$messageHistory = @()

    if ('' -ne $Prompt) {
        $messageHistory += Format-Message `
            -Text $Prompt `
            -Role 'system'
    }

    Write-MenuHeader `
        -Header 'New chat' `
        -Subheaders ($config.Model)

    while ($true) {
        $userInput = Read-Input

        if ($userInput.Trim() -eq '') {
            continue
        }

        Write-Host "You: $userInput`n"

        $messageHistory += Format-Message `
            -Text $userInput `
            -Role 'user'

        $reader = New-StreamReader `
            -Messages $messageHistory `
            -ApiKey $Config.ApiKey `
            -Model $Config.Model `
            -Url $Config.Url


        $modelResponse = ''

        while (-not $reader.EndOfStream) {

            $modelResponse += Read-StreamLine `
                -Reader $reader `
                -Color 'Cyan'
        }

        $messageHistory += Format-Message `
            -Text $modelResponse `
            -Role 'assistant'

        Write-Host `n
    }
}