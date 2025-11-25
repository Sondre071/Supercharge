Set-StrictMode -Version Latest

$ProjectRoot = $PSScriptRoot

function su() {
    $options = Get-ChildItem `
        -Path (Join-Path $ProjectRoot 'Scripts') `
        -File
    | Where-Object { $_.Extension -eq '.ps1' }
    | ForEach-Object {
        @{
            Name = $_.BaseName -creplace '(?<!^)([A-Z])', ' $1'
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

Export-ModuleMember -Function su