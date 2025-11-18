Set-StrictMode -Version Latest

$ProjectRoot = $PSScriptRoot

function su($Command) {

    Confirm-LocalFiles

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

    $choice = Read-Menu -Options ($options) -ExitOption 'Exit'

    if ($choice -eq 'Exit') { return }
    
    . $choice.Path -ProjectRoot $ProjectRoot
}

function Confirm-LocalFiles() {
    $filesToCopy = Get-ChildItem `
        -Path (Join-Path $ProjectRoot 'BuildScripts')

    foreach ($file in $filesToCopy) {
        Copy-Item -Path $file.FullName -Destination "$ProjectRoot/Scripts/$($file.Name)"
    }
}

Export-ModuleMember -Function su