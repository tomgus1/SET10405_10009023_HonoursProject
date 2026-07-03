use gpui::{
    App, ClickEvent, CursorStyle, Div, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, Stateful, StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder,
};

use crate::framework::theme::ActiveTheme;
use crate::framework::widgets::ClickHandler;

/// A selectable row, e.g. for a sidebar list of records. Built on `div()` +
/// `InteractiveElement`; GPUI has no built-in list-item widget.
#[derive(IntoElement)]
pub struct ListItem {
    id: ElementId,
    selected: bool,
    content: Vec<gpui::AnyElement>,
    on_click: Option<ClickHandler>,
}

impl ListItem {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            selected: false,
            content: Vec::new(),
            on_click: None,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
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

impl ParentElement for ListItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.content.extend(elements);
    }
}

impl RenderOnce for ListItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = *cx.theme();

        let mut item: Stateful<Div> = div()
            .id(self.id)
            .w_full()
            .px_2()
            .py_1()
            .rounded(theme.radius)
            .cursor(CursorStyle::PointingHand)
            .when(self.selected, |el| el.bg(theme.primary).text_color(theme.primary_foreground))
            .when(!self.selected, |el| {
                el.hover(|style| style.bg(theme.surface))
            })
            .children(self.content);

        if let Some(on_click) = self.on_click {
            item = item.on_click(move |event, window, cx| on_click(event, window, cx));
        }

        item
    }
}
