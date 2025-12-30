import { createTheme } from '@mui/material/styles';
import type { PaletteMode } from '@mui/material';

/**
 * Returns a MUI theme configured for the given mode ('light' | 'dark').
 * Call with getTheme(mode) from your ThemeProvider wrapper.
 */
export default function getTheme(mode: 'light' | 'dark') {
  // Guard against unexpected values for `mode` at runtime (avoid `[object Object]`)
  const paletteMode = mode === 'dark' ? 'dark' : 'light';
  const isDark = paletteMode === 'dark';

    const palette = {
      mode: paletteMode as PaletteMode,
      primary: {
        main: '#5E35B1',
      },
    secondary: {
      main: '#5E35B1',
    },
    error: {
      main: '#D32F2F',
    },
    background: isDark
      ? { default: '#242424', paper: '#1f1f1f' }
      : { default: '#F8F8F8', paper: '#FFFFFF' },
    text: isDark
      ? { primary: 'rgba(255,255,255,0.87)', secondary: 'rgba(255,255,255,0.7)' }
      : { primary: '#212121', secondary: '#616161' },
  };

  return createTheme({
    palette,
    typography: {
      fontFamily: 'Roboto, sans-serif',
      h1: {
        fontSize: '2.5rem',
        fontWeight: 500,
      },
      body1: {
        fontSize: '1rem',
      },
    },
    components: {
      MuiButton: {
        styleOverrides: {
          root: {
            borderRadius: 8,
          },
        },
      },
      MuiStepper: {
        styleOverrides: {
          root: {
            padding: '24px 0',
          },
        },
      },
      MuiStepLabel: {
        styleOverrides: {
          label: {
            '&.Mui-active': {
              fontWeight: 600,
              color: '#5E35B1',
            },
            '&.Mui-completed': {
              fontWeight: 600,
              color: '#5E35B1',
            },
          },
        },
      },
    },
  });
}
