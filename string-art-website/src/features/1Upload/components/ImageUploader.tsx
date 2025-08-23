import React, { useCallback, useState } from 'react';

interface ImageUploaderProps {
  onImageSelected: (imageData: Uint8Array, imageUrl: string) => void;
  disabled?: boolean;
}

export const ImageUploader: React.FC<ImageUploaderProps> = ({ onImageSelected, disabled = false }) => {
  const [isDragOver, setIsDragOver] = useState(false);

  const handleFileSelect = useCallback(async (file: File) => {
    if (!file.type.startsWith('image/')) {
      alert('Please select an image file');
      return;
    }

    try {
      const arrayBuffer = await file.arrayBuffer();
      const imageData = new Uint8Array(arrayBuffer);
      const imageUrl = URL.createObjectURL(file);
      onImageSelected(imageData, imageUrl);
    } catch (error) {
      console.error('Error reading file:', error);
      alert('Error reading file');
    }
  }, [onImageSelected]);

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setIsDragOver(false);

    if (disabled) return;

    const files = Array.from(e.dataTransfer.files);
    if (files.length > 0) {
      handleFileSelect(files[0]);
    }
  }, [handleFileSelect, disabled]);

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    if (!disabled) {
      setIsDragOver(true);
    }
  }, [disabled]);

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setIsDragOver(false);
  }, []);

  const handleFileInputChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (files && files.length > 0) {
      handleFileSelect(files[0]);
    }
  }, [handleFileSelect]);

  return (
    <div
      className={`
        image-uploader
        ${isDragOver ? 'drag-over' : ''}
        ${disabled ? 'disabled' : ''}
      `}
      onDrop={handleDrop}
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
    >
      <div className="upload-content">
        <div className="upload-icon">📸</div>
        <p className="upload-text">
          {disabled 
            ? 'Upload disabled during generation'
            : 'Drag & drop an image here, or click to select'
          }
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
          onClick={() => document.querySelector<HTMLInputElement>('.file-input')?.click()}
        >
          Choose Image
        </button>
      </div>

      <style>{`
        .image-uploader {
          border: 2px dashed #ccc;
          border-radius: 8px;
          padding: 2rem;
          text-align: center;
          background: #fafafa;
          transition: all 0.3s ease;
          cursor: pointer;
        }

        .image-uploader:hover:not(.disabled) {
          border-color: #007bff;
          background: #f0f8ff;
        }

        .image-uploader.drag-over {
          border-color: #007bff;
          background: #e3f2fd;
          transform: scale(1.02);
        }

        .image-uploader.disabled {
          opacity: 0.6;
          cursor: not-allowed;
          background: #f5f5f5;
        }

        .upload-content {
          display: flex;
          flex-direction: column;
          align-items: center;
          gap: 1rem;
        }

        .upload-icon {
          font-size: 3rem;
          opacity: 0.6;
        }

        .upload-text {
          margin: 0;
          color: #666;
          font-size: 1rem;
        }

        .file-input {
          display: none;
        }

        .upload-button {
          background: #007bff;
          color: white;
          border: none;
          padding: 0.75rem 1.5rem;
          border-radius: 4px;
          cursor: pointer;
          font-size: 1rem;
          transition: background 0.3s ease;
        }

        .upload-button:hover:not(:disabled) {
          background: #0056b3;
        }

        .upload-button:disabled {
          background: #ccc;
          cursor: not-allowed;
        }
      `}</style>
    </div>
  );
};
