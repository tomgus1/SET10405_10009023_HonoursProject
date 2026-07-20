import { StatusBar } from 'expo-status-bar';
import { useCallback, useEffect, useMemo, useState } from 'react';
import {
  Alert,
  FlatList,
  Pressable,
  SafeAreaView,
  StyleSheet,
  Text,
  TextInput,
  useWindowDimensions,
  View,
} from 'react-native';

import { Note } from './src/models/Note';
import { noteService } from './src/services/noteService';
import { opposite, Theme, themes, ThemeName } from './src/theme/theme';

const PREVIEW_LIMIT = 96;

function formatPreview(note: Note): string {
  const collapsed = note.content.replace(/\s+/g, ' ');
  const preview = collapsed.length > PREVIEW_LIMIT ? `${collapsed.slice(0, PREVIEW_LIMIT - 1)}…` : collapsed;
  const created = new Date(note.createdAt);
  const createdLabel = Number.isNaN(created.getTime())
    ? note.createdAt
    : created.toLocaleString(undefined, {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
      });
  return `${preview} • ${createdLabel}`;
}

export default function App() {
  const { width } = useWindowDimensions();
  const isWide = width >= 820;

  const [themeName, setThemeName] = useState<ThemeName>('light');
  const theme = themes[themeName];

  const [notes, setNotes] = useState<Note[]>([]);
  const [selectedNoteId, setSelectedNoteId] = useState<number | null>(null);
  const [titleInput, setTitleInput] = useState('');
  const [contentInput, setContentInput] = useState('');
  const [searchQuery, setSearchQuery] = useState('');
  const [status, setStatus] = useState('Ready');

  const refreshNotes = useCallback(async () => {
    const allNotes = await noteService.getAllNotes();
    setNotes(allNotes);
    setStatus(allNotes.length === 0 ? 'No notes yet. Create your first note.' : `Loaded ${allNotes.length} notes`);
    return allNotes;
  }, []);

  useEffect(() => {
    refreshNotes();
  }, [refreshNotes]);

  const clearEditor = useCallback(() => {
    setSelectedNoteId(null);
    setTitleInput('');
    setContentInput('');
  }, []);

  const selectNote = useCallback((note: Note) => {
    setSelectedNoteId(note.id);
    setTitleInput(note.title);
    setContentInput(note.content);
  }, []);

  const handleAdd = useCallback(async () => {
    try {
      const note = await noteService.createNote(titleInput, contentInput);
      setStatus(`Created note ${note.id}`);
      clearEditor();
      const allNotes = await refreshNotes();
      const created = allNotes.find((n) => n.id === note.id);
      if (created) {
        selectNote(created);
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      setStatus(message);
      Alert.alert('Notes App', message);
    }
  }, [titleInput, contentInput, clearEditor, refreshNotes, selectNote]);

  const handleSave = useCallback(async () => {
    if (selectedNoteId === null) {
      setStatus('Select a note to edit.');
      Alert.alert('Notes App', 'Select a note to edit.');
      return;
    }
    try {
      const updated = await noteService.updateNote(selectedNoteId, titleInput, contentInput);
      setStatus(`Updated note ${updated.id}`);
      const allNotes = await refreshNotes();
      const found = allNotes.find((n) => n.id === updated.id);
      if (found) {
        selectNote(found);
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      setStatus(message);
      Alert.alert('Notes App', message);
    }
  }, [selectedNoteId, titleInput, contentInput, refreshNotes, selectNote]);

  const handleDelete = useCallback(() => {
    if (selectedNoteId === null) {
      setStatus('Select a note to delete.');
      Alert.alert('Notes App', 'Select a note to delete.');
      return;
    }
    const idToDelete = selectedNoteId;
    Alert.alert('Confirm delete', 'Delete the selected note?', [
      { text: 'Cancel', style: 'cancel' },
      {
        text: 'Delete',
        style: 'destructive',
        onPress: async () => {
          const deleted = await noteService.deleteNote(idToDelete);
          setStatus(deleted ? `Deleted note ${idToDelete}` : 'Note was not found');
          clearEditor();
          await refreshNotes();
        },
      },
    ]);
  }, [selectedNoteId, clearEditor, refreshNotes]);

  const handleSearch = useCallback(async () => {
    const results = await noteService.searchNotes(searchQuery);
    setNotes(results);
    setStatus(results.length === 0 ? 'No notes match your search.' : `Showing ${results.length} notes`);
  }, [searchQuery]);

  const toggleTheme = useCallback(() => {
    setThemeName((current) => opposite(current));
  }, []);

  const styles = useMemo(() => createStyles(theme), [theme]);

  return (
    <SafeAreaView style={styles.safeArea}>
      <StatusBar style={themeName === 'light' ? 'dark' : 'light'} />
      <View style={styles.header}>
        <View>
          <Text style={styles.title}>Notes App</Text>
          <Text style={styles.tagline}>Store, search, and manage notes locally</Text>
        </View>
        <Pressable style={styles.button} onPress={toggleTheme}>
          <Text style={styles.buttonText}>{theme.toggleLabel}</Text>
        </Pressable>
      </View>

      <View style={[styles.content, isWide && styles.contentWide]}>
        <View style={[styles.sidebar, isWide && styles.sidebarWide]}>
          <Text style={styles.sectionLabel}>All Notes</Text>
          <View style={styles.searchRow}>
            <TextInput
              style={styles.input}
              placeholder="Search notes"
              placeholderTextColor={theme.mutedForeground}
              value={searchQuery}
              onChangeText={setSearchQuery}
              onSubmitEditing={handleSearch}
            />
            <Pressable style={styles.button} onPress={handleSearch}>
              <Text style={styles.buttonText}>Search</Text>
            </Pressable>
          </View>

          <FlatList
            style={styles.list}
            data={notes}
            keyExtractor={(note) => String(note.id)}
            ListEmptyComponent={<Text style={styles.emptyText}>No notes yet. Create your first note.</Text>}
            renderItem={({ item }) => {
              const isSelected = item.id === selectedNoteId;
              return (
                <Pressable
                  style={[styles.listItem, isSelected && styles.listItemSelected]}
                  onPress={() => selectNote(item)}
                >
                  <Text style={styles.listItemTitle}>{item.title}</Text>
                  <Text style={styles.listItemPreview}>{formatPreview(item)}</Text>
                </Pressable>
              );
            }}
          />
        </View>

        <View style={[styles.editor, isWide && styles.editorWide]}>
          <Text style={styles.sectionLabel}>Note Details</Text>
          <Text style={styles.fieldLabel}>Title</Text>
          <TextInput
            style={styles.input}
            value={titleInput}
            onChangeText={setTitleInput}
            placeholder="Note title"
            placeholderTextColor={theme.mutedForeground}
          />
          <Text style={styles.fieldLabel}>Content</Text>
          <TextInput
            style={[styles.input, styles.contentInput]}
            value={contentInput}
            onChangeText={setContentInput}
            placeholder="Note content"
            placeholderTextColor={theme.mutedForeground}
            multiline
            textAlignVertical="top"
          />

          <View style={styles.buttonRow}>
            <Pressable style={styles.button} onPress={handleAdd}>
              <Text style={styles.buttonText}>Add Note</Text>
            </Pressable>
            <Pressable
              style={[styles.button, selectedNoteId === null && styles.buttonDisabled]}
              onPress={handleSave}
              disabled={selectedNoteId === null}
            >
              <Text style={styles.buttonText}>Save Changes</Text>
            </Pressable>
            <Pressable style={styles.button} onPress={handleDelete}>
              <Text style={styles.buttonText}>Delete Selected</Text>
            </Pressable>
            <Pressable style={styles.button} onPress={clearEditor}>
              <Text style={styles.buttonText}>Clear Fields</Text>
            </Pressable>
            <Pressable style={styles.button} onPress={refreshNotes}>
              <Text style={styles.buttonText}>Refresh</Text>
            </Pressable>
          </View>
        </View>
      </View>

      <View style={styles.footer}>
        <Text style={styles.statusText}>{status}</Text>
      </View>
    </SafeAreaView>
  );
}

function createStyles(theme: Theme) {
  return StyleSheet.create({
    safeArea: {
      flex: 1,
      backgroundColor: theme.background,
    },
    header: {
      flexDirection: 'row',
      justifyContent: 'space-between',
      alignItems: 'flex-start',
      padding: 16,
    },
    title: {
      fontSize: 24,
      fontWeight: '700',
      color: theme.foreground,
    },
    tagline: {
      marginTop: 4,
      color: theme.mutedForeground,
    },
    content: {
      flex: 1,
      paddingHorizontal: 16,
      gap: 16,
    },
    contentWide: {
      flexDirection: 'row',
    },
    sidebar: {
      flex: 1,
      minHeight: 200,
    },
    sidebarWide: {
      flex: 0.4,
    },
    editor: {
      flex: 1,
    },
    editorWide: {
      flex: 0.6,
    },
    sectionLabel: {
      fontSize: 14,
      fontWeight: '700',
      color: theme.foreground,
      marginBottom: 8,
    },
    searchRow: {
      flexDirection: 'row',
      gap: 8,
      marginBottom: 8,
    },
    input: {
      flex: 1,
      borderWidth: 1,
      borderColor: theme.borderColor,
      backgroundColor: theme.panelBackground,
      color: theme.foreground,
      borderRadius: 6,
      paddingHorizontal: 10,
      paddingVertical: 8,
    },
    contentInput: {
      flex: 1,
      minHeight: 160,
      marginBottom: 12,
    },
    fieldLabel: {
      color: theme.mutedForeground,
      marginBottom: 4,
      marginTop: 8,
    },
    list: {
      flex: 1,
      borderWidth: 1,
      borderColor: theme.borderColor,
      borderRadius: 6,
      backgroundColor: theme.panelBackground,
    },
    listItem: {
      padding: 10,
      borderBottomWidth: 1,
      borderBottomColor: theme.borderColor,
    },
    listItemSelected: {
      backgroundColor: theme.selectionBackground,
    },
    listItemTitle: {
      fontWeight: '700',
      color: theme.foreground,
    },
    listItemPreview: {
      marginTop: 2,
      fontSize: 12,
      color: theme.mutedForeground,
    },
    emptyText: {
      padding: 16,
      color: theme.mutedForeground,
    },
    buttonRow: {
      flexDirection: 'row',
      flexWrap: 'wrap',
      gap: 8,
      marginTop: 4,
    },
    button: {
      borderWidth: 1,
      borderColor: theme.borderColor,
      backgroundColor: theme.buttonBackground,
      borderRadius: 6,
      paddingHorizontal: 12,
      paddingVertical: 8,
    },
    buttonDisabled: {
      opacity: 0.5,
    },
    buttonText: {
      color: theme.buttonForeground,
      fontWeight: '600',
    },
    footer: {
      paddingHorizontal: 16,
      paddingVertical: 12,
    },
    statusText: {
      color: theme.mutedForeground,
    },
  });
}
