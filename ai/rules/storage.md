# Storage Rules

- SQLite is the MVP database.
- PostgreSQL + pgvector is the intended production evolution.
- Do not put SQL directly in HTTP handlers.
- Keep migrations/init logic centralized in the storage module.
- Keep storage APIs explicit and small.
- Prefer simple SQL over ORM-like abstractions.
- Store timestamps as RFC3339 strings or SQLite-compatible text consistently.
- Use UUIDs as text IDs.
- Do not store secrets in plaintext.
- Keep schema MVP-focused.
