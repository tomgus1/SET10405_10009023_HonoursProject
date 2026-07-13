# Notes App Workspace Instructions

- [x] Verify that the `copilot-instructions.md` file in the `.github` directory is created.
- [x] Clarify project requirements. The project is a Rust/Cargo desktop notes app using GPUI.
- [x] Scaffold the project. The app uses a layered structure with model, repository, service, and GPUI ui modules.
- [x] Customize the project. JSON file-backed persistence and a GPUI desktop GUI are implemented.
- [x] Install required extensions. None required for this project.
- [x] Compile the project. `cargo build` is clean.
- [x] Create and run task. Not needed for this project.
- [x] Launch the project. The app starts from `src/main.rs`.
- [x] Ensure documentation is complete. README and workspace instructions match the current implementation.

- Use idiomatic Rust and keep the code organized by responsibility (model / repository / service / ui).
- Keep persistence file-backed (JSON) and local by default.
- Prefer small, focused modules with clear responsibilities.
