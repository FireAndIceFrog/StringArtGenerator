import { useCallback } from "react";
import { useDispatch, useSelector } from "react-redux";
import type { AppDispatch } from "../shared/redux/store";
import { type StringArtState, resetState, setImageData, setImageUrl } from "../shared/redux/stringArtSlice";
import { ImageUploader } from "./components/ImageUploader";

interface UploadScreenProps {
  onImageSelected?: () => void;
}

export default function UploadScreen({ onImageSelected }: UploadScreenProps) {
  const dispatch = useDispatch<AppDispatch>();
  const {
    isGenerating
  } = useSelector((state: { stringArt: StringArtState }) => state.stringArt);

  const handleImageSelected = useCallback(
    (data: Uint8Array, url: string) => {
      dispatch(resetState());
      dispatch(setImageData(data));
      dispatch(setImageUrl(url));
      if (onImageSelected) {
        onImageSelected();
      }
    },
    [dispatch, onImageSelected]
  );

  return (
    <div className="upload-section">
      <ImageUploader
        onImageSelected={handleImageSelected}
        disabled={isGenerating}
      />
    </div>
  );
};
