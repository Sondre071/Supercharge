$projectRoot = Split-Path $(Split-Path -Path $MyInvocation.MyCommand.Path -Parent) -Parent

if ($projectRoot -ne (Get-Location).Path) {
    Write-Host "Script has to be invoked from within the project root."
    return
}


Write-Host "Building rust code..." -ForegroundColor DarkGray
cargo build --target-dir target

Write-Host "Building go code..." -ForegroundColor DarkGray
$binPath = Join-Path 'target' 'debug' 'bin'

go build `
    -o "$(Join-Path $binPath 'models_request.exe')" `
    "$(Join-Path '.' 'cmd' 'models_request')"

go build `
    -o "$(Join-Path $binPath 'stream_reader_request.exe')" `
    "$(Join-Path '.' 'cmd' 'stream_reader_request')"

cargo run