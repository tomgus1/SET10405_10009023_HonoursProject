mod editor;
mod sidebar;
mod status_bar;

use gpui::{
    AppContext, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render, Styled,
    Window, div, px,
};

use crate::framework::layout::{column, row};
use crate::framework::theme::{ActiveTheme, Theme, ThemeMode};
use crate::framework::widgets::{Button, render_dialog_layer};
use crate::viewmodel::NotesViewModel;

use editor::NoteEditor;
use sidebar::NoteList;
use status_bar::StatusBar;

const APP_TITLE: &str = "Notes App";
const TAGLINE: &str = "Store, search, and manage notes locally";

/// The application root. Composes independent child views (sidebar, editor,
/// status bar) rather than holding all rendering logic itself, so that each
/// child re-renders only in response to the state it actually watches.
pub struct NotesApp {
    note_list: Entity<NoteList>,
    note_editor: Entity<NoteEditor>,
    status_bar: Entity<StatusBar>,
}

impl NotesApp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let view_model = cx.new(NotesViewModel::new);
        let note_list = cx.new(|cx| NoteList::new(&view_model, cx));
        let note_editor = cx.new(|cx| NoteEditor::new(&view_model, cx));
        let status_bar = cx.new(|cx| StatusBar::new(&view_model, cx));

        Self { note_list, note_editor, status_bar }
    }

    fn toggle_theme(&mut self, cx: &mut Context<Self>) {
        let next_mode = if cx.theme().mode.is_dark() {
            ThemeMode::Light
        } else {
            ThemeMode::Dark
        };
        Theme::set_mode(next_mode, cx);
    }
}

impl Render for NotesApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = *cx.theme();
        let is_dark = theme.mode.is_dark();

        div()
            .id("notes-app")
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .child(
                column()
                    .size_full()
                    .child(
                        column()
                            .p_4()
                            .gap_1()
                            .child(
                                row()
                                    .justify_between()
                                    .items_center()
                                    .child(
                                        div()
                                            .text_xl()
                                            .font_weight(gpui::FontWeight::BOLD)
                                            .child(APP_TITLE),
                                    )
                                    .child(
                                        Button::new(
                                            "theme-toggle",
                                            if is_dark { "Light mode" } else { "Dark mode" },
                                        )
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.toggle_theme(cx);
                                        })),
                                    ),
                            )
                            .child(div().text_sm().text_color(theme.muted_foreground).child(TAGLINE)),
                    )
                    .child(
                        row()
                            .flex_1()
                            .min_h(px(0.))
                            .gap_4()
                            .px_4()
                            .child(
                                div()
                                    .w(px(340.))
                                    .h_full()
                                    .min_h(px(0.))
                                    .child(self.note_list.clone()),
                            )
                            .child(div().flex_1().h_full().child(self.note_editor.clone())),
                    )
                    .child(self.status_bar.clone()),
            )
            .children(render_dialog_layer(window, cx))
    }
}
