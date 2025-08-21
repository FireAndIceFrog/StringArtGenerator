import { useSelector } from "react-redux";
import { StringArtCanvas } from "./components/StringArtCanvas/StringArtCanvas";
import type { StringArtState } from "../shared/redux/stringArtSlice";

export default function CanvasScreen() {
  const { imageUrl, isGenerating, currentPath, nailCoords } = useSelector(
    (state: { stringArt: StringArtState }) => state.stringArt
  );
  return (
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
  );
}
