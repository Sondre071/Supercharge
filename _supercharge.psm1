
function SU() {
    go build -o "$PSScriptRoot/bin/supercharge.exe" "$PSScriptRoot/cmd/supercharge/main.go"

    & "$PSScriptRoot/bin/supercharge.exe"
}

Export-ModuleMember -Function SU