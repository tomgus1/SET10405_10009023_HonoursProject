use gpui::{px, Context, IntoElement, ParentElement, Styled, Window};
use gpui_component::{
    button::{Button, ButtonVariants},
    h_flex,
    input::Input,
    v_flex, ActiveTheme, Disableable, WindowExt,
};

use super::NotesApp;

impl NotesApp {
    pub(super) fn render_editor(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let has_selection = self.selected_id.is_some();
        let entity = cx.entity();

        v_flex()
            .h_full()
            .gap_3()
            .child(gpui::div().font_weight(gpui::FontWeight::BOLD).child("Note Details"))
            .child(gpui::div().text_sm().text_color(cx.theme().muted_foreground).child("Title"))
            .child(Input::new(&self.title_input).cleanable(true))
            .child(gpui::div().text_sm().text_color(cx.theme().muted_foreground).child("Content"))
            .child(Input::new(&self.content_input).h(px(220.)))
            .child(
                h_flex()
                    .gap_2()
                    .child(Button::new("add-note").primary().label("Add Note").on_click(
                        cx.listener(|this, _, window, cx| this.add_note(window, cx)),
                    ))
                    .child(
                        Button::new("update-note")
                            .label("Update Note")
                            .disabled(!has_selection)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.update_selected_note(window, cx);
                            })),
                    )
                    .child(
                        Button::new("delete-note")
                            .danger()
                            .label("Delete Selected")
                            .disabled(!has_selection)
                            .on_click(move |_, window, cx| {
                                let Some(id) = entity.read(cx).selected_id else {
                                    return;
                                };
                                let entity = entity.clone();
                                window.open_dialog(cx, move |dialog, _window, _cx| {
                                    let entity = entity.clone();
                                    dialog
                                        .title("Confirm delete")
                                        .child("Delete the selected note?")
                                        .confirm()
                                        .on_ok(move |_, _window, cx| {
                                            entity.update(cx, |this, cx| {
                                                this.delete_note(id, cx);
                                            });
                                            true
                                        })
                                });
                            }),
                    )
                    .child(Button::new("clear-fields").label("Clear Fields").on_click(
                        cx.listener(|this, _, window, cx| this.clear_editor(window, cx)),
                    ))
                    .child(Button::new("refresh").label("Refresh").on_click(cx.listener(
                        |this, _, _window, cx| this.refresh_notes(cx),
                    ))),
            )
    }
}
