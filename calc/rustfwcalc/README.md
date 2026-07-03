# Rust Calculator

A desktop calculator application written in Rust on top of a small, hand-built
GUI framework (`src/framework`), which is itself built directly on
[GPUI](https://gpui.rs) (Zed's GPU-accelerated UI framework) — no third-party
widget crate involved.

## Architecture

```
Application code (src/gui.rs, src/viewmodel.rs, src/display.rs)
        |
src/framework  — component abstraction, reactive Signal<T> state,
        |         layout, theming, and window management, plus
        |         hand-rolled widgets (Button, ListItem, TextInput,
        |         dialogs, scrollbar)
        v
       gpui   — windowing, event loop, scene graph, GPU rendering
```

- **`src/framework`** — the reusable GUI layer. Nothing in here knows about
  the calculator; it only depends on `gpui`.
  - `theme.rs` — design tokens (`Theme`/`ThemeMode`) installed as a global.
  - `layout.rs` — `row()`/`column()`, a thin surface over GPUI's built-in
    Taffy flexbox.
  - `state.rs` — `Signal<T>`, a reactive value backed by its own `Entity`, so
    a view that watches one signal isn't re-rendered when a sibling signal
    changes.
  - `window.rs` — cross-platform window creation.
  - `widgets/` — `Button` (used by this app), plus `ListItem`,
    `TextInput`/`TextInputState` (hand-rolled cursor/selection/IME/clipboard
    text editing against GPUI's `EntityInputHandler`), `confirm_dialog`, and
    a draggable `scrollbar` — not exercised by the calculator, but shared
    with (and originally written for) the notes app built on the same
    framework.
- **`src/model.rs`** — **Model**: `CalculatorModel`, a pure state machine
  (current input, expression text, pending operator, error state) plus the
  `Action`/`Operation` types. No GPUI dependency; fully unit-tested in
  isolation.
- **`src/viewmodel.rs`** — **ViewModel**: `CalculatorViewModel` wraps
  `CalculatorModel`, exposing `dispatch(Action)` as the single entry point
  for mutation, and two `Signal<String>`s (`input`, `expression`) for the
  View to bind against.
- **`src/display.rs`** — **View**: `DisplayView`, the expression/input
  display panel as its own `Entity`. It watches only the two signals above,
  so it's the part of the tree that re-renders on every keystroke — the
  (static) button grid, owned separately by `CalculatorView`, never has to.
  This is the framework's "propagate only to dependent components" model in
  practice, not just in name.
- **`src/gui.rs`** — **View**: `CalculatorView`, the root. Builds the 5×4
  button grid from `framework::widgets::Button`, and forwards all
  keyboard/click input to `CalculatorViewModel::dispatch`. It never touches
  `CalculatorModel` directly.
- **`src/main.rs`** — window bootstrap via `framework::init` and
  `framework::window::open_window`.

## Features

- Addition (`+`), Subtraction (`-`), Multiplication (`×`), Division (`÷`)
  with double-precision floating point calculations
- Dual expression display: a preview bar showing the active calculation
  alongside the primary input/result display
- Entry & clear controls: Clear (`C`), Clear Entry (`CE`), Backspace (`⌫`),
  Sign Flip (`±`)
- Full keyboard input: numbers (`0-9`), decimal (`.`), operators
  (`+`, `-`, `*`, `/`), equals (`Enter` / `=`), backspace (`Backspace`), and
  clear (`Esc` / `C`)
- Clean integer/floating-point output formatting, enforcing a single decimal
  point per entry
- Division-by-zero detection reporting `Error: Division by zero`

## Requirements

- Rust 1.80+ / Cargo
- On Linux: `libxkbcommon-x11` (`build.rs` works around a missing dev symlink
  at link time if the `-devel` package isn't installed)

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
