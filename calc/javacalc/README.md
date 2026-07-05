# Java Calculator GUI

A standard desktop calculator application built with Java Swing and Maven. Serves as a baseline implementation for comparison against a future Rust GUI rewrite.

## Features

- **Basic Arithmetic**: Addition (`+`), Subtraction (`-`), Multiplication (`×`), Division (`÷`)
- **Expression Display**: Top preview bar showing active calculation steps and operands
- **Entry & Clear Controls**: Clear (`C`), Clear Entry (`CE`), Backspace (`⌫`), Sign Flip (`±`)
- **Keyboard Input**: Full key bindings for numbers (`0-9`), decimal (`.`), operators (`+`, `-`, `*`, `/`), equals (`Enter`), backspace (`Backspace`), clear (`Esc` / `C`)
- **Number Formatting**: Clean integer and floating point formatting, prevention of multiple decimal points
- **Error Handling**: Graceful division-by-zero error reporting (`Error: Division by zero`)
- **Wayland Support**: Included launch script for Linux/Wayland desktop environments

## How to Run

### Using Maven (Recommended)

1. Build the project:
   ```bash
   mvn clean package
   ```
2. Run the application:
   ```bash
   java -jar target/java-calculator-1.0.0.jar
   ```

*Note: If your system `.m2` repository permissions require custom local paths, run:*
```bash
mvn clean package -Dmaven.repo.local=./.m2/repository
```

### Using Wayland (Linux)

```bash
chmod +x run-wayland.sh
./run-wayland.sh
```

## Project Structure

```
javacalc/
├── src/
│   └── main/
│       └── java/
│           └── com/javacalc/
│               ├── Main.java            # Window entry point & configuration
│               └── CalculatorPanel.java # GUI layout, state machine & key bindings
├── pom.xml              # Maven build configuration
└── run-wayland.sh       # Wayland compatibility script
```