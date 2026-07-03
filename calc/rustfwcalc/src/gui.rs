use crate::display::DisplayView;
use crate::framework::layout::{column, row};
use crate::framework::theme::ActiveTheme;
use crate::framework::widgets::Button;
use crate::model::Action;
use crate::viewmodel::CalculatorViewModel;
use gpui::{
    AppContext, Context, Entity, FocusHandle, InteractiveElement, IntoElement, KeyDownEvent,
    ParentElement, Render, Styled, Window, div,
};

/// All button labels in row-major order (5 rows × 4 columns).
const BUTTON_GRID: [[&str; 4]; 5] = [
    ["C", "CE", "⌫", "/"],
    ["7", "8", "9", "*"],
    ["4", "5", "6", "-"],
    ["1", "2", "3", "+"],
    ["±", "0", ".", "="],
];

/// The root GPUI view. Owns the [`CalculatorViewModel`], the display's
/// child entity, and a focus handle. The button grid is built entirely
/// from `framework::widgets::Button` — the same component the notes app
/// uses — and colors come from `framework::theme::Theme`, not hard-coded
/// hex values.
pub struct CalculatorView {
    viewmodel: CalculatorViewModel,
    display: Entity<DisplayView>,
    focus_handle: FocusHandle,
}

impl CalculatorView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let viewmodel = CalculatorViewModel::new(cx);
        let input = viewmodel.input().clone();
        let expression = viewmodel.expression().clone();
        let display = cx.new(|cx| DisplayView::new(input, expression, cx));

        Self {
            viewmodel,
            display,
            focus_handle: cx.focus_handle(),
        }
    }

    fn dispatch_action(&mut self, action: Action, cx: &mut Context<Self>) {
        self.viewmodel.dispatch(action, cx);
    }

    /// Handles raw keyboard events by translating them into `Action`s.
    fn on_key_down(&mut self, event: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let key = event.keystroke.key.to_lowercase();
        if let Some(action) = Action::from_key(&key) {
            self.dispatch_action(action, cx);
        }
    }

    /// Renders a single calculator button, filling its grid cell.
    fn render_button(&self, label: &'static str, cx: &Context<Self>) -> impl IntoElement {
        let entity = cx.entity();
        let button = Button::new(label, label).large();
        let button = match label {
            "=" => button.primary(),
            "C" | "CE" | "⌫" => button.danger(),
            "/" | "*" | "-" | "+" => button.operator(),
            _ => button,
        };
        let button = button.on_click(move |_event, _window, cx| {
            if let Some(action) = Action::from_label(label) {
                entity.update(cx, |this, cx| this.dispatch_action(action, cx));
            }
        });

        div().flex_1().h_full().child(button)
    }

    /// Renders the full 5×4 button grid.
    fn render_button_grid(&self, cx: &Context<Self>) -> impl IntoElement {
        column().w_full().h_full().gap_2().children(BUTTON_GRID.iter().map(|labels| {
            row()
                .w_full()
                .h_full()
                .gap_2()
                .children(labels.iter().map(|&label| self.render_button(label, cx)))
        }))
    }
}

impl Render for CalculatorView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = *cx.theme();
        let grid = self.render_button_grid(cx);

        column()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::on_key_down))
            .size_full()
            .bg(theme.background)
            .p_4()
            .gap_3()
            .child(self.display.clone())
            .child(grid)
    }
}
