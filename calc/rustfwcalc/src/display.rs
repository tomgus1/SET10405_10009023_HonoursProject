use crate::framework::layout::column;
use crate::framework::state::Signal;
use crate::framework::theme::ActiveTheme;
use gpui::{Context, IntoElement, ParentElement, Render, Styled, Subscription, Window, div, px};

/// The calculator's dual-line display panel, as its own `Entity`.
///
/// It watches only the `input`/`expression` signals — not the whole
/// `CalculatorViewModel` — so it is the part of the tree that re-renders
/// on every keystroke. The button grid, owned separately by
/// `CalculatorView`, never has to: it doesn't watch anything, since none
/// of its own state changes when the calculator's numbers do.
pub struct DisplayView {
    input: Signal<String>,
    expression: Signal<String>,
    _subscriptions: [Subscription; 2],
}

impl DisplayView {
    pub fn new(input: Signal<String>, expression: Signal<String>, cx: &mut Context<Self>) -> Self {
        let _subscriptions = [input.watch(cx), expression.watch(cx)];
        Self {
            input,
            expression,
            _subscriptions,
        }
    }
}

impl Render for DisplayView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = *cx.theme();

        column()
            .w_full()
            .items_end()
            .bg(theme.surface)
            .rounded(theme.radius)
            .p_3()
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(theme.muted_foreground)
                    .child(self.expression.read(cx).clone()),
            )
            .child(
                div()
                    .text_size(px(32.0))
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(theme.foreground)
                    .child(self.input.read(cx).clone()),
            )
    }
}
