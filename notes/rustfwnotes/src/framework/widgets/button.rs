use gpui::{
    App, ClickEvent, CursorStyle, Div, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, Stateful, StatefulInteractiveElement, Styled, Window, div,
};

use crate::framework::theme::ActiveTheme;
use crate::framework::widgets::ClickHandler;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Plain,
    Danger,
}

/// A clickable button built directly on `div()` + `InteractiveElement` — GPUI
/// has no built-in `Button`; this is the framework's component for it.
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    disabled: bool,
    on_click: Option<ClickHandler>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            variant: ButtonVariant::Plain,
            disabled: false,
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

        let (bg, fg, border) = match self.variant {
            ButtonVariant::Primary => (theme.primary, theme.primary_foreground, theme.primary),
            ButtonVariant::Danger => (theme.danger, theme.danger_foreground, theme.danger),
            ButtonVariant::Plain => (theme.surface, theme.foreground, theme.border),
        };

        let mut button: Stateful<Div> = div()
            .id(self.id)
            .px_3()
            .py_1p5()
            .rounded(theme.radius)
            .border_1()
            .border_color(border)
            .bg(bg)
            .text_color(fg)
            .text_sm()
            .items_center()
            .justify_center()
            .child(self.label);

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
