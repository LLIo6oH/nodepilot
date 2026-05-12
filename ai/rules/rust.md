# Rust Rules

- Default to async-first service code.
- Keep route handlers thin; delegate logic to modules.
- Use explicit error handling and meaningful error types.
- Avoid unnecessary traits/generics in MVP code paths.
- Keep modules small and responsibility-driven.
- Keep business logic outside transport handlers.
