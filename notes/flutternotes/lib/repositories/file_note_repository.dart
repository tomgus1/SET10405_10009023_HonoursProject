import 'dart:convert';
import 'dart:io';

import '../models/note.dart';
import 'note_repository.dart';

class FileNoteRepository implements NoteRepository {
  final File storageFile;
  final Map<int, Note> _notes = {};
  int _nextId = 1;
  bool _loaded = false;

  FileNoteRepository(this.storageFile);

  static Future<FileNoteRepository> createDefault() async {
    final home = Platform.environment['HOME'] ??
        Platform.environment['USERPROFILE'] ??
        Directory.current.path;
    final storageFile =
        File('$home${Platform.pathSeparator}.notes-app${Platform.pathSeparator}notes.json');
    final repository = FileNoteRepository(storageFile);
    await repository._loadFromDisk();
    return repository;
  }

  @override
  Future<Note> save(Note note) async {
    await _ensureLoaded();
    final persisted = note.id == 0 ? note.withId(_nextId++) : note;
    _notes[persisted.id] = persisted;
    await _persistToDisk();
    return persisted;
  }

  @override
  Future<List<Note>> findAll() async {
    await _ensureLoaded();
    final all = _notes.values.toList()..sort((a, b) => a.id.compareTo(b.id));
    return List.unmodifiable(all);
  }

  @override
  Future<Note?> findById(int id) async {
    await _ensureLoaded();
    return _notes[id];
  }

  @override
  Future<bool> deleteById(int id) async {
    await _ensureLoaded();
    final removed = _notes.remove(id) != null;
    if (removed) {
      await _persistToDisk();
    }
    return removed;
  }

  Future<void> _ensureLoaded() async {
    if (!_loaded) {
      await _loadFromDisk();
    }
  }

  Future<void> _loadFromDisk() async {
    _loaded = true;
    if (!await storageFile.exists()) {
      return;
    }

    final content = (await storageFile.readAsString()).trim();
    if (content.isEmpty) {
      return;
    }

    try {
      final decoded = jsonDecode(content);
      if (decoded is! List) {
        throw const FormatException('Invalid JSON note storage file');
      }
      for (final entry in decoded) {
        final note = Note.fromJson(entry as Map<String, dynamic>);
        _notes[note.id] = note;
        _nextId = _nextId > note.id + 1 ? _nextId : note.id + 1;
      }
    } on FormatException {
      await _quarantineCorruptStorage();
    }
  }

  Future<void> _persistToDisk() async {
    final parent = storageFile.parent;
    await parent.create(recursive: true);

    final tempFile = File(
        '${parent.path}${Platform.pathSeparator}notes-${DateTime.now().microsecondsSinceEpoch}.tmp');
    final notes = (await findAll()).map((note) => note.toJson()).toList();
    await tempFile.writeAsString(
        const JsonEncoder.withIndent('  ').convert(notes));
    await tempFile.rename(storageFile.path);
  }

  Future<void> _quarantineCorruptStorage() async {
    if (await storageFile.exists()) {
      await storageFile.rename('${storageFile.path}.corrupt');
    }
    _notes.clear();
    _nextId = 1;
  }
}
