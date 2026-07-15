package com.example.notesapp.service;

import com.example.notesapp.model.Note;
import com.example.notesapp.repository.NoteRepository;

import java.time.LocalDateTime;
import java.util.List;
import java.util.Locale;
import java.util.Objects;
import java.util.Optional;

public class NoteService {
    private final NoteRepository noteRepository;

    public NoteService(NoteRepository noteRepository) {
        this.noteRepository = java.util.Objects.requireNonNull(noteRepository, "noteRepository must not be null");
    }

    public Note createNote(String title, String content) {
        validateText(title, "title");
        validateText(content, "content");

        Note note = new Note(0L, title.trim(), content.trim(), LocalDateTime.now());
        return noteRepository.save(note);
    }

    public List<Note> getAllNotes() {
        return noteRepository.findAll();
    }

    public Optional<Note> findNoteById(long id) {
        return noteRepository.findById(id);
    }

    public boolean deleteNote(long id) {
        return noteRepository.deleteById(id);
    }

    public Note updateNote(long id, String title, String content) {
        validateText(title, "title");
        validateText(content, "content");

        Note existingNote = noteRepository.findById(id)
                .orElseThrow(() -> new IllegalArgumentException("Note not found: " + id));
        Note updatedNote = new Note(existingNote.getId(), title.trim(), content.trim(), existingNote.getCreatedAt());
        return noteRepository.save(updatedNote);
    }

    public List<Note> searchNotes(String query) {
        String normalizedQuery = query == null ? "" : query.trim().toLowerCase(Locale.ROOT);
        if (normalizedQuery.isEmpty()) {
            return getAllNotes();
        }

        return noteRepository.findAll().stream()
                .filter(note -> note.getTitle().toLowerCase(Locale.ROOT).contains(normalizedQuery)
                        || note.getContent().toLowerCase(Locale.ROOT).contains(normalizedQuery))
                .toList();
    }

    private static void validateText(String value, String fieldName) {
        if (value == null || value.trim().isEmpty()) {
            throw new IllegalArgumentException(fieldName + " must not be blank");
        }
    }
}