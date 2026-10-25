$ErrorActionPreference = "Stop"
if (-not (Get-Command ollama -ErrorAction SilentlyContinue)) { throw "Ollama was not found on PATH. Install Ollama first." }
if (-not (Test-Path ".env")) { Copy-Item ".env.example" ".env" }
New-Item -ItemType Directory -Force "workspace" | Out-Null
ollama pull llama3.2
if ($LASTEXITCODE -ne 0) { throw "Could not pull Ollama model." }
cargo check
Write-Host "Weebo AI setup complete."
