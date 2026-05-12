## Backend (Rust + Axum)

### Environment

- `DATABASE_URL` (optional): SQLite connection string.
- Default: `sqlite://nodepilot.sqlite`

### Run locally (default SQLite)

```bash
cd backend
cargo run
```

Backend API listens on `http://localhost:8080`.

### Verify health endpoint

```bash
curl http://localhost:8080/health
```
