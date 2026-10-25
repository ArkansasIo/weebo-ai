# Weebo AI

Weebo AI is a local computer-AI platform written in Rust and powered by Ollama.

## Features

- Ollama chat integration
- REST API with OpenAI-style chat request shape
- SQLite conversation persistence
- Calculator tool
- Workspace file reader and opt-in writer
- Opt-in shell command runner
- Browser chat interface
- Health/model endpoints
- PowerShell setup, run, and test scripts
- GitHub Actions CI build/test workflow

## Requirements

- Rust toolchain
- Ollama installed and running

## Quick start (Windows PowerShell)

```powershell
Copy-Item .env.example .env
ollama pull llama3.2
cargo run
```

Open http://127.0.0.1:8080/.

## API

- `GET /health`
- `GET /v1/models`
- `POST /v1/chat/completions`
- `GET /v1/tools`
- `POST /v1/tools/calculate`
- `POST /v1/tools/read-file`
- `POST /v1/tools/write-file`
- `POST /v1/tools/shell`

Chat request example:

```json
{"messages":[{"role":"user","content":"Hello Weebo"}]}
```

Calculator request example:

```json
{"input":"10 * 4 + 2"}
```

## Security defaults

Shell execution and file writing are disabled by default. File operations are intended to stay inside the configured workspace. Do not expose this development API to an untrusted network. Before production use, add authentication, OS-level sandboxing, request limits, rate limits, and audit logging.

## Layout

- `src/api.rs`: HTTP routes
- `src/ollama.rs`: local Ollama provider
- `src/memory.rs`: SQLite conversation persistence
- `src/tools.rs`: calculator/filesystem/shell tools
- `src/config.rs`: environment settings
- `migrations`: database schema
- `web`: chat UI
- `docs`: architecture, API, roadmap
