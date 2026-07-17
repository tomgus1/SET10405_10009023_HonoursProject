import 'package:flutter_test/flutter_test.dart';
import 'package:notes_app/repositories/in_memory_note_repository.dart';
import 'package:notes_app/services/note_service.dart';

void main() {
  late NoteService noteService;

  setUp(() {
    noteService = NoteService(InMemoryNoteRepository());
  });

  test('creates notes with generated ids', () async {
    final note = await noteService.createNote('Shopping', 'Buy milk');

    expect(note.id, 1);
    expect(note.title, 'Shopping');
    expect(note.content, 'Buy milk');
  });

  test('searches by title and content', () async {
    await noteService.createNote('Shopping', 'Buy milk');
    await noteService.createNote('Work', 'Finish report');

    expect((await noteService.searchNotes('milk')).length, 1);
    expect((await noteService.searchNotes('work')).length, 1);
  });

  test('deletes existing notes', () async {
    final note = await noteService.createNote('Task', 'Do it');

    expect(await noteService.deleteNote(note.id), isTrue);
    expect(await noteService.deleteNote(note.id), isFalse);
  });

  test('updates existing notes without changing creation time', () async {
    final note = await noteService.createNote('Task', 'Do it');
    final updated = await noteService.updateNote(note.id, 'Task updated', 'Do it better');

    expect(updated.id, note.id);
    expect(updated.createdAt, note.createdAt);
    expect(updated.title, 'Task updated');
    expect(updated.content, 'Do it better');
  });

  test('rejects unknown notes when updating', () async {
    expect(
      () => noteService.updateNote(99, 'Title', 'Body'),
      throwsArgumentError,
    );
  });

  test('rejects blank titles', () async {
    expect(
      () => noteService.createNote(' ', 'Body'),
      throwsArgumentError,
    );
  });
}
