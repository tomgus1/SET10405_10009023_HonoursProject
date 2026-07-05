package com.javacalc;

import javax.swing.*;
import java.awt.*;

public class Main {
    public static void main(String[] args) {
        SwingUtilities.invokeLater(() -> {
            JFrame frame = new JFrame("Java Calculator");
            frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
            frame.setContentPane(new CalculatorPanel());
            frame.pack();
            frame.setMinimumSize(new Dimension(320, 450));
            frame.setLocationRelativeTo(null);
            frame.setVisible(true);
        });
    }
}