$ErrorActionPreference = "Stop"
cargo fmt --all -- --check
cargo test
cargo check
