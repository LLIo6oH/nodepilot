# Commit Message Prompt

Generate a concise conventional commit message for NodePilot changes.

Rules:
- Format: `type(scope): summary`
- Types: `feat`, `fix`, `refactor`, `docs`, `chore`, `test`
- Scope must match changed area (e.g. `backend`, `frontend`, `infra`, `ai`).
- Summary: present tense, specific, <= 72 chars.
- Optional body: short bullets only when needed.

Example:
`feat(backend): add health route and shared app state`
