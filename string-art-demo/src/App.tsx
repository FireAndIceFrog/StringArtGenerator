import { useState, useCallback } from 'react';
import { ImageUploader } from './components/ImageUploader';
import { StringArtCanvas } from './components/StringArtCanvas';
import './App.css';
import StringArtConfigSection from './components/StringArtConfig/StringArtConfigSection';
import { useStringArt, type ProgressInfo } from './useStringArt';
function App() {
  const [imageData, setImageData] = useState<Uint8Array | null>(null);
  const [imageUrl, setImageUrl] = useState<string>('');
  const [isGenerating, setIsGenerating] = useState(false);
  const [currentPath, setCurrentPath] = useState<number[]>([]);
  const [nailCoords, setNailCoords] = useState<Array<[number, number]>>([]);
  const [progress, setProgress] = useState<ProgressInfo | null>(null);
  const { generateStringArt, isLoading, error, settings } = useStringArt();
  const handleImageSelected = useCallback((data: Uint8Array, url: string) => {
    setImageData(data);
    setImageUrl(url);
    setCurrentPath([]);
    setProgress(null);
    setNailCoords([]); // Clear until we generate the string art
  }, []);

  const handleStartGeneration = async () => {
    if (!imageData) return;

    setIsGenerating(true);
    setCurrentPath([]);
    setProgress(null);

    try {
      const onProgress = (progressInfo: ProgressInfo) => {
        setProgress(progressInfo);
        
        // Update the path in real-time
        setCurrentPath((prevPath) => [...prevPath, ...progressInfo.current_path]);
      };

      const onNailCoords = (coords: Array<[number, number]>) => {
        setNailCoords(coords);
      };

      const result = await generateStringArt(imageData, onProgress, onNailCoords);
      if (result.path) {
        setCurrentPath((prevPath) => [...prevPath, ...(result.path || [])]);
      }
      if (result.nailCoords.length > 0) {
        setNailCoords(result.nailCoords);
      }
    } catch (err) {
      console.error('Generation failed:', err);
      alert('String art generation failed. Please try again.');
    } finally {
      setIsGenerating(false);
    }
  };

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
          <div className="upload-section">
            <ImageUploader 
              onImageSelected={handleImageSelected}
              disabled={isGenerating}
            />
          </div>

          {imageData && (
            <div className="generation-controls">
              <StringArtConfigSection key={"stringArt"} settings={settings} />
              
              <button 
                onClick={handleStartGeneration}
                disabled={isGenerating}
                className="generate-button"
              >
                {isGenerating ? 'Generating...' : 'Generate String Art'}
              </button>

              {progress && (
                <div className="progress-section">
                  <div className="progress-header">
                    <span>Progress: {progress.completion_percent.toFixed(1)}%</span>
                    <span>Lines: {progress.lines_completed}/{progress.total_lines}</span>
                  </div>
                  <div className="progress-bar">
                    <div 
                      className="progress-fill"
                      style={{ width: `${progress.completion_percent}%` }}
                    ></div>
                  </div>
                  <div className="progress-details">
                    <span>Score: {progress.score.toFixed(1)}</span>
                  </div>
                </div>
              )}
            </div>
          )}
        </div>

        <div className="canvas-section">
          <StringArtCanvas 
            width={500}
            height={500}
            nailCoords={nailCoords}
            currentPath={currentPath}
            isAnimating={isGenerating}
            imageUrl={imageUrl}
          />
        </div>
      </main>
    </div>
  );
}

export default App;
