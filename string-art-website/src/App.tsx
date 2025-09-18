import { useSelector } from 'react-redux';
import './App.css';
import StepperScreen from './features/Stepper/StepperScreen';
import {
  type StringArtState
} from './features/shared/redux/stringArtSlice';
import { ThemeProvider } from '@mui/material/styles';
import theme from './theme';
import {useTranslation} from 'react-i18next';
function App() {
  const {
    isLoading,
    error,
  } = useSelector((state: { stringArt: StringArtState }) => state.stringArt);
  const i18next = useTranslation();
  if (isLoading) {
    return (
      <div className="app-loading">
        <div className="loading-spinner"></div>
        <h2>{i18next.t('Loading WASM Module')}</h2>
        <p>{i18next.t('Please wait while we initialize the string art generator.')}</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="app-error">
        <h2>{i18next.t('Error Loading WASM Module')}</h2>
        <p>{error}</p>
        <button onClick={() => window.location.reload()}>{i18next.t('Retry')}</button>
      </div>
    );
  }

  return (
    <ThemeProvider theme={theme}>
      <div className="app">
        <header className="app-header">
          <h1>{i18next.t('String Art Generator')}</h1>
          <p>{i18next.t('Upload an image and watch it transform into beautiful string art in real-time!')}</p>
        </header>

        <StepperScreen />
      </div>
    </ThemeProvider>
  );
}

export default App;
