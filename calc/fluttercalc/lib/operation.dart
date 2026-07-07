enum Operation {
  add('+', '＋'),
  subtract('-', '－'),
  multiply('*', '×'),
  divide('/', '÷');

  const Operation(this.symbol, this.displaySymbol);

  final String symbol;
  final String displaySymbol;

  double apply(double a, double b) {
    switch (this) {
      case Operation.add:
        return a + b;
      case Operation.subtract:
        return a - b;
      case Operation.multiply:
        return a * b;
      case Operation.divide:
        if (b == 0) {
          throw ArgumentError('Division by zero');
        }
        return a / b;
    }
  }

  static Operation? fromSymbol(String symbol) {
    for (final op in Operation.values) {
      if (op.symbol == symbol) return op;
    }
    return null;
  }
}
