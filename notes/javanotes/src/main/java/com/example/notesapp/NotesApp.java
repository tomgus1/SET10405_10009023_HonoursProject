package com.example.notesapp;

import com.example.notesapp.repository.FileNoteRepository;
import com.example.notesapp.service.NoteService;
import com.example.notesapp.ui.NotesFrame;

import javax.swing.SwingUtilities;

public final class NotesApp {
    private NotesApp() {
    }

    public static void main(String[] args) {
        NoteService noteService = new NoteService(FileNoteRepository.createDefault());
        SwingUtilities.invokeLater(() -> new NotesFrame(noteService).setVisible(true));
    }
}