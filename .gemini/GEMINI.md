# Toss Project Guidelines

## UI & Navigation Design
- **Lazydocker-inspired**: The TUI must visually resemble lazydocker (Left sidebar for lists, Right main area for context/details).
- **Drill-Down Layers**: Navigation must follow the logical layers (Collections -> Properties -> Details).
- **Shortcuts**: Use `Enter`/`l` to drill down, and `Esc`/`h` to pop up. Avoid treating all panels as equal peers cycleable by `Tab` only.

## Architecture & Structure
- Rigorously adhere to the modular architecture defined in `docs/architecture.md`.
- Ensure a strict separation of concerns between `engine`, `core`, `cli`, and `tui`.
- Do not add business logic to `main.rs`; use it only as a thin entry point for dispatching.

## Testing Protocol
- After each testable implementation, do not perform the verification tests yourself.
- Instead, provide the user with clear, step-by-step instructions (commands and expected outcomes) to test the implementation manually.
- Wait for user confirmation or feedback before proceeding to the next task.
