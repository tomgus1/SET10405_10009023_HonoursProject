use gpui::{Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled, Window, div};

use crate::framework::state::Signal;
use crate::framework::theme::ActiveTheme;
use crate::viewmodel::NotesViewModel;

/// Renders only the status line. Because it watches the `status` signal
/// alone (not the whole `NotesViewModel`), editing note content or
/// selecting a different note does not cause this entity to re-render.
pub struct StatusBar {
    status: Signal<SharedString>,
    _subscription: gpui::Subscription,
}

impl StatusBar {
    pub fn new(view_model: &Entity<NotesViewModel>, cx: &mut Context<Self>) -> Self {
        let status = view_model.read(cx).status().clone();
        let _subscription = status.watch(cx);
        Self { status, _subscription }
    }
}

impl Render for StatusBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let status = self.status.read(cx).clone();
        div().p_4().text_sm().text_color(cx.theme().muted_foreground).child(status)
    }
}
