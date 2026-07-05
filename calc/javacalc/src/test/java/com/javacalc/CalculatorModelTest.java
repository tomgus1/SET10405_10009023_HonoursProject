package com.javacalc;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class CalculatorModelTest {

    private CalculatorModel model;

    @BeforeEach
    void setUp() {
        model = new CalculatorModel();
    }

    @Test
    void testInitialState() {
        assertEquals("0", model.getCurrentInput());
        assertEquals(" ", model.getExpressionText());
        assertFalse(model.isError());
    }

    @Test
    void testDigitInput() {
        model.enterDigit("1");
        model.enterDigit("2");
        model.enterDigit("3");
        assertEquals("123", model.getCurrentInput());
    }

    @Test
    void testDecimalInput() {
        model.enterDigit("5");
        model.enterDecimal();
        model.enterDigit("2");
        assertEquals("5.2", model.getCurrentInput());

        // Repeated decimal points should be ignored
        model.enterDecimal();
        assertEquals("5.2", model.getCurrentInput());
    }

    @Test
    void testAddition() {
        model.enterDigit("5");
        model.setOperator(Operation.ADD);
        assertEquals("5 ＋", model.getExpressionText());

        model.enterDigit("3");
        model.calculateEquals();

        assertEquals("8", model.getCurrentInput());
        assertEquals("5 ＋ 3 =", model.getExpressionText());
    }

    @Test
    void testDivisionByZero() {
        model.enterDigit("9");
        model.setOperator(Operation.DIVIDE);
        model.enterDigit("0");
        model.calculateEquals();

        assertTrue(model.isError());
        assertEquals("Error: Division by zero", model.getCurrentInput());
    }

    @Test
    void testChainedOperations() {
        model.enterDigit("1");
        model.enterDigit("0"); // 10
        model.setOperator(Operation.SUBTRACT);
        model.enterDigit("4"); // - 4
        model.setOperator(Operation.MULTIPLY); // evaluates to 6, then pending *
        assertEquals("6", model.getCurrentInput());
        assertEquals("6 ×", model.getExpressionText());

        model.enterDigit("5");
        model.calculateEquals();
        assertEquals("30", model.getCurrentInput());
    }

    @Test
    void testBackspace() {
        model.enterDigit("1");
        model.enterDigit("2");
        model.enterDigit("3");
        model.backspace();
        assertEquals("12", model.getCurrentInput());

        model.backspace();
        model.backspace();
        assertEquals("0", model.getCurrentInput());
    }

    @Test
    void testToggleSign() {
        model.enterDigit("7");
        model.toggleSign();
        assertEquals("-7", model.getCurrentInput());

        model.toggleSign();
        assertEquals("7", model.getCurrentInput());
    }

    @Test
    void testReset() {
        model.enterDigit("8");
        model.setOperator(Operation.ADD);
        model.enterDigit("4");
        model.reset();

        assertEquals("0", model.getCurrentInput());
        assertEquals(" ", model.getExpressionText());
        assertNull(model.getCurrentOperator());
    }
}
