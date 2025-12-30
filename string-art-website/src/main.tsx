 // main.tsx
import { useMemo } from 'react';
import { createRoot } from 'react-dom/client';
import { Provider } from 'react-redux';
import { store } from './features/shared/redux/store';
import './index.css';
import App from './App.tsx';
import { I18nextProvider } from 'react-i18next';
import i18n from './i18n.tsx';
import { ThemeProvider, CssBaseline, useMediaQuery } from '@mui/material';
import getTheme from './theme';

function AppWrapper() {
  const prefersDark = useMediaQuery('(prefers-color-scheme: dark)');
  const mode: 'light' | 'dark' = prefersDark ? 'dark' : 'light';
  const theme = useMemo(() => getTheme(mode), [mode]);

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <App />
    </ThemeProvider>
  );
}

(async () => {
  await i18n.init ();
  createRoot(document.getElementById('root')!).render(
    <Provider store={store}>
      <I18nextProvider i18n={i18n} >
        <AppWrapper />
      </I18nextProvider>
    </Provider>
  );
})();
