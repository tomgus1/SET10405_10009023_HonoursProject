# Honours Project: Desktop App Framework Comparison

Two small desktop apps — a **calculator** and a **notes app** — each implemented five
times in different languages/frameworks, to compare them on things like lines of
code, dependency count, build time, and artifact size.

## Implementations

| Folder                | Stack                                                          |
| ---------------------- | --------------------------------------------------------------- |
| `rustcalc` / `rustnotes`     | Rust, using [GPUI](https://gpui.rs) (Zed's GPU-accelerated UI framework) directly |
| `rustfwcalc` / `rustfwnotes` | Rust, using a small hand-built GUI framework on top of GPUI (no third-party widget library) |
| `javacalc` / `javanotes`     | Java (Swing + Maven)                                           |
| `fluttercalc` / `flutternotes` | Flutter/Dart                                                  |
| `reactcalc` / `reactnotes`   | React Native (Expo) wrapped as a desktop app with Electron      |

Each implementation lives under `calc/<name>` or `notes/<name>` and has its own
README with build/run instructions specific to that stack.

## CI

Every push to `main` builds all 10 apps for Linux, Windows, and macOS via the
workflows in [`.github/workflows`](.github/workflows). Linux Rust/Flutter builds
are packaged as portable `.AppImage`s; macOS builds are ad-hoc signed (see note
below); Electron and Windows Flutter/Rust builds produce native installers.

### Releases

Pushing a tag like `rustcalc-v0.1.0` (pattern: `<app>-v<version>`) builds that
app and publishes a GitHub Release with the installer for each platform:

- `.AppImage` — Linux
- `.dmg` — macOS
- `.exe` — Windows (Rust ships as a plain binary; Flutter and Electron ship as
  proper installers)
- `.jar` — Java (platform-independent, one file covers all three)

### macOS Gatekeeper

Builds are ad-hoc signed (no paid Apple Developer account), which avoids the
"app is damaged" error but does **not** remove Gatekeeper's "cannot verify
developer" warning. To open a downloaded build: right-click the app → **Open**,
or in Terminal: `xattr -cr YourApp.app` / `YourApp.dmg`.

## Benchmarking

[`calc/bench/calcBench.sh`](calc/bench/calcBench.sh) and
[`notes/bench/notesBench.sh`](notes/bench/notesBench.sh) run each implementation
through the same set of measurements (lines of code, dependency count,
clean/incremental build time, artifact size) and write a timestamped
`results-*.md` report into `calc/bench/results/` or `notes/bench/results/`.
