# NodePilot Submission Notes

## 1. Overview
NodePilot is an original AI agent runtime MVP built for the SkyKoi Senior Platform Engineer take-home. It implements a personal-agent workflow where a user launches Atlas, provisions an isolated runtime session, and interacts with it through a web workspace.

## 2. Live Links
- Technical overview: http://34.224.168.109/
- Demo app: http://34.224.168.109/app
- Health check: http://34.224.168.109/api/health
- Repository: https://github.com/LLIo6oH/nodepilot
- Architecture diagram: available on the technical overview landing page

## 3. What I Built
I built a full-stack MVP with a clear end-to-end flow:
open app -> launch Atlas -> provision runtime -> enter workspace/chat -> execute tools.

Implemented capabilities include:
- Agent creation and lifecycle events
- Provisioning flow with runtime readiness handoff
- Docker-backed runtime container per provisioned agent
- Workspace-scoped file tools (`file.write`, `file.read`)
- Runtime-scoped shell tool (`shell.run_limited`)
- Simulated `email.compose`
- Demo account/authentication flow for entering the product experience
- Submission landing page at `/` and interactive demo at `/app`

## 4. Architecture and Design Decisions
- Frontend: Vue 3 + TypeScript + Vite
- Backend: Rust + Axum + Tokio
- API integration: nginx serves frontend and proxies `/api/*` same-origin to backend
- Persistence: SQLite (MVP)
- Packaging/deploy: Docker + Docker Compose on AWS EC2
- Runtime design: one Docker runtime session per provisioned agent
- Isolation model: workspace per agent at `/workspaces/{agent_id}`

This design keeps the core runtime architecture explicit while remaining feasible within free-tier constraints.

## 5. Runtime Provisioning Flow
User clicks **Launch Atlas**
-> backend creates agent record
-> backend creates `/workspaces/{agent_id}`
-> backend starts `nodepilot-runtime-{agent_id}`
-> runtime mounts shared workspaces volume
-> tools are registered
-> user enters workspace/chat
-> shell commands execute inside `/workspaces/{agent_id}`

Technical proof points implemented:
- Provisioning starts a real container named `nodepilot-runtime-{agent_id}`
- `Run pwd` returns `/workspaces/{agent_id}`
- `Run rm -rf /` is blocked by safety policy

## 6. Strategy and Prioritization
I prioritized the highest-value platform core first:
- agent runtime lifecycle
- isolated execution environment
- tool safety boundary
- live cloud deployment
- coherent product UX

Deferred items were intentionally scoped out to avoid shallow breadth and keep implementation depth on runtime architecture.
I treated authentication as demo-level for this submission and focused deeper implementation effort on runtime provisioning and tool execution.

## 7. Tools and Services Used
- Rust
- Axum
- Tokio
- SQLx / SQLite
- Vue 3
- TypeScript
- Vite
- nginx
- Docker
- Docker Compose
- AWS EC2
- EBS
- AWS Security Groups

## 8. Key Challenges and Solutions
- **Originality vs platform parity**: built a distinct NodePilot brand/UX while preserving core agent-runtime product concepts.
- **Real provisioning vs simulated UI**: connected provisioning to real Docker runtime startup and lifecycle events.
- **Per-agent execution context**: routed shell execution into per-agent containers rather than backend process context.
- **Free-tier compatibility**: used single-host EC2 + Docker Compose with isolated runtime containers.
- **Shell safety**: enforced allowlist/blocklist boundaries for dangerous commands.
- **Operational simplicity**: packaged full stack with reproducible compose deployment.

## 9. Straightforward vs Difficult
**Straightforward**
- Basic full-stack wiring
- Health checks
- Docker Compose packaging
- Route split between submission landing (`/`) and demo app (`/app`)

**Difficult**
- Choosing practical runtime isolation level for MVP
- Implementing Docker-backed per-agent runtimes without overbuilding orchestration
- Aligning UX messaging with real provisioning behavior
- Preserving safe shell boundaries while keeping it useful

## 10. Reverse-Engineered vs Inferred
From SkyKoi’s public product surface, I inferred core concepts: personal agent, channels/tools, runtime isolation, dashboard UX, and security posture.

I did not clone SkyKoi visual identity. NodePilot is an original implementation of the same platform idea. Some production architecture details were inferred because SkyKoi internals are not public.

## 11. Assumptions
- A “cloud compute instance” can be represented in MVP as an isolated Docker runtime session on AWS EC2.
- Free-tier constraints favor a single EC2 host over per-user EC2/ECS tasks.
- Demo-level auth is acceptable in take-home scope if production auth path is explicit.
- Runtime architecture depth is more valuable than broad but shallow integrations in one-week scope.

## 12. Deviations and Improvements
- Used per-agent Docker runtime sessions instead of separate EC2 instance per user.
- Added a technical submission landing page for reviewer clarity.
- Added provisioning UX and runtime-oriented workspace dashboard.
- Deferred full MCP integrations, but kept modular tool-router architecture to support them.

## 13. Production Next Steps
- Real authentication/session management
- Runtime orchestration via ECS/Fargate, Firecracker, or dedicated runtime orchestrator
- MCP tool registry and permissions model
- Gmail/Notion/Calendar integrations
- Persistent memory + vector storage
- Cleanup jobs for stale runtimes
- HTTPS + domain
- Observability (metrics, tracing, alerting)
- Billing/token analytics
- Voice and messaging channels

## 14. How to Verify
Live verification:
- Visit http://34.224.168.109/
- Open demo at http://34.224.168.109/app
- Health check:
```bash
curl http://34.224.168.109/api/health
```
- In UI run:
  - `Run pwd`
  - `Run rm -rf /`
  - `Create a file named roadmap.txt with startup goals`
  - `Read roadmap.txt`

Local verification:
```bash
docker compose up -d --build
curl http://localhost/api/health
```
