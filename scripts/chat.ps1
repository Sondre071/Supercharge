param(
    [string]$ProjectRoot
)
if (-not $ProjectRoot) { throw "project root path not provided." }

function Open-MainMenu() {
    $options = ('New chat', 'Prompts', 'Models')

    $action = Read-Menu -Header 'Chat' -Options $options -ExitOption 'Exit'

    switch ($action) {

        'New chat' { New-Chat }

        'Prompts' { Open-PromptsMenu }

        'Models' { Open-ModelsMenu }

        'Exit' { return }
    }
}

function New-Chat() {
    $messageHistory = @()

    $availablePrompts = @()
    
    if (Test-Path -Path "$ProjectRoot/data/prompts") {
        $availablePrompts += Get-ChildItem -Path "$ProjectRoot/data/prompts" | ForEach-Object { $_.BaseName }
    }

    if ($availablePrompts.Length -gt 0) {

        $selectedPrompt = Read-Menu -Header 'Select prompt' -Options $availablePrompts -ExitOption 'Exit'

        switch ($selectedPrompt) {
            'Exit' { return }

            default { 

                # Select first file to match file name. This way we can use any file extension.
                $promptData = Get-ChildItem `
                    -Path "$ProjectRoot/data/prompts" | Where-Object {
                    $_.BaseName -eq $selectedPrompt
                }[0] | Get-Content
        
                $messageHistory += @{
                    role = "system"; content = $promptData
                }
            }
        }
    }

    Write-MenuHeader -Header "Chat session"

    while ($true) {
        $processInfo = [System.Diagnostics.ProcessStartInfo]::new()
        $processInfo.FileName = "$ProjectRoot/bin/chat.exe"
        $processInfo.RedirectStandardOutput = $true
        $processInfo.UseShellExecute = $false
        $processInfo.StandardOutputEncoding = [System.Text.Encoding]::UTF8

        $userInput = Read-Host "`nYou"

        if ($userInput -eq "") { break }

        $messageHistory += @{
            role = "user"; content = $userInput
        }

        $jsonToSend = $messageHistory | ConvertTo-Json `
            -AsArray `
            -Compress `
            -Depth 10

        $base64Json = [Convert]::ToBase64String([System.Text.Encoding]::UTF8.GetBytes($jsonToSend))

        $processInfo.Arguments = "`"$ProjectRoot`" `"$base64Json`""
        $process = [System.Diagnostics.Process]::Start($processInfo)
        ##process.StandardInput.WriteLine($jsonToSend)
        #$process.StandardInput.Close()

        $modelResponse = ""


        Write-Host 
        while ($null -ne ($jsonBytes = $process.StandardOutput.ReadLine())) {
            # TODO: Add a way to cancel the stream. Perhaps by writing to the binary std.

            $responseChunk = $jsonBytes | ConvertFrom-Json

            Write-Host $responseChunk -ForegroundColor Cyan -NoNewLine
            $modelResponse += $responseChunk
        }
        Write-Host 

        $messageHistory += @{
            role = "assistant"; content = $modelResponse
        } 
    }
}

Open-MainMenu