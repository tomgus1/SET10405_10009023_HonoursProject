package com.example.notesapp.repository;

import com.example.notesapp.model.Note;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.AtomicMoveNotSupportedException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.time.LocalDateTime;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public final class FileNoteRepository implements NoteRepository {
    private static final Path STORAGE_DIRECTORY = Path.of(System.getProperty("user.home"), ".notes-app");
    private static final Path JSON_STORAGE_FILE = STORAGE_DIRECTORY.resolve("notes.json");
    private static final Path LEGACY_STORAGE_FILE = STORAGE_DIRECTORY.resolve("notes.db");
    private static final Pattern JSON_STRING_FIELD_PATTERN = Pattern.compile("\\\"%s\\\"\\s*:\\s*\\\"((?:\\\\.|[^\\\"])*)\\\"", Pattern.DOTALL);
    private static final Pattern JSON_NUMBER_FIELD_PATTERN = Pattern.compile("\\\"%s\\\"\\s*:\\s*(\\d+)");

    private final Path storageFile;
    private final Map<Long, Note> notes = new HashMap<>();
    private long nextId = 1L;

    public FileNoteRepository(Path storageFile) {
        this.storageFile = Objects.requireNonNull(storageFile, "storageFile must not be null");
        loadFromDisk();
    }

    public static FileNoteRepository createDefault() {
        Path preferredStorageFile = Files.exists(JSON_STORAGE_FILE) || !Files.exists(LEGACY_STORAGE_FILE)
                ? JSON_STORAGE_FILE
                : LEGACY_STORAGE_FILE;
        return new FileNoteRepository(preferredStorageFile);
    }

    @Override
    public synchronized Note save(Note note) {
        Note persistedNote = note.getId() == 0 ? note.withId(nextId++) : note;
        notes.put(persistedNote.getId(), persistedNote);
        persistToDisk();
        return persistedNote;
    }

    @Override
    public synchronized List<Note> findAll() {
        List<Note> allNotes = new ArrayList<>(notes.values());
        allNotes.sort(Comparator.comparingLong(Note::getId));
        return List.copyOf(allNotes);
    }

    @Override
    public synchronized Optional<Note> findById(long id) {
        return Optional.ofNullable(notes.get(id));
    }

    @Override
    public synchronized boolean deleteById(long id) {
        boolean removed = notes.remove(id) != null;
        if (removed) {
            persistToDisk();
        }
        return removed;
    }

    private void loadFromDisk() {
        if (!Files.exists(storageFile)) {
            return;
        }

        try {
            String content = Files.readString(storageFile, StandardCharsets.UTF_8).trim();
            if (content.isEmpty()) {
                return;
            }

            if (content.startsWith("[")) {
                loadJson(content);
            } else {
                loadLegacy(content);
            }
        } catch (IOException exception) {
            throw new IllegalStateException("Failed to load notes from disk", exception);
        } catch (RuntimeException exception) {
            quarantineCorruptStorage();
        }
    }

    private void persistToDisk() {
        try {
            Path parentDirectory = storageFile.getParent();
            if (parentDirectory != null) {
                Files.createDirectories(parentDirectory);
            }

            Path tempFile = parentDirectory == null
                    ? Files.createTempFile("notes", ".tmp")
                    : Files.createTempFile(parentDirectory, "notes", ".tmp");
            Files.writeString(tempFile, serializeNotes(findAll()), StandardCharsets.UTF_8);
            try {
                Files.move(tempFile, storageFile, StandardCopyOption.REPLACE_EXISTING, StandardCopyOption.ATOMIC_MOVE);
            } catch (AtomicMoveNotSupportedException exception) {
                Files.move(tempFile, storageFile, StandardCopyOption.REPLACE_EXISTING);
            }
        } catch (IOException exception) {
            throw new IllegalStateException("Failed to save notes to disk", exception);
        }
    }

    private void loadJson(String content) {
        List<String> objects = extractJsonObjects(content);
        for (String objectContent : objects) {
            Note note = new Note(
                    readJsonLong(objectContent, "id"),
                    readJsonString(objectContent, "title"),
                    readJsonString(objectContent, "content"),
                    LocalDateTime.parse(readJsonString(objectContent, "createdAt")));
            notes.put(note.getId(), note);
            nextId = Math.max(nextId, note.getId() + 1);
        }

        if (objects.isEmpty() && !"[]".equals(content.replaceAll("\\s+", ""))) {
            throw new IllegalStateException("Invalid JSON note storage file");
        }
    }

    private void loadLegacy(String content) {
        for (String line : content.split("\\R")) {
            if (line.isBlank()) {
                continue;
            }

            Note note = deserializeLegacy(line);
            notes.put(note.getId(), note);
            nextId = Math.max(nextId, note.getId() + 1);
        }
    }

    private static String serializeNotes(List<Note> notes) {
        StringBuilder builder = new StringBuilder();
        builder.append("[\n");
        for (int index = 0; index < notes.size(); index++) {
            Note note = notes.get(index);
            builder.append("  {")
                    .append("\"id\": ").append(note.getId()).append(", ")
                    .append("\"title\": \"").append(escapeJson(note.getTitle())).append("\", ")
                    .append("\"content\": \"").append(escapeJson(note.getContent())).append("\", ")
                    .append("\"createdAt\": \"").append(escapeJson(note.getCreatedAt().toString())).append("\"")
                    .append("}");
            if (index < notes.size() - 1) {
                builder.append(',');
            }
            builder.append('\n');
        }
        builder.append("]\n");
        return builder.toString();
    }

    private static Note deserializeLegacy(String line) {
        String[] parts = line.split("\\|", -1);
        if (parts.length != 4) {
            throw new IllegalStateException("Invalid legacy note record: " + line);
        }

        return new Note(
                Long.parseLong(parts[0]),
                decodeLegacy(parts[2]),
                decodeLegacy(parts[3]),
                LocalDateTime.parse(parts[1]));
    }

    private static String escapeJson(String value) {
        StringBuilder builder = new StringBuilder();
        for (int index = 0; index < value.length(); index++) {
            char character = value.charAt(index);
            switch (character) {
                case '"' -> builder.append("\\\"");
                case '\\' -> builder.append("\\\\");
                case '\b' -> builder.append("\\b");
                case '\f' -> builder.append("\\f");
                case '\n' -> builder.append("\\n");
                case '\r' -> builder.append("\\r");
                case '\t' -> builder.append("\\t");
                default -> {
                    if (character < 0x20) {
                        builder.append(String.format("\\u%04x", (int) character));
                    } else {
                        builder.append(character);
                    }
                }
            }
        }
        return builder.toString();
    }

    private static String unescapeJson(String value) {
        StringBuilder builder = new StringBuilder();
        for (int index = 0; index < value.length(); index++) {
            char character = value.charAt(index);
            if (character != '\\' || index == value.length() - 1) {
                builder.append(character);
                continue;
            }

            char escape = value.charAt(++index);
            switch (escape) {
                case '"' -> builder.append('"');
                case '\\' -> builder.append('\\');
                case '/' -> builder.append('/');
                case 'b' -> builder.append('\b');
                case 'f' -> builder.append('\f');
                case 'n' -> builder.append('\n');
                case 'r' -> builder.append('\r');
                case 't' -> builder.append('\t');
                case 'u' -> {
                    if (index + 4 >= value.length()) {
                        throw new IllegalStateException("Invalid unicode escape in JSON note storage");
                    }
                    String hexDigits = value.substring(index + 1, index + 5);
                    builder.append((char) Integer.parseInt(hexDigits, 16));
                    index += 4;
                }
                default -> throw new IllegalStateException("Invalid escape sequence in JSON note storage: \\" + escape);
            }
        }
        return builder.toString();
    }

    private static List<String> extractJsonObjects(String content) {
        List<String> objects = new ArrayList<>();
        int depth = 0;
        int objectStart = -1;
        boolean inString = false;
        boolean escaped = false;

        for (int index = 0; index < content.length(); index++) {
            char character = content.charAt(index);

            if (inString) {
                if (escaped) {
                    escaped = false;
                } else if (character == '\\') {
                    escaped = true;
                } else if (character == '"') {
                    inString = false;
                }
                continue;
            }

            if (character == '"') {
                inString = true;
                continue;
            }

            if (character == '{') {
                if (depth == 0) {
                    objectStart = index;
                }
                depth++;
            } else if (character == '}') {
                depth--;
                if (depth < 0) {
                    throw new IllegalStateException("Invalid JSON note storage file");
                }
                if (depth == 0 && objectStart >= 0) {
                    objects.add(content.substring(objectStart, index + 1));
                    objectStart = -1;
                }
            }
        }

        if (depth != 0 || inString) {
            throw new IllegalStateException("Invalid JSON note storage file");
        }

        return objects;
    }

    private static long readJsonLong(String objectContent, String fieldName) {
        Pattern fieldPattern = Pattern.compile(String.format(JSON_NUMBER_FIELD_PATTERN.pattern(), Pattern.quote(fieldName)));
        Matcher matcher = fieldPattern.matcher(objectContent);
        if (!matcher.find()) {
            throw new IllegalStateException("Missing JSON field: " + fieldName);
        }
        return Long.parseLong(matcher.group(1));
    }

    private static String readJsonString(String objectContent, String fieldName) {
        Pattern fieldPattern = Pattern.compile(String.format(JSON_STRING_FIELD_PATTERN.pattern(), Pattern.quote(fieldName)), Pattern.DOTALL);
        Matcher matcher = fieldPattern.matcher(objectContent);
        if (!matcher.find()) {
            throw new IllegalStateException("Missing JSON field: " + fieldName);
        }
        return unescapeJson(matcher.group(1));
    }

    private void quarantineCorruptStorage() {
        try {
            if (Files.exists(storageFile)) {
                Path backupFile = storageFile.resolveSibling(storageFile.getFileName() + ".corrupt");
                Files.move(storageFile, backupFile, StandardCopyOption.REPLACE_EXISTING);
            }
        } catch (IOException ignored) {
            // If quarantine fails, start with an empty in-memory store so the app can launch.
        }
        notes.clear();
        nextId = 1L;
    }

    private static String decodeLegacy(String value) {
        return new String(java.util.Base64.getDecoder().decode(value), StandardCharsets.UTF_8);
    }
}