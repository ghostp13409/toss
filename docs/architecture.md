# Toss Architecture Specification

This document outlines the architectural design for **Toss**, ensuring scalability for a complex TUI and a robust CLI.

## 1. Directory Structure

```text
src/
├── main.rs          # Entry point, dispatches to CLI or TUI
├── cli/             # CLI-specific logic (clap definitions, runners)
│   ├── mod.rs
│   └── args.rs      # Command-line argument structures
├── tui/             # TUI-specific logic (Ratatui)
│   ├── mod.rs
│   ├── app.rs       # Main Application state and event loop
│   ├── ui.rs        # UI rendering logic (layout, panels)
│   └── input/       # Vim-inspired input handling (state machine)
├── engine/          # Core request logic (protocol agnostic)
│   ├── mod.rs
│   └── http.rs      # reqwest wrapper
├── core/            # Business logic and shared models
│   ├── mod.rs
│   ├── env.rs       # Environment variable management
│   ├── collection.rs # Request/Folder tree structures
│   └── config.rs    # User configuration (themes, keybinds)
└── utils/           # Shared utility functions
    └── mod.rs
```

## 2. Core Architectural Principles

### A. Separation of Concerns
- **Engine**: Purely responsible for network I/O. It should not know about the TUI or CLI.
- **Core**: Contains the "Source of Truth" (Collections, Envs, Config).
- **View Layers (CLI/TUI)**: Consumer layers that use the Core and Engine.

### B. State Management (TUI)
- Use a **Single Source of Truth** in `App` state.
- Implement an `Action` system or a clean `match` loop for state transitions (Normal -> Editing -> Command).

### C. Resource Lifecycle
- **Tokio** handles the async runtime for the Engine.
- **Crossterm** handles the terminal raw mode and event polling for the TUI.

## 3. Data Flow

1. **Input**: User triggers a command (CLI) or a keypress (TUI).
2. **Logic**: The Core layer processes variables/environments.
3. **Execution**: The Engine sends the request via `reqwest`.
4. **Output**: The View layer (CLI/TUI) formats and displays the response.
