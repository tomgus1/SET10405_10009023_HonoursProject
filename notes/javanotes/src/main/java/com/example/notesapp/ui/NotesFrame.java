package com.example.notesapp.ui;

import com.example.notesapp.model.Note;
import com.example.notesapp.service.NoteService;

import javax.swing.BorderFactory;
import javax.swing.DefaultListModel;
import javax.swing.JButton;
import javax.swing.JFrame;
import javax.swing.JLabel;
import javax.swing.JList;
import javax.swing.JOptionPane;
import javax.swing.JPanel;
import javax.swing.JScrollPane;
import javax.swing.JSplitPane;
import javax.swing.JTextArea;
import javax.swing.JTextField;
import javax.swing.ListSelectionModel;
import javax.swing.SwingConstants;
import javax.swing.border.EmptyBorder;
import java.awt.BorderLayout;
import java.awt.Component;
import java.awt.Dimension;
import java.awt.FlowLayout;
import java.awt.Font;
import java.awt.GridBagConstraints;
import java.awt.GridBagLayout;
import java.awt.Insets;
import java.time.format.DateTimeFormatter;
import java.util.List;
import java.util.Objects;
import java.util.Optional;

public final class NotesFrame extends JFrame {
    private static final String APP_TITLE = "Notes App";
    private static final String TAGLINE = "Store, search, and manage notes locally";
    private static final String LIST_TITLE = "All Notes";
    private static final String EMPTY_LIST_MESSAGE = "No notes yet. Create your first note on the right.";
    private static final String READY_MESSAGE = "Ready";
    private static final String EMPTY_SELECTION_MESSAGE = "Select a note to delete.";
    private static final String CONFIRM_DELETE_TITLE = "Confirm delete";
    private static final String ERROR_DIALOG_TITLE = "Notes App";
    private static final int LIST_WIDTH = 380;
    private static final int MIN_WIDTH = 1080;
    private static final int MIN_HEIGHT = 680;
    private static final int PREVIEW_LIMIT = 96;
    private static final DateTimeFormatter CREATED_FORMATTER = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm");

    private final NoteService noteService;
    private final DefaultListModel<Note> noteListModel = new DefaultListModel<>();
    private final JList<Note> noteList = new JList<>(noteListModel);
    private final JTextField titleField = new JTextField();
    private final JTextArea contentArea = new JTextArea(10, 30);
    private final JTextField searchField = new JTextField();
    private final JButton saveButton = new JButton("Save Changes");
    private final JButton themeToggleButton = new JButton();
    private final JLabel statusLabel = new JLabel(READY_MESSAGE, SwingConstants.LEFT);
    private ThemeMode currentTheme = ThemeMode.LIGHT;
    private Long selectedNoteId;

    public NotesFrame(NoteService noteService) {
        this.noteService = Objects.requireNonNull(noteService, "noteService must not be null");
        configureFrame();
        add(buildHeader(), BorderLayout.NORTH);
        add(buildContent(), BorderLayout.CENTER);
        add(buildFooter(), BorderLayout.SOUTH);
        applyTheme(currentTheme);
        refreshNotes();
    }

    private void configureFrame() {
        setTitle(APP_TITLE);
        setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        setMinimumSize(new Dimension(MIN_WIDTH, MIN_HEIGHT));
        setLocationRelativeTo(null);
        setLayout(new BorderLayout());
    }

    private Component buildHeader() {
        JPanel header = new JPanel(new BorderLayout());
        header.setBorder(new EmptyBorder(16, 16, 8, 16));

        JPanel titlePanel = new JPanel(new BorderLayout(0, 4));
        titlePanel.setOpaque(false);
        JLabel title = new JLabel(APP_TITLE);
        title.setFont(title.getFont().deriveFont(Font.BOLD, 26f));
        titlePanel.add(title, BorderLayout.NORTH);
        titlePanel.add(new JLabel(TAGLINE), BorderLayout.SOUTH);
        header.add(titlePanel, BorderLayout.WEST);

        themeToggleButton.addActionListener(event -> toggleTheme());
        header.add(themeToggleButton, BorderLayout.EAST);
        return header;
    }

    private Component buildContent() {
        JPanel content = new JPanel(new BorderLayout(16, 16));
        content.setBorder(new EmptyBorder(8, 16, 16, 16));
        JSplitPane splitPane = new JSplitPane(JSplitPane.HORIZONTAL_SPLIT, buildSidebar(), buildEditor());
        splitPane.setResizeWeight(0.32);
        splitPane.setDividerLocation(LIST_WIDTH);
        splitPane.setBorder(null);
        content.add(splitPane, BorderLayout.CENTER);
        return content;
    }

    private Component buildSidebar() {
        JPanel sidebar = new JPanel(new BorderLayout(8, 8));
        sidebar.setPreferredSize(new Dimension(LIST_WIDTH, 0));

        JLabel listLabel = new JLabel(LIST_TITLE);
        listLabel.setFont(listLabel.getFont().deriveFont(Font.BOLD, 14f));

        JPanel searchPanel = new JPanel(new BorderLayout(8, 8));
        searchPanel.add(searchField, BorderLayout.CENTER);
        JButton searchButton = new JButton("Search");
        searchButton.addActionListener(event -> applySearch());
        searchPanel.add(searchButton, BorderLayout.EAST);

        noteList.setVisibleRowCount(12);
        noteList.setSelectionMode(ListSelectionModel.SINGLE_SELECTION);
        noteList.addListSelectionListener(event -> {
            if (!event.getValueIsAdjusting()) {
                showSelectedNote();
            }
        });
        noteList.setCellRenderer((list, note, index, isSelected, cellHasFocus) -> {
            JLabel label = new JLabel(formatNotePreview(note, currentTheme));
            label.setOpaque(true);
            label.setBorder(new EmptyBorder(10, 10, 10, 10));
            label.setBackground(isSelected ? list.getSelectionBackground() : list.getBackground());
            label.setForeground(isSelected ? list.getSelectionForeground() : list.getForeground());
            return label;
        });

        JPanel sidebarHeader = new JPanel(new BorderLayout(0, 8));
        sidebarHeader.add(listLabel, BorderLayout.NORTH);
        sidebarHeader.add(searchPanel, BorderLayout.CENTER);

        sidebar.add(sidebarHeader, BorderLayout.NORTH);
        sidebar.add(new JScrollPane(noteList), BorderLayout.CENTER);
        return sidebar;
    }

    private Component buildEditor() {
        JPanel editor = new JPanel(new GridBagLayout());
        editor.setBorder(BorderFactory.createTitledBorder("Note Details"));

        GridBagConstraints constraints = new GridBagConstraints();
        constraints.insets = new Insets(8, 8, 8, 8);
        constraints.fill = GridBagConstraints.HORIZONTAL;
        constraints.gridx = 0;
        constraints.weightx = 0;

        constraints.gridy = 0;
        editor.add(new JLabel("Title"), constraints);
        constraints.gridy = 1;
        constraints.weightx = 1;
        editor.add(titleField, constraints);

        constraints.gridy = 2;
        constraints.weightx = 0;
        editor.add(new JLabel("Content"), constraints);
        constraints.gridy = 3;
        constraints.fill = GridBagConstraints.BOTH;
        constraints.weighty = 1;
        contentArea.setLineWrap(true);
        contentArea.setWrapStyleWord(true);
        editor.add(new JScrollPane(contentArea), constraints);

        JPanel buttons = new JPanel(new FlowLayout(FlowLayout.LEFT));
        JButton addButton = new JButton("Add Note");
        addButton.addActionListener(event -> addNote());
        saveButton.setEnabled(false);
        saveButton.addActionListener(event -> saveChanges());
        JButton deleteButton = new JButton("Delete Selected");
        deleteButton.addActionListener(event -> deleteSelectedNote());
        JButton clearButton = new JButton("Clear Fields");
        clearButton.addActionListener(event -> clearEditor());
        JButton refreshButton = new JButton("Refresh");
        refreshButton.addActionListener(event -> refreshNotes());
        buttons.add(addButton);
        buttons.add(saveButton);
        buttons.add(deleteButton);
        buttons.add(clearButton);
        buttons.add(refreshButton);

        constraints.gridy = 4;
        constraints.weighty = 0;
        constraints.fill = GridBagConstraints.HORIZONTAL;
        editor.add(buttons, constraints);

        return editor;
    }

    private Component buildFooter() {
        JPanel footer = new JPanel(new BorderLayout());
        footer.setBorder(new EmptyBorder(0, 16, 16, 16));
        footer.add(statusLabel, BorderLayout.WEST);
        return footer;
    }

    private void addNote() {
        try {
            Note note = noteService.createNote(titleField.getText(), contentArea.getText());
            statusLabel.setText("Created note " + note.getId());
            clearEditor();
            refreshNotes();
            selectNote(note.getId());
        } catch (IllegalArgumentException exception) {
            showError(exception.getMessage());
        } catch (IllegalStateException exception) {
            showError("Unable to save note: " + exception.getMessage());
        }
    }

    private void saveChanges() {
        Note selectedNote = getSelectedNote();
        if (selectedNote == null) {
            showError("Select a note to edit.");
            return;
        }

        try {
            Note updatedNote = noteService.updateNote(selectedNote.getId(), titleField.getText(), contentArea.getText());
            statusLabel.setText("Updated note " + updatedNote.getId());
            refreshNotes();
            selectNote(updatedNote.getId());
        } catch (IllegalArgumentException exception) {
            showError(exception.getMessage());
        } catch (IllegalStateException exception) {
            showError("Unable to update note: " + exception.getMessage());
        }
    }

    private void deleteSelectedNote() {
        Note selectedNote = getSelectedNote();
        if (selectedNote == null) {
            showError(EMPTY_SELECTION_MESSAGE);
            return;
        }

        int choice = JOptionPane.showConfirmDialog(this, "Delete the selected note?", CONFIRM_DELETE_TITLE, JOptionPane.YES_NO_OPTION);
        if (choice != JOptionPane.YES_OPTION) {
            return;
        }

        boolean deleted = noteService.deleteNote(selectedNote.getId());
        statusLabel.setText(deleted ? "Deleted note " + selectedNote.getId() : "Note was not found");
        refreshNotes();
        clearEditor();
    }

    private void applySearch() {
        List<Note> notes = noteService.searchNotes(searchField.getText());
        updateList(notes);
        statusLabel.setText(notes.isEmpty() ? "No notes match your search." : "Showing " + notes.size() + " notes");
    }

    private void refreshNotes() {
        List<Note> notes = noteService.getAllNotes();
        updateList(notes);
        statusLabel.setText(notes.isEmpty() ? EMPTY_LIST_MESSAGE : "Loaded " + notes.size() + " notes");
    }

    private void updateList(List<Note> notes) {
        noteListModel.clear();
        notes.forEach(noteListModel::addElement);
        if (!noteListModel.isEmpty()) {
            noteList.setEnabled(true);
            noteList.setSelectedIndex(0);
        } else {
            clearEditor();
            noteList.setEnabled(false);
        }
        noteList.repaint();
    }

    private void showSelectedNote() {
        Optional.ofNullable(noteList.getSelectedValue()).ifPresentOrElse(note -> {
            selectedNoteId = note.getId();
            titleField.setText(note.getTitle());
            contentArea.setText(note.getContent());
            saveButton.setEnabled(true);
        }, this::clearEditor);
    }

    private void clearEditor() {
        noteList.clearSelection();
        titleField.setText("");
        contentArea.setText("");
        selectedNoteId = null;
        saveButton.setEnabled(false);
    }

    private void selectNote(long noteId) {
        for (int index = 0; index < noteListModel.size(); index++) {
            if (noteListModel.get(index).getId() == noteId) {
                noteList.setSelectedIndex(index);
                noteList.ensureIndexIsVisible(index);
                break;
            }
        }
    }

    private Note getSelectedNote() {
        if (selectedNoteId == null) {
            return null;
        }

        return noteService.findNoteById(selectedNoteId).orElse(null);
    }

    private void toggleTheme() {
        currentTheme = currentTheme.opposite();
        applyTheme(currentTheme);
        noteList.repaint();
    }

    private void applyTheme(ThemeMode themeMode) {
        themeToggleButton.setText(themeMode.toggleButtonText());
        themeMode.applyTo(this);
    }

    private void showError(String message) {
        statusLabel.setText(message);
        JOptionPane.showMessageDialog(this, message, ERROR_DIALOG_TITLE, JOptionPane.ERROR_MESSAGE);
    }

    private static String formatNotePreview(Note note, ThemeMode themeMode) {
        String preview = note.getContent().replaceAll("\\s+", " ");
        if (preview.length() > PREVIEW_LIMIT) {
            preview = preview.substring(0, PREVIEW_LIMIT - 1) + "…";
        }

        return String.format(
                "<html><b style='color:%s'>%s</b><br><span style='font-size: 0.9em; color: %s;'>%s</span></html>",
                themeMode.htmlColor(themeMode.foreground()),
                escapeHtml(note.getTitle()),
                themeMode.htmlColor(themeMode.mutedForeground()),
                escapeHtml(preview + " • " + CREATED_FORMATTER.format(note.getCreatedAt())));
    }

    private static String escapeHtml(String value) {
        return value
                .replace("&", "&amp;")
                .replace("<", "&lt;")
                .replace(">", "&gt;")
                .replace("\"", "&quot;");
    }
}