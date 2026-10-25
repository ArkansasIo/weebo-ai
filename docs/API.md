# API reference

- `GET /health` — service and Ollama status.
- `GET /v1/models` — configured model.
- `POST /v1/chat/completions` — chat.
- `GET /v1/tools` — tool list.
- `POST /v1/tools/calculate` — body `{"input":"2 + 3 * 4"}`.
- `POST /v1/tools/read-file` — body `{"input":"notes.txt"}`.
- `POST /v1/tools/write-file` — body `{"path":"notes.txt","content":"hello"}`.
- `POST /v1/tools/shell` — body `{"input":"cargo check"}`.

File reads/writes are restricted to the workspace. Shell and file writing are disabled by default.
