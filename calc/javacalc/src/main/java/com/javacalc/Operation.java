package com.javacalc;

public enum Operation {
    ADD("+", "＋"),
    SUBTRACT("-", "－"),
    MULTIPLY("*", "×"),
    DIVIDE("/", "÷");

    private final String symbol;
    private final String displaySymbol;

    Operation(String symbol, String displaySymbol) {
        this.symbol = symbol;
        this.displaySymbol = displaySymbol;
    }

    public String getSymbol() {
        return symbol;
    }

    public String getDisplaySymbol() {
        return displaySymbol;
    }

    public double apply(double a, double b) {
        return switch (this) {
            case ADD -> a + b;
            case SUBTRACT -> a - b;
            case MULTIPLY -> a * b;
            case DIVIDE -> {
                if (b == 0) {
                    throw new ArithmeticException("Division by zero");
                }
                yield a / b;
            }
        };
    }

    public static Operation fromSymbol(String symbol) {
        for (Operation op : values()) {
            if (op.symbol.equals(symbol)) {
                return op;
            }
        }
        return null;
    }
}
