param(
    [string]$ProjectRoot
)
if (-not $ProjectRoot) { throw "project root path not provided." }

$messageHistory = @()

Write-MenuHeader -Header "Chat session"

while ($true) {
    $processInfo = [System.Diagnostics.ProcessStartInfo]::new()
    $processInfo.FileName = "$ProjectRoot/bin/chat.exe"
    $processInfo.RedirectStandardOutput = $true
    $processInfo.UseShellExecute = $false
    $processInfo.StandardOutputEncoding = [System.Text.Encoding]::UTF8

    $userInput = Read-Host "`nYou"

    if ($userInput -eq "") { break }

    $messageHistory += @{ role = "user"; content = $userInput }

    $jsonToSend = ` $messageHistory | ConvertTo-Json `
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
        $responseChunk = $jsonBytes | ConvertFrom-Json

        Write-Host $responseChunk -ForegroundColor Yellow -NoNewLine
        $modelResponse += $responseChunk
    }
    Write-Host 

    $messageHistory += @{
        role = "assistant"; content = $modelResponse
    } 
}