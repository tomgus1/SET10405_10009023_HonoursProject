import '../models/note.dart';

abstract class NoteRepository {
  Future<Note> save(Note note);

  Future<List<Note>> findAll();

  Future<Note?> findById(int id);

  Future<bool> deleteById(int id);
}
