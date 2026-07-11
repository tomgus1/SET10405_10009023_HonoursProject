mod editor;
mod sidebar;

use chrono::NaiveDateTime;
use gpui::{
    AppContext, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render,
    SharedString, Styled, Window, div,
};
use gpui_component::{
    ActiveTheme, Root,
    input::InputState,
    theme::{Theme, ThemeMode},
    v_flex,
};

use crate::model::Note;
use crate::repository::FileNoteRepository;
use crate::service::NoteService;

const APP_TITLE: &str = "Notes App";
const TAGLINE: &str = "Store, search, and manage notes locally";
const EMPTY_LIST_MESSAGE: &str = "No notes yet. Create your first note on the right.";
const READY_MESSAGE: &str = "Ready";
const PREVIEW_LIMIT: usize = 96;

pub struct NotesApp {
    note_service: NoteService<FileNoteRepository>,
    notes: Vec<Note>,
    selected_id: Option<u64>,
    title_input: Entity<InputState>,
    content_input: Entity<InputState>,
    search_input: Entity<InputState>,
    status: SharedString,
    _subscriptions: Vec<gpui::Subscription>,
}

impl NotesApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let note_service = NoteService::new(FileNoteRepository::create_default());

        let title_input = cx.new(|cx| InputState::new(window, cx).placeholder("Title"));
        let content_input = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .rows(10)
                .placeholder("Write your note here...")
        });
        let search_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Search notes..."));

        let _subscriptions = vec![cx.subscribe_in(
            &search_input,
            window,
            |this, _search_input, event: &gpui_component::input::InputEvent, _window, cx| {
                if matches!(event, gpui_component::input::InputEvent::Change) {
                    this.apply_search(cx);
                }
            },
        )];

        let mut app = Self {
            note_service,
            notes: Vec::new(),
            selected_id: None,
            title_input,
            content_input,
            search_input,
            status: SharedString::from(READY_MESSAGE),
            _subscriptions,
        };
        app.refresh_notes(cx);
        app
    }

    fn refresh_notes(&mut self, cx: &mut Context<Self>) {
        self.notes = self.note_service.get_all_notes();
        self.status = if self.notes.is_empty() {
            SharedString::from(EMPTY_LIST_MESSAGE)
        } else {
            SharedString::from(format!("Loaded {} notes", self.notes.len()))
        };
        cx.notify();
    }

    fn apply_search(&mut self, cx: &mut Context<Self>) {
        let query = self.search_input.read(cx).value().to_string();
        self.notes = self.note_service.search_notes(&query);
        self.status = SharedString::from(format!("Showing {} notes", self.notes.len()));
        cx.notify();
    }

    fn add_note(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let title = self.title_input.read(cx).value().to_string();
        let content = self.content_input.read(cx).value().to_string();

        match self.note_service.create_note(&title, &content) {
            Ok(note) => {
                self.status = SharedString::from(format!("Created note {}", note.id));
                self.clear_editor(window, cx);
                self.refresh_notes(cx);
                self.select_note(note.id, window, cx);
            }
            Err(message) => self.show_error(message, cx),
        }
    }

    fn update_selected_note(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(id) = self.selected_id else {
            self.show_error("Select a note to update.".to_string(), cx);
            return;
        };

        let title = self.title_input.read(cx).value().to_string();
        let content = self.content_input.read(cx).value().to_string();

        match self.note_service.update_note(id, &title, &content) {
            Ok(note) => {
                self.status = SharedString::from(format!("Updated note {}", note.id));
                self.refresh_notes(cx);
                self.select_note(note.id, window, cx);
            }
            Err(message) => self.show_error(message, cx),
        }
    }

    fn delete_note(&mut self, id: u64, cx: &mut Context<Self>) {
        let deleted = self.note_service.delete_note(id);
        self.status = SharedString::from(if deleted {
            format!("Deleted note {id}")
        } else {
            "Note was not found".to_string()
        });
        self.selected_id = None;
        self.refresh_notes(cx);
    }

    fn select_note(&mut self, id: u64, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(note) = self.note_service.find_note_by_id(id) {
            self.selected_id = Some(note.id);
            self.title_input.update(cx, |input, cx| {
                input.set_value(note.title.clone(), window, cx);
            });
            self.content_input.update(cx, |input, cx| {
                input.set_value(note.content.clone(), window, cx);
            });
            cx.notify();
        }
    }

    fn clear_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.selected_id = None;
        self.title_input.update(cx, |input, cx| {
            input.set_value("", window, cx);
        });
        self.content_input.update(cx, |input, cx| {
            input.set_value("", window, cx);
        });
        cx.notify();
    }

    fn show_error(&mut self, message: String, cx: &mut Context<Self>) {
        self.status = SharedString::from(message);
        cx.notify();
    }

    fn toggle_theme(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let next_mode = if cx.theme().mode.is_dark() {
            ThemeMode::Light
        } else {
            ThemeMode::Dark
        };
        Theme::change(next_mode, Some(window), cx);
        cx.notify();
    }
}

impl NotesApp {
    fn render_header(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        use gpui_component::{Sizable, button::Button, h_flex};

        let is_dark = cx.theme().mode.is_dark();

        v_flex()
            .p_4()
            .gap_1()
            .child(
                h_flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(APP_TITLE),
                    )
                    .child(
                        Button::new("theme-toggle")
                            .small()
                            .label(if is_dark { "Light mode" } else { "Dark mode" })
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.toggle_theme(window, cx);
                            })),
                    ),
            )
            .child(div().text_sm().text_color(cx.theme().muted_foreground).child(TAGLINE))
    }

    fn render_content(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        use gpui_component::h_flex;

        h_flex()
            .flex_1()
            .gap_4()
            .px_4()
            .child(
                div()
                    .w(gpui::px(340.))
                    .h_full()
                    .child(self.render_sidebar(window, cx)),
            )
            .child(div().flex_1().h_full().child(self.render_editor(window, cx)))
    }

    fn render_footer(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p_4()
            .text_sm()
            .text_color(cx.theme().muted_foreground)
            .child(self.status.clone())
    }
}

impl Render for NotesApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("notes-app")
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(
                v_flex()
                    .size_full()
                    .child(self.render_header(window, cx))
                    .child(self.render_content(window, cx))
                    .child(self.render_footer(cx)),
            )
            .children(Root::render_dialog_layer(window, cx))
            .children(Root::render_notification_layer(window, cx))
    }
}

fn format_created_at(created_at: NaiveDateTime) -> String {
    created_at.format("%Y-%m-%d %H:%M").to_string()
}

fn format_note_preview(note: &Note) -> String {
    let collapsed: String = note.content.split_whitespace().collect::<Vec<_>>().join(" ");
    let preview = if collapsed.chars().count() > PREVIEW_LIMIT {
        let truncated: String = collapsed.chars().take(PREVIEW_LIMIT.saturating_sub(1)).collect();
        format!("{truncated}\u{2026}")
    } else {
        collapsed
    };
    format!("{} \u{2022} {}", preview, format_created_at(note.created_at))
}
