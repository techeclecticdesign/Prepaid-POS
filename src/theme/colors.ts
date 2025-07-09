export interface ColorPalette {
  primary: string;
  secondary: string;
  background: string;
  surface: string;
  textPrimary: string;
  textSecondary: string;
}

export const colors: { light: ColorPalette; dark: ColorPalette } = {
  light: {
    primary: "#1976d2",
    secondary: "#9c27b0",
    background: "#fafafa",
    surface: "#ffffff",
    textPrimary: "#111827",
    textSecondary: "#6b7280",
  },
  dark: {
    primary: "#90caf9",
    secondary: "#ce93d8",
    background: "#121212",
    surface: "#1e1e1e",
    textPrimary: "#e0e0e0",
    textSecondary: "#9e9e9e",
  },
};
