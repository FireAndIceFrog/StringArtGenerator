import { useSelector } from 'react-redux';
import './App.css';
import {
  type StringArtState
} from './features/shared/redux/stringArtSlice';
import RenderImageScreen from './features/3RenderImage/RenderImageScreen';

function App() {
  const {
    isLoading,
    error,
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
    <div className="app">
      <header className="app-header">
        <h1>🎨 String Art Generator</h1>
        <p>Upload an image and watch it transform into beautiful string art in real-time!</p>
      </header>

      <RenderImageScreen/>
    </div>
  );
}

export default App;
