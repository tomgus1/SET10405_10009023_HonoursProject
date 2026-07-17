import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:notes_app/models/note.dart';
import 'package:notes_app/repositories/file_note_repository.dart';

void main() {
  late Directory tempDir;

  setUp(() async {
    tempDir = await Directory.systemTemp.createTemp('notes_app_test');
  });

  tearDown(() async {
    await tempDir.delete(recursive: true);
  });

  test('persists notes across instances as json', () async {
    final storageFile = File('${tempDir.path}/notes.json');

    final firstRepository = FileNoteRepository(storageFile);
    final createdNote = await firstRepository.save(Note(
      id: 0,
      title: 'Title',
      content: 'Content',
      createdAt: DateTime(2026, 8, 9, 12, 0),
    ));

    final secondRepository = FileNoteRepository(storageFile);

    expect(createdNote.id, 1);
    expect((await secondRepository.findAll()).length, 1);
    expect((await secondRepository.findById(createdNote.id))!.title, 'Title');
  });

  test('edits notes and persists removal', () async {
    final storageFile = File('${tempDir.path}/notes.json');
    final repository = FileNoteRepository(storageFile);
    final createdNote = await repository.save(Note(
      id: 0,
      title: 'Title',
      content: 'Content',
      createdAt: DateTime(2026, 8, 9, 12, 0),
    ));

    await repository.save(Note(
      id: createdNote.id,
      title: 'Updated title',
      content: 'Updated content',
      createdAt: createdNote.createdAt,
    ));

    final reloadedRepository = FileNoteRepository(storageFile);
    expect((await reloadedRepository.findById(createdNote.id))!.title, 'Updated title');

    expect(await repository.deleteById(createdNote.id), isTrue);
    expect(await repository.deleteById(createdNote.id), isFalse);

    final afterDeleteRepository = FileNoteRepository(storageFile);
    expect(await afterDeleteRepository.findAll(), isEmpty);
  });

  test('round trips special characters through json', () async {
    final storageFile = File('${tempDir.path}/notes.json');
    final repository = FileNoteRepository(storageFile);
    const title = 'Quotes "and" slashes \\';
    const content = 'Line 1\nLine 2\nTabbed\tvalue';

    final createdNote = await repository.save(Note(
      id: 0,
      title: title,
      content: content,
      createdAt: DateTime(2026, 8, 9, 12, 0),
    ));

    final reloadedRepository = FileNoteRepository(storageFile);
    final reloadedNote = await reloadedRepository.findById(createdNote.id);

    expect(reloadedNote!.title, title);
    expect(reloadedNote.content, content);
  });
}
