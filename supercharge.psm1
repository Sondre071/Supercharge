$devMode = $true

function su($Command) {

    if ($true -eq $devMode) {
        Write-Host "Building..." -NoNewLine -ForegroundColor DarkGray
    
        go build -o "$PSScriptRoot/bin/files.exe" "$PSScriptRoot/internal/files/files.go"
    
        Write-Host " Done!`n" -ForegroundColor DarkGray
    }

    . $PSScriptRoot\Scripts\Menu\HomeMenu.ps1 -ProjectRoot $PSScriptRoot
}

Export-ModuleMember -Function su