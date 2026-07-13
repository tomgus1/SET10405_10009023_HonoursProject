use gpui::{Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div};

use crate::framework::layout::{column, row};
use crate::framework::state::Signal;
use crate::framework::theme::ActiveTheme;
use crate::framework::widgets::{confirm_dialog, Button, TextInput, TextInputState};
use crate::viewmodel::NotesViewModel;

/// Renders the note editor form. Watches `selected_id` only (to enable/
/// disable the Update/Delete buttons) — typing in the title/content
/// fields does not cause this entity to re-render, since those inputs
/// are separate entities that render themselves independently.
pub struct NoteEditor {
    view_model: Entity<NotesViewModel>,
    selected_id: Signal<Option<u64>>,
    title_input: Entity<TextInputState>,
    content_input: Entity<TextInputState>,
    _subscription: gpui::Subscription,
}

impl NoteEditor {
    pub fn new(view_model: &Entity<NotesViewModel>, cx: &mut Context<Self>) -> Self {
        let (selected_id, title_input, content_input) = {
            let vm = view_model.read(cx);
            (vm.selected_id().clone(), vm.title_input().clone(), vm.content_input().clone())
        };
        let _subscription = selected_id.watch(cx);

        Self {
            view_model: view_model.clone(),
            selected_id,
            title_input,
            content_input,
            _subscription,
        }
    }
}

impl Render for NoteEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_selection = self.selected_id.read(cx).is_some();
        let view_model = self.view_model.clone();
        let selected_id = self.selected_id.clone();

        column()
            .h_full()
            .gap_3()
            .child(div().font_weight(gpui::FontWeight::BOLD).child("Note Details"))
            .child(div().text_sm().text_color(cx.theme().muted_foreground).child("Title"))
            .child(TextInput::new(&self.title_input))
            .child(div().text_sm().text_color(cx.theme().muted_foreground).child("Content"))
            .child(TextInput::new(&self.content_input))
            .child(
                row()
                    .gap_2()
                    .child(Button::new("add-note", "Add Note").primary().on_click({
                        let view_model = view_model.clone();
                        move |_, _window, cx| {
                            view_model.update(cx, |vm, cx| vm.add_note(cx));
                        }
                    }))
                    .child(
                        Button::new("update-note", "Update Note")
                            .disabled(!has_selection)
                            .on_click({
                                let view_model = view_model.clone();
                                move |_, _window, cx| {
                                    view_model.update(cx, |vm, cx| vm.update_selected_note(cx));
                                }
                            }),
                    )
                    .child(
                        Button::new("delete-note", "Delete Selected")
                            .danger()
                            .disabled(!has_selection)
                            .on_click({
                                let view_model = view_model.clone();
                                let selected_id = selected_id.clone();
                                move |_, window, cx| {
                                    let Some(id) = *selected_id.read(cx) else {
                                        return;
                                    };
                                    let view_model = view_model.clone();
                                    confirm_dialog(
                                        window,
                                        cx,
                                        "Confirm delete",
                                        "Delete the selected note?",
                                        move |_window, cx| {
                                            view_model.update(cx, |vm, cx| vm.delete_note(id, cx));
                                        },
                                    );
                                }
                            }),
                    )
                    .child(Button::new("clear-fields", "Clear Fields").on_click({
                        let view_model = view_model.clone();
                        move |_, _window, cx| {
                            view_model.update(cx, |vm, cx| vm.clear_editor(cx));
                        }
                    }))
                    .child(Button::new("refresh", "Refresh").on_click({
                        let view_model = view_model.clone();
                        move |_, _window, cx| {
                            view_model.update(cx, |vm, cx| vm.refresh_notes(cx));
                        }
                    })),
            )
    }
}
