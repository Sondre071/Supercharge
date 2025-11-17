Set-StrictMode -Version Latest

$ProjectRoot = $PSScriptRoot

function su($Command) {

    Confirm-LocalFiles

    [psobject[]]$options = Get-ChildItem `
        -Path (Join-Path $ProjectRoot 'scripts') `
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
    $path = Join-Path $ProjectRoot 'data'

    if (-not (Test-Path $path)) {
        New-Item -Path $path -ItemType Directory
    }

    if (-not (Test-Path (Join-Path $path 'config.json'))) {
        $configContent = [psobject]@{
            OpenRouter = [psobject]@{
                ApiKey        = ''
                ApiUrll       = ''
                CurrentModel  = ''
                CurrentPreset = ''
            }
        }

        $jsonContent = $configContent | ConvertTo-Json -Depth 3

        New-Item -Path (Join-Path $path 'config.json') -ItemType File -Value $jsonContent
    }

    $filesToCopy = Get-ChildItem `
        -Path (Join-Path $ProjectRoot 'BuildScripts')

    foreach ($file in $filesToCopy) {
        Copy-Item -Path $file.FullName -Destination "$ProjectRoot/Scripts/$($file.Name)"
    }
}

Export-ModuleMember -Function su