param (
    [string]$ProjectRoot
)

$dataFolder = "$ProjectRoot/data"
$currentFolder = ''

while ($true) {
    $path = Join-Path $dataFolder $currentFolder

    $files = Get-ChildItem `
        -Path $path `
    | Select-Object Name, PSIsContainer, @{
        Name       = 'FormattedName'
        Expression = { if ($_.PSIsContainer -eq $True) { "📂 $($_.Name)" } else { "📝 $($_.Name)" } }
    } | Sort-Object FormattedName

    $choice = Read-Menu -Options ($files | Select-Object -ExpandProperty FormattedName)  -ExitOption '❌ Back'

    $selectedFile = $files | Where-Object { $_.FormattedName -eq $choice }

    switch ($choice) {
        '❌ Back' {
            if ($currentFolder -eq '') { return }

            $currentFolder = Split-Path -Parent $currentFolder
        }
        default {

            if ($selectedFile.PSIsContainer -eq $true) {
                $currentFolder = $selectedFile.Name
            }
            else {
                $filePath = Join-Path $dataFolder $currentFolder $selectedFile.Name
                nvim $filePath
            }
        }
    }

}
