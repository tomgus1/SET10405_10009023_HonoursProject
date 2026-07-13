use gpui::{Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px};

use crate::framework::layout::column;
use crate::framework::state::Signal;
use crate::framework::theme::ActiveTheme;
use crate::framework::widgets::{ListItem, ScrollbarState, TextInput, TextInputState, scroll_container, scrollbar};
use crate::model::Note;
use crate::viewmodel::NotesViewModel;

const LIST_TITLE: &str = "All Notes";
const PREVIEW_LIMIT: usize = 96;

/// Renders the note list and search field. Watches `notes` and
/// `selected_id` only, so it re-renders when a note is added/removed/
/// selected but not when the status message or editor fields change.
pub struct NoteList {
    view_model: Entity<NotesViewModel>,
    notes: Signal<Vec<Note>>,
    selected_id: Signal<Option<u64>>,
    search_input: Entity<TextInputState>,
    scrollbar: ScrollbarState,
    _subscriptions: Vec<gpui::Subscription>,
}

impl NoteList {
    pub fn new(view_model: &Entity<NotesViewModel>, cx: &mut Context<Self>) -> Self {
        let (notes, selected_id, search_input) = {
            let vm = view_model.read(cx);
            (vm.notes().clone(), vm.selected_id().clone(), vm.search_input().clone())
        };
        let _subscriptions = vec![notes.watch(cx), selected_id.watch(cx)];

        Self {
            view_model: view_model.clone(),
            notes,
            selected_id,
            search_input,
            scrollbar: ScrollbarState::new(),
            _subscriptions,
        }
    }
}

impl Render for NoteList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let notes = self.notes.read(cx).clone();
        let selected_id = *self.selected_id.read(cx);
        let view_model = self.view_model.clone();

        let note_rows = notes.into_iter().map(|note| {
            let id = note.id;
            let is_selected = selected_id == Some(id);
            let view_model = view_model.clone();
            let theme = *cx.theme();
            let preview_color = if is_selected {
                theme.primary_foreground
            } else {
                theme.muted_foreground
            };

            ListItem::new(("note", id))
                .selected(is_selected)
                .on_click(move |_, _window, cx| {
                    view_model.update(cx, |vm, cx| vm.select_note(id, cx));
                })
                .child(
                    column()
                        .gap_0p5()
                        .py_1()
                        .child(div().font_weight(gpui::FontWeight::BOLD).child(note.title.clone()))
                        .child(
                            div()
                                .text_sm()
                                .text_color(preview_color)
                                .child(format_note_preview(&note)),
                        ),
                )
        });

        column()
            .h_full()
            .min_h(px(0.))
            .gap_2()
            .child(div().font_weight(gpui::FontWeight::BOLD).child(LIST_TITLE))
            .child(TextInput::new(&self.search_input))
            .child(
                div()
                    .relative()
                    .flex_1()
                    .min_h(px(0.))
                    .child(
                        scroll_container("note-list", &self.scrollbar)
                            .absolute()
                            .inset_0()
                            .child(column().gap_1().children(note_rows)),
                    )
                    .child(scrollbar(&self.scrollbar, cx)),
            )
    }
}

fn format_note_preview(note: &Note) -> String {
    let collapsed: String = note.content.split_whitespace().collect::<Vec<_>>().join(" ");
    let preview = if collapsed.chars().count() > PREVIEW_LIMIT {
        let truncated: String = collapsed.chars().take(PREVIEW_LIMIT.saturating_sub(1)).collect();
        format!("{truncated}\u{2026}")
    } else {
        collapsed
    };
    format!("{} \u{2022} {}", preview, note.created_at.format("%Y-%m-%d %H:%M"))
}
