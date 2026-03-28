# Roadmap

Developing **Toss** requires a structured approach to balance its high-performance Rust backend with a complex, Vim-inspired TUI. This roadmap breaks the project into logical milestones, ensuring all "Power User" and "Vim-first" requirements are met.

---

## Phase 1: Foundation & The "Request Engine" (Completed)

Build a functional CLI-first tool to handle core networking before adding the TUI layer.

- **Project Scaffolding**: Initialize Rust project with `tokio` async runtime.
- **CLI Argument Parsing**: Implement `clap` for `toss send` and global flags.
- **HTTP Core**: Integrate `reqwest` for `GET`, `POST`, `PUT`, `PATCH`, and `DELETE`.
- **Environment System**: Basic JSON/YAML parser for `{{variable}}` substitution.

## Phase 2: The TUI Skeleton & Layered Navigation

Focus on the visual layout and the core "Vim-first" state machine, adopting a lazydocker-inspired design.

- **Terminal Loop**: Set up `crossterm` and `ratatui` for raw terminal mode and rendering.
- **Layered Layout**: Implement the 30/70 split screen with drill-down layers (Collections -> Properties -> Details).
- **Vim State Machine**: Implement `InputMode` (Normal, Editing, Command) to handle different keyboard contexts.
- **Drill-Down Navigation**: Implement the logic to shift focus deeper (`Enter`/`l`) or pop up (`Esc`/`h`) between logical layers.
- **Command Mode (`:`)**: Build the bottom-bar command line for quick actions like `:set env` or `:save`.

## Phase 3: Data Management, CRUD & Multi-format Imports

Organize requests into a persistent tree structure with full management capabilities.

- **Tree Implementation**: Build the Collections and APIs panels with nested folder support.
- **CRUD Operations**: Implement `a` (Add), `r` (Rename), and `d` (Delete) functionality within the trees.
- **Search & Filter (`/`)**: Add real-time tree filtering for large collections.
- **Multi-format Support**: Integrate `postman_collection` and build parsers for **Insomnia** and **Swagger/OpenAPI**.
- **Persistence Layer**: Implement local storage (JSON or SQLite) for history and collections.

## Phase 4: Advanced REST, Highlighting & Editor Integration

Transform the tool into a high-end developer environment.

- **Properties Implementation**: Add all listed properties to the properties panel with proper input handling. (Params, Headers, Auth, Body, Scripts)
- **Authentication Suites**: Full support for Bearer, Basic, API Keys, and OAuth1-2.
- **Rich Body & Beautification**: Support for `form-data`, `x-www-form-urlencoded`, and JSON/GraphQL.
- **Syntax Highlighting**: Integrate `syntect` for both the Response and Body (Selected Prop) panels.
- **External Editor (`v`)**: Allow opening the request body in the user's system `$EDITOR` (e.g., Neovim).
- **Response Stats**: Real-time calculation of response time, size, and protocol details.

## Phase 5: CLI Mode, Automation & Scripting

Expand the CLI for CI/CD and implement the scripting engine.

- **`toss run` Subcommand**: A high-performance collection runner (Postman's _Newman_ equivalent).
- **Scripting Engine**: Integrate a JavaScript runtime (e.g., `deno_core` or `boa`) for pre-request/post-response logic.
- **Result Reporting**: Summary outputs and exit codes for automated testing pipelines.

## Phase 6: Configuration, Optimization & Polish

Final tuning for performance and user customization.

- **Full Configuration System**: Move themes, layouts, and global settings to a YAML/TOML config file.
- **Dynamic Themes**: Support for terminal colors and custom TUI styling.
- **Performance Tuning**: Optimize `syntect` rendering and tree traversal for massive collections.
- **Cross-Platform Validation**: Ensure consistent behavior across Windows, Linux, and future macOS support.

---

### Project Complexity Overview

| Phase | Main Focus  | Risk Level | Primary Crate                  |
| :---- | :---------- | :--------- | :----------------------------- |
| **1** | Networking  | Low        | `reqwest`                      |
| **2** | Vim-UX / UI | Medium     | `ratatui`                      |
| **3** | Tree / Data | High       | `serde` / `postman_collection` |
| **4** | Editor / DX | Medium     | `syntect`                      |
| **5** | Scripting   | High       | `deno_core` / `boa`            |
