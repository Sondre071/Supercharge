function New-Chat {
    [OutputType([void])]
    param (
        [Parameter(Mandatory)]
        [AllowEmptyString()]
        [string]$SystemPrompt
    )

    [hashtable[]]$messageHistory = @()

    if ('' -ne $SystemPrompt) {
        $messageHistory += Format-Message `
            -Text $SystemPrompt `
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
            -Messages $messageHistory

        $modelResponse = ''

        while (-not $reader.EndOfStream) {
            $text = Read-StreamLine `
                -LineStr $reader.ReadLine()
            
            if (-not $text) { continue }

            Write-Host $text -NoNewLine -ForegroundColor 'Cyan'

            $modelResponse += $text
        }

        $messageHistory += Format-Message `
            -Text $modelResponse `
            -Role 'assistant'

        Write-Host `n
    }
}