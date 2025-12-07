$projectRoot = Split-Path $(Split-Path -Path $MyInvocation.MyCommand.Path -Parent) -Parent
Set-Location $projectRoot

$rustBinaryPath = Join-Path $projectRoot 'target' 'release' 'su.exe'

$binPath = Join-Path $projectRoot 'cmd'
$targetDirectory = Join-Path $env:USERPROFILE '.supercharge'
$targetBin = Join-Path $targetDirectory 'bin'

# Compile Rust.
Write-Host "Compiling rust code..." -ForegroundColor DarkGray
cargo build --release --target-dir target

# Copy Rust binary over.
Copy-Item -Path $rustBinaryPath -Destination $targetDirectory -Force

# Compile Go.
Write-Host "Compiling go code..." -ForegroundColor DarkGray

Get-ChildItem -Path $binPath -File -Recurse | ForEach-Object {
    $name = $_.Directory.Name
    $packagePath = Split-Path $_.FullName -Parent
    $domainPath = Split-Path $packagePath -Parent
    $domainName = Split-Path $domainPath -Leaf

    Write-Host "$domainName/$name.exe" -ForegroundColor DarkGray

    go build `
    -o "$(Join-Path $targetBin $domainName $name).exe" `
    $packagePath
}