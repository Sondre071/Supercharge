Import-Module "$PSScriptRoot/scripts/helpers/Read-Menu.psm1"

# Recompile each time.
$devMode = $true

function SU() {
    if ($devMode -or (Get-ChildItem -Path "./bin" -Filter *.exe) -lt 1) { . "$PSScriptRoot/scripts/helpers/build.ps1" -ProjectRoot $PSScriptRoot }

    $scriptFiles = Get-ChildItem -Path "$PSScriptRoot/scripts" -File -Filter *.ps1 | Sort-Object Name

    $options = $scriptFiles | ForEach-Object { $_.BaseName.Substring(0,1).ToUpper() + $_.BaseName.Substring(1) }

    $action = Read-Menu -Options $options -Header "Supercharge071" -ExitOption 'Exit'

    if ($action -eq "Exit") { return }

    . "$PSScriptRoot/scripts/$action.ps1" -ProjectRoot $PSScriptRoot
}

Export-ModuleMember -Function SU