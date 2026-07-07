import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_calc/calculator_model.dart';
import 'package:flutter_calc/operation.dart';

void main() {
  late CalculatorModel model;

  setUp(() {
    model = CalculatorModel();
  });

  test('initial state', () {
    expect(model.currentInput, '0');
    expect(model.expressionText, ' ');
    expect(model.isError, isFalse);
  });

  test('digit input', () {
    model.enterDigit('1');
    model.enterDigit('2');
    model.enterDigit('3');
    expect(model.currentInput, '123');
  });

  test('decimal input', () {
    model.enterDigit('5');
    model.enterDecimal();
    model.enterDigit('2');
    expect(model.currentInput, '5.2');

    // Repeated decimal points should be ignored
    model.enterDecimal();
    expect(model.currentInput, '5.2');
  });

  test('addition', () {
    model.enterDigit('5');
    model.setOperator(Operation.add);
    expect(model.expressionText, '5 ＋');

    model.enterDigit('3');
    model.calculateEquals();

    expect(model.currentInput, '8');
    expect(model.expressionText, '5 ＋ 3 =');
  });

  test('division by zero', () {
    model.enterDigit('9');
    model.setOperator(Operation.divide);
    model.enterDigit('0');
    model.calculateEquals();

    expect(model.isError, isTrue);
    expect(model.currentInput, 'Error: Division by zero');
  });

  test('chained operations', () {
    model.enterDigit('1');
    model.enterDigit('0'); // 10
    model.setOperator(Operation.subtract);
    model.enterDigit('4'); // - 4
    model.setOperator(Operation.multiply); // evaluates to 6, then pending *
    expect(model.currentInput, '6');
    expect(model.expressionText, '6 ×');

    model.enterDigit('5');
    model.calculateEquals();
    expect(model.currentInput, '30');
  });

  test('backspace', () {
    model.enterDigit('1');
    model.enterDigit('2');
    model.enterDigit('3');
    model.backspace();
    expect(model.currentInput, '12');

    model.backspace();
    model.backspace();
    expect(model.currentInput, '0');
  });

  test('toggle sign', () {
    model.enterDigit('7');
    model.toggleSign();
    expect(model.currentInput, '-7');

    model.toggleSign();
    expect(model.currentInput, '7');
  });

  test('reset', () {
    model.enterDigit('8');
    model.setOperator(Operation.add);
    model.enterDigit('4');
    model.reset();

    expect(model.currentInput, '0');
    expect(model.expressionText, ' ');
    expect(model.currentOperator, isNull);
  });
}
