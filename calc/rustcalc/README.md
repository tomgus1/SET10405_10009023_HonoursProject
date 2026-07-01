# Rust Calculator GUI

A high-performance desktop calculator application written completely in Rust, using Zed Industries' **GPUI** framework for GPU-accelerated frontend rendering and an idiomatic Rust state machine for backend arithmetic calculations.

## Features

- **Backend & Frontend in Pure Rust**: Built end-to-end using Rust with no JVM runtime or webview wrappers.
- **GPU-Accelerated UI**: Built with Zed Industries' **GPUI** framework (`gpui`), providing crisp typography, sub-millisecond layout passes, and native Wayland/X11 Linux support.
- **Basic Arithmetic**: Addition (`+`), Subtraction (`-`), Multiplication (`×`), Division (`÷`) with double-precision floating point calculations.
- **Dual Expression Display**: Top preview bar showing active calculation steps alongside the primary input result display.
- **Entry & Clear Controls**: Clear (`C`), Clear Entry (`CE`), Backspace (`⌫`), Sign Flip (`±`).
- **Keyboard Input**: Full key bindings for numbers (`0-9`), decimal (`.`), operators (`+`, `-`, `*`, `/`), equals (`Enter` / `=`), backspace (`Backspace`), and clear (`Esc` / `C`).
- **Number Formatting**: Clean integer and floating-point output formatting, enforcing single decimal point input.
- **Error Handling**: Division-by-zero detection reporting `Error: Division by zero`.

## How to Run

### Prerequisites

Ensure you have Rust and Cargo installed:
```bash
cargo --version
```

### Build & Run Application

1. Run the application directly:
   ```bash
   cargo run --release
   ```

2. Run the unit & integration test suite:
   ```bash
   cargo test
   ```

3. Check compilation and static analysis:
   ```bash
   cargo check
   ```

## Project Structure

```
rustcalc/
├── Cargo.toml                     # Cargo build configuration & GPUI dependency
├── README.md                      # Project documentation
├── src/
│   ├── lib.rs                     # Core library exports (CalculatorView, CalculatorModel)
│   ├── main.rs                    # Window entry point & GPUI application startup
│   ├── model.rs                   # Calculator state machine & unit tests
│   └── gui.rs                     # GPUI layout, view rendering & event handlers
└── tests/
    └── calculator_model_test.rs   # Integration test suite
```