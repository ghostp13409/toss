# TUI API Client — Design Specification

A terminal-based API client inspired by Postman, built for keyboard-first workflows.

---

## Overview

The application is a full-screen TUI (Terminal User Interface) divided into logical panels. It allows developers to organize API requests into collections, configure request parameters, send HTTP requests, and inspect responses — all without leaving the terminal.

---

## Design Inspiration & Layout

The design is heavily inspired by **lazydocker**. It prioritizes a clean, efficient layout with intuitive drill-down keyboard navigation. It should look visually identical to lazydocker in terms of structure, but adapted for an API client use case.

The layout uses a split-screen approach:
- **Left Column (30%)**: Dedicated to request organization (Collections and APIs).
- **Right Column (70%)**: Dedicated to request configuration, editing, and response inspection.

### Visual Layout

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ Toss 1.0.0                                                    by ghostp134   │
├─────────────────────┬────────────────────────────────────────────────────────┤
│ Collections         │ [GET] https://api.example.com/users           [Send]   │
│ > Auth Flows        ├────────────────────────────────────────────────────────┤
│ v Test Folder       │ Properties                                             │
│   GET  Users        │ > Params                                               │
│   POST Create User  │ > Headers                                              │
│                     │ > Body (JSON)                                          │
│                     ├────────────────────────────────────────────────────────┤
│ APIs                │ Property Details / Editor                              │
│ GET Users           │ {                                                      │
│ POST Create User    │   "username": "admin",                                 │
│                     │   "password": "password"                               │
│                     │ }                                                      │
│                     ├─────────────────────────────────────────┬──────────────┤
│                     │ Response [200 OK]                       │ Stat         │
│                     │ {                                       │ Time: 45ms   │
│                     │   "token": "ey12345..."                 │ Size: 1.2KB  │
│                     │ }                                       │              │
├─────────────────────┴─────────────────────────────────────────┴──────────────┤
│ [Collections] j/k: Navigate | Enter: Select | Esc: Back | Ctrl+Enter: Send   │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## Panels & Logical Layers

The UI is divided into **Logical Layers** to facilitate intuitive drill-down navigation.

### Layer 1: Organization (Left Column)
- **Collections Panel**: Displays saved API collections in a tree structure. Folders and Requests.
- **APIs Panel**: Scoped to the selected folder's requests. Allows quick navigation between requests within the active folder.

### Layer 2 & 3: Configuration (Right Column, Top/Middle)
- **Request Bar**: Shows HTTP method, URL, and a visual Send button indicator.
- **Properties Panel (Layer 2)**: List of configurable properties for the selected request (Params, Auth, Headers, Body, Scripts, Settings).
- **Property Details Panel (Layer 3)**: The editor/input area for the currently selected property. E.g., if "Body" is selected in Layer 2, Layer 3 shows the JSON editor.

### Layer 4 & 5: Inspection (Right Column, Bottom)
- **Response Panel (Layer 4)**: Displays the response body with syntax highlighting and the status code badge.
- **Response Stat Panel (Layer 5)**: Meta-information (Response Time, Size, Network details).

---

## Navigation Flow

The navigation flow uses a "Drill-Down" and "Pop-Up" layer system, similar to Lazydocker or Vim-like directory trees.

1. **Initial State (Layer 1)**: By default, the application starts with the **Collections** panel focused.
2. **Layer 1 Navigation**: 
   - `j`/`k` to navigate up and down the tree.
   - `Tab` cycles focus strictly between the `Collections` and `APIs` panels.
   - When a folder is highlighted, the `APIs` panel updates to show its contents.
3. **Drill Down to Layer 2 (Properties)**: 
   - When the user selects a request (by pressing `Enter` or `l`), focus **automatically shifts** to the **Properties Panel** (Layer 2).
   - This allows users to immediately configure the request.
4. **Drill Down to Layer 3 (Details)**: 
   - From the Properties panel, pressing `Enter` or `l` on a specific property (e.g., Headers or Body) shifts focus to the **Property Details Panel** (Layer 3) to edit the values.
5. **Pop-Up / Go Back**: 
   - At any point in Layer 2 or 3, pressing `Esc` or `h` will return focus to the previous layer (e.g., from Details back to Properties, or from Properties back to Collections).
6. **Execution & Layer 4/5**: 
   - Pressing `Ctrl+Enter` sends the request.
   - Upon receiving a response, focus automatically shifts to the **Response Panel** (Layer 4) for immediate inspection.
   - Pressing `Esc` from the Response panel returns focus to the previously active configuration panel.

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
| Selected item   | Highlighted / reversed |
| Active panel    | Bold border / accent   |

---

## Implementation Notes

- **Responsive Design**: The Left/Right split should maintain a 30/70 or 25/75 ratio. The right column should dynamically resize its vertical sections based on content or user preference.
- **Vim Modes**: While navigating layers, the app operates in `Normal` mode. When editing a URL or a Body field, it enters `Editing` mode where keystrokes are captured as text.
- **Cross-Platform**: Built with `crossterm` for Windows/Linux compatibility.
- **Theme Support**: Colors and borders should eventually be configurable.
