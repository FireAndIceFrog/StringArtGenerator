// main.tsx
import { createRoot } from 'react-dom/client';
import { Provider } from 'react-redux';
import { store } from './features/shared/redux/store';
import './index.css';
import App from './App.tsx';
import { I18nextProvider } from 'react-i18next';
import i18n from './i18n.tsx';

(async () => {
  await i18n.init ();
  createRoot(document.getElementById('root')!).render(
    <Provider store={store}>
      <I18nextProvider i18n={i18n} >
        <App />
      </I18nextProvider>
    </Provider>
  );
})();