# CLI Mode

To complement the rich, interactive experience of the **Toss** TUI, a robust CLI mode should focus on **automation**, **speed**, and **integration**. While the TUI is for exploration and manual debugging, the CLI is for "set and forget" tasks like CI/CD pipelines and shell scripts.

Here are the essential features and subcommands you should include in the **Toss** CLI mode:

### 1. The "Ad-hoc" Sender (`toss send`)

This provides a quick, human-readable alternative to `curl` for one-off requests without needing to open the TUI.

- **Syntax**: `toss send GET /users -H "Auth: Bearer 123"`.
- **Feature**: Inherit global settings or active environments even in the CLI.
- **Usage**: Great for developers who just want to fire off a quick request they’ve already configured in their collections.

### 2. The Collection Runner (`toss run`)

This is your version of _Newman_ (Postman's CLI). It allows users to execute an entire folder or collection at once.

- **Automation**: Essential for CI/CD pipelines (e.g., running tests after a build).
- **Exit Codes**: Critical for scripting—return `0` on success and `1` (or specific error codes) if any request fails or an assertion is not met.
- **Example**: `toss run "Auth Flow" --env "Production"`.

### 3. Environment & Variable Management (`toss env`)

Allow users to switch or view contexts without launching the full interface.

- **Commands**:
  - `toss env list`: Show all available environments.
  - `toss env use <name>`: Set the default environment for subsequent CLI/TUI sessions.
  - `toss env get <key>`: Quickly check the value of a specific variable (useful for checking tokens).

### 4. Import / Export Utility (`toss import`)

Since **Toss** aims for cross-platform utility, making it easy to bring in data from other tools via the command line is a high-value "basic" feature.

- **Functionality**: Support importing Postman collections, Swagger/OpenAPI specs, or Insomnia files directly via `toss import ./swagger.json`.

---

### 5. DX (Developer Experience) for Scripting

To make **Toss** a "good citizen" in the terminal ecosystem, include these technical flags:

| Flag             | Purpose                                                                             |
| :--------------- | :---------------------------------------------------------------------------------- |
| `--silent`       | Suppress all output except the actual response body (essential for piping to `jq`). |
| `--json`         | Force the output to be raw JSON, disabling the colorized "fancy" formatting.        |
| `--headers-only` | Print only the response headers (similar to `curl -I`).                             |
| `--offline`      | Validate the request parameters and variables without actually sending the request. |
