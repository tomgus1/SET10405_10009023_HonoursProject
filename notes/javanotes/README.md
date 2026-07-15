# Notes App

A simple desktop notes application written in plain Java with Maven.

## Features

- Create notes with a title and body
- Edit existing notes in place
- Browse notes in a desktop list view
- Search notes by keyword
- Delete notes by id
- Persist notes as JSON between runs
- Toggle between light and dark themes

## Requirements

- Java 17+
- Maven 3.9+

## Build

```bash
mvn clean test package
```

## Run

```bash
mvn clean package
java -jar target/notes-app-1.0.0-SNAPSHOT.jar
```

Notes are stored in `~/.notes-app/notes.json` by default. Existing `notes.db` files are still loaded and rewritten as JSON.