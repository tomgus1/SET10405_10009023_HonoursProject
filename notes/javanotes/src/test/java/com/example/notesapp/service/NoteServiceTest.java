package com.example.notesapp.service;

import com.example.notesapp.model.Note;
import com.example.notesapp.repository.InMemoryNoteRepository;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

class NoteServiceTest {
    private final NoteService noteService = new NoteService(new InMemoryNoteRepository());

    @Test
    void createsNotesWithGeneratedIds() {
        Note note = noteService.createNote("Shopping", "Buy milk");

        assertEquals(1L, note.getId());
        assertEquals("Shopping", note.getTitle());
        assertEquals("Buy milk", note.getContent());
    }

    @Test
    void searchesByTitleAndContent() {
        noteService.createNote("Shopping", "Buy milk");
        noteService.createNote("Work", "Finish report");

        assertEquals(1, noteService.searchNotes("milk").size());
        assertEquals(1, noteService.searchNotes("work").size());
    }

    @Test
    void deletesExistingNotes() {
        Note note = noteService.createNote("Task", "Do it");

        assertTrue(noteService.deleteNote(note.getId()));
        assertFalse(noteService.deleteNote(note.getId()));
    }

    @Test
    void updatesExistingNotesWithoutChangingCreationTime() {
        Note note = noteService.createNote("Task", "Do it");
        Note updatedNote = noteService.updateNote(note.getId(), "Task updated", "Do it better");

        assertEquals(note.getId(), updatedNote.getId());
        assertEquals(note.getCreatedAt(), updatedNote.getCreatedAt());
        assertEquals("Task updated", updatedNote.getTitle());
        assertEquals("Do it better", updatedNote.getContent());
    }

    @Test
    void rejectsUnknownNotesWhenUpdating() {
        assertThrows(IllegalArgumentException.class, () -> noteService.updateNote(99L, "Title", "Body"));
    }

    @Test
    void rejectsBlankTitles() {
        assertThrows(IllegalArgumentException.class, () -> noteService.createNote(" ", "Body"));
    }
}