## NodePilot MVP

NodePilot is a cloud-resident personal AI runtime prototype with:
- Rust + Axum backend
- Vue 3 + TypeScript frontend
- SQLite persistence
- Atlas agent provisioning lifecycle
- SSE runtime events
- deterministic tool-routed chat
- Level 2 runtime sessions: one Docker runtime container per Atlas agent (MVP)

## Local Development

### Terminal 1 (backend)

```bash
cd backend && cargo run
```

Backend URL: `http://localhost:8080`

### Terminal 2 (frontend)

```bash
cd frontend && npm install
cd frontend && npm run dev
```

Frontend URL: `http://localhost:5173`

## Build Checks

```bash
cd backend && cargo check
cd frontend && npm run build
```

## Docker Production-Style Run

Run full stack:

```bash
docker compose up --build
```

URLs:
- Frontend: `http://localhost`
- Backend via frontend reverse proxy: `http://localhost/api/health`
- Direct backend debug URL (if needed): `http://localhost:8080/health`

Runtime notes:
- Backend orchestrates per-agent runtime containers through `/var/run/docker.sock`.
- Runtime image tag: `nodepilot-runtime:latest`.
- Shared workspace volume: `nodepilot-workspaces`, mounted at `/workspaces` in backend and runtime containers.
- Agent working directory inside runtime: `/workspaces/{agent_id}`.
- Docker socket access is MVP-only and should be replaced by a safer runtime control-plane in production.

Stop services:

```bash
docker compose down
```

Reset persisted volumes (SQLite + workspaces):

```bash
docker compose down -v
```

### Deployment Note (EC2 + DuckDNS)

When deployed on EC2 and mapped to DuckDNS, app traffic should use one origin:
- App: `http://nodepilot-demo.duckdns.org`
- API through nginx proxy: `http://nodepilot-demo.duckdns.org/api/health`

## Demo Flow

1. Launch frontend at `http://localhost:5173` (local dev) or `http://localhost` (Docker).
2. Click `Create account & launch Atlas`.
3. Watch provisioning updates from SSE event stream.
4. Wait for automatic transition to workspace when Atlas is online.
5. Try prompts:
- `Create roadmap.txt`
- `Read roadmap.txt`
- `Write follow-up email`
- `Run pwd`
- `Try blocked shell command` (`Run rm -rf /`)

Verify runtime container provisioning:

```bash
docker ps | grep nodepilot-runtime
curl http://localhost/api/health
```

Expected shell behavior from UI:
- `Run pwd` returns `/workspaces/<agent_id>` (from inside the runtime container when `RUNTIME_MODE=docker`).
- `Run rm -rf /` remains blocked by safety policy.

## Backend API (direct backend paths)

- `GET /health`
- `POST /agents`
- `POST /agents/{id}/provision`
- `GET /agents/{id}/events`
- `POST /agents/{id}/chat`

When using Docker frontend proxy, these are called as `/api/*` from browser.

## Supported Tool Routing

- `Create a file named roadmap.txt with startup goals` -> `file.write`
- `Read roadmap.txt` -> `file.read`
- `Write an interview follow-up email` -> `email.compose`
- `Run pwd` -> `shell.run_limited`
- `Run rm -rf /` -> blocked with error

## Known Limitations

- Demo auth only (no real authentication provider).
- Provisioning is simulated.
- Workspace is local sandbox storage.
- Tool routing is deterministic (no LLM planner yet).
- No real MCP / Gmail / OAuth integrations yet.
- No real per-agent EC2 runtime yet.
