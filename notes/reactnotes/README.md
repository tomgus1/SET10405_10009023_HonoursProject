# Notes App

A simple notes application built with React Native (Expo) and TypeScript.

## Features

- Create notes with a title and body
- Edit existing notes in place
- Browse notes in a list view
- Search notes by keyword
- Delete notes by id
- Persist notes on-device via AsyncStorage
- Toggle between light and dark themes

## Requirements

- Node.js 18+
- Expo CLI (via `npx`)

## Run

```bash
npm install
npm start
```

Then press `i` for iOS simulator, `a` for Android emulator, or `w` for web — or scan the QR code with Expo Go on a physical device.

## Desktop (Linux/Windows/Mac)

The app also runs as a desktop window via Electron, wrapping the Expo web export:

```bash
npm run desktop
```

This exports the web build to `dist/` and opens it in an Electron window. Run the two steps separately during development with `npm run build:web` and `npm run electron`.

To produce an installable package for the current platform:

```bash
npm run desktop:package
```

This outputs to `release/` (an `.AppImage` on Linux). Cross-compiling for another OS from your current one is unreliable with Electron — use `desktop:package:linux` / `desktop:package:win` / `desktop:package:mac` on that OS (or in CI) to build its native installer.

## Project structure

- `App.tsx` — main screen (note list, editor, search, theme toggle)
- `src/models/Note.ts` — note data model
- `src/storage/noteStorage.ts` — AsyncStorage-backed persistence
- `src/services/noteService.ts` — validation and business logic
- `src/theme/theme.ts` — light/dark theme definitions
- `electron/main.js` — Electron main process; serves the web export locally and opens it in a `BrowserWindow`
