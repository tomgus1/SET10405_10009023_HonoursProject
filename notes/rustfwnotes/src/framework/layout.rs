use gpui::{Div, Styled, div};

/// A vertically-stacked flex container: GPUI's `div()` + `Styled` already
/// implements the box-model/flex layout algorithm (via Taffy); this is the
/// framework's declarative entry point into it.
pub fn column() -> Div {
    div().flex().flex_col()
}

/// A horizontally-stacked flex container.
pub fn row() -> Div {
    div().flex().flex_row()
}
