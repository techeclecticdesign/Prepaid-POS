import { createTheme, type Theme } from "@mui/material/styles";
import { colors, type ColorPalette } from "./colors";

export const getMuiTheme = (mode: keyof typeof colors = "light"): Theme =>
  createTheme({
    palette: {
      mode,
      primary: { main: (colors[mode] as ColorPalette).primary },
      secondary: { main: colors[mode].secondary },
      background: {
        default: colors[mode].background,
        paper: colors[mode].surface,
      },
      text: {
        primary: colors[mode].textPrimary,
        secondary: colors[mode].textSecondary,
      },
    },
    typography: {
      fontFamily: '"Lato", sans-serif',
    },
    breakpoints: {
      values: {
        xs: 0,
        sm: 768,
        md: 1024,
        lg: 1280,
        xl: 1536,
      },
    },
    components: {
      MuiCssBaseline: {
        styleOverrides: (theme: Theme) => ({
          "::-webkit-scrollbar": {
            width: 8,
          },
          "::-webkit-scrollbar-track": {
            background: theme.palette.background.paper,
          },
          "::-webkit-scrollbar-thumb": {
            backgroundColor: theme.palette.text.secondary,
            borderRadius: 4,
          },
          "::-webkit-scrollbar-thumb:hover": {
            backgroundColor: theme.palette.text.primary,
          },
        }),
      },
    },
  });
