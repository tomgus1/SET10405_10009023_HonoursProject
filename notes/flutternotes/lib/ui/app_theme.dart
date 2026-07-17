import 'package:flutter/material.dart';

class AppPalette {
  final Color background;
  final Color panelBackground;
  final Color foreground;
  final Color mutedForeground;
  final Color borderColor;
  final Color buttonBackground;
  final Color buttonForeground;
  final Color selectionBackground;
  final Color selectionForeground;

  const AppPalette({
    required this.background,
    required this.panelBackground,
    required this.foreground,
    required this.mutedForeground,
    required this.borderColor,
    required this.buttonBackground,
    required this.buttonForeground,
    required this.selectionBackground,
    required this.selectionForeground,
  });

  static const light = AppPalette(
    background: Color(0xFFF5F7FA),
    panelBackground: Color(0xFFFFFFFF),
    foreground: Color(0xFF111827),
    mutedForeground: Color(0xFF6B7280),
    borderColor: Color(0xFFD1D5DB),
    buttonBackground: Color(0xFFE5E7EB),
    buttonForeground: Color(0xFF111827),
    selectionBackground: Color(0xFFDBE4FF),
    selectionForeground: Color(0xFF111827),
  );

  static const dark = AppPalette(
    background: Color(0xFF0F172A),
    panelBackground: Color(0xFF172439),
    foreground: Color(0xFFE2E8F0),
    mutedForeground: Color(0xFF94A3B8),
    borderColor: Color(0xFF334155),
    buttonBackground: Color(0xFF384A5F),
    buttonForeground: Color(0xFFE2E8F0),
    selectionBackground: Color(0xFF1D4E89),
    selectionForeground: Color(0xFFE2E8F0),
  );
}

ThemeData buildAppTheme(AppPalette palette, Brightness brightness) {
  return ThemeData(
    brightness: brightness,
    scaffoldBackgroundColor: palette.background,
    colorScheme: ColorScheme.fromSeed(
      seedColor: palette.selectionBackground,
      brightness: brightness,
      surface: palette.panelBackground,
    ),
    cardColor: palette.panelBackground,
    dividerColor: palette.borderColor,
    textTheme: ThemeData(brightness: brightness).textTheme.apply(
          bodyColor: palette.foreground,
          displayColor: palette.foreground,
        ),
    inputDecorationTheme: InputDecorationTheme(
      filled: true,
      fillColor: palette.panelBackground,
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(6),
        borderSide: BorderSide(color: palette.borderColor),
      ),
      contentPadding: const EdgeInsets.symmetric(horizontal: 10, vertical: 8),
    ),
    elevatedButtonTheme: ElevatedButtonThemeData(
      style: ElevatedButton.styleFrom(
        backgroundColor: palette.buttonBackground,
        foregroundColor: palette.buttonForeground,
        elevation: 0,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(6),
          side: BorderSide(color: palette.borderColor),
        ),
      ),
    ),
    useMaterial3: true,
  );
}
