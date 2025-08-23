import { createTheme } from '@mui/material/styles';

const theme = createTheme({
  palette: {
    primary: {
      main: '#5E35B1', // A slightly darker purple for better contrast
    },
    secondary: {
      main: '#5E35B1', // A slightly darker teal for better contrast
    },
    error: {
      main: '#D32F2F', // Standard Material-UI error red
    },
    background: {
      default: '#F8F8F8', // Lighter grey background
      paper: '#FFFFFF', // White for paper-like surfaces
    },
    text: {
      primary: '#212121', // Dark grey for primary text
      secondary: '#616161', // Slightly darker grey for secondary text
    },
  },
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

export default theme;
