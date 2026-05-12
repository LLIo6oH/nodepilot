# AGENTS.md

## Purpose
NodePilot is a cloud-resident personal AI agent platform, inspired by systems like SkyKoi. This repository is optimized for a fast MVP vertical slice and clear boundaries between system planes.

## Repository Structure
- `backend/` Rust + Axum services for API, orchestration, and control-plane logic.
- `frontend/` Vue 3 + TypeScript UI for operator/user interactions.
- `infra/` Docker Compose and deployment artifacts for AWS EC2 free-tier.
- `docs/` Architecture and design decisions (`docs/architecture.md` is the source of truth).
- `workspaces/` Runtime workspace data and scratch state (non-critical artifacts).

## Architectural Principles
- Separate responsibilities by plane:
  - Control plane: identity, policy, orchestration, task lifecycle.
  - Runtime plane: agent/job execution, queues, ephemeral state.
  - Tool plane: integrations, external API/tool adapters, side-effect boundaries.
- Keep inter-plane contracts explicit and versioned (DTOs/events first, implementation second).
- Favor narrow interfaces and composable modules over early abstraction.
- Default to stateless services; persist only what is required for recovery/audit.

## Runtime Safety Constraints
- No implicit privileged execution. All tool actions must be explicit and auditable.
- Treat all external tool input/output as untrusted; validate and sanitize at boundaries.
- Enforce timeouts, cancellation, and bounded retries on all outbound operations.
- Prevent cross-workspace data leakage; scope data access by workspace/tenant ID.
- Never commit secrets, tokens, or private keys. Use environment-based secret injection.

## Infrastructure & Deployment Expectations
- Primary target: AWS EC2 free-tier single-host deployment using Docker Compose.
- Services must be container-first, reproducible, and bootable with one compose command.
- Prefer low-memory, low-CPU defaults; free-tier resource limits are a hard constraint.
- Health/readiness checks are required for backend services.
- Logging should be structured and stdout-first for simple host-level aggregation.

## Coding Conventions
- Rust (backend):
  - Use stable Rust and idiomatic Axum patterns.
  - Return typed errors; avoid `unwrap()` in request/runtime paths.
  - Keep modules small; isolate side effects behind traits/adapters.
- Vue + TypeScript (frontend):
  - Composition API with strict TypeScript types.
  - Keep business logic out of components when possible (use composables/services).
  - Prefer explicit API client types shared from backend contracts where feasible.
- General:
  - Write minimal, high-signal comments (focus on intent/constraints).
  - Add/update tests for changed behavior when practical in MVP scope.
  - Keep PRs small and reviewable.

## Scope Constraints (MVP)
- MVP is an **8-hour vertical slice**: prioritize end-to-end flow over completeness.
- Build only what is needed to demonstrate one reliable agent task lifecycle.
- Defer non-critical features (multi-region, advanced authz, deep observability, autoscaling).
- Prefer simple in-process or single-node designs unless complexity is essential.

## Instructions for AI Coding Agents
- Before coding:
  - Read `docs/architecture.md` and align with plane separation.
  - Confirm the change fits the 8-hour MVP scope.
- While coding:
  - Preserve control/runtime/tool plane boundaries; do not collapse them for convenience.
  - Make the smallest change that delivers observable value.
  - Avoid broad refactors unless required to unblock the slice.
  - If assumptions are needed, document them briefly in code or PR notes.
- Safety and ops:
  - Do not add secrets to code, logs, fixtures, or commits.
  - Add timeouts/retry limits to network/tool calls.
  - Ensure new services are runnable through Docker Compose on EC2 free-tier constraints.
- Handoff quality:
  - Include a short verification checklist (build, run, basic smoke path).
  - List known gaps explicitly rather than silently expanding scope.

## Definition of Done (MVP-friendly)
- Feature works end-to-end in local Docker Compose.
- Core failure paths are handled (timeout/error surfaced clearly).
- Changes respect architecture boundaries and do not introduce obvious security regressions.
- Docs/config are updated enough for the next engineer or agent to continue quickly.
