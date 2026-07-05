package com.javacalc;

import javax.swing.*;
import javax.swing.border.EmptyBorder;
import java.awt.*;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import java.awt.event.KeyEvent;

public class CalculatorPanel extends JPanel {
    private final JLabel expressionLabel;
    private final JTextField displayField;
    private final CalculatorModel model;

    public CalculatorPanel() {
        this.model = new CalculatorModel();

        setLayout(new BorderLayout(10, 10));
        setBorder(new EmptyBorder(15, 15, 15, 15));
        setBackground(new Color(240, 240, 245));

        // Top display panel containing secondary expression and primary display
        JPanel displayPanel = new JPanel();
        displayPanel.setLayout(new BoxLayout(displayPanel, BoxLayout.Y_AXIS));
        displayPanel.setBackground(new Color(255, 255, 255));
        displayPanel.setBorder(BorderFactory.createCompoundBorder(
                BorderFactory.createLineBorder(new Color(210, 210, 220), 1),
                new EmptyBorder(8, 12, 8, 12)
        ));

        expressionLabel = new JLabel(" ", SwingConstants.RIGHT);
        expressionLabel.setFont(new Font("SansSerif", Font.PLAIN, 13));
        expressionLabel.setForeground(new Color(120, 120, 130));
        expressionLabel.setAlignmentX(Component.RIGHT_ALIGNMENT);

        displayField = new JTextField("0");
        displayField.setFont(new Font("SansSerif", Font.BOLD, 28));
        displayField.setHorizontalAlignment(JTextField.RIGHT);
        displayField.setEditable(false);
        displayField.setBorder(null);
        displayField.setBackground(new Color(255, 255, 255));
        displayField.setForeground(new Color(30, 30, 30));
        displayField.setAlignmentX(Component.RIGHT_ALIGNMENT);

        displayPanel.add(expressionLabel);
        displayPanel.add(Box.createVerticalStrut(4));
        displayPanel.add(displayField);

        add(displayPanel, BorderLayout.NORTH);

        // 5x4 Grid for Calculator Buttons
        JPanel buttonPanel = new JPanel(new GridLayout(5, 4, 8, 8));
        buttonPanel.setOpaque(false);

        String[][] buttonGrid = {
                {"C", "CE", "⌫", "/"},
                {"7", "8", "9", "*"},
                {"4", "5", "6", "-"},
                {"1", "2", "3", "+"},
                {"±", "0", ".", "="}
        };

        ActionListener buttonListener = e -> handleInput(e.getActionCommand());

        for (String[] row : buttonGrid) {
            for (String label : row) {
                JButton button = createStyledButton(label);
                button.addActionListener(buttonListener);
                buttonPanel.add(button);
            }
        }

        add(buttonPanel, BorderLayout.CENTER);

        setupKeyboardShortcuts();
        updateDisplay();
    }

    private JButton createStyledButton(String text) {
        JButton button = new JButton(text);
        button.setFont(new Font("SansSerif", Font.BOLD, 18));
        button.setFocusPainted(false);
        button.setCursor(new Cursor(Cursor.HAND_CURSOR));

        // Color coding for button types
        if (text.equals("=")) {
            button.setBackground(new Color(51, 122, 183));
            button.setForeground(Color.WHITE);
        } else if (text.matches("[/\\*\\-\\+]")) {
            button.setBackground(new Color(230, 235, 245));
            button.setForeground(new Color(30, 60, 120));
        } else if (text.matches("(C|CE|⌫)")) {
            button.setBackground(new Color(245, 225, 225));
            button.setForeground(new Color(150, 40, 40));
        } else {
            button.setBackground(Color.WHITE);
            button.setForeground(new Color(40, 40, 40));
        }

        return button;
    }

    private void handleInput(String command) {
        if (command.matches("\\d")) {
            model.enterDigit(command);
        } else if (command.equals(".")) {
            model.enterDecimal();
        } else if (command.equals("⌫")) {
            model.backspace();
        } else if (command.equals("C")) {
            model.reset();
        } else if (command.equals("CE")) {
            model.clearEntry();
        } else if (command.equals("±")) {
            model.toggleSign();
        } else if (command.equals("=")) {
            model.calculateEquals();
        } else {
            Operation op = Operation.fromSymbol(command);
            if (op != null) {
                model.setOperator(op);
            }
        }
        updateDisplay();
    }

    private void updateDisplay() {
        displayField.setText(model.getCurrentInput());
        expressionLabel.setText(model.getExpressionText());
    }

    private void setupKeyboardShortcuts() {
        InputMap inputMap = getInputMap(JComponent.WHEN_IN_FOCUSED_WINDOW);
        ActionMap actionMap = getActionMap();

        // Digits 0-9
        for (int i = 0; i <= 9; i++) {
            String digit = String.valueOf(i);
            inputMap.put(KeyStroke.getKeyStroke(digit), "digit_" + digit);
            inputMap.put(KeyStroke.getKeyStroke(KeyEvent.VK_NUMPAD0 + i, 0), "digit_" + digit);
            actionMap.put("digit_" + digit, new AbstractAction() {
                @Override
                public void actionPerformed(ActionEvent e) {
                    handleInput(digit);
                }
            });
        }

        // Operators
        bindKey(inputMap, actionMap, "+", "+", KeyEvent.VK_ADD);
        bindKey(inputMap, actionMap, "-", "-", KeyEvent.VK_SUBTRACT);
        bindKey(inputMap, actionMap, "*", "*", KeyEvent.VK_MULTIPLY);
        bindKey(inputMap, actionMap, "/", "/", KeyEvent.VK_DIVIDE);
        bindKey(inputMap, actionMap, ".", ".", KeyEvent.VK_DECIMAL);

        // Equals / Enter
        inputMap.put(KeyStroke.getKeyStroke(KeyEvent.VK_ENTER, 0), "equals");
        inputMap.put(KeyStroke.getKeyStroke("="), "equals");
        actionMap.put("equals", new AbstractAction() {
            @Override
            public void actionPerformed(ActionEvent e) {
                handleInput("=");
            }
        });

        // Backspace
        inputMap.put(KeyStroke.getKeyStroke(KeyEvent.VK_BACK_SPACE, 0), "backspace");
        actionMap.put("backspace", new AbstractAction() {
            @Override
            public void actionPerformed(ActionEvent e) {
                handleInput("⌫");
            }
        });

        // Escape / Clear
        inputMap.put(KeyStroke.getKeyStroke(KeyEvent.VK_ESCAPE, 0), "clear");
        inputMap.put(KeyStroke.getKeyStroke("c"), "clear");
        inputMap.put(KeyStroke.getKeyStroke("C"), "clear");
        actionMap.put("clear", new AbstractAction() {
            @Override
            public void actionPerformed(ActionEvent e) {
                handleInput("C");
            }
        });
    }

    private void bindKey(InputMap inputMap, ActionMap actionMap, String keyChar, String command, int keyCode) {
        inputMap.put(KeyStroke.getKeyStroke(keyChar), "key_" + command);
        if (keyCode != 0) {
            inputMap.put(KeyStroke.getKeyStroke(keyCode, 0), "key_" + command);
        }
        actionMap.put("key_" + command, new AbstractAction() {
            @Override
            public void actionPerformed(ActionEvent e) {
                handleInput(command);
            }
        });
    }
}