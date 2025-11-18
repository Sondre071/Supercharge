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

    $choice = Read-Menu -Header 'Supercharge' -Options ($options) -ExitOption 'Exit'

    if ($choice -eq 'Exit') { return }
    
    . $choice.Path -ProjectRoot $ProjectRoot
}

function Confirm-LocalFiles() {
    $foldersToCopy = Get-ChildItem `
        -Path (Join-Path $ProjectRoot 'BuildScripts') `
        -Directory

    foreach ($folder in $foldersToCopy) {
        $categoryName = Split-Path $folder -Leaf

        New-Item -Path (Join-Path $ProjectRoot 'Scripts' 'Helpers' $categoryName) -ItemType Directory -Force | Out-Null

        # Copy helper scripts.
        Get-ChildItem (Join-Path $folder.FullName 'Helpers') -File -Recurse | ForEach-Object { Copy-Item -Path $_.FullName -Destination (Join-Path $ProjectRoot 'Scripts' 'Helpers' $categoryName $_.Name) -Recurse -Force }

        # Copy main scripts.
        Get-ChildItem $folder.FullName -File | ForEach-Object { Copy-Item -Path $_.FullName -Destination (Join-Path $ProjectRoot 'Scripts') }
    }
}

Export-ModuleMember -Function su