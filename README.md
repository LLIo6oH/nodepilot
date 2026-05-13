## Backend (Rust + Axum)

### Environment

- `DATABASE_URL` (optional): SQLite connection string.
- Default: `sqlite://nodepilot.sqlite`
- `WORKSPACES_DIR` (optional): agent workspace root.
- Default: `../workspaces` (relative to `backend/`)

### Run backend

```bash
cd backend
cargo run
```

Backend API: `http://localhost:8080`

### Workspace behavior

Each agent gets:

- `workspaces/<agent_id>/`

Workspace is created automatically when provisioning starts or first chat tool interaction occurs.

### Supported tools

- `file.write`
- `file.read`
- `email.compose`
- `shell.run_limited` (`pwd`, `ls`, `whoami`, `date`, `uname` only)

## Frontend (Vue 3 + TypeScript + Vite)

### Run frontend

```bash
cd frontend
npm install
npm run dev
```

Frontend app: `http://localhost:5173`

To build production assets:

```bash
cd frontend
npm run build
```

### Demo flow

1. Open frontend and click `Create account & launch Atlas`.
2. Atlas is created via `POST /agents` and provisioning starts via `POST /agents/{id}/provision`.
3. Provisioning screen subscribes to `GET /agents/{id}/events` (SSE).
4. When Atlas is ready, open workspace and send chat prompts.
5. Chat calls `POST /agents/{id}/chat` and shows tool responses.

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

# Stream events (SSE)
curl -N http://localhost:8080/agents/<agent_id>/events

# Chat: file.write
curl -X POST http://localhost:8080/agents/<agent_id>/chat \
  -H "Content-Type: application/json" \
  -d '{"message":"Create a file named roadmap.txt with startup goals"}'

# Chat: file.read
curl -X POST http://localhost:8080/agents/<agent_id>/chat \
  -H "Content-Type: application/json" \
  -d '{"message":"Read roadmap.txt"}'

# Chat: email.compose
curl -X POST http://localhost:8080/agents/<agent_id>/chat \
  -H "Content-Type: application/json" \
  -d '{"message":"Write an interview follow-up email"}'

# Chat: shell.run_limited
curl -X POST http://localhost:8080/agents/<agent_id>/chat \
  -H "Content-Type: application/json" \
  -d '{"message":"Run pwd"}'
```
