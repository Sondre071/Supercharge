Set-StrictMode -Version Latest

$ProjectRoot = $PSScriptRoot

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