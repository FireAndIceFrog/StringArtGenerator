import { useSelector } from 'react-redux';
import './App.css';
import {
  type StringArtState
} from './features/shared/redux/stringArtSlice';
import UploadScreen from './features/1Upload/UploadScreen';
import RenderImageScreen from './features/3RenderImage/RenderImageScreen';
import CanvasScreen from './features/3RenderImage/CanvasScreen';

function App() {
  const {
    imageData,
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

      <main className="app-main">
        <div className="controls-section">
          <UploadScreen/>

          {imageData && (
            <RenderImageScreen/>
          )}
        </div>

        <CanvasScreen/>
      </main>
    </div>
  );
}

export default App;
