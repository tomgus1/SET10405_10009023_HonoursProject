use crate::model::{Action, CalculatorModel};
use gpui::*;

// ── Button styling ────────────────────────────────────────────────────────────

/// Classifies a calculator button for consistent visual styling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ButtonKind {
    /// The equals (confirm) button.
    Equals,
    /// Arithmetic operator buttons (+, -, *, /).
    Operator,
    /// Destructive/clear buttons (C, CE, ⌫).
    Clear,
    /// Digit and decimal point buttons.
    Digit,
}

impl ButtonKind {
    fn from_label(label: &str) -> Self {
        match label {
            "=" => ButtonKind::Equals,
            "/" | "*" | "-" | "+" => ButtonKind::Operator,
            "C" | "CE" | "⌫" => ButtonKind::Clear,
            _ => ButtonKind::Digit,
        }
    }

    fn bg_color(self) -> Rgba {
        match self {
            ButtonKind::Equals => rgb(0x2563eb),
            ButtonKind::Operator => rgb(0x3f3f46),
            ButtonKind::Clear => rgb(0x7f1d1d),
            ButtonKind::Digit => rgb(0x27272a),
        }
    }

    fn hover_color(self) -> Rgba {
        match self {
            ButtonKind::Equals => rgb(0x3b82f6),
            ButtonKind::Operator => rgb(0x52525b),
            ButtonKind::Clear => rgb(0x991b1b),
            ButtonKind::Digit => rgb(0x3f3f46),
        }
    }
}

// ── Button grid layout ────────────────────────────────────────────────────────

/// All button labels in row-major order (5 rows × 4 columns).
const BUTTON_GRID: [[&str; 4]; 5] = [
    ["C", "CE", "⌫", "/"],
    ["7", "8", "9", "*"],
    ["4", "5", "6", "-"],
    ["1", "2", "3", "+"],
    ["±", "0", ".", "="],
];

// ── CalculatorView ────────────────────────────────────────────────────────────

/// The root GPUI view that owns the calculator model and renders the entire UI.
pub struct CalculatorView {
    model: CalculatorModel,
    focus_handle: FocusHandle,
}

impl CalculatorView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            model: CalculatorModel::new(),
            focus_handle: cx.focus_handle(),
        }
    }

    // ── Event handling ────────────────────────────────────────────────────────

    /// Handles raw keyboard events by translating them into `Action`s.
    fn on_key_down(&mut self, event: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let key = event.keystroke.key.to_lowercase();
        if let Some(action) = Action::from_key(&key) {
            self.model.dispatch(action);
            cx.notify();
        }
    }

    /// Handles button click events by translating the label into an `Action`.
    fn on_button_click(&mut self, label: &str, cx: &mut Context<Self>) {
        if let Some(action) = Action::from_label(label) {
            self.model.dispatch(action);
            cx.notify();
        }
    }

    // ── Render helpers ────────────────────────────────────────────────────────

    /// Renders the dual-line display panel (expression history + current value).
    fn render_display(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .bg(rgb(0x27272a))
            .rounded_lg()
            .p_3()
            .items_end()
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(rgb(0xa1a1aa))
                    .child(self.model.expression_text().to_string()),
            )
            .child(
                div()
                    .text_size(px(32.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xf4f4f5))
                    .child(self.model.current_input().to_string()),
            )
    }

    /// Renders a single calculator button.
    fn render_button(&self, label: &'static str, cx: &mut Context<Self>) -> impl IntoElement {
        let kind = ButtonKind::from_label(label);
        let bg = kind.bg_color();
        let hover = kind.hover_color();

        div()
            .flex()
            .flex_1()
            .h_full()
            .items_center()
            .justify_center()
            .bg(bg)
            .hover(move |s| s.bg(hover))
            .rounded_md()
            .cursor_pointer()
            .text_size(px(20.0))
            .font_weight(FontWeight::BOLD)
            .text_color(rgb(0xf4f4f5))
            .child(label)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _event, _window, cx| {
                    this.on_button_click(label, cx);
                }),
            )
    }

    /// Renders the full 5×4 button grid.
    fn render_button_grid(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .gap_2()
            .children(BUTTON_GRID.iter().map(|row| {
                div()
                    .flex()
                    .flex_row()
                    .w_full()
                    .h_full()
                    .gap_2()
                    .children(row.iter().map(|&label| self.render_button(label, cx)))
            }))
    }
}

// ── GPUI Render impl ──────────────────────────────────────────────────────────

impl Render for CalculatorView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let display = self.render_display();
        let grid = self.render_button_grid(cx);

        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::on_key_down))
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x18181b))
            .p_4()
            .gap_3()
            .child(display)
            .child(grid)
    }
}
