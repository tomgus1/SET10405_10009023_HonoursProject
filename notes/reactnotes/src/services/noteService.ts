import { Note } from '../models/Note';
import { noteStorage } from '../storage/noteStorage';

function validateText(value: string, fieldName: string): void {
  if (!value || !value.trim()) {
    throw new Error(`${fieldName} must not be blank`);
  }
}

export const noteService = {
  async createNote(title: string, content: string): Promise<Note> {
    validateText(title, 'title');
    validateText(content, 'content');

    const note: Note = {
      id: 0,
      title: title.trim(),
      content: content.trim(),
      createdAt: new Date().toISOString(),
    };
    return noteStorage.save(note);
  },

  async getAllNotes(): Promise<Note[]> {
    return noteStorage.findAll();
  },

  async findNoteById(id: number): Promise<Note | undefined> {
    return noteStorage.findById(id);
  },

  async deleteNote(id: number): Promise<boolean> {
    return noteStorage.deleteById(id);
  },

  async updateNote(id: number, title: string, content: string): Promise<Note> {
    validateText(title, 'title');
    validateText(content, 'content');

    const existing = await noteStorage.findById(id);
    if (!existing) {
      throw new Error(`Note not found: ${id}`);
    }

    const updated: Note = { ...existing, title: title.trim(), content: content.trim() };
    return noteStorage.save(updated);
  },

  async searchNotes(query: string): Promise<Note[]> {
    const normalizedQuery = (query ?? '').trim().toLowerCase();
    if (!normalizedQuery) {
      return noteService.getAllNotes();
    }

    const notes = await noteStorage.findAll();
    return notes.filter(
      (note) =>
        note.title.toLowerCase().includes(normalizedQuery) ||
        note.content.toLowerCase().includes(normalizedQuery)
    );
  },
};
