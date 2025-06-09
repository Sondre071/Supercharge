param(
    [string]$ProjectRoot
)

if (-not $ProjectRoot) { throw "project root path not provided." }

Get-ChildItem -Path "$ProjectRoot\go-src" -Directory | ForEach-Object {
    $name = $_.BaseName
    $sourcePath = "$ProjectRoot\go-src\$name"
    $outputPath = "$ProjectRoot\bin\$name.exe"

    Set-Location "$ProjectRoot"

    Write-Host "Building $name..." -NoNewLine -ForegroundColor DarkGray

    & go build -o $outputPath $sourcePath

    Write-Host "Done!`n" -ForegroundColor DarkGray

    # Jump back.
    Set-Location -
}
