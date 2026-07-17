import 'package:flutter/material.dart';
import 'package:intl/intl.dart';

import '../models/note.dart';
import '../services/note_service.dart';
import 'app_theme.dart';

const _emptyListMessage = 'No notes yet. Create your first note on the right.';
const _readyMessage = 'Ready';
const _emptySelectionMessage = 'Select a note to delete.';
const _previewLimit = 96;
final _createdFormatter = DateFormat('yyyy-MM-dd HH:mm');

class NotesHomePage extends StatefulWidget {
  final NoteService noteService;
  final ThemeMode themeMode;
  final VoidCallback onToggleTheme;

  const NotesHomePage({
    super.key,
    required this.noteService,
    required this.themeMode,
    required this.onToggleTheme,
  });

  @override
  State<NotesHomePage> createState() => _NotesHomePageState();
}

class _NotesHomePageState extends State<NotesHomePage> {
  final _titleController = TextEditingController();
  final _contentController = TextEditingController();
  final _searchController = TextEditingController();

  List<Note> _notes = [];
  int? _selectedNoteId;
  String _statusMessage = _readyMessage;

  @override
  void initState() {
    super.initState();
    _refreshNotes();
  }

  @override
  void dispose() {
    _titleController.dispose();
    _contentController.dispose();
    _searchController.dispose();
    super.dispose();
  }

  AppPalette get _palette =>
      widget.themeMode == ThemeMode.dark ? AppPalette.dark : AppPalette.light;

  Future<void> _refreshNotes() async {
    final notes = await widget.noteService.getAllNotes();
    setState(() {
      _notes = notes;
      _statusMessage = notes.isEmpty
          ? _emptyListMessage
          : 'Loaded ${notes.length} notes';
    });
    if (notes.isNotEmpty) {
      _selectNote(notes.first.id);
    } else {
      _clearEditor();
    }
  }

  Future<void> _applySearch() async {
    final notes = await widget.noteService.searchNotes(_searchController.text);
    setState(() {
      _notes = notes;
      _statusMessage =
          notes.isEmpty ? 'No notes match your search.' : 'Showing ${notes.length} notes';
    });
    if (notes.isNotEmpty) {
      _selectNote(notes.first.id);
    } else {
      _clearEditor();
    }
  }

  Future<void> _addNote() async {
    try {
      final note = await widget.noteService
          .createNote(_titleController.text, _contentController.text);
      setState(() => _statusMessage = 'Created note ${note.id}');
      await _refreshNotes();
      _selectNote(note.id);
    } on ArgumentError catch (error) {
      _showError(error.message.toString());
    }
  }

  Future<void> _saveChanges() async {
    final selected = _selectedNoteId;
    if (selected == null) {
      _showError('Select a note to edit.');
      return;
    }

    try {
      final updated = await widget.noteService
          .updateNote(selected, _titleController.text, _contentController.text);
      setState(() => _statusMessage = 'Updated note ${updated.id}');
      await _refreshNotes();
      _selectNote(updated.id);
    } on ArgumentError catch (error) {
      _showError(error.message.toString());
    }
  }

  Future<void> _deleteSelectedNote() async {
    final selected = _selectedNoteId;
    if (selected == null) {
      _showError(_emptySelectionMessage);
      return;
    }

    final confirmed = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Confirm delete'),
        content: const Text('Delete the selected note?'),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(false),
            child: const Text('No'),
          ),
          TextButton(
            onPressed: () => Navigator.of(context).pop(true),
            child: const Text('Yes'),
          ),
        ],
      ),
    );

    if (confirmed != true) {
      return;
    }

    final deleted = await widget.noteService.deleteNote(selected);
    setState(() {
      _statusMessage = deleted ? 'Deleted note $selected' : 'Note was not found';
    });
    await _refreshNotes();
  }

  void _selectNote(int noteId) {
    final note = _notes.where((n) => n.id == noteId).firstOrNull;
    if (note == null) {
      _clearEditor();
      return;
    }
    setState(() {
      _selectedNoteId = note.id;
      _titleController.text = note.title;
      _contentController.text = note.content;
    });
  }

  void _clearEditor() {
    setState(() {
      _selectedNoteId = null;
      _titleController.text = '';
      _contentController.text = '';
    });
  }

  void _showError(String message) {
    setState(() => _statusMessage = message);
    showDialog<void>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Notes App'),
        content: Text(message),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('OK'),
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final palette = _palette;
    return Scaffold(
      backgroundColor: palette.background,
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            _buildHeader(palette),
            const SizedBox(height: 8),
            Expanded(
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  SizedBox(width: 380, child: _buildSidebar(palette)),
                  const SizedBox(width: 16),
                  Expanded(child: _buildEditor(palette)),
                ],
              ),
            ),
            const SizedBox(height: 8),
            _buildFooter(palette),
          ],
        ),
      ),
    );
  }

  Widget _buildHeader(AppPalette palette) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                'Notes App',
                style: TextStyle(
                  fontSize: 26,
                  fontWeight: FontWeight.bold,
                  color: palette.foreground,
                ),
              ),
              const SizedBox(height: 4),
              Text(
                'Store, search, and manage notes locally',
                style: TextStyle(color: palette.mutedForeground),
              ),
            ],
          ),
        ),
        ElevatedButton(
          onPressed: widget.onToggleTheme,
          child: Text(widget.themeMode == ThemeMode.dark ? 'Light mode' : 'Dark mode'),
        ),
      ],
    );
  }

  Widget _buildSidebar(AppPalette palette) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Text(
          'All Notes',
          style: TextStyle(
            fontWeight: FontWeight.bold,
            fontSize: 14,
            color: palette.foreground,
          ),
        ),
        const SizedBox(height: 8),
        Row(
          children: [
            Expanded(
              child: TextField(
                controller: _searchController,
                onSubmitted: (_) => _applySearch(),
              ),
            ),
            const SizedBox(width: 8),
            ElevatedButton(
              onPressed: _applySearch,
              child: const Text('Search'),
            ),
          ],
        ),
        const SizedBox(height: 8),
        Expanded(
          child: Container(
            decoration: BoxDecoration(
              color: palette.panelBackground,
              border: Border.all(color: palette.borderColor),
              borderRadius: BorderRadius.circular(6),
            ),
            child: _notes.isEmpty
                ? Center(
                    child: Padding(
                      padding: const EdgeInsets.all(16),
                      child: Text(
                        _emptyListMessage,
                        style: TextStyle(color: palette.mutedForeground),
                        textAlign: TextAlign.center,
                      ),
                    ),
                  )
                : ListView.builder(
                    itemCount: _notes.length,
                    itemBuilder: (context, index) {
                      final note = _notes[index];
                      final isSelected = note.id == _selectedNoteId;
                      return InkWell(
                        onTap: () => _selectNote(note.id),
                        child: Container(
                          color: isSelected
                              ? palette.selectionBackground
                              : palette.panelBackground,
                          padding: const EdgeInsets.all(10),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                note.title,
                                style: TextStyle(
                                  fontWeight: FontWeight.bold,
                                  color: isSelected
                                      ? palette.selectionForeground
                                      : palette.foreground,
                                ),
                              ),
                              Text(
                                _formatPreview(note),
                                style: TextStyle(
                                  fontSize: 12,
                                  color: isSelected
                                      ? palette.selectionForeground
                                      : palette.mutedForeground,
                                ),
                              ),
                            ],
                          ),
                        ),
                      );
                    },
                  ),
          ),
        ),
      ],
    );
  }

  Widget _buildEditor(AppPalette palette) {
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        border: Border.all(color: palette.borderColor),
        borderRadius: BorderRadius.circular(6),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Text('Note Details',
              style: TextStyle(fontWeight: FontWeight.bold, color: palette.foreground)),
          const SizedBox(height: 12),
          Text('Title', style: TextStyle(color: palette.foreground)),
          const SizedBox(height: 4),
          TextField(controller: _titleController),
          const SizedBox(height: 12),
          Text('Content', style: TextStyle(color: palette.foreground)),
          const SizedBox(height: 4),
          Expanded(
            child: TextField(
              controller: _contentController,
              maxLines: null,
              expands: true,
              textAlignVertical: TextAlignVertical.top,
            ),
          ),
          const SizedBox(height: 12),
          Wrap(
            spacing: 8,
            runSpacing: 8,
            children: [
              ElevatedButton(onPressed: _addNote, child: const Text('Add Note')),
              ElevatedButton(
                onPressed: _selectedNoteId == null ? null : _saveChanges,
                child: const Text('Save Changes'),
              ),
              ElevatedButton(
                onPressed: _deleteSelectedNote,
                child: const Text('Delete Selected'),
              ),
              ElevatedButton(onPressed: _clearEditor, child: const Text('Clear Fields')),
              ElevatedButton(onPressed: _refreshNotes, child: const Text('Refresh')),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildFooter(AppPalette palette) {
    return Text(_statusMessage, style: TextStyle(color: palette.mutedForeground));
  }

  static String _formatPreview(Note note) {
    var preview = note.content.replaceAll(RegExp(r'\s+'), ' ');
    if (preview.length > _previewLimit) {
      preview = '${preview.substring(0, _previewLimit - 1)}…';
    }
    return '$preview • ${_createdFormatter.format(note.createdAt)}';
  }
}

extension _FirstOrNull<T> on Iterable<T> {
  T? get firstOrNull => isEmpty ? null : first;
}
