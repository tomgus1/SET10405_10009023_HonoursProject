import 'operation.dart';

class DivisionByZeroException implements Exception {
  final String message;
  DivisionByZeroException(this.message);
}

class CalculatorModel {
  double _storedValue = 0;
  String _currentInput = '0';
  Operation? _currentOperator;
  bool _startNewNumber = true;
  bool _isError = false;
  String _expressionText = ' ';

  CalculatorModel() {
    reset();
  }

  void reset() {
    _storedValue = 0;
    _currentInput = '0';
    _currentOperator = null;
    _startNewNumber = true;
    _isError = false;
    _expressionText = ' ';
  }

  void clearEntry() {
    _currentInput = '0';
    _startNewNumber = true;
    _isError = false;
  }

  void enterDigit(String digit) {
    if (_startNewNumber || _isError) {
      _currentInput = digit;
      _startNewNumber = false;
      _isError = false;
    } else if (_currentInput == '0') {
      _currentInput = digit;
    } else {
      _currentInput += digit;
    }
  }

  void enterDecimal() {
    if (_startNewNumber || _isError) {
      _currentInput = '0.';
      _startNewNumber = false;
      _isError = false;
    } else if (!_currentInput.contains('.')) {
      _currentInput += '.';
    }
  }

  void backspace() {
    if (_startNewNumber || _isError) return;

    if (_currentInput.length > 1) {
      _currentInput = _currentInput.substring(0, _currentInput.length - 1);
      if (_currentInput == '-') {
        _currentInput = '0';
        _startNewNumber = true;
      }
    } else {
      _currentInput = '0';
      _startNewNumber = true;
    }
  }

  void toggleSign() {
    if (_isError) return;
    final val = double.tryParse(_currentInput);
    if (val == null) return;
    if (val != 0) {
      _currentInput = formatNumber(-val);
    }
  }

  void setOperator(Operation op) {
    if (_isError) return;

    final currentValue = double.tryParse(_currentInput);
    if (currentValue == null) return;
    var newValue = currentValue;

    try {
      if (_currentOperator != null && !_startNewNumber) {
        newValue = _currentOperator!.apply(_storedValue, currentValue);
        _currentInput = formatNumber(newValue);
      }

      _storedValue = newValue;
      _currentOperator = op;
      _expressionText = '${formatNumber(_storedValue)} ${op.displaySymbol}';
      _startNewNumber = true;
    } on ArgumentError catch (ex) {
      _handleError(ex.message.toString());
    }
  }

  void calculateEquals() {
    if (_isError || _currentOperator == null) return;

    final secondOperand = double.tryParse(_currentInput);
    if (secondOperand == null) return;

    try {
      _expressionText =
          '${formatNumber(_storedValue)} ${_currentOperator!.displaySymbol} ${formatNumber(secondOperand)} =';

      final result = _currentOperator!.apply(_storedValue, secondOperand);
      _currentInput = formatNumber(result);

      _storedValue = result;
      _currentOperator = null;
      _startNewNumber = true;
    } on ArgumentError {
      _handleError('Error: Division by zero');
    }
  }

  void _handleError(String message) {
    _currentInput = message;
    _expressionText = ' ';
    _isError = true;
    _startNewNumber = true;
  }

  String formatNumber(double value) {
    if (value.isNaN || value.isInfinite) {
      return 'Error';
    }
    if (value == value.truncateToDouble()) {
      return value.truncate().toString();
    }
    return value.toString();
  }

  // Getters
  String get currentInput => _currentInput;
  String get expressionText => _expressionText;
  bool get isError => _isError;
  Operation? get currentOperator => _currentOperator;
}
