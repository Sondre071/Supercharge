$projectRoot = Split-Path $(Split-Path -Path $MyInvocation.MyCommand.Path -Parent) -Parent
Set-Location $projectRoot

Write-Host "Compiling rust code..." -ForegroundColor DarkGray
cargo build --target-dir target

Write-Host "Compiling go code..." -ForegroundColor DarkGray

$binPath = Join-Path $projectRoot 'cmd'
$binTargetPath = Join-Path $projectRoot 'target' 'debug' 'bin'

Get-ChildItem -Path $binPath -File -Recurse | ForEach-Object {
    $name = $_.Directory.Name
    $packagePath = Split-Path $_.FullName -Parent
    $domainPath = Split-Path $packagePath -Parent
    $domainName = Split-Path $domainPath -Leaf

    Write-Host "$domainName/$name.exe" -ForegroundColor DarkGray

    go build `
    -o "$(Join-Path $binTargetPath $domainName $name).exe" `
    $packagePath
}

cargo run
