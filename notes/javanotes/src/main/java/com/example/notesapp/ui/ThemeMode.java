package com.example.notesapp.ui;

import javax.swing.BorderFactory;
import javax.swing.JButton;
import javax.swing.JComponent;
import javax.swing.JFrame;
import javax.swing.JLabel;
import javax.swing.JList;
import javax.swing.JPanel;
import javax.swing.JScrollPane;
import javax.swing.JSplitPane;
import javax.swing.JTextField;
import javax.swing.JViewport;
import javax.swing.border.EmptyBorder;
import javax.swing.text.JTextComponent;
import java.awt.Color;
import java.awt.Component;
import java.awt.Container;

public enum ThemeMode {
    LIGHT(
            new Color(0xF5, 0xF7, 0xFA),
            new Color(0xFF, 0xFF, 0xFF),
            new Color(0x11, 0x18, 0x27),
            new Color(0x6B, 0x72, 0x80),
            new Color(0xD1, 0xD5, 0xDB),
            new Color(0xE5, 0xE7, 0xEB),
            new Color(0x11, 0x18, 0x27),
            new Color(0xDB, 0xE4, 0xFF),
            new Color(0xFF, 0xFF, 0xFF)),
    DARK(
            new Color(0x0F, 0x17, 0x2A),
            new Color(0x17, 0x24, 0x39),
            new Color(0xE2, 0xE8, 0xF0),
            new Color(0x94, 0xA3, 0xB8),
            new Color(0x33, 0x41, 0x55),
            new Color(0x38, 0x4A, 0x5F),
            new Color(0xE2, 0xE8, 0xF0),
            new Color(0x1D, 0x4E, 0x89),
            new Color(0x1E, 0x29, 0x3B));

    private final Color background;
    private final Color panelBackground;
    private final Color foreground;
    private final Color mutedForeground;
    private final Color borderColor;
    private final Color buttonBackground;
    private final Color buttonForeground;
    private final Color selectionBackground;
    private final Color selectionForeground;

    ThemeMode(Color background, Color panelBackground, Color foreground, Color mutedForeground, Color borderColor,
              Color buttonBackground, Color buttonForeground, Color selectionBackground, Color selectionForeground) {
        this.background = background;
        this.panelBackground = panelBackground;
        this.foreground = foreground;
        this.mutedForeground = mutedForeground;
        this.borderColor = borderColor;
        this.buttonBackground = buttonBackground;
        this.buttonForeground = buttonForeground;
        this.selectionBackground = selectionBackground;
        this.selectionForeground = selectionForeground;
    }

    public ThemeMode opposite() {
        return this == LIGHT ? DARK : LIGHT;
    }

    public String toggleButtonText() {
        return this == LIGHT ? "Dark mode" : "Light mode";
    }

    public Color background() {
        return background;
    }

    public Color panelBackground() {
        return panelBackground;
    }

    public Color foreground() {
        return foreground;
    }

    public Color mutedForeground() {
        return mutedForeground;
    }

    public Color selectionBackground() {
        return selectionBackground;
    }

    public Color selectionForeground() {
        return selectionForeground;
    }

    public String htmlColor(Color color) {
        return String.format("#%02x%02x%02x", color.getRed(), color.getGreen(), color.getBlue());
    }

    public void applyTo(JFrame frame) {
        frame.getContentPane().setBackground(background);
        applyToComponent(frame.getContentPane());
    }

    private void applyToComponent(Component component) {
        if (component instanceof JList<?> list) {
            list.setBackground(panelBackground);
            list.setForeground(foreground);
            list.setSelectionBackground(selectionBackground);
            list.setSelectionForeground(selectionForeground);
        } else if (component instanceof JTextComponent textComponent) {
            textComponent.setBackground(panelBackground);
            textComponent.setForeground(foreground);
            textComponent.setCaretColor(foreground);
            textComponent.setBorder(BorderFactory.createCompoundBorder(
                    BorderFactory.createLineBorder(borderColor),
                    new EmptyBorder(8, 10, 8, 10)));
        } else if (component instanceof JButton button) {
            button.setBackground(buttonBackground);
            button.setForeground(buttonForeground);
            button.setFocusPainted(false);
            button.setBorder(BorderFactory.createCompoundBorder(
                    BorderFactory.createLineBorder(borderColor),
                    new EmptyBorder(6, 10, 6, 10)));
        } else if (component instanceof JLabel label) {
            label.setForeground(foreground);
        } else if (component instanceof JScrollPane scrollPane) {
            scrollPane.getViewport().setBackground(panelBackground);
            scrollPane.setBackground(panelBackground);
            scrollPane.setBorder(BorderFactory.createLineBorder(borderColor));
        } else if (component instanceof JSplitPane splitPane) {
            splitPane.setBackground(background);
            splitPane.setDividerSize(8);
            splitPane.setBorder(null);
        } else if (component instanceof JPanel panel) {
            panel.setBackground(panelBackground);
        } else if (component instanceof JViewport viewport) {
            viewport.setBackground(panelBackground);
        } else if (component instanceof JComponent jComponent) {
            jComponent.setOpaque(true);
        }

        if (component instanceof Container container) {
            for (Component child : container.getComponents()) {
                applyToComponent(child);
            }
        }
    }
}