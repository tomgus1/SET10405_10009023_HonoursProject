import { StatusBar } from 'expo-status-bar';
import CalculatorScreen from './src/CalculatorScreen';

export default function App() {
  return (
    <>
      <CalculatorScreen />
      <StatusBar style="auto" />
    </>
  );
}
