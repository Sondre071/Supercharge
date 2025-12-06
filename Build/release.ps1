Write-Host "Compiling rust code..." -ForegroundColor DarkGray
cargo build --release --target-dir target

Write-Host "Compiling go code..." -ForegroundColor DarkGray
$binPath = Join-Path 'target' 'release' 'bin'

go build `
    -o "$(Join-Path $binPath 'models_request.exe')" `
    "$(Join-Path '.' 'cmd' 'models_request')"

go build `
    -o "$(Join-Path $binPath 'stream_reader_request.exe')" `
    "$(Join-Path '.' 'cmd' 'stream_reader_request')"


Write-Host "Copying binaries..." -ForegroundColor DarkGray
$releasePath = Join-Path '.' 'target' 'release'
$targetPath = Join-Path $env:USERPROFILE '.supercharge'

Write-Host "Done!" -ForegroundColor Green
Copy-Item -Path $(Join-Path $releasePath 'su.exe') -Destination $targetPath -Recurse -Force
Copy-Item -Path $(Join-Path $releasePath 'bin') -Destination $targetPath -Recurse -Force