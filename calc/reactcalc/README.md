# React Native Calculator

A standard calculator app built with Expo, React Native, and TypeScript, wrapped in Electron for desktop distribution. Rewrite of an earlier Java Swing baseline implementation, kept for comparison against other GUI stacks.

## Features

- **Basic Arithmetic**: Addition (`+`), Subtraction (`-`), Multiplication (`×`), Division (`÷`)
- **Expression Display**: Top preview bar showing active calculation steps and operands
- **Entry & Clear Controls**: Clear (`C`), Clear Entry (`CE`), Backspace (`⌫`), Sign Flip (`±`)
- **Keyboard Input** (web/desktop): Full key bindings for numbers (`0-9`), decimal (`.`), operators (`+`, `-`, `*`, `/`), equals (`Enter`), backspace (`Backspace`), clear (`Esc` / `C`)
- **Number Formatting**: Clean integer and floating point formatting, prevention of multiple decimal points
- **Error Handling**: Graceful division-by-zero error reporting (`Error: Division by zero`)
- **Cross-platform**: same React Native source runs on iOS, Android, web, and desktop (via Electron)

## How to Run

```bash
npm install
npm start          # opens Expo dev tools; press w/a/i to target web/Android/iOS
npm run web         # run directly in the browser
npm run android     # run on Android (emulator or device)
npm run ios         # run on iOS (macOS + Xcode required)
```

### Desktop (Electron)

```bash
npm run desktop:install   # first time only — installs the electron/ subproject's deps
npm run desktop           # builds the web export and launches it in an Electron window
```

`npm run desktop` rebuilds a static web export each time and serves it locally to the
Electron window — no live reload, but it doesn't depend on Metro's file watcher, which
can hit OS file-descriptor/inotify limits on some Linux setups.

If you want live reload during desktop development (requires the Metro dev server to
run without hitting watcher limits):

```bash
npm run desktop:watch
```

To produce an installable package for the current platform:

```bash
npm run desktop:package
```

This outputs to `electron/release/` (an `.AppImage` on Linux). Cross-compiling for
another OS from your current one is unreliable with Electron — use
`desktop:package:linux` / `desktop:package:win` / `desktop:package:mac` on that OS
(or in CI) to build its native installer.

## Tests

```bash
npm test
```

## Project Structure

```
reactcalc/
├── App.tsx                          # App entry point
├── src/
│   ├── CalculatorScreen.tsx         # UI: display, button grid, keyboard shortcuts
│   ├── calculatorModel.ts           # Calculator state machine
│   ├── calculatorModel.test.ts      # Jest unit tests for the state machine
│   └── operation.ts                 # Arithmetic operations enum/helpers
├── electron/                        # Desktop wrapper (electron-vite)
│   ├── src/main/index.ts            # Electron main process — loads the web build
│   ├── src/main/staticServer.ts     # Tiny local HTTP server for the static export
│   └── electron-builder.yml         # Desktop packaging config
├── app.json                         # Expo app configuration
└── package.json
```
