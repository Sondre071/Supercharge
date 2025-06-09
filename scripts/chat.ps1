[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

$messageHistory = @()

Write-MenuHeader -Header "Chat session"

while ($true) {
    $processInfo = [System.Diagnostics.ProcessStartInfo]::new()
    $processInfo.FileName = "go"
    $processInfo.Arguments = "run ./cmd/chat/"
    $processInfo.RedirectStandardInput = $true
    $processInfo.RedirectStandardOutput = $true
    $processInfo.UseShellExecute = $false

    $userInput = Read-Host "`nYou"

    if ($userInput -eq "") { continue }

    $messageHistory += @{ role = "user"; content = $userInput }

    $messagesJson = $messageHistory | ConvertTo-Json -AsArray -Compress

    $process = [System.Diagnostics.Process]::Start($processInfo)

    $process.StandardInput.WriteLine($messagesJson)
    $process.StandardInput.Close()

    $modelResponse = ""

    Write-Host 
    while ($null -ne ($line = $process.StandardOutput.ReadLine())) {
        $modelResponse += $line
        Write-Host $line -ForegroundColor Yellow -NoNewLine
    }
    Write-Host 

    $messageHistory += @{
        role = "assistant"; content = $modelResponse
    } 
}