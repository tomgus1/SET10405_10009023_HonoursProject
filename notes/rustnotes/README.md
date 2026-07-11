# Notes App

A simple desktop notes application written in Rust, using [GPUI](https://gpui.rs) (Zed's GPU-accelerated UI framework) and [gpui-component](https://github.com/longbridge/gpui-component) for widgets.

## Features

- Create notes with a title and body
- Browse notes in a desktop list view
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
