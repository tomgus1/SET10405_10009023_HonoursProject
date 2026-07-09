import React, { useCallback, useEffect, useRef, useState } from 'react';
import {
  Platform,
  Pressable,
  SafeAreaView,
  StyleSheet,
  Text,
  View,
} from 'react-native';
import { CalculatorModel, CalculatorState } from './calculatorModel';
import { operationFromSymbol } from './operation';

type ButtonKind = 'equals' | 'operator' | 'action' | 'digit';

const BUTTON_GRID: string[][] = [
  ['C', 'CE', '⌫', '/'],
  ['7', '8', '9', '*'],
  ['4', '5', '6', '-'],
  ['1', '2', '3', '+'],
  ['±', '0', '.', '='],
];

function kindOf(label: string): ButtonKind {
  if (label === '=') return 'equals';
  if (['/', '*', '-', '+'].includes(label)) return 'operator';
  if (['C', 'CE', '⌫'].includes(label)) return 'action';
  return 'digit';
}

export default function CalculatorScreen() {
  const modelRef = useRef(new CalculatorModel());
  const [state, setState] = useState<CalculatorState>(modelRef.current.getState());

  const refresh = useCallback(() => {
    setState(modelRef.current.getState());
  }, []);

  const handleInput = useCallback(
    (command: string) => {
      const model = modelRef.current;
      if (/^\d$/.test(command)) {
        model.enterDigit(command);
      } else if (command === '.') {
        model.enterDecimal();
      } else if (command === '⌫') {
        model.backspace();
      } else if (command === 'C') {
        model.reset();
      } else if (command === 'CE') {
        model.clearEntry();
      } else if (command === '±') {
        model.toggleSign();
      } else if (command === '=') {
        model.calculateEquals();
      } else {
        const op = operationFromSymbol(command);
        if (op) {
          model.setOperator(op);
        }
      }
      refresh();
    },
    [refresh]
  );

  useEffect(() => {
    if (Platform.OS !== 'web') return;

    const KEY_COMMANDS: Record<string, string> = {
      Enter: '=',
      Backspace: '⌫',
      Escape: 'C',
      c: 'C',
      C: 'C',
    };

    const onKeyDown = (e: KeyboardEvent) => {
      const command = /^\d$/.test(e.key)
        ? e.key
        : ['+', '-', '*', '/', '.'].includes(e.key)
        ? e.key
        : KEY_COMMANDS[e.key];
      if (command) {
        e.preventDefault();
        handleInput(command);
      }
    };

    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
  }, [handleInput]);

  return (
    <SafeAreaView style={styles.safeArea}>
      <View style={styles.container}>
        <View style={styles.displayPanel}>
          <Text style={styles.expressionLabel} numberOfLines={1}>
            {state.expressionText}
          </Text>
          <Text style={styles.displayField} numberOfLines={1} adjustsFontSizeToFit>
            {state.currentInput}
          </Text>
        </View>

        <View style={styles.buttonGrid}>
          {BUTTON_GRID.map((row, rowIndex) => (
            <View style={styles.buttonRow} key={rowIndex}>
              {row.map((label) => (
                <CalculatorButton key={label} label={label} onPress={handleInput} />
              ))}
            </View>
          ))}
        </View>
      </View>
    </SafeAreaView>
  );
}

function CalculatorButton({
  label,
  onPress,
}: {
  label: string;
  onPress: (command: string) => void;
}) {
  const kind = kindOf(label);
  return (
    <Pressable
      style={({ pressed }) => [
        styles.button,
        buttonKindStyles[kind],
        pressed && styles.buttonPressed,
      ]}
      onPress={() => onPress(label)}
    >
      <Text style={[styles.buttonText, buttonTextKindStyles[kind]]}>{label}</Text>
    </Pressable>
  );
}

const COLORS = {
  background: '#f0f0f5',
  displayBackground: '#ffffff',
  displayBorder: '#d2d2dc',
  expressionText: '#78788a',
  displayText: '#1e1e1e',
  digitBackground: '#ffffff',
  digitText: '#282828',
  operatorBackground: '#e6ebf5',
  operatorText: '#1e3c78',
  actionBackground: '#f5e1e1',
  actionText: '#962828',
  equalsBackground: '#337ab7',
  equalsText: '#ffffff',
};

const styles = StyleSheet.create({
  safeArea: {
    flex: 1,
    backgroundColor: COLORS.background,
  },
  container: {
    flex: 1,
    padding: 15,
    justifyContent: 'flex-end',
    maxWidth: 420,
    width: '100%',
    alignSelf: 'center',
  },
  displayPanel: {
    backgroundColor: COLORS.displayBackground,
    borderWidth: 1,
    borderColor: COLORS.displayBorder,
    borderRadius: 8,
    paddingHorizontal: 12,
    paddingVertical: 8,
    marginBottom: 10,
  },
  expressionLabel: {
    textAlign: 'right',
    fontSize: 13,
    color: COLORS.expressionText,
  },
  displayField: {
    textAlign: 'right',
    fontSize: 34,
    fontWeight: 'bold',
    color: COLORS.displayText,
    marginTop: 4,
  },
  buttonGrid: {
    gap: 8,
  },
  buttonRow: {
    flexDirection: 'row',
    gap: 8,
  },
  button: {
    flex: 1,
    aspectRatio: 1.3,
    borderRadius: 10,
    alignItems: 'center',
    justifyContent: 'center',
    ...Platform.select({
      web: { cursor: 'pointer' },
      default: {},
    }),
  },
  buttonPressed: {
    opacity: 0.7,
  },
  buttonText: {
    fontSize: 20,
    fontWeight: 'bold',
  },
});

const buttonKindStyles: Record<ButtonKind, { backgroundColor: string }> = StyleSheet.create({
  digit: { backgroundColor: COLORS.digitBackground },
  operator: { backgroundColor: COLORS.operatorBackground },
  action: { backgroundColor: COLORS.actionBackground },
  equals: { backgroundColor: COLORS.equalsBackground },
});

const buttonTextKindStyles: Record<ButtonKind, { color: string }> = StyleSheet.create({
  digit: { color: COLORS.digitText },
  operator: { color: COLORS.operatorText },
  action: { color: COLORS.actionText },
  equals: { color: COLORS.equalsText },
});
