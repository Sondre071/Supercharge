$devMode = $true

function su($Command) {

    Write-Host "Building..." -NoNewLine -ForegroundColor DarkGray

    go build -o "$PSScriptRoot/bin/supercharge.exe" "$PSScriptRoot/internal/files/files.go"

    Write-Host " Done!`n" -ForegroundColor DarkGray

    . $PSScriptRoot\Scripts\Menu\HomeMenu.ps1 -ProjectRoot $PSScriptRoot
}

Export-ModuleMember -Function su