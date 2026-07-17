import '../models/note.dart';
import '../repositories/note_repository.dart';

class NoteService {
  final NoteRepository noteRepository;

  NoteService(this.noteRepository);

  Future<Note> createNote(String title, String content) async {
    _validateText(title, 'title');
    _validateText(content, 'content');

    final note = Note(
      id: 0,
      title: title.trim(),
      content: content.trim(),
      createdAt: DateTime.now(),
    );
    return noteRepository.save(note);
  }

  Future<List<Note>> getAllNotes() => noteRepository.findAll();

  Future<Note?> findNoteById(int id) => noteRepository.findById(id);

  Future<bool> deleteNote(int id) => noteRepository.deleteById(id);

  Future<Note> updateNote(int id, String title, String content) async {
    _validateText(title, 'title');
    _validateText(content, 'content');

    final existing = await noteRepository.findById(id);
    if (existing == null) {
      throw ArgumentError('Note not found: $id');
    }

    final updated = Note(
      id: existing.id,
      title: title.trim(),
      content: content.trim(),
      createdAt: existing.createdAt,
    );
    return noteRepository.save(updated);
  }

  Future<List<Note>> searchNotes(String query) async {
    final normalized = query.trim().toLowerCase();
    if (normalized.isEmpty) {
      return getAllNotes();
    }

    final all = await noteRepository.findAll();
    return all
        .where((note) =>
            note.title.toLowerCase().contains(normalized) ||
            note.content.toLowerCase().contains(normalized))
        .toList();
  }

  void _validateText(String value, String fieldName) {
    if (value.trim().isEmpty) {
      throw ArgumentError('$fieldName must not be blank');
    }
  }
}
