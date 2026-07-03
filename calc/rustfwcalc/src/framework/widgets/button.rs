use gpui::{
    App, ClickEvent, CursorStyle, Div, ElementId, FontWeight, InteractiveElement, IntoElement,
    ParentElement, RenderOnce, SharedString, Stateful, StatefulInteractiveElement, Styled, Window,
    div, px,
};

use crate::framework::theme::ActiveTheme;
use crate::framework::widgets::ClickHandler;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Plain,
    Danger,
    /// Distinct from `Plain` without being a primary/danger action — e.g.
    /// a calculator's operator keys standing apart from its digit keys.
    Operator,
}

/// A clickable button built directly on `div()` + `InteractiveElement` — GPUI
/// has no built-in `Button`; this is the framework's component for it.
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    disabled: bool,
    /// A borderless, bold, larger-text tile that fills its parent rather
    /// than sizing to its label — for grid-of-keys layouts (a calculator
    /// keypad) as opposed to compact toolbar/dialog buttons.
    large: bool,
    on_click: Option<ClickHandler>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            variant: ButtonVariant::Plain,
            disabled: false,
            large: false,
            on_click: None,
        }
    }

    pub fn primary(mut self) -> Self {
        self.variant = ButtonVariant::Primary;
        self
    }

    pub fn danger(mut self) -> Self {
        self.variant = ButtonVariant::Danger;
        self
    }

    pub fn operator(mut self) -> Self {
        self.variant = ButtonVariant::Operator;
        self
    }

    pub fn large(mut self) -> Self {
        self.large = true;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = *cx.theme();
        let disabled = self.disabled;
        let large = self.large;

        let (bg, fg, border) = match self.variant {
            ButtonVariant::Primary => (theme.primary, theme.primary_foreground, theme.primary),
            ButtonVariant::Danger => (theme.danger, theme.danger_foreground, theme.danger),
            ButtonVariant::Operator => (theme.operator, theme.foreground, theme.operator),
            ButtonVariant::Plain => (theme.surface, theme.foreground, theme.border),
        };

        let mut button: Stateful<Div> = div()
            .id(self.id)
            .rounded(theme.radius)
            .bg(bg)
            .text_color(fg)
            .items_center()
            .justify_center()
            .child(self.label);

        button = if large {
            button.flex().w_full().h_full().text_size(px(20.)).font_weight(FontWeight::BOLD)
        } else {
            button.px_3().py_1p5().border_1().border_color(border).text_sm()
        };

        if disabled {
            button = button.opacity(0.5);
        } else {
            button = button.cursor(CursorStyle::PointingHand);
            if let Some(on_click) = self.on_click {
                button = button.on_click(move |event, window, cx| on_click(event, window, cx));
            }
        }

        button
    }
}
