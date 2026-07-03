use crate::framework::state::Signal;
use crate::model::{Action, CalculatorModel, Operation};
use gpui::App;

/// Mediates between [`CalculatorModel`] and the View layer (`CalculatorView`
/// and `DisplayView`).
///
/// The View never mutates `CalculatorModel` directly; all input flows
/// through [`dispatch`](Self::dispatch). State is exposed as two
/// [`Signal`]s rather than plain getters so that a view which only watches
/// `input()`/`expression()` (see `src/display.rs`) re-renders on every
/// keystroke, while the (static) button grid never has to — the
/// "propagate only to dependent components" behaviour described in the
/// interim report's §3.2.
pub struct CalculatorViewModel {
    model: CalculatorModel,
    input: Signal<String>,
    expression: Signal<String>,
}

impl CalculatorViewModel {
    pub fn new(cx: &mut App) -> Self {
        let model = CalculatorModel::new();
        let input = Signal::new(cx, model.current_input().to_string());
        let expression = Signal::new(cx, model.expression_text().to_string());
        Self { model, input, expression }
    }

    /// Applies a user `Action`, mutating the model and pushing the new
    /// derived display strings into the signals.
    pub fn dispatch(&mut self, action: Action, cx: &mut App) {
        self.model.dispatch(action);
        self.input.set(cx, self.model.current_input().to_string());
        self.expression.set(cx, self.model.expression_text().to_string());
    }

    pub fn input(&self) -> &Signal<String> {
        &self.input
    }

    pub fn expression(&self) -> &Signal<String> {
        &self.expression
    }

    pub fn is_error(&self) -> bool {
        self.model.is_error()
    }

    pub fn current_operator(&self) -> Option<Operation> {
        self.model.current_operator()
    }
}
