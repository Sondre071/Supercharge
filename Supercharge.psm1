Set-StrictMode -Version Latest

$ProjectRoot = $PSScriptRoot

function su() {

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

    $choice = Read-Menu -Header 'Supercharge' -Options $options -ExitOption 'Exit'

    if ($choice -eq 'Exit') { return }

    . $choice.Path -ProjectRoot $ProjectRoot
}

function Confirm-LocalFiles() {
    $scripts = Get-ChildItem ` -Path (Join-Path $ProjectRoot 'BuildScripts') ` -File

    if (-not (Test-Path (Join-Path $ProjectRoot 'Scripts'))) {
        New-Item `
            -Path (Join-Path $ProjectRoot 'Scripts') `
            -ItemType Directory `
        | Out-Null
    }

    foreach ($script in $scripts) {
        $destination = Join-Path $ProjectRoot 'Scripts' $script.Name

        Copy-Item `
            -Path $script.FullName `
            -Destination $destination `
            -Force
    }

    $scriptHelpers = Get-ChildItem `
        -Path (Join-Path $ProjectRoot 'BuildScripts' 'Helpers') `
        -File `
        -Recurse

    if (-not (Test-Path (Join-Path $ProjectRoot 'Scripts' 'Helpers'))) {
        New-Item `
            -Path (Join-Path $ProjectRoot 'Scripts' 'Helpers') `
            -ItemType Directory `
        | Out-Null
    }


    foreach ($script in $scriptHelpers) {
        if (-not (Test-Path (Join-Path $ProjectRoot 'Scripts' 'Helpers' $script.Directory.Name))) {
            New-Item `
                -Path (Join-Path $ProjectRoot 'Scripts' 'Helpers' $script.Directory.Name) `
                -ItemType Directory `
            | Out-Null
        }

        $destination = Join-Path $ProjectRoot 'Scripts' 'Helpers' $script.Directory.Name $script.Name

        Copy-Item `
            -Path $script.FullName `
            -Destination $destination `
            -Force
    }
}

Export-ModuleMember -Function su