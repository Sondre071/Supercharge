$projectRoot = Split-Path $(Split-Path -Path $MyInvocation.MyCommand.Path -Parent) -Parent
Set-Location $projectRoot

Write-Host "Compiling rust code..." -ForegroundColor DarkGray
cargo build --release --target-dir target

Write-Host "Compiling go code..." -ForegroundColor DarkGray
$binPath = Join-Path '.' 'target' 'release' 'bin'

go build `
    -o "$(Join-Path $binPath 'fetch_models.exe')" `
    "$(Join-Path '.' 'cmd' 'openrouter' 'fetch_models')"

go build `
    -o "$(Join-Path $binPath 'post_message.exe')" `
    "$(Join-Path '.' 'cmd' 'openrouter' 'post_message')"


Write-Host "Copying binaries..." -ForegroundColor DarkGray
$releasePath = Join-Path '.' 'target' 'release'
$targetPath = Join-Path $env:USERPROFILE '.supercharge'

Copy-Item -Path $(Join-Path $releasePath 'su.exe') -Destination $targetPath -Force
Copy-Item -Path $(Join-Path $releasePath 'bin') -Destination $targetPath -Recurse -Force

Write-Host "Done!" -ForegroundColor Green
