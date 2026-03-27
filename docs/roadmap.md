# Roadmap

Developing **Toss** requires a structured approach to balance its high-performance Rust backend with a complex, Vim-inspired TUI. This roadmap breaks the project into logical milestones, starting with the core engine and ending with the distribution-ready CLI.

---

## Phase 1: Foundation & The "Request Engine"

The first goal is to build a functional CLI-first tool that can send requests before building the visual TUI layers.

- **Project Scaffolding**: Initialize the Rust project and set up `tokio` as the asynchronous runtime.
- **CLI Argument Parsing**: Implement `clap` to handle basic commands like `toss send` and global flags.
- **HTTP Core**: Integrate `reqwest` to handle the primary REST methods: `GET`, `POST`, `PUT`, `PATCH`, and `DELETE`.
- **Environment Setup**: Create a basic JSON/YAML parser to handle global and environment-specific variables.

## Phase 2: The TUI Skeleton (Ratatui)

This phase focuses on the visual layout and the immediate-mode rendering engine.

- **Terminal Loop**: Set up `crossterm` for the event loop and raw terminal mode.
- **Grid Layout**: Build the 8-panel layout specified in the design docs:
  - **Top**: Request Bar (Method/URL).
  - **Middle Row**: Collections, Props, and Prop Details (Body).
  - **Bottom Row**: APIs, Response, and Response Stat.
  - **Footer**: Key Bindings Bar.
- **Vim-Navigation**: Implement the `InputMode` state machine to switch between "Normal" navigation (`hjkl`, `Tab`) and "Editing" mode for input fields.

## Phase 3: Data Management & Collections

Organizing requests into a persistent tree structure is essential for a professional workflow.

- **Tree Implementation**: Develop the logic for the Collections and APIs panels to display folders and nested requests.
- **Postman Integration**: Use the `postman_collection` crate to allow users to import and export existing data.
- **Persistence Layer**: Implement local storage (JSON files or SQLite) to save user collections and history between sessions.

## Phase 4: Advanced REST & Syntax Highlighting

This milestone transforms the tool from a basic utility into a high-end developer tool.

- **Authentication Suites**: Add support for Bearer tokens, Basic auth, API Keys, and OAuth1-2.
- **Rich Body Support**: Build editors for `form-data`, `x-www-form-urlencoded`, and `raw` JSON/GraphQL.
- **Beautification & Highlighting**: Integrate `syntect` into the Response Panel to provide syntax highlighting for JSON, XML, and HTML.
- **Response Stats**: Calculate and display response time (ms), payload size, and network protocol details in the dedicated Stat Panel.

## Phase 5: CLI Mode & Automation

Expanding the CLI allows for integration into CI/CD pipelines and automated testing.

- **`toss run` Subcommand**: Develop a collection runner that executes a folder of requests in sequence.
- **Scripting Engine**: Implement support for pre-request and post-request JavaScript snippets.
- **Result Reporting**: Create a summary output for the CLI that reports success/failure counts for automated runs.

## Phase 6: Optimization & Polish

Final steps to ensure Toss is fast and works across multiple platforms.

- **Cross-Platform Validation**: Ensure the terminal drawing works consistently on Windows and Linux.
- **Performance Tuning**: Optimize the `syntect` rendering for large JSON files to prevent TUI lag.
- **Customizable Keybindings**: Move keybindings into a configuration file so users can adjust the Vim-inspired defaults.

---

### Project Complexity Overview

| Phase | Main Focus | Risk Level | Primary Crate        |
| :---- | :--------- | :--------- | :------------------- |
| **1** | Networking | Low        | `reqwest`            |
| **2** | UI Layout  | Medium     | `ratatui`            |
| **3** | Tree State | High       | `postman_collection` |
| **4** | UI Polish  | Medium     | `syntect`            |
