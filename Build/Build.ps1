Write-Host "Building rust binary..." -ForegroundColor DarkGray
cargo build --target-dir target

Write-Host "Building go binaries..." -ForegroundColor DarkGray
go build -o target/debug/bin/models_request.exe ./cmd/models_request
go build -o target/debug/bin/stream_reader_request.exe ./cmd/stream_reader_request

cargo run