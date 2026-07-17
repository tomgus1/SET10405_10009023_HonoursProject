import 'package:flutter/material.dart';

import 'repositories/file_note_repository.dart';
import 'services/note_service.dart';
import 'ui/app_theme.dart';
import 'ui/notes_home_page.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  final repository = await FileNoteRepository.createDefault();
  runApp(NotesApp(noteService: NoteService(repository)));
}

class NotesApp extends StatefulWidget {
  final NoteService noteService;

  const NotesApp({super.key, required this.noteService});

  @override
  State<NotesApp> createState() => _NotesAppState();
}

class _NotesAppState extends State<NotesApp> {
  ThemeMode _themeMode = ThemeMode.light;

  void _toggleTheme() {
    setState(() {
      _themeMode = _themeMode == ThemeMode.light ? ThemeMode.dark : ThemeMode.light;
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Notes App',
      debugShowCheckedModeBanner: false,
      theme: buildAppTheme(AppPalette.light, Brightness.light),
      darkTheme: buildAppTheme(AppPalette.dark, Brightness.dark),
      themeMode: _themeMode,
      home: NotesHomePage(
        noteService: widget.noteService,
        themeMode: _themeMode,
        onToggleTheme: _toggleTheme,
      ),
    );
  }
}
