use std::fmt;

/// The four basic arithmetic operations supported by the calculator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    /// Returns the ASCII operator symbol used for keyboard/button matching.
    pub fn symbol(&self) -> &'static str {
        match self {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "*",
            Operation::Divide => "/",
        }
    }

    /// Evaluates the operation, returning `Err` on division by zero.
    pub fn apply(&self, a: f64, b: f64) -> Result<f64, &'static str> {
        match self {
            Operation::Add => Ok(a + b),
            Operation::Subtract => Ok(a - b),
            Operation::Multiply => Ok(a * b),
            Operation::Divide => {
                if b == 0.0 {
                    Err("Error: Division by zero")
                } else {
                    Ok(a / b)
                }
            }
        }
    }

    /// Parses an ASCII operator symbol into an `Operation`.
    pub fn from_symbol(symbol: &str) -> Option<Operation> {
        match symbol {
            "+" | "＋" => Some(Operation::Add),
            "-" | "－" => Some(Operation::Subtract),
            "*" | "×" => Some(Operation::Multiply),
            "/" | "÷" => Some(Operation::Divide),
            _ => None,
        }
    }
}

/// Display uses the full-width Unicode symbol for rendering in the expression bar.
impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Operation::Add => "＋",
            Operation::Subtract => "－",
            Operation::Multiply => "×",
            Operation::Divide => "÷",
        };
        write!(f, "{symbol}")
    }
}

/// All possible user inputs the calculator can receive.
/// Unifies keyboard events and button clicks into a single dispatch path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Digit(char),
    Decimal,
    Backspace,
    Clear,
    ClearEntry,
    ToggleSign,
    Equals,
    SetOperator(Operation),
}

impl Action {
    /// Parses a button label string into the corresponding `Action`.
    pub fn from_label(label: &str) -> Option<Action> {
        match label {
            "C" => Some(Action::Clear),
            "CE" => Some(Action::ClearEntry),
            "⌫" => Some(Action::Backspace),
            "±" => Some(Action::ToggleSign),
            "=" => Some(Action::Equals),
            "." => Some(Action::Decimal),
            d if d.len() == 1 && d.chars().next().is_some_and(|c| c.is_ascii_digit()) => {
                Some(Action::Digit(d.chars().next().unwrap()))
            }
            op => Operation::from_symbol(op).map(Action::SetOperator),
        }
    }

    /// Parses a keyboard key string (lowercase) into the corresponding `Action`.
    pub fn from_key(key: &str) -> Option<Action> {
        match key {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                Some(Action::Digit(key.chars().next().unwrap()))
            }
            "." => Some(Action::Decimal),
            "backspace" => Some(Action::Backspace),
            "escape" | "c" => Some(Action::Clear),
            "=" | "enter" | "return" => Some(Action::Equals),
            op => Operation::from_symbol(op).map(Action::SetOperator),
        }
    }
}

/// The calculator's core state machine.
///
/// All state is encapsulated; mutation occurs exclusively through public methods.
pub struct CalculatorModel {
    stored_value: f64,
    current_input: String,
    current_operator: Option<Operation>,
    /// When `true`, the next digit input will replace the current display.
    start_new_number: bool,
    is_error: bool,
    expression_text: String,
}

impl Default for CalculatorModel {
    fn default() -> Self {
        Self::new()
    }
}

impl CalculatorModel {
    pub fn new() -> Self {
        Self {
            stored_value: 0.0,
            current_input: String::from("0"),
            current_operator: None,
            start_new_number: true,
            is_error: false,
            expression_text: String::from(" "),
        }
    }

    /// Dispatches a user `Action` to the appropriate model mutation method.
    pub fn dispatch(&mut self, action: Action) {
        match action {
            Action::Digit(d) => self.enter_digit(d),
            Action::Decimal => self.enter_decimal(),
            Action::Backspace => self.backspace(),
            Action::Clear => self.reset(),
            Action::ClearEntry => self.clear_entry(),
            Action::ToggleSign => self.toggle_sign(),
            Action::Equals => self.calculate_equals(),
            Action::SetOperator(op) => self.set_operator(op),
        }
    }

    /// Resets the calculator to its initial state.
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Clears the current input only, leaving the pending operator intact.
    pub fn clear_entry(&mut self) {
        self.current_input = String::from("0");
        self.start_new_number = true;
        self.is_error = false;
    }

    pub fn enter_digit(&mut self, digit: char) {
        if self.start_new_number || self.is_error {
            self.current_input = digit.to_string();
            self.start_new_number = false;
            self.is_error = false;
        } else if self.current_input == "0" {
            self.current_input = digit.to_string();
        } else {
            self.current_input.push(digit);
        }
    }

    pub fn enter_decimal(&mut self) {
        if self.start_new_number || self.is_error {
            self.current_input = String::from("0.");
            self.start_new_number = false;
            self.is_error = false;
        } else if !self.current_input.contains('.') {
            self.current_input.push('.');
        }
    }

    pub fn backspace(&mut self) {
        if self.start_new_number || self.is_error {
            return;
        }
        if self.current_input.len() > 1 {
            self.current_input.pop();
            // "-" alone is not a valid number; reset to zero
            if self.current_input == "-" {
                self.current_input = String::from("0");
                self.start_new_number = true;
            }
        } else {
            self.current_input = String::from("0");
            self.start_new_number = true;
        }
    }

    pub fn toggle_sign(&mut self) {
        if self.is_error {
            return;
        }
        if let Ok(val) = self.current_input.parse::<f64>() {
            if val != 0.0 {
                self.current_input = format_number(-val);
            }
        }
    }

    pub fn set_operator(&mut self, op: Operation) {
        if self.is_error {
            return;
        }
        if let Ok(mut current_val) = self.current_input.parse::<f64>() {
            // Chain: evaluate the pending operation before setting the new one
            if let Some(pending_op) = self.current_operator {
                if !self.start_new_number {
                    match pending_op.apply(self.stored_value, current_val) {
                        Ok(result) => {
                            current_val = result;
                            self.current_input = format_number(current_val);
                        }
                        Err(msg) => {
                            self.handle_error(msg);
                            return;
                        }
                    }
                }
            }
            self.stored_value = current_val;
            self.current_operator = Some(op);
            self.expression_text = format!("{} {}", format_number(self.stored_value), op);
            self.start_new_number = true;
        }
    }

    pub fn calculate_equals(&mut self) {
        if self.is_error || self.current_operator.is_none() {
            return;
        }
        let op = self.current_operator.unwrap();
        if let Ok(rhs) = self.current_input.parse::<f64>() {
            self.expression_text = format!(
                "{} {} {} =",
                format_number(self.stored_value),
                op,
                format_number(rhs)
            );
            match op.apply(self.stored_value, rhs) {
                Ok(result) => {
                    self.current_input = format_number(result);
                    self.stored_value = result;
                    self.current_operator = None;
                    self.start_new_number = true;
                }
                Err(msg) => self.handle_error(msg),
            }
        }
    }

    fn handle_error(&mut self, message: &str) {
        self.current_input = message.to_string();
        self.expression_text = String::from(" ");
        self.is_error = true;
        self.start_new_number = true;
    }

    // ── Getters ──────────────────────────────────────────────────────────────

    pub fn current_input(&self) -> &str {
        &self.current_input
    }

    pub fn expression_text(&self) -> &str {
        &self.expression_text
    }

    pub fn is_error(&self) -> bool {
        self.is_error
    }

    pub fn current_operator(&self) -> Option<Operation> {
        self.current_operator
    }
}

/// Formats a floating-point value as a clean integer string when it has no
/// fractional part, otherwise falls back to Rust's default float formatting.
pub fn format_number(value: f64) -> String {
    if value.is_nan() || value.is_infinite() {
        return String::from("Error");
    }
    if value.fract() == 0.0 && value.abs() <= i64::MAX as f64 {
        format!("{}", value as i64)
    } else {
        format!("{}", value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let model = CalculatorModel::new();
        assert_eq!(model.current_input(), "0");
        assert_eq!(model.expression_text(), " ");
        assert!(!model.is_error());
    }

    #[test]
    fn test_digit_input() {
        let mut model = CalculatorModel::new();
        model.dispatch(Action::Digit('1'));
        model.dispatch(Action::Digit('2'));
        model.dispatch(Action::Digit('3'));
        assert_eq!(model.current_input(), "123");
    }

    #[test]
    fn test_decimal_input() {
        let mut model = CalculatorModel::new();
        model.dispatch(Action::Digit('5'));
        model.dispatch(Action::Decimal);
        model.dispatch(Action::Digit('2'));
        assert_eq!(model.current_input(), "5.2");

        // Repeated decimal points should be ignored
        model.dispatch(Action::Decimal);
        assert_eq!(model.current_input(), "5.2");
    }

    #[test]
    fn test_addition() {
        let mut model = CalculatorModel::new();
        model.dispatch(Action::Digit('5'));
        model.dispatch(Action::SetOperator(Operation::Add));
        assert_eq!(model.expression_text(), "5 ＋");

        model.dispatch(Action::Digit('3'));
        model.dispatch(Action::Equals);

        assert_eq!(model.current_input(), "8");
        assert_eq!(model.expression_text(), "5 ＋ 3 =");
    }

    #[test]
    fn test_division_by_zero() {
        let mut model = CalculatorModel::new();
        model.dispatch(Action::Digit('9'));
        model.dispatch(Action::SetOperator(Operation::Divide));
        model.dispatch(Action::Digit('0'));
        model.dispatch(Action::Equals);

        assert!(model.is_error());
        assert_eq!(model.current_input(), "Error: Division by zero");
    }

    #[test]
    fn test_chained_operations() {
        let mut model = CalculatorModel::new();
        model.dispatch(Action::Digit('1'));
        model.dispatch(Action::Digit('0'));
        model.dispatch(Action::SetOperator(Operation::Subtract));
        model.dispatch(Action::Digit('4'));
        model.dispatch(Action::SetOperator(Operation::Multiply));
        assert_eq!(model.current_input(), "6");
        assert_eq!(model.expression_text(), "6 ×");

        model.dispatch(Action::Digit('5'));
        model.dispatch(Action::Equals);
        assert_eq!(model.current_input(), "30");
    }

    #[test]
    fn test_backspace() {
        let mut model = CalculatorModel::new();
        model.dispatch(Action::Digit('1'));
        model.dispatch(Action::Digit('2'));
        model.dispatch(Action::Digit('3'));
        model.dispatch(Action::Backspace);
        assert_eq!(model.current_input(), "12");

        model.dispatch(Action::Backspace);
        model.dispatch(Action::Backspace);
        assert_eq!(model.current_input(), "0");
    }

    #[test]
    fn test_toggle_sign() {
        let mut model = CalculatorModel::new();
        model.dispatch(Action::Digit('7'));
        model.dispatch(Action::ToggleSign);
        assert_eq!(model.current_input(), "-7");

        model.dispatch(Action::ToggleSign);
        assert_eq!(model.current_input(), "7");
    }

    #[test]
    fn test_reset() {
        let mut model = CalculatorModel::new();
        model.dispatch(Action::Digit('8'));
        model.dispatch(Action::SetOperator(Operation::Add));
        model.dispatch(Action::Digit('4'));
        model.dispatch(Action::Clear);

        assert_eq!(model.current_input(), "0");
        assert_eq!(model.expression_text(), " ");
        assert_eq!(model.current_operator(), None);
    }
}
