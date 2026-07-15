package com.example.notesapp.repository;

import com.example.notesapp.model.Note;

import java.util.List;
import java.util.Optional;

public interface NoteRepository {
    Note save(Note note);

    List<Note> findAll();

    Optional<Note> findById(long id);

    boolean deleteById(long id);
}