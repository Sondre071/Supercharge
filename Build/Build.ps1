$projectRoot = Split-Path $(Split-Path -Path $MyInvocation.MyCommand.Path -Parent) -Parent
Set-Location $projectRoot

Write-Host "Building rust code..." -ForegroundColor DarkGray
cargo build --target-dir target

Write-Host "Building go code..." -ForegroundColor DarkGray
$binPath = Join-Path '.' 'target' 'debug' 'bin'

go build `
    -o "$(Join-Path $binPath 'fetch_models.exe')" `
    "$(Join-Path '.' 'cmd' 'openrouter' 'fetch_models')"

go build `
    -o "$(Join-Path $binPath 'post_message.exe')" `
    "$(Join-Path '.' 'cmd' 'openrouter' 'post_message')"

cargo run
