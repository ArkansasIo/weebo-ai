$ErrorActionPreference = "Stop"
$env:RUST_LOG = "weebo_ai=info,tower_http=info"
cargo run
