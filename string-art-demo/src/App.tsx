import { useSelector } from 'react-redux';
import './App.css';
import StepperScreen from './features/Stepper/StepperScreen';
import {
  type StringArtState
} from './features/shared/redux/stringArtSlice';
import { ThemeProvider } from '@mui/material/styles';
import theme from './theme';

function App() {
  const {
    isLoading,
    error,
    isGenerating,
  } = useSelector((state: { stringArt: StringArtState }) => state.stringArt);

  if (isLoading) {
    return (
      <div className="app-loading">
        <div className="loading-spinner"></div>
        <h2>Loading WASM Module...</h2>
        <p>Please wait while we initialize the string art generator.</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="app-error">
        <h2>Error Loading WASM Module</h2>
        <p>{error}</p>
        <button onClick={() => window.location.reload()}>Retry</button>
      </div>
    );
  }

  return (
    <ThemeProvider theme={theme}>
      <div className="app">
        <header className="app-header">
          <h1>{'String Art Generator'}</h1>
          <p>Upload an image and watch it transform into beautiful string art in real-time!</p>
        </header>

        <StepperScreen />
      </div>
    </ThemeProvider>
  );
}

export default App;
