use gpui::{AppContext, Context, Entity, SharedString};

use crate::framework::state::Signal;
use crate::framework::widgets::TextInputState;
use crate::model::Note;
use crate::repository::FileNoteRepository;
use crate::service::NoteService;

const EMPTY_LIST_MESSAGE: &str = "No notes yet. Create your first note on the right.";
const READY_MESSAGE: &str = "Ready";

/// Owns application state and behaviour; views read from it and dispatch
/// intents into it, but never talk to `NoteService`/`NoteRepository`
/// directly. State is split into independent `Signal`s (rather than one
/// struct with many fields behind a single `Entity`) so that a view which
/// only watches, say, `status()` is not re-rendered when `notes()` changes.
pub struct NotesViewModel {
    note_service: NoteService<FileNoteRepository>,
    notes: Signal<Vec<Note>>,
    selected_id: Signal<Option<u64>>,
    status: Signal<SharedString>,
    title_input: Entity<TextInputState>,
    content_input: Entity<TextInputState>,
    search_input: Entity<TextInputState>,
    _subscriptions: Vec<gpui::Subscription>,
}

impl NotesViewModel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let note_service = NoteService::new(FileNoteRepository::create_default());

        let title_input = cx.new(|cx| TextInputState::new(cx).placeholder("Title"));
        let content_input = cx.new(|cx| {
            TextInputState::new(cx)
                .multi_line(true)
                .rows(10)
                .placeholder("Write your note here...")
        });
        let search_input = cx.new(|cx| TextInputState::new(cx).placeholder("Search notes..."));

        let notes = Signal::new(cx, Vec::new());
        let selected_id = Signal::new(cx, None);
        let status = Signal::new(cx, SharedString::from(READY_MESSAGE));

        let _subscriptions = vec![cx.observe(&search_input, |this, _search_input, cx| {
            this.apply_search(cx);
        })];

        let mut view_model = Self {
            note_service,
            notes,
            selected_id,
            status,
            title_input,
            content_input,
            search_input,
            _subscriptions,
        };
        view_model.refresh_notes(cx);
        view_model
    }

    pub fn notes(&self) -> &Signal<Vec<Note>> {
        &self.notes
    }

    pub fn selected_id(&self) -> &Signal<Option<u64>> {
        &self.selected_id
    }

    pub fn status(&self) -> &Signal<SharedString> {
        &self.status
    }

    pub fn title_input(&self) -> &Entity<TextInputState> {
        &self.title_input
    }

    pub fn content_input(&self) -> &Entity<TextInputState> {
        &self.content_input
    }

    pub fn search_input(&self) -> &Entity<TextInputState> {
        &self.search_input
    }

    pub fn refresh_notes(&mut self, cx: &mut Context<Self>) {
        let notes = self.note_service.get_all_notes();
        let status = if notes.is_empty() {
            SharedString::from(EMPTY_LIST_MESSAGE)
        } else {
            SharedString::from(format!("Loaded {} notes", notes.len()))
        };
        self.notes.set(cx, notes);
        self.status.set(cx, status);
    }

    pub fn apply_search(&mut self, cx: &mut Context<Self>) {
        let query = self.search_input.read(cx).value().to_string();
        let notes = self.note_service.search_notes(&query);
        self.status.set(cx, SharedString::from(format!("Showing {} notes", notes.len())));
        self.notes.set(cx, notes);
    }

    pub fn add_note(&mut self, cx: &mut Context<Self>) {
        let title = self.title_input.read(cx).value().to_string();
        let content = self.content_input.read(cx).value().to_string();

        match self.note_service.create_note(&title, &content) {
            Ok(note) => {
                self.status.set(cx, SharedString::from(format!("Created note {}", note.id)));
                self.clear_editor(cx);
                self.refresh_notes(cx);
                self.select_note(note.id, cx);
            }
            Err(message) => self.show_error(message, cx),
        }
    }

    pub fn update_selected_note(&mut self, cx: &mut Context<Self>) {
        let Some(id) = *self.selected_id.read(cx) else {
            self.show_error("Select a note to update.".to_string(), cx);
            return;
        };

        let title = self.title_input.read(cx).value().to_string();
        let content = self.content_input.read(cx).value().to_string();

        match self.note_service.update_note(id, &title, &content) {
            Ok(note) => {
                self.status.set(cx, SharedString::from(format!("Updated note {}", note.id)));
                self.refresh_notes(cx);
                self.select_note(note.id, cx);
            }
            Err(message) => self.show_error(message, cx),
        }
    }

    pub fn delete_note(&mut self, id: u64, cx: &mut Context<Self>) {
        let deleted = self.note_service.delete_note(id);
        self.status.set(
            cx,
            SharedString::from(if deleted {
                format!("Deleted note {id}")
            } else {
                "Note was not found".to_string()
            }),
        );
        self.selected_id.set(cx, None);
        self.refresh_notes(cx);
    }

    pub fn select_note(&mut self, id: u64, cx: &mut Context<Self>) {
        if let Some(note) = self.note_service.find_note_by_id(id) {
            self.selected_id.set(cx, Some(note.id));
            self.title_input.update(cx, |input, cx| input.set_value(note.title.clone(), cx));
            self.content_input
                .update(cx, |input, cx| input.set_value(note.content.clone(), cx));
        }
    }

    pub fn clear_editor(&mut self, cx: &mut Context<Self>) {
        self.selected_id.set(cx, None);
        self.title_input.update(cx, |input, cx| input.set_value("", cx));
        self.content_input.update(cx, |input, cx| input.set_value("", cx));
    }

    fn show_error(&mut self, message: String, cx: &mut Context<Self>) {
        self.status.set(cx, SharedString::from(message));
    }
}
