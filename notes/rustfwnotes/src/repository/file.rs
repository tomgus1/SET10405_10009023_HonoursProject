use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::model::Note;

use super::NoteRepository;

pub struct FileNoteRepository {
    storage_file: PathBuf,
    notes: HashMap<u64, Note>,
    next_id: u64,
}

impl FileNoteRepository {
    pub fn new(storage_file: impl Into<PathBuf>) -> Self {
        let storage_file = storage_file.into();
        let mut repository = Self {
            storage_file,
            notes: HashMap::new(),
            next_id: 1,
        };
        repository.load_from_disk();
        repository
    }

    pub fn create_default() -> Self {
        let home = Self::home_dir().expect("could not determine the user's home directory");
        let storage_dir = home.join(".notes-app");
        Self::new(storage_dir.join("notes.json"))
    }

    fn home_dir() -> Option<PathBuf> {
        // Unix sets HOME; Windows sets USERPROFILE instead.
        std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .map(PathBuf::from)
    }

    fn load_from_disk(&mut self) {
        if !self.storage_file.exists() {
            return;
        }

        let contents = fs::read_to_string(&self.storage_file)
            .unwrap_or_else(|error| panic!("failed to read notes from disk: {error}"));
        if contents.trim().is_empty() {
            return;
        }

        let notes: Vec<Note> = match serde_json::from_str(&contents) {
            Ok(notes) => notes,
            Err(error) => {
                eprintln!(
                    "warning: {} is not a valid notes file ({error}); moving it aside and starting with an empty notes list",
                    self.storage_file.display()
                );
                let backup_file = self.storage_file.with_extension("json.invalid");
                let _ = fs::rename(&self.storage_file, &backup_file);
                return;
            }
        };
        for note in notes {
            self.next_id = self.next_id.max(note.id + 1);
            self.notes.insert(note.id, note);
        }
    }

    fn persist_to_disk(&self) {
        if let Some(parent) = self.storage_file.parent() {
            fs::create_dir_all(parent)
                .unwrap_or_else(|error| panic!("failed to create notes directory: {error}"));
        }

        let notes = self.find_all();
        let json = serde_json::to_string_pretty(&notes)
            .unwrap_or_else(|error| panic!("failed to serialize notes: {error}"));

        let temp_file = self.storage_file.with_extension("json.tmp");
        fs::write(&temp_file, json)
            .unwrap_or_else(|error| panic!("failed to write temporary notes file: {error}"));
        fs::rename(&temp_file, &self.storage_file)
            .unwrap_or_else(|error| panic!("failed to move notes file into place: {error}"));
    }
}

impl NoteRepository for FileNoteRepository {
    fn save(&mut self, note: Note) -> Note {
        let persisted = if note.id == 0 {
            let id = self.next_id;
            self.next_id += 1;
            note.with_id(id)
        } else {
            note
        };
        self.notes.insert(persisted.id, persisted.clone());
        self.persist_to_disk();
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
        let removed = self.notes.remove(&id).is_some();
        if removed {
            self.persist_to_disk();
        }
        removed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn sample_time() -> chrono::NaiveDateTime {
        NaiveDate::from_ymd_opt(2026, 8, 9)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap()
    }

    #[test]
    fn persists_notes_across_instances() {
        let dir = tempfile::tempdir().unwrap();
        let storage_file = dir.path().join("notes.json");

        let mut first_repository = FileNoteRepository::new(&storage_file);
        let created_note = first_repository.save(Note::new(0, "Title", "Content", sample_time()));

        let second_repository = FileNoteRepository::new(&storage_file);

        assert_eq!(1, created_note.id);
        assert_eq!(1, second_repository.find_all().len());
        assert_eq!("Title", second_repository.find_by_id(created_note.id).unwrap().title);
    }

    #[test]
    fn deletes_notes_and_persists_removal() {
        let dir = tempfile::tempdir().unwrap();
        let storage_file = dir.path().join("notes.json");

        let mut repository = FileNoteRepository::new(&storage_file);
        let created_note = repository.save(Note::new(0, "Title", "Content", sample_time()));

        assert!(repository.delete_by_id(created_note.id));
        assert!(!repository.delete_by_id(created_note.id));

        let reloaded_repository = FileNoteRepository::new(&storage_file);
        assert!(reloaded_repository.find_all().is_empty());
    }
}
