package com.javacalc;

public class CalculatorModel {
    private double storedValue = 0;
    private String currentInput = "0";
    private Operation currentOperator = null;
    private boolean startNewNumber = true;
    private boolean isError = false;
    private String expressionText = " ";

    public CalculatorModel() {
        reset();
    }

    public void reset() {
        storedValue = 0;
        currentInput = "0";
        currentOperator = null;
        startNewNumber = true;
        isError = false;
        expressionText = " ";
    }

    public void clearEntry() {
        currentInput = "0";
        startNewNumber = true;
        isError = false;
    }

    public void enterDigit(String digit) {
        if (startNewNumber || isError) {
            currentInput = digit;
            startNewNumber = false;
            isError = false;
        } else {
            if (currentInput.equals("0")) {
                currentInput = digit;
            } else {
                currentInput += digit;
            }
        }
    }

    public void enterDecimal() {
        if (startNewNumber || isError) {
            currentInput = "0.";
            startNewNumber = false;
            isError = false;
        } else if (!currentInput.contains(".")) {
            currentInput += ".";
        }
    }

    public void backspace() {
        if (startNewNumber || isError) return;

        if (currentInput.length() > 1) {
            currentInput = currentInput.substring(0, currentInput.length() - 1);
            if (currentInput.equals("-")) {
                currentInput = "0";
                startNewNumber = true;
            }
        } else {
            currentInput = "0";
            startNewNumber = true;
        }
    }

    public void toggleSign() {
        if (isError) return;
        try {
            double val = Double.parseDouble(currentInput);
            if (val != 0) {
                val = -val;
                currentInput = formatNumber(val);
            }
        } catch (NumberFormatException ignored) {}
    }

    public void setOperator(Operation op) {
        if (isError) return;

        try {
            double currentValue = Double.parseDouble(currentInput);

            if (currentOperator != null && !startNewNumber) {
                currentValue = currentOperator.apply(storedValue, currentValue);
                currentInput = formatNumber(currentValue);
            }

            storedValue = currentValue;
            currentOperator = op;
            expressionText = formatNumber(storedValue) + " " + op.getDisplaySymbol();
            startNewNumber = true;
        } catch (ArithmeticException ex) {
            handleError(ex.getMessage());
        } catch (NumberFormatException ignored) {}
    }

    public void calculateEquals() {
        if (isError || currentOperator == null) return;

        try {
            double secondOperand = Double.parseDouble(currentInput);
            expressionText = formatNumber(storedValue) + " " + currentOperator.getDisplaySymbol() + " " + formatNumber(secondOperand) + " =";

            double result = currentOperator.apply(storedValue, secondOperand);
            currentInput = formatNumber(result);

            storedValue = result;
            currentOperator = null;
            startNewNumber = true;
        } catch (ArithmeticException ex) {
            handleError("Error: Division by zero");
        } catch (NumberFormatException ignored) {}
    }

    private void handleError(String message) {
        currentInput = message;
        expressionText = " ";
        isError = true;
        startNewNumber = true;
    }

    public String formatNumber(double value) {
        if (Double.isNaN(value) || Double.isInfinite(value)) {
            return "Error";
        }
        if (value == (long) value) {
            return String.format("%d", (long) value);
        }
        return String.valueOf(value);
    }

    // Getters
    public String getCurrentInput() {
        return currentInput;
    }

    public String getExpressionText() {
        return expressionText;
    }

    public boolean isError() {
        return isError;
    }

    public Operation getCurrentOperator() {
        return currentOperator;
    }
}
