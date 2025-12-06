go build -o bin/models_request.exe ./cmd/models_request
go build -o bin/stream_reader_request.exe ./cmd/stream_reader_request

cargo run


# $targetPath = Join-Path $env:USERPROFILE '.supercharge'
# $binPath = Join-Path $targetPath 'bin'

# if (-not (Test-Path $targetPath)) { New-Item -ItemType Directory $targetPath }
# if (-not (Test-Path $binPath)) { New-Item -ItemType Directory $binPath }


# $openrouterBinPath = Join-Path $binPath 'openrouter.exe'
# go build -o $