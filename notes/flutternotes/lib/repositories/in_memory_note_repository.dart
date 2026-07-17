import '../models/note.dart';
import 'note_repository.dart';

class InMemoryNoteRepository implements NoteRepository {
  final Map<int, Note> _notes = {};
  int _nextId = 1;

  @override
  Future<Note> save(Note note) async {
    final persisted = note.id == 0 ? note.withId(_nextId++) : note;
    _notes[persisted.id] = persisted;
    return persisted;
  }

  @override
  Future<List<Note>> findAll() async {
    final all = _notes.values.toList()..sort((a, b) => a.id.compareTo(b.id));
    return List.unmodifiable(all);
  }

  @override
  Future<Note?> findById(int id) async => _notes[id];

  @override
  Future<bool> deleteById(int id) async => _notes.remove(id) != null;
}
