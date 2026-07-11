use gpui::{
    Context, InteractiveElement, IntoElement, ParentElement, StatefulInteractiveElement, Styled,
    Window, div,
};
use gpui_component::{ActiveTheme, list::ListItem, v_flex};

use super::{format_note_preview, NotesApp};

const LIST_TITLE: &str = "All Notes";

impl NotesApp {
    pub(super) fn render_sidebar(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        use gpui_component::input::Input;

        let notes = self.notes.clone();
        let selected_id = self.selected_id;
        let entity = cx.entity();

        v_flex()
            .h_full()
            .gap_2()
            .child(div().font_weight(gpui::FontWeight::BOLD).child(LIST_TITLE))
            .child(Input::new(&self.search_input).cleanable(true))
            .child(
                div()
                    .id("note-list")
                    .flex_1()
                    .overflow_y_scroll()
                    .child(
                        v_flex().gap_1().children(notes.into_iter().map(|note| {
                            let id = note.id;
                            let is_selected = selected_id == Some(id);
                            let entity = entity.clone();

                            ListItem::new(("note", id))
                                .selected(is_selected)
                                .on_click(move |_, window, cx| {
                                    entity.update(cx, |this, cx| {
                                        this.select_note(id, window, cx);
                                    });
                                })
                                .child(
                                    v_flex()
                                        .gap_0p5()
                                        .py_1()
                                        .child(div().font_weight(gpui::FontWeight::BOLD).child(note.title.clone()))
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(cx.theme().muted_foreground)
                                                .child(format_note_preview(&note)),
                                        ),
                                )
                        })),
                    ),
            )
    }
}
