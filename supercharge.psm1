$devMode = $true

function SU() {

    try {

        if ($devMode) {

            Build-Project
        }
        else {
            if (-not (Test-Path -Path "$PSScriptRoot/bin/supercharge.exe")) {
                Build-Project
            }
        }

        & "$PSScriptRoot/bin/supercharge.exe"

    }
    catch {
        Write-Host $_ -ForegroundColor Red
    }
}

function Build-Project() {
            Write-Host "Building..." -NoNewLine -ForegroundColor DarkGray

            go build -o "$PSScriptRoot/bin/supercharge.exe" "$PSScriptRoot/cmd/supercharge/main.go"

            Write-Host " Done!`n" -ForegroundColor DarkGray
}

Export-ModuleMember -Function SU