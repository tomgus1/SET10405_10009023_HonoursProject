import AsyncStorage from '@react-native-async-storage/async-storage';
import { Note } from '../models/Note';

const STORAGE_KEY = 'notes-app:notes';

async function readAll(): Promise<Note[]> {
  const raw = await AsyncStorage.getItem(STORAGE_KEY);
  if (!raw) {
    return [];
  }
  return JSON.parse(raw) as Note[];
}

async function writeAll(notes: Note[]): Promise<void> {
  await AsyncStorage.setItem(STORAGE_KEY, JSON.stringify(notes));
}

function sortedById(notes: Note[]): Note[] {
  return [...notes].sort((a, b) => a.id - b.id);
}

export const noteStorage = {
  async save(note: Note): Promise<Note> {
    const notes = await readAll();
    let persisted = note;
    if (note.id === 0) {
      const nextId = notes.reduce((max, existing) => Math.max(max, existing.id), 0) + 1;
      persisted = { ...note, id: nextId };
    }
    const withoutExisting = notes.filter((existing) => existing.id !== persisted.id);
    await writeAll([...withoutExisting, persisted]);
    return persisted;
  },

  async findAll(): Promise<Note[]> {
    return sortedById(await readAll());
  },

  async findById(id: number): Promise<Note | undefined> {
    const notes = await readAll();
    return notes.find((note) => note.id === id);
  },

  async deleteById(id: number): Promise<boolean> {
    const notes = await readAll();
    const remaining = notes.filter((note) => note.id !== id);
    if (remaining.length === notes.length) {
      return false;
    }
    await writeAll(remaining);
    return true;
  },
};
