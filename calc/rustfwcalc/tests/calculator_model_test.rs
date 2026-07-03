use rustcalc::model::{Action, CalculatorModel, Operation};

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
