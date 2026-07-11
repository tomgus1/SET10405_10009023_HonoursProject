use std::collections::HashMap;

use crate::model::Note;

use super::NoteRepository;

#[derive(Default)]
pub struct InMemoryNoteRepository {
    notes: HashMap<u64, Note>,
    next_id: u64,
}

impl InMemoryNoteRepository {
    pub fn new() -> Self {
        Self {
            notes: HashMap::new(),
            next_id: 1,
        }
    }
}

impl NoteRepository for InMemoryNoteRepository {
    fn save(&mut self, note: Note) -> Note {
        let persisted = if note.id == 0 {
            let id = self.next_id;
            self.next_id += 1;
            note.with_id(id)
        } else {
            note
        };
        self.notes.insert(persisted.id, persisted.clone());
        persisted
    }

    fn find_all(&self) -> Vec<Note> {
        let mut all: Vec<Note> = self.notes.values().cloned().collect();
        all.sort_by_key(|note| note.id);
        all
    }

    fn find_by_id(&self, id: u64) -> Option<Note> {
        self.notes.get(&id).cloned()
    }

    fn delete_by_id(&mut self, id: u64) -> bool {
        self.notes.remove(&id).is_some()
    }
}
