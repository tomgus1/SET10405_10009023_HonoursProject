# Flutter Calculator

A standard desktop calculator application built with Flutter.

## Features

- **Basic Arithmetic**: Addition (`+`), Subtraction (`-`), Multiplication (`×`), Division (`÷`)
- **Expression Display**: Top preview bar showing active calculation steps and operands
- **Entry & Clear Controls**: Clear (`C`), Clear Entry (`CE`), Backspace (`⌫`), Sign Flip (`±`)
- **Keyboard Input**: Full key bindings for numbers (`0-9`), decimal (`.`), operators (`+`, `-`, `*`, `/`), equals (`Enter`), backspace (`Backspace`), clear (`Esc` / `C`)
- **Number Formatting**: Clean integer and floating point formatting, prevention of multiple decimal points
- **Error Handling**: Graceful division-by-zero error reporting (`Error: Division by zero`)

## How to Run

```bash
flutter pub get
flutter run -d linux   # or macos / windows / chrome
```

## How to Test

```bash
flutter test
```

## Project Structure

```
lib/
├── main.dart              # App entry point
├── calculator_model.dart  # Calculator state machine
├── operation.dart         # Arithmetic operations enum
└── calculator_panel.dart  # UI: display, button grid, keyboard shortcuts
test/
└── calculator_model_test.dart
```
