# Notes App

A simple desktop notes application written in Flutter/Dart.

## Features

- Create notes with a title and body
- Edit existing notes in place
- Browse notes in a desktop list view
- Search notes by keyword
- Delete notes by id
- Persist notes as JSON between runs
- Toggle between light and dark themes

## Requirements

- Flutter 3.x (stable channel) with desktop support enabled

## Run

```bash
flutter pub get
flutter run -d linux   # or -d macos / -d windows
```

## Test

```bash
flutter test
```

## Build

```bash
flutter build linux    # or macos / windows
```

Notes are stored in `~/.notes-app/notes.json` by default.
