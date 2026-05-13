## Backend (Rust + Axum)

### Environment

- `DATABASE_URL` (optional): SQLite connection string.
- Default: `sqlite://nodepilot.sqlite`
- `WORKSPACES_DIR` (optional): agent workspace root.
- Default: `../workspaces` (relative to `backend/`)

### Run locally

```bash
cd backend
cargo run
```

Backend API listens on `http://localhost:8080`.

### Workspace behavior

Each agent gets a workspace directory:

- `workspaces/<agent_id>/`

Workspace is created automatically on:

- provisioning start (`POST /agents/{id}/provision`), or
- first chat/tool interaction (`POST /agents/{id}/chat`)

### Supported tools (deterministic routing)

- `file.write`: create/update files in agent workspace.
- `file.read`: read files from agent workspace.
- `email.compose`: create drafts under `drafts/email_<timestamp>.md`.
- `shell.run_limited`: allowlist only `pwd`, `ls`, `whoami`, `date`, `uname`.

### API examples

```bash
# Health
curl http://localhost:8080/health

# Create agent
curl -X POST http://localhost:8080/agents \
  -H "Content-Type: application/json" \
  -d '{"name":"Atlas"}'

# List agents
curl http://localhost:8080/agents

# Start provisioning
curl -X POST http://localhost:8080/agents/<agent_id>/provision

# Stream lifecycle/tool events (SSE)
curl -N http://localhost:8080/agents/<agent_id>/events

# Chat: file write
curl -X POST http://localhost:8080/agents/<agent_id>/chat \
  -H "Content-Type: application/json" \
  -d '{"message":"create a file named roadmap.txt"}'

# Chat: file read
curl -X POST http://localhost:8080/agents/<agent_id>/chat \
  -H "Content-Type: application/json" \
  -d '{"message":"read roadmap.txt"}'

# Chat: email compose
curl -X POST http://localhost:8080/agents/<agent_id>/chat \
  -H "Content-Type: application/json" \
  -d '{"message":"write an interview follow-up email"}'

# Chat: limited shell
curl -X POST http://localhost:8080/agents/<agent_id>/chat \
  -H "Content-Type: application/json" \
  -d '{"message":"pwd"}'
```
