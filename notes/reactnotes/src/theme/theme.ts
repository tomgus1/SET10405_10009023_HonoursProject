export type ThemeName = 'light' | 'dark';

export interface Theme {
  name: ThemeName;
  background: string;
  panelBackground: string;
  foreground: string;
  mutedForeground: string;
  borderColor: string;
  buttonBackground: string;
  buttonForeground: string;
  selectionBackground: string;
  toggleLabel: string;
}

export const themes: Record<ThemeName, Theme> = {
  light: {
    name: 'light',
    background: '#F5F7FA',
    panelBackground: '#FFFFFF',
    foreground: '#111827',
    mutedForeground: '#6B7280',
    borderColor: '#D1D5DB',
    buttonBackground: '#E5E7EB',
    buttonForeground: '#111827',
    selectionBackground: '#DBE4FF',
    toggleLabel: 'Dark mode',
  },
  dark: {
    name: 'dark',
    background: '#0F172A',
    panelBackground: '#172439',
    foreground: '#E2E8F0',
    mutedForeground: '#94A3B8',
    borderColor: '#334155',
    buttonBackground: '#384A5F',
    buttonForeground: '#E2E8F0',
    selectionBackground: '#1D4E89',
    toggleLabel: 'Light mode',
  },
};

export function opposite(name: ThemeName): ThemeName {
  return name === 'light' ? 'dark' : 'light';
}
