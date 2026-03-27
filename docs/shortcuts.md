Building a **Ratatui** application with a Vim-inspired control scheme is a great choice. Power users who prefer the terminal usually have "Vim muscle memory," so aligning with those patterns will make **Toss** feel like a native extension of their workflow.

Based on your design specification and feature list, here is a suggested keyboard mapping that balances Vim's logic with the specific needs of an API client.

---

## 1. Global & Navigation Shortcuts

These should work regardless of which panel is focused (unless you are in an "Insert/Input" mode).

| Key                                 | Action                                                                 | Why it's Vim-intuitive                          |
| :---------------------------------- | :--------------------------------------------------------------------- | :---------------------------------------------- |
| `Tab`                               | Cycle focus forward through panels                                     | Standard TUI navigation.                        |
| `Shift + Tab`                       | Cycle focus backward through panels                                    | Standard TUI navigation.                        |
| `Ctrl + h/j/k/l`                    | Direct jump to Left/Down/Up/Right panel                                | Mimics `Ctrl+w` + direction for window jumping. |
| `leader + w`                        | Cycle focus between the "Request" (Top) and "Response" (Bottom) halves | Similar to jumping between windows.             |
| `Ctrl + Enter`                      | **Send Request**                                                       | High-visibility global trigger.                 |
| `leader + q` or `KeyboardInterrupt` | Quit / Close Modal / Exit Input Mode                                   | Standard "Escape" behavior.                     |
| `?` or `leader + h`                 | Toggle Help Menu                                                       | Standard TUI convention.                        |

---

## 2. Panel-Specific Shortcuts

### **Collections & APIs Panels** (Tree Navigation)

- `j / k`: Move cursor up and down.
- `h / l`: Collapse / Expand folder.
- `Enter`: Select an Item (e.g., load a request into the Request Bar, open folder, select option, etc.).
- `a`: **Add** new Request/Folder (opens a small input modal).
- `r`: **Rename** selected item.
- `d`: **Delete** selected item (with a "y/n" confirmation).
- `/`: **Search/Filter** collections (instantly filters the tree as you type).

### **Props & Prop Details Panels** (Configuration)

- `Enter`: Toggle/Expand a section (Params, Auth, etc.).
- `i`: Enter **Insert Mode** for the selected field (e.g., editing a Header key).
- `Tab` (inside Detail): Move between Key and Value fields.
- `b`: **Beautify** (for the Body prop if set to JSON/XML).
- `v`: Open the current Body in your system's `$EDITOR` (e.g., Neovim) for heavy editing.

### **Request Bar** (Top)

- `m`: Cycle through **Methods** (GET → POST → PUT, etc.).
- `e`: Focus the **URL Input** field for editing.

### **Response Panel** (Bottom)

- `G`: Scroll to the bottom of the response.
- `gg`: Scroll to the top of the response.
- `y`: **Yank** (Copy) the entire response body to the system clipboard.
- `f`: Toggle "Full Screen" for the response (hides top panels to see more data).

---

## 3. The "Vim Power User" Layer

To truly make it feel like Vim, consider these state-dependent shortcuts:

- **Command Mode (`:`):** If you press `:`, a small command line appears at the bottom.
  - `:set env [name]` — Quickly switch environment variables.
  - `:save [name]` — Save the current ad-hoc request to a collection.
  - `:import [path]` — Trigger the import flow for Postman/Swagger.
- **Visual Mode for Response:** Allow users to use `v` to start selecting text within the response body to copy specific snippets.

---

## 4. The Bottom Shortcut Tip Panel

The bottom bar should only show the "Life Raft" keys—the ones that keep the user from getting stuck. Since screen real estate is limited, keep it high-level.

**Recommended Bottom Bar Display:**

> `Tab` Focus `j/k` Nav `Enter` Select `Ctrl+Enter` Send `/` Search `?` Help `q` Quit

---

## Implementation Tip for Ratatui

Since you are using `crossterm` and `tokio`, I recommend implementing an `InputMode` enum in your `app.rs`:

```rust
enum InputMode {
    Normal,   // Standard Vim navigation (hjkl, Tab, etc.)
    Editing,  // Typing in a URL or Header field (ignore hjkl)
    Command,  // Typing a :command
}
```

When `InputMode` is `Normal`, your event handler listens for `j`, `k`, `h`, `l`. When it's `Editing`, it passes all keypresses directly into the string buffer for the active text field. This prevents the user from accidentally jumping panels while trying to type a URL.

**Would you like me to help you draft the `match` statement for your event handler in Rust to handle these different modes?**
