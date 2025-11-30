Set-StrictMode -Version Latest

$ProjectRoot = $PSScriptRoot

if (-not (Test-Path (Join-Path $PSScriptRoot 'bin' 'release' 'read-menu.exe'))) {
    $buildScript = Join-Path $PSScriptRoot 'Build' 'Build.ps1'

    Write-Host 'Compiling binaries..... ' -NoNewLine -ForegroundColor DarkGray
    & $buildScript
    Write-Host 'Done!' -ForegroundColor DarkGray
}

. (Join-Path $PSScriptRoot 'Scripts' 'Helpers' 'Shared' 'Read-Menu.ps1')

function SU() {
    $options = Get-ChildItem `
        -Path (Join-Path $ProjectRoot 'Scripts') `
        -File
    | Where-Object { $_.Extension -eq '.ps1' }
    | ForEach-Object {
        @{
            Name = $_.BaseName -creplace '(?<!^)(_)', ' '
            Path = $_.FullName
        }
    }

    while ($true) {
        $choice = Read-Menu `
            -Header 'Supercharge' `
            -Options $options `
            -ExitOption 'Exit'

        if ($choice -eq 'Exit') { return }

        . $choice.Path -ProjectRoot $ProjectRoot
    }
}

Export-ModuleMember -Function SU