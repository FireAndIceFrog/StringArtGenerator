import { useDispatch, useSelector } from "react-redux";
import type { AppDispatch } from "../shared/redux/store";
import { generateStringArtThunk, type StringArtState } from "../shared/redux/stringArtSlice";
import StringArtConfigSection from "./components/StringArtConfig/StringArtConfigSection";

export default function RenderImageScreen() {
  const dispatch = useDispatch<AppDispatch>();
  const {
    imageData,
    isGenerating,
    progress,
    settings,
  } = useSelector((state: { stringArt: StringArtState }) => state.stringArt);

  const handleStartGeneration = () => {
    if (!imageData) return;
    dispatch(
      generateStringArtThunk({ imageData, settings })
    );
  };

  return (
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
            <span>Progress: {progress.completion_percent.toFixed(1)}%</span>
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
  );
}
