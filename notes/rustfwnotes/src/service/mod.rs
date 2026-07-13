use chrono::Local;

use crate::model::Note;
use crate::repository::NoteRepository;

pub struct NoteService<R: NoteRepository> {
    note_repository: R,
}

impl<R: NoteRepository> NoteService<R> {
    pub fn new(note_repository: R) -> Self {
        Self { note_repository }
    }

    pub fn create_note(&mut self, title: &str, content: &str) -> Result<Note, String> {
        validate_text(title, "title")?;
        validate_text(content, "content")?;

        let note = Note::new(0, title.trim(), content.trim(), Local::now().naive_local());
        Ok(self.note_repository.save(note))
    }

    pub fn update_note(&mut self, id: u64, title: &str, content: &str) -> Result<Note, String> {
        validate_text(title, "title")?;
        validate_text(content, "content")?;

        let existing = self
            .note_repository
            .find_by_id(id)
            .ok_or_else(|| format!("note {id} not found"))?;

        let updated = Note::new(id, title.trim(), content.trim(), existing.created_at);
        Ok(self.note_repository.save(updated))
    }

    pub fn get_all_notes(&self) -> Vec<Note> {
        self.note_repository.find_all()
    }

    pub fn find_note_by_id(&self, id: u64) -> Option<Note> {
        self.note_repository.find_by_id(id)
    }

    pub fn delete_note(&mut self, id: u64) -> bool {
        self.note_repository.delete_by_id(id)
    }

    pub fn search_notes(&self, query: &str) -> Vec<Note> {
        let normalized_query = query.trim().to_lowercase();
        if normalized_query.is_empty() {
            return self.get_all_notes();
        }

        self.note_repository
            .find_all()
            .into_iter()
            .filter(|note| {
                note.title.to_lowercase().contains(&normalized_query)
                    || note.content.to_lowercase().contains(&normalized_query)
            })
            .collect()
    }
}

fn validate_text(value: &str, field_name: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(format!("{field_name} must not be blank"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::InMemoryNoteRepository;

    fn service() -> NoteService<InMemoryNoteRepository> {
        NoteService::new(InMemoryNoteRepository::new())
    }

    #[test]
    fn creates_notes_with_generated_ids() {
        let mut service = service();
        let note = service.create_note("Shopping", "Buy milk").unwrap();

        assert_eq!(1, note.id);
        assert_eq!("Shopping", note.title);
        assert_eq!("Buy milk", note.content);
    }

    #[test]
    fn searches_by_title_and_content() {
        let mut service = service();
        service.create_note("Shopping", "Buy milk").unwrap();
        service.create_note("Work", "Finish report").unwrap();

        assert_eq!(1, service.search_notes("milk").len());
        assert_eq!(1, service.search_notes("work").len());
    }

    #[test]
    fn deletes_existing_notes() {
        let mut service = service();
        let note = service.create_note("Task", "Do it").unwrap();

        assert!(service.delete_note(note.id));
        assert!(!service.delete_note(note.id));
    }

    #[test]
    fn rejects_blank_titles() {
        let mut service = service();
        assert!(service.create_note(" ", "Body").is_err());
    }

    #[test]
    fn updates_existing_notes() {
        let mut service = service();
        let note = service.create_note("Task", "Do it").unwrap();

        let updated = service.update_note(note.id, "Task v2", "Do it now").unwrap();

        assert_eq!(note.id, updated.id);
        assert_eq!("Task v2", updated.title);
        assert_eq!("Do it now", updated.content);
        assert_eq!(note.created_at, updated.created_at);
    }

    #[test]
    fn rejects_update_for_missing_note() {
        let mut service = service();
        assert!(service.update_note(42, "Title", "Body").is_err());
    }
}
