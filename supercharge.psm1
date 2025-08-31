$devMode = $true

function SU() {

    try {

        if ($devMode -or (-not (Test-Path -Path "$PSScriptRoot/bin/supercharge.exe"))) {

            Write-Host "Building..." -NoNewLine -ForegroundColor DarkGray

            go build -o "$PSScriptRoot/bin/supercharge.exe" "$PSScriptRoot/cmd/supercharge/main.go"

            Write-Host " Done!`n" -ForegroundColor DarkGray

        }

        & "$PSScriptRoot/bin/supercharge.exe"

    }
    catch {
        Write-Host $_ -ForegroundColor Red
    }
}

Export-ModuleMember -Function SU