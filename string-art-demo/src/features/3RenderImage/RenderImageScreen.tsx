import React, { useRef, forwardRef, useImperativeHandle } from 'react';
import { useDispatch, useSelector } from "react-redux";
import type { AppDispatch } from "../shared/redux/store";
import {
  generateStringArtThunk,
  type StringArtState,
} from "../shared/redux/stringArtSlice";
import StringArtConfigSection from "./components/StringArtConfig/StringArtConfigSection";
import { StringArtCanvas } from "./components/StringArtCanvas/StringArtCanvas";

interface RenderImageScreenProps {
  onDownloadImage?: () => void;
}

const RenderImageScreen = forwardRef<HTMLCanvasElement, RenderImageScreenProps>(({ onDownloadImage }, ref) => {
  const dispatch = useDispatch<AppDispatch>();
  const { imageData, isGenerating, progress, settings } = useSelector(
    (state: { stringArt: StringArtState }) => state.stringArt
  );

  const canvasRef = useRef<HTMLCanvasElement>(null);

  useImperativeHandle(ref, () => canvasRef.current as HTMLCanvasElement);

  const handleStartGeneration = () => {
    if (!imageData) return;
    dispatch(generateStringArtThunk({ imageData, settings }));
  };

  return (
    <main className="app-main">
      <div className="controls-section">
        {imageData && (
          <div className="generation-controls">
            <StringArtConfigSection key={"stringArt"} />

            <button
              onClick={handleStartGeneration}
              disabled={isGenerating}
              className="generate-button"
            >
              {isGenerating ? "Generating..." : "Generate String Art"}
            </button>

            {progress && (
              <div className="progress-section">
                <div className="progress-header">
                  <span>
                    Progress: {progress.completion_percent.toFixed(1)}%
                  </span>
                  <span>
                    Lines: {progress.lines_completed}/{progress.total_lines}
                  </span>
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
              ref={canvasRef}
              width={500}
              height={500}
            />
          </div>
    </main>
  );
});

export default RenderImageScreen;
