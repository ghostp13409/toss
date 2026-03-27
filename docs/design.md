# TUI API Client — Design Specification

A terminal-based API client inspired by Postman, built for keyboard-first workflows.

---

## Overview

The application is a full-screen TUI (Terminal User Interface) divided into panels. It allows developers to organize API requests into collections, configure request parameters, send HTTP requests, and inspect responses — all without leaving the terminal.

---

## Design Inspiration

The design is heavily inspired by lazydocker and other Rust-based TUIs that prioritize a clean, efficient layout with intuitive keyboard navigation. The goal is to create a tool that feels natural for developers who prefer working in the terminal while still providing powerful features for API testing and exploration.

The layout is heavily influenced by Postman’s interface, adapted for a terminal environment. The left side focuses on request organization (collections and APIs), while the right side is dedicated to request configuration and response inspection.

## Layout Structure

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│  Toss 1.0.2                                                         by ghostp134    │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                  [ METHOD ] [ URL INPUT FIELD          ] [ Send ]                   │
├──────────────────────┬───────────────────┬──────────────────────────────────────────┤
│  Collections         │  Props            │  Body (Selected Prop.)                   │
│                      │                   │                                          │
│  Test Collection     │  > Params         │  {                                       │
│  └ Test Folder       │  > Authorization  │    "email": "test@admin.com",            │
│    ├ GET  TestAPi    │  > Headers        │    "password": "test"                    │
│    ├ POST TestAPi2   │  > Body           │  }                                       │
│    └ POST TestAPi3   │  > Scripts        │                                          │
│  Test Collection 2   │  > Settings       │                                          │
├──────────────────────┼───────────────────┴──────────────────────────────────────────┤
│  APIs                │  Response              [ 200 OK ]   Props                    │
│                      │                                                              │
│  Test Folder         │  {                     > Status Code                         │
│  ├ GET  TestAPi      │    "id": 45,           > Response Time                       │
│  ├ POST TestAPi2     │    "email": "...",     > Response Size                       │
│  └ POST TestAPi3     │    "password": "test"  > Network                             │
│                      │  }                                                           │
├──────────────────────┴──────────────────────────────────────────────────────────────┤
│  Key Bindings:  Switch Tab: tab   Navigate: jkhl   Select: Enter   Send: Ctrl+Enter │
└─────────────────────────────────────────────────────────────────────────────────────┘
```

---

## Panels

### Title

- Displays the application name and the version in a stylish way.
- on the right side, it should show the author name and donate link

### 1. Request Bar (Top)

The persistent top bar that is always visible. shows currently selected HTTP method with a send button

| Element      | Description                                                                                                                               |
| ------------ | ----------------------------------------------------------------------------------------------------------------------------------------- |
| Method badge | Displays the HTTP method (e.g. `GET`, `POST`, `PUT`, `DELETE`). Color-coded: GET = green, POST = orange/yellow, PUT = blue, DELETE = red. |
| URL input    | Editable text field showing the full request URL.                                                                                         |
| Send button  | Triggers the request.                                                                                                                     |

---

### 2. Collections Panel (Top-Left)

Displays saved API collections in a tree structure.

- **Collections** are top-level groups (e.g. `Test Collection`, `Test Collection 2`).
- **Folders** are sub-groups within a collection (e.g. `Test Folder`).
- **Requests** are leaf nodes within folders. Each request shows its HTTP method and name.

**Visual conventions:**

- The currently selected folder is highlighted.
- Request lables should be color-coded by method (e.g., `GET` in green, `POST` in orange/yellow, etc.).
- Tree indentation.

---

### 3. Props Panel (Top-Center)

Displays configurable request properties for the selected request. Each item is a collapsible section toggled via `Enter`.

| Property      | Description                                      |
| ------------- | ------------------------------------------------ |
| Params        | URL query parameters                             |
| Authorization | Auth schemes (Bearer, Basic, API Key, etc.)      |
| Headers       | Custom request headers                           |
| Body          | Request body (JSON, form data, raw, etc.)        |
| Scripts       | Pre-request and post-response scripts            |
| Settings      | Per-request settings (timeouts, redirects, etc.) |

**Note**: Selected Item should be displayed in the Prop Details Panel Next to it

---

### 4. Prop Details Panel (Top-Right)

Displays the content of the currently selected Prop (e.g., the Body).

- Shows a syntax-highlighted preview of the request body.
- For JSON, displays formatted JSON with proper indentation.
- Panel label reads `Body (Selected Prop.)` to indicate it reflects the active prop selection.

**Note**: Figure out how to display other options within the perticular prop (e.g., for Body, Select type, Beautify for Raw type, etc.)

---

### 5. APIs Panel (Bottom-Left)

Similar to the Collections panel, but scoped to the selected folder's requests and folders only.

- Mirrors the tree structure of the currently active folder.
- Allows quick navigation between requests without browsing the full collection tree.
- Useful when working within a single folder for an extended session.

---

### 6. Response Panel (Bottom-Center)

Displays the response from the most recent request.

- The status code badge (e.g., `200 OK`) is shown prominently in the panel header.
  - `2xx` → green badge
  - `4xx` → orange/red badge
  - `5xx` → red badge
- The response body is displayed below with syntax highlighting.

**Example:**

```json
{
  "id": 45,
  "email": "test@admin.com",
  "password": "test"
}
```

---

### 7. Response Stat Panel (Bottom-Right)

Provides meta-information about the last response.

| Property      | Description                                            |
| ------------- | ------------------------------------------------------ |
| Status Code   | HTTP status code and text (e.g. `200 OK`)              |
| Response Time | Time taken for the round-trip (ms)                     |
| Response Size | Size of the response body (bytes/KB)                   |
| Network       | Protocol details (HTTP/1.1, HTTP/2, TLS version, etc.) |

---

### 8. Key Bindings Bar (Bottom)

A persistent footer strip showing helpful keyboard shortcuts.

| Key          | Action                                         |
| ------------ | ---------------------------------------------- |
| `Tab`        | Switch focus between panels                    |
| `j / k`      | Move down / up within a panel                  |
| `h / l`      | Move left / right (collapse/expand tree nodes) |
| `Enter`      | Select the focused item                        |
| `Ctrl+Enter` | Send the current request                       |
| `?`          | Open a help menu with all key bindings         |

---

## Color Coding

| Element         | Color                  |
| --------------- | ---------------------- |
| `GET` method    | Green                  |
| `POST` method   | Orange / Yellow        |
| `PUT` method    | Blue                   |
| `DELETE` method | Red                    |
| `2xx` status    | Green                  |
| `4xx` status    | Orange                 |
| `5xx` status    | Red                    |
| Selected item   | Highlighted / bordered |
| Active panel    | Border color changes   |

---

## Navigation Model

- Focus cycles through panels using `Tab`.
- Within a panel, `j`/`k` move the cursor vertically.
- `h`/`l` collapse/expand tree nodes in the Collections and APIs panels.
- `Enter` selects the focused item (e.g., loads a request from Collections into the request bar).

---

## Implementation Notes

- The application should support **mouse interaction** as an optional enhancement over the keyboard-first design.
- All panels should **resize proportionally** when the terminal window is resized.
- The Response and Body panels should support **vertical scrolling** for large payloads.
- Syntax highlighting should be applied to JSON, XML, HTML, and plain text responses.
- Method badges and status badges use **colored text or background** depending on terminal color support (256-color or truecolor).
- It should be compatible with both **Windows and Linux** terminals, using `crossterm` for cross-platform terminal handling.
- The themes and colors should be dynamic with hyprland and other tiling window managers, allowing users to easily integrate it into their existing workflows.

### AI Notes:

- Focus more on the core concepts and reasoning of the design rather than getting every detail perfect. The goal is to create a solid foundation.
- The design should be flexible enough to allow for future enhancements (e.g., adding more HTTP methods, supporting GraphQL, etc.) without requiring a complete overhaul of the UI.
- Don't be too strict on the exact layout if it doesn't fit well in the terminal. The key is to maintain a clear separation of concerns between request organization, configuration, and response inspection while optimizing for usability in a terminal environment.
- Feel free to suggest alternative UI layouts and implementation if you think it would improve the user experience or technical feasibility.
