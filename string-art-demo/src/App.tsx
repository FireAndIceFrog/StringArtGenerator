import React, { useState, useCallback } from 'react';
import { ImageUploader } from './components/ImageUploader';
import { StringArtCanvas } from './components/StringArtCanvas';
import { useStringArt, type StringArtConfig, type ProgressInfo } from './hooks/useStringArt';
import './App.css';

function App() {
  const { wasmModule, isLoading, error, generateStringArt, presets } = useStringArt();
  const [imageData, setImageData] = useState<Uint8Array | null>(null);
  const [imageUrl, setImageUrl] = useState<string>('');
  const [isGenerating, setIsGenerating] = useState(false);
  const [currentPath, setCurrentPath] = useState<number[]>([]);
  const [nailCoords, setNailCoords] = useState<Array<[number, number]>>([]);
  const [progress, setProgress] = useState<ProgressInfo | null>(null);
  const [config, setConfig] = useState<StringArtConfig>(presets.balanced());

  const handleImageSelected = useCallback((data: Uint8Array, url: string) => {
    setImageData(data);
    setImageUrl(url);
    setCurrentPath([]);
    setProgress(null);
    setNailCoords([]); // Clear until we generate the string art
  }, []);

  const handleStartGeneration = useCallback(async () => {
    if (!imageData || !wasmModule) return;

    setIsGenerating(true);
    setCurrentPath([]);
    setProgress(null);

    try {
      const onProgress = (progressInfo: ProgressInfo) => {
        setProgress(progressInfo);
        
        // Update the path in real-time
        setCurrentPath(progressInfo.current_path || []);
      };

      const onNailCoords = (coords: Array<[number, number]>) => {
        setNailCoords(coords);
      };

      const result = await generateStringArt(imageData, config, onProgress, onNailCoords);
      if (result.path) {
        setCurrentPath(result.path);
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
  }, [imageData, wasmModule, config, generateStringArt]);

  const handlePresetChange = useCallback((presetName: 'fast' | 'balanced' | 'highQuality') => {
    setConfig(presets[presetName]());
  }, [presets]);

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
              <div className="config-section">
                <h3>Quality Presets</h3>
                <div className="preset-buttons">
                  <button 
                    onClick={() => handlePresetChange('fast')}
                    className={`preset-button ${config.num_nails === 360 ? 'active' : ''}`}
                    disabled={isGenerating}
                  >
                    Fast (360 nails)
                  </button>
                  <button 
                    onClick={() => handlePresetChange('balanced')}
                    className={`preset-button ${config.num_nails === 720 ? 'active' : ''}`}
                    disabled={isGenerating}
                  >
                    Balanced (720 nails)
                  </button>
                  <button 
                    onClick={() => handlePresetChange('highQuality')}
                    className={`preset-button ${config.num_nails === 1440 ? 'active' : ''}`}
                    disabled={isGenerating}
                  >
                    High Quality (1440 nails)
                  </button>
                </div>
              </div>

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
                    <span>Current: Nail {progress.current_nail} → {progress.next_nail}</span>
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
