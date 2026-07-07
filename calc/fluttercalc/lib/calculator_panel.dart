import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import 'calculator_model.dart';
import 'operation.dart';

class CalculatorPanel extends StatefulWidget {
  const CalculatorPanel({super.key});

  @override
  State<CalculatorPanel> createState() => _CalculatorPanelState();
}

class _CalculatorPanelState extends State<CalculatorPanel> {
  final CalculatorModel _model = CalculatorModel();
  final FocusNode _focusNode = FocusNode();

  static const _buttonGrid = [
    ['C', 'CE', '⌫', '/'],
    ['7', '8', '9', '*'],
    ['4', '5', '6', '-'],
    ['1', '2', '3', '+'],
    ['±', '0', '.', '='],
  ];

  @override
  void dispose() {
    _focusNode.dispose();
    super.dispose();
  }

  void _handleInput(String command) {
    setState(() {
      if (RegExp(r'^\d$').hasMatch(command)) {
        _model.enterDigit(command);
      } else if (command == '.') {
        _model.enterDecimal();
      } else if (command == '⌫') {
        _model.backspace();
      } else if (command == 'C') {
        _model.reset();
      } else if (command == 'CE') {
        _model.clearEntry();
      } else if (command == '±') {
        _model.toggleSign();
      } else if (command == '=') {
        _model.calculateEquals();
      } else {
        final op = Operation.fromSymbol(command);
        if (op != null) _model.setOperator(op);
      }
    });
  }

  KeyEventResult _handleKey(FocusNode node, KeyEvent event) {
    if (event is! KeyDownEvent) return KeyEventResult.ignored;

    final key = event.logicalKey;
    final label = key.keyLabel;

    if (RegExp(r'^\d$').hasMatch(label)) {
      _handleInput(label);
      return KeyEventResult.handled;
    }
    if (key == LogicalKeyboardKey.numpadAdd || label == '+') {
      _handleInput('+');
      return KeyEventResult.handled;
    }
    if (key == LogicalKeyboardKey.numpadSubtract || label == '-') {
      _handleInput('-');
      return KeyEventResult.handled;
    }
    if (key == LogicalKeyboardKey.numpadMultiply || label == '*') {
      _handleInput('*');
      return KeyEventResult.handled;
    }
    if (key == LogicalKeyboardKey.numpadDivide || label == '/') {
      _handleInput('/');
      return KeyEventResult.handled;
    }
    if (key == LogicalKeyboardKey.numpadDecimal || label == '.') {
      _handleInput('.');
      return KeyEventResult.handled;
    }
    if (key == LogicalKeyboardKey.enter ||
        key == LogicalKeyboardKey.numpadEnter ||
        label == '=') {
      _handleInput('=');
      return KeyEventResult.handled;
    }
    if (key == LogicalKeyboardKey.backspace) {
      _handleInput('⌫');
      return KeyEventResult.handled;
    }
    if (key == LogicalKeyboardKey.escape || label == 'c' || label == 'C') {
      _handleInput('C');
      return KeyEventResult.handled;
    }
    return KeyEventResult.ignored;
  }

  Color? _buttonColor(String label) {
    if (label == '=') return const Color(0xFF337AB7);
    if (RegExp(r'^[/*\-+]$').hasMatch(label)) return const Color(0xFFE6EBF5);
    if (label == 'C' || label == 'CE' || label == '⌫') {
      return const Color(0xFFF5E1E1);
    }
    return Colors.white;
  }

  Color _textColor(String label) {
    if (label == '=') return Colors.white;
    if (RegExp(r'^[/*\-+]$').hasMatch(label)) return const Color(0xFF1E3C78);
    if (label == 'C' || label == 'CE' || label == '⌫') {
      return const Color(0xFF962828);
    }
    return const Color(0xFF282828);
  }

  @override
  Widget build(BuildContext context) {
    return Focus(
      focusNode: _focusNode,
      autofocus: true,
      onKeyEvent: _handleKey,
      child: Container(
        color: const Color(0xFFF0F0F5),
        padding: const EdgeInsets.all(15),
        child: Column(
          children: [
            _buildDisplay(),
            const SizedBox(height: 10),
            Expanded(child: _buildButtonGrid()),
          ],
        ),
      ),
    );
  }

  Widget _buildDisplay() {
    return Container(
      width: double.infinity,
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
      decoration: BoxDecoration(
        color: Colors.white,
        border: Border.all(color: const Color(0xFFD2D2DC)),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.end,
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            _model.expressionText,
            style: const TextStyle(
              fontSize: 13,
              color: Color(0xFF78788C),
            ),
          ),
          const SizedBox(height: 4),
          Text(
            _model.currentInput,
            style: const TextStyle(
              fontSize: 28,
              fontWeight: FontWeight.bold,
              color: Color(0xFF1E1E1E),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildButtonGrid() {
    return Column(
      children: [
        for (final row in _buttonGrid) ...[
          Expanded(
            child: Row(
              children: [
                for (final label in row) ...[
                  Expanded(child: _buildButton(label)),
                  if (label != row.last) const SizedBox(width: 8),
                ],
              ],
            ),
          ),
          if (row != _buttonGrid.last) const SizedBox(height: 8),
        ],
      ],
    );
  }

  Widget _buildButton(String label) {
    return ElevatedButton(
      onPressed: () => _handleInput(label),
      style: ElevatedButton.styleFrom(
        backgroundColor: _buttonColor(label),
        foregroundColor: _textColor(label),
        elevation: 0,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(4),
        ),
      ),
      child: Text(
        label,
        style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
      ),
    );
  }
}
