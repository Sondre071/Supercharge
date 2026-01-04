Add-Type -AssemblyName System.Windows.Forms

$clear_line = "`r`e[2K";

$yellow = "`e[0;93m"
$white = "`e[0;97m"
$gray = "`e[0;37m"

$browser = [System.Windows.Forms.FolderBrowserDialog]::new()
$browser.ShowDialog() | Out-Null
$path = $browser.SelectedPath;

if ($path -eq '') {
    Write-Host "No folder selected." -ForegroundColor Yellow
}

Write-Host "Searching for duplicates`n" -ForegroundColor Yellow

$files_number = Get-ChildItem -Path $path -File -Recurse `
| Measure-Object `
| Select-Object -ExpandProperty Count

$current_number = 0;

$hashes = Get-ChildItem -Path $path -File -Recurse `
| ForEach-Object {
    $current_number++;

    $text = "{0}Hashing file {1}/{2}: {3}{4}{5}" -f $yellow, $current_number, $files_number, $white, $_.FullName, $gray;

    $formatted_text = "{0}{1}" -f $clear_line, $text;

    Write-Host $formatted_text -NoNewLine

    Get-FileHash -Path $_.FullName -Algorithm SHA256
} `
| Group-Object -Property Hash `
| Where-Object { $_.Count -gt 1 }

Write-Host $("{0}" -f $clear_line) -NoNewLine

if ($hashes.Count -gt 0) {
    Write-Host "Duplicates found!`n" -ForegroundColor Yellow

    foreach ($item in $hashes) {
        Write-Host "Hash: $($item.Name)" -ForegroundColor Yellow;
    
        ForEach ($path in $item.Group.Path) {
            Write-Host "$path" -ForegroundColor White;
        }

        Write-Host
    }
} else {
    Write-Host "No duplicates!" -ForegroundColor Green
}
