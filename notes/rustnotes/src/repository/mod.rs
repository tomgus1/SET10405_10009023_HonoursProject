mod file;
#[cfg(test)]
mod memory;

pub use file::FileNoteRepository;
#[cfg(test)]
pub use memory::InMemoryNoteRepository;

use crate::model::Note;

pub trait NoteRepository {
    fn save(&mut self, note: Note) -> Note;
    fn find_all(&self) -> Vec<Note>;
    fn find_by_id(&self, id: u64) -> Option<Note>;
    fn delete_by_id(&mut self, id: u64) -> bool;
}
