import React, { useCallback, useState } from "react";
import { useTranslation } from "react-i18next";
import "./style.css";
interface ImageUploaderProps {
  onImageSelected: (imageData: Uint8Array, imageUrl: string) => void;
  disabled?: boolean;
}

export const ImageUploader: React.FC<ImageUploaderProps> = ({
  onImageSelected,
  disabled = false,
}) => {
  const [isDragOver, setIsDragOver] = useState(false);
  const i18next = useTranslation();
  const handleFileSelect = useCallback(
    async (file: File) => {
      if (!file.type.startsWith("image/")) {
        alert(i18next.t("Please select an image file"));
        return;
      }

      try {
        const arrayBuffer = await file.arrayBuffer();
        const imageData = new Uint8Array(arrayBuffer);
        const imageUrl = URL.createObjectURL(file);
        onImageSelected(imageData, imageUrl);
      } catch (error) {
        console.error(i18next.t("Error reading file:"), error);
        alert(i18next.t("Error reading file"));
      }
    },
    [onImageSelected, i18next]
  );

  const handleDrop = useCallback(
    (e: React.DragEvent) => {
      e.preventDefault();
      setIsDragOver(false);

      if (disabled) return;

      const files = Array.from(e.dataTransfer.files);
      if (files.length > 0) {
        handleFileSelect(files[0]);
      }
    },
    [handleFileSelect, disabled]
  );

  const handleDragOver = useCallback(
    (e: React.DragEvent) => {
      e.preventDefault();
      if (!disabled) {
        setIsDragOver(true);
      }
    },
    [disabled]
  );

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setIsDragOver(false);
  }, []);

  const handleFileInputChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const files = e.target.files;
      if (files && files.length > 0) {
        handleFileSelect(files[0]);
      }
    },
    [handleFileSelect]
  );

  return (
    <div
      className={`
        image-uploader
        ${isDragOver ? "drag-over" : ""}
        ${disabled ? "disabled" : ""}
      `}
      onDrop={handleDrop}
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
    >
      <div className="upload-content">
        <div className="upload-icon">📸</div>
        <p className="upload-text">
          {disabled
            ? i18next.t("Upload disabled during generation")
            : i18next.t("Drag & drop an image here, or click to select")}
        </p>
        <input
          type="file"
          accept="image/*"
          onChange={handleFileInputChange}
          disabled={disabled}
          className="file-input"
        />
        <button
          className="upload-button"
          disabled={disabled}
          onClick={() =>
            document.querySelector<HTMLInputElement>(".file-input")?.click()
          }
        >
          {i18next.t("Choose Image")}
        </button>
      </div>
    </div>
  );
};
