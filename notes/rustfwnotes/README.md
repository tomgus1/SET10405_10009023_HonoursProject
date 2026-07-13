# Notes App

A desktop notes application written in Rust on top of a small, hand-built GUI
framework (`src/framework`), which is itself built directly on
[GPUI](https://gpui.rs) (Zed's GPU-accelerated UI framework) — no third-party
widget crate involved.

## Architecture

```
Application code (src/ui, src/viewmodel)
        |
src/framework  — component abstraction, reactive Signal<T> state,
        |         layout, theming, window management, and hand-rolled
        |         widgets (Button, ListItem, TextInput, dialogs, scrollbar)
        v
       gpui   — windowing, event loop, scene graph, GPU rendering
```

- **`src/framework`** — the reusable GUI layer. Nothing in here knows about
  notes; it only depends on `gpui`.
  - `theme.rs` — design tokens (`Theme`/`ThemeMode`) installed as a global.
  - `layout.rs` — `row()`/`column()`, a thin surface over GPUI's built-in
    Taffy flexbox.
  - `state.rs` — `Signal<T>`, a reactive value backed by its own `Entity`, so
    a view that watches one signal isn't re-rendered when a sibling signal
    changes.
  - `window.rs` — cross-platform window creation.
  - `widgets/` — `Button`, `ListItem`, `TextInput`/`TextInputState` (hand-rolled
    cursor/selection/IME/clipboard/multi-line text editing against GPUI's
    `EntityInputHandler`), `confirm_dialog`/`render_dialog_layer` (modal
    backdrop/focus-trap on top of GPUI's `deferred()`), and a draggable
    `scrollbar`.
- **`src/viewmodel`** — `NotesViewModel`: owns `NoteService` plus the app's
  `Signal`-backed state (notes, selection, status) and text-input entities.
  Exposes intents (`add_note`, `select_note`, ...); never renders anything.
- **`src/ui`** — the view layer: `NotesApp` (root) composes `NoteList`,
  `NoteEditor`, and `StatusBar` as independent entities, each watching only
  the signals it needs, so e.g. typing in the editor doesn't re-render the
  sidebar or status bar.
- **`src/model` / `src/service` / `src/repository`** — the domain: `Note`,
  `NoteService` (validation, search), and `NoteRepository` (JSON file
  persistence, with an in-memory implementation for tests).

## Features

- Create notes with a title and body
- Browse notes in a scrollable desktop list view
- Edit an existing note's title and content
- Search notes by keyword
- Delete notes by id (with a confirmation dialog)
- Toggle between light and dark theme
- Persist notes to a local file between runs

## Requirements

- Rust 1.80+ / Cargo
- On Linux: `libxkbcommon-x11` (a `build.rs` script works around a missing dev symlink at link time if the `-devel` package isn't installed)

## Build

```bash
cargo build
```

## Test

```bash
cargo test
```

## Run

```bash
cargo run
```

Notes are stored as JSON in `~/.notes-app/notes.json` by default.
