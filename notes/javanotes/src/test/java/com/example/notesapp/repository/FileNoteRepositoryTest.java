package com.example.notesapp.repository;

import com.example.notesapp.model.Note;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import java.nio.file.Path;
import java.time.LocalDateTime;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

class FileNoteRepositoryTest {
    @TempDir
    Path tempDir;

    @Test
    void persistsNotesAcrossInstancesAsJson() {
        Path storageFile = tempDir.resolve("notes.db");

        FileNoteRepository firstRepository = new FileNoteRepository(storageFile);
        Note createdNote = firstRepository.save(new Note(0L, "Title", "Content", LocalDateTime.of(2026, 8, 9, 12, 0)));

        FileNoteRepository secondRepository = new FileNoteRepository(storageFile);

        assertEquals(1L, createdNote.getId());
        assertEquals(1, secondRepository.findAll().size());
        assertEquals("Title", secondRepository.findById(createdNote.getId()).orElseThrow().getTitle());
    }

    @Test
    void editsNotesAndPersistsRemoval() {
        Path storageFile = tempDir.resolve("notes.db");
        FileNoteRepository repository = new FileNoteRepository(storageFile);
        Note createdNote = repository.save(new Note(0L, "Title", "Content", LocalDateTime.of(2026, 8, 9, 12, 0)));

        repository.save(new Note(createdNote.getId(), "Updated title", "Updated content", createdNote.getCreatedAt()));

        FileNoteRepository reloadedRepository = new FileNoteRepository(storageFile);
        assertEquals("Updated title", reloadedRepository.findById(createdNote.getId()).orElseThrow().getTitle());

        assertTrue(repository.deleteById(createdNote.getId()));
        assertFalse(repository.deleteById(createdNote.getId()));

        FileNoteRepository afterDeleteRepository = new FileNoteRepository(storageFile);
        assertTrue(afterDeleteRepository.findAll().isEmpty());
    }

    @Test
    void roundTripsSpecialCharactersThroughJson() {
        Path storageFile = tempDir.resolve("notes.db");
        FileNoteRepository repository = new FileNoteRepository(storageFile);
        String title = "Quotes \"and\" slashes \\";
        String content = "Line 1\nLine 2\nTabbed\tvalue";

        Note createdNote = repository.save(new Note(0L, title, content, LocalDateTime.of(2026, 8, 9, 12, 0)));

        FileNoteRepository reloadedRepository = new FileNoteRepository(storageFile);
        Note reloadedNote = reloadedRepository.findById(createdNote.getId()).orElseThrow();

        assertEquals(title, reloadedNote.getTitle());
        assertEquals(content, reloadedNote.getContent());
    }
}