# Architecture Rules

- Frontend remains presentation-focused.
- Runtime and orchestration logic belong to backend services.
- Tool execution must go through the tool layer.
- Orchestrator owns task lifecycle and state transitions.
- Runtime abstraction must remain provider-independent.
