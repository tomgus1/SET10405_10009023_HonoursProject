import {
  Operation,
  applyOperation,
  DivisionByZeroError,
  getDisplaySymbol,
} from './operation';

export interface CalculatorState {
  currentInput: string;
  expressionText: string;
  isError: boolean;
  currentOperator: Operation | null;
}

export class CalculatorModel {
  private storedValue = 0;
  private currentInput = '0';
  private currentOperator: Operation | null = null;
  private startNewNumber = true;
  private errorState = false;
  private expressionText = ' ';

  constructor() {
    this.reset();
  }

  reset(): void {
    this.storedValue = 0;
    this.currentInput = '0';
    this.currentOperator = null;
    this.startNewNumber = true;
    this.errorState = false;
    this.expressionText = ' ';
  }

  clearEntry(): void {
    this.currentInput = '0';
    this.startNewNumber = true;
    this.errorState = false;
  }

  enterDigit(digit: string): void {
    if (this.startNewNumber || this.errorState) {
      this.currentInput = digit;
      this.startNewNumber = false;
      this.errorState = false;
    } else if (this.currentInput === '0') {
      this.currentInput = digit;
    } else {
      this.currentInput += digit;
    }
  }

  enterDecimal(): void {
    if (this.startNewNumber || this.errorState) {
      this.currentInput = '0.';
      this.startNewNumber = false;
      this.errorState = false;
    } else if (!this.currentInput.includes('.')) {
      this.currentInput += '.';
    }
  }

  backspace(): void {
    if (this.startNewNumber || this.errorState) return;

    if (this.currentInput.length > 1) {
      this.currentInput = this.currentInput.slice(0, -1);
      if (this.currentInput === '-') {
        this.currentInput = '0';
        this.startNewNumber = true;
      }
    } else {
      this.currentInput = '0';
      this.startNewNumber = true;
    }
  }

  toggleSign(): void {
    if (this.errorState) return;
    const val = Number.parseFloat(this.currentInput);
    if (Number.isNaN(val)) return;
    if (val !== 0) {
      this.currentInput = this.formatNumber(-val);
    }
  }

  setOperator(op: Operation): void {
    if (this.errorState) return;

    const parsed = Number.parseFloat(this.currentInput);
    if (Number.isNaN(parsed)) return;

    let currentValue = parsed;

    try {
      if (this.currentOperator !== null && !this.startNewNumber) {
        currentValue = applyOperation(this.currentOperator, this.storedValue, currentValue);
        this.currentInput = this.formatNumber(currentValue);
      }

      this.storedValue = currentValue;
      this.currentOperator = op;
      this.expressionText = `${this.formatNumber(this.storedValue)} ${getDisplaySymbol(op)}`;
      this.startNewNumber = true;
    } catch (ex) {
      if (ex instanceof DivisionByZeroError) {
        this.handleError(ex.message);
      } else {
        throw ex;
      }
    }
  }

  calculateEquals(): void {
    if (this.errorState || this.currentOperator === null) return;

    const secondOperand = Number.parseFloat(this.currentInput);
    if (Number.isNaN(secondOperand)) return;

    try {
      this.expressionText = `${this.formatNumber(this.storedValue)} ${getDisplaySymbol(
        this.currentOperator
      )} ${this.formatNumber(secondOperand)} =`;

      const result = applyOperation(this.currentOperator, this.storedValue, secondOperand);
      this.currentInput = this.formatNumber(result);

      this.storedValue = result;
      this.currentOperator = null;
      this.startNewNumber = true;
    } catch (ex) {
      if (ex instanceof DivisionByZeroError) {
        this.handleError('Error: Division by zero');
      } else {
        throw ex;
      }
    }
  }

  private handleError(message: string): void {
    this.currentInput = message;
    this.expressionText = ' ';
    this.errorState = true;
    this.startNewNumber = true;
  }

  formatNumber(value: number): string {
    if (Number.isNaN(value) || !Number.isFinite(value)) {
      return 'Error';
    }
    return String(value);
  }

  // Getters
  getCurrentInput(): string {
    return this.currentInput;
  }

  getExpressionText(): string {
    return this.expressionText;
  }

  isError(): boolean {
    return this.errorState;
  }

  getCurrentOperator(): Operation | null {
    return this.currentOperator;
  }

  getState(): CalculatorState {
    return {
      currentInput: this.currentInput,
      expressionText: this.expressionText,
      isError: this.errorState,
      currentOperator: this.currentOperator,
    };
  }
}
