# Weebo AI Architecture

- **API layer:** Axum routes for chat, health, models, and tools.
- **Model layer:** Ollama provider isolates model HTTP calls from the rest of the application.
- **Memory layer:** SQLite stores conversation messages.
- **Tool layer:** calculator, workspace file operations, optional shell.
- **UI layer:** static browser chat at `/`.

## Safety

Shell execution and file writing are disabled unless explicitly enabled in environment configuration. Keep the API bound to localhost during development. Before exposing it to other machines, add authentication, per-tool authorization, rate limiting, audit logging, and OS-level isolation.
