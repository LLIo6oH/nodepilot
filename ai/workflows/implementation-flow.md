# Implementation Flow

## Stages
1. Planner
2. Implementer
3. Reviewer
4. Commit Generator

## Flow
1. Planner analyzes the task, defines exact MVP scope, and lists affected modules.
2. Planner proposes concrete implementation steps and risks; no code is written.
3. Implementer executes only approved scope using repo rules and architecture constraints.
4. Reviewer checks boundaries, complexity, runtime safety, and MVP alignment.
5. Commit Generator produces a concise conventional commit message from final changes.

## Guardrails
- Stop scope expansion unless explicitly approved.
- Reject unnecessary abstractions and speculative work.
- Prefer one end-to-end vertical slice over partial multi-feature progress.
