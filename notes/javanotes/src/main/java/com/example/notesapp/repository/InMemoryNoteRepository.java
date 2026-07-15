package com.example.notesapp.repository;

import com.example.notesapp.model.Note;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;

public class InMemoryNoteRepository implements NoteRepository {
    private final Map<Long, Note> notes = new HashMap<>();
    private long nextId = 1L;

    @Override
    public synchronized Note save(Note note) {
        Note persistedNote = note.getId() == 0 ? note.withId(nextId++) : note;
        notes.put(persistedNote.getId(), persistedNote);
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
        return notes.remove(id) != null;
    }
}