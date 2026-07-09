import { CalculatorModel } from './calculatorModel';
import { Operation } from './operation';

describe('CalculatorModel', () => {
  let model: CalculatorModel;

  beforeEach(() => {
    model = new CalculatorModel();
  });

  test('initial state', () => {
    expect(model.getCurrentInput()).toBe('0');
    expect(model.getExpressionText()).toBe(' ');
    expect(model.isError()).toBe(false);
  });

  test('digit input', () => {
    model.enterDigit('1');
    model.enterDigit('2');
    model.enterDigit('3');
    expect(model.getCurrentInput()).toBe('123');
  });

  test('decimal input', () => {
    model.enterDigit('5');
    model.enterDecimal();
    model.enterDigit('2');
    expect(model.getCurrentInput()).toBe('5.2');

    // Repeated decimal points should be ignored
    model.enterDecimal();
    expect(model.getCurrentInput()).toBe('5.2');
  });

  test('addition', () => {
    model.enterDigit('5');
    model.setOperator(Operation.ADD);
    expect(model.getExpressionText()).toBe('5 ＋');

    model.enterDigit('3');
    model.calculateEquals();

    expect(model.getCurrentInput()).toBe('8');
    expect(model.getExpressionText()).toBe('5 ＋ 3 =');
  });

  test('division by zero', () => {
    model.enterDigit('9');
    model.setOperator(Operation.DIVIDE);
    model.enterDigit('0');
    model.calculateEquals();

    expect(model.isError()).toBe(true);
    expect(model.getCurrentInput()).toBe('Error: Division by zero');
  });

  test('chained operations', () => {
    model.enterDigit('1');
    model.enterDigit('0'); // 10
    model.setOperator(Operation.SUBTRACT);
    model.enterDigit('4'); // - 4
    model.setOperator(Operation.MULTIPLY); // evaluates to 6, then pending *
    expect(model.getCurrentInput()).toBe('6');
    expect(model.getExpressionText()).toBe('6 ×');

    model.enterDigit('5');
    model.calculateEquals();
    expect(model.getCurrentInput()).toBe('30');
  });

  test('backspace', () => {
    model.enterDigit('1');
    model.enterDigit('2');
    model.enterDigit('3');
    model.backspace();
    expect(model.getCurrentInput()).toBe('12');

    model.backspace();
    model.backspace();
    expect(model.getCurrentInput()).toBe('0');
  });

  test('toggle sign', () => {
    model.enterDigit('7');
    model.toggleSign();
    expect(model.getCurrentInput()).toBe('-7');

    model.toggleSign();
    expect(model.getCurrentInput()).toBe('7');
  });

  test('reset', () => {
    model.enterDigit('8');
    model.setOperator(Operation.ADD);
    model.enterDigit('4');
    model.reset();

    expect(model.getCurrentInput()).toBe('0');
    expect(model.getExpressionText()).toBe(' ');
    expect(model.getCurrentOperator()).toBeNull();
  });
});
