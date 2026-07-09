export enum Operation {
  ADD = 'ADD',
  SUBTRACT = 'SUBTRACT',
  MULTIPLY = 'MULTIPLY',
  DIVIDE = 'DIVIDE',
}

export class DivisionByZeroError extends Error {
  constructor() {
    super('Division by zero');
    this.name = 'DivisionByZeroError';
  }
}

const SYMBOLS: Record<Operation, string> = {
  [Operation.ADD]: '+',
  [Operation.SUBTRACT]: '-',
  [Operation.MULTIPLY]: '*',
  [Operation.DIVIDE]: '/',
};

const DISPLAY_SYMBOLS: Record<Operation, string> = {
  [Operation.ADD]: '＋',
  [Operation.SUBTRACT]: '－',
  [Operation.MULTIPLY]: '×',
  [Operation.DIVIDE]: '÷',
};

export function getSymbol(op: Operation): string {
  return SYMBOLS[op];
}

export function getDisplaySymbol(op: Operation): string {
  return DISPLAY_SYMBOLS[op];
}

export function applyOperation(op: Operation, a: number, b: number): number {
  switch (op) {
    case Operation.ADD:
      return a + b;
    case Operation.SUBTRACT:
      return a - b;
    case Operation.MULTIPLY:
      return a * b;
    case Operation.DIVIDE:
      if (b === 0) {
        throw new DivisionByZeroError();
      }
      return a / b;
  }
}

export function operationFromSymbol(symbol: string): Operation | null {
  const entry = (Object.entries(SYMBOLS) as [Operation, string][]).find(
    ([, s]) => s === symbol
  );
  return entry ? entry[0] : null;
}
