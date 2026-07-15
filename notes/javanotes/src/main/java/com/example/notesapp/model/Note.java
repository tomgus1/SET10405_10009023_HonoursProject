package com.example.notesapp.model;

import java.time.LocalDateTime;
import java.util.Objects;

public final class Note {
    private final long id;
    private final String title;
    private final String content;
    private final LocalDateTime createdAt;

    public Note(long id, String title, String content, LocalDateTime createdAt) {
        this.id = id;
        this.title = Objects.requireNonNull(title, "title must not be null");
        this.content = Objects.requireNonNull(content, "content must not be null");
        this.createdAt = Objects.requireNonNull(createdAt, "createdAt must not be null");
    }

    public long getId() {
        return id;
    }

    public String getTitle() {
        return title;
    }

    public String getContent() {
        return content;
    }

    public LocalDateTime getCreatedAt() {
        return createdAt;
    }

    public Note withId(long newId) {
        return new Note(newId, title, content, createdAt);
    }

    @Override
    public boolean equals(Object other) {
        if (this == other) {
            return true;
        }
        if (other == null || getClass() != other.getClass()) {
            return false;
        }
        Note note = (Note) other;
        return id == note.id
                && title.equals(note.title)
                && content.equals(note.content)
                && createdAt.equals(note.createdAt);
    }

    @Override
    public int hashCode() {
        return Objects.hash(id, title, content, createdAt);
    }

    @Override
    public String toString() {
        return "Note{" +
                "id=" + id +
                ", title='" + title + '\'' +
                ", content='" + content + '\'' +
                ", createdAt=" + createdAt +
                '}';
    }
}