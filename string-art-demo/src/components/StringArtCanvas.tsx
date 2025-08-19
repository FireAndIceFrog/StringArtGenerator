import React, { useRef, useEffect, useState } from 'react';

interface StringArtCanvasProps {
  width: number;
  height: number;
  nailCoords: Array<[number, number]>;
  currentPath: number[];
  isAnimating: boolean;
  imageUrl?: string;
}

export const StringArtCanvas: React.FC<StringArtCanvasProps> = ({
  width,
  height,
  nailCoords,
  currentPath,
  isAnimating,
  imageUrl
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [showOriginal, setShowOriginal] = useState(false);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Always clear the canvas at the start of a render
    ctx.fillStyle = 'white';
    ctx.clearRect(0, 0, width, height);

    let isCancelled = false;

    if (showOriginal && imageUrl) {
      const img = new Image();
      img.onload = () => {
        if (isCancelled) return;

        // Clear canvas again right before drawing to prevent race conditions
        ctx.fillStyle = 'white';
        ctx.fillRect(0, 0, width, height);

        const scale = Math.min(width / img.width, height / img.height);
        const scaledWidth = img.width * scale;
        const scaledHeight = img.height * scale;
        const x = (width - scaledWidth) / 2;
        const y = (height - scaledHeight) / 2;
        
        ctx.drawImage(img, x, y, scaledWidth, scaledHeight);
      };
      img.src = imageUrl;
    } else {
      // Draw nails as small circles
      ctx.fillStyle = '#666';
      nailCoords.forEach(([x, y]) => {
        ctx.beginPath();
        ctx.arc(x, y, 2, 0, 2 * Math.PI);
        ctx.fill();
      });

      // Draw string path
      if (currentPath.length > 1) {
        ctx.strokeStyle = 'rgba(0, 0, 0, 0.3)';
        ctx.lineWidth = 0.5;
        ctx.globalCompositeOperation = 'multiply';

        for (let i = 0; i < currentPath.length - 1; i++) {
          const fromNail = currentPath[i];
          const toNail = currentPath[i + 1];
          
          if (fromNail < nailCoords.length && toNail < nailCoords.length) {
            const [x1, y1] = nailCoords[fromNail];
            const [x2, y2] = nailCoords[toNail];
            
            ctx.beginPath();
            ctx.moveTo(x1, y1);
            ctx.lineTo(x2, y2);
            ctx.stroke();
          }
        }
      }
    }

    return () => {
      isCancelled = true;
    };
  }, [width, height, nailCoords, currentPath, showOriginal, imageUrl]);

  return (
    <div className="string-art-canvas-container">
      <div className="canvas-header">
        <h3>String Art Preview</h3>
        <div className="canvas-controls">
          <button 
            onClick={() => setShowOriginal(!showOriginal)}
            className="toggle-button"
            disabled={!imageUrl}
          >
            {showOriginal ? 'Show String Art' : 'Show Original'}
          </button>
        </div>
      </div>
      
      <div className="canvas-wrapper">
        <canvas
          ref={canvasRef}
          width={width}
          height={height}
          className={`string-art-canvas ${isAnimating ? 'animating' : ''}`}
        />
        
        {isAnimating && (
          <div className="animation-overlay">
            <div className="pulse">🎨</div>
          </div>
        )}
      </div>

      <style>{`
        .string-art-canvas-container {
          border: 1px solid #ddd;
          border-radius: 8px;
          overflow: hidden;
          background: white;
        }

        .canvas-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          padding: 1rem;
          background: #f8f9fa;
          border-bottom: 1px solid #ddd;
        }

        .canvas-header h3 {
          margin: 0;
          color: #333;
        }

        .canvas-controls {
          display: flex;
          gap: 0.5rem;
        }

        .toggle-button {
          background: #28a745;
          color: white;
          border: none;
          padding: 0.5rem 1rem;
          border-radius: 4px;
          cursor: pointer;
          font-size: 0.875rem;
          transition: background 0.3s ease;
        }

        .toggle-button:hover:not(:disabled) {
          background: #218838;
        }

        .toggle-button:disabled {
          background: #ccc;
          cursor: not-allowed;
        }

        .canvas-wrapper {
          position: relative;
          display: flex;
          justify-content: center;
          align-items: center;
          padding: 1rem;
          background: #f9f9f9;
        }

        .string-art-canvas {
          border: 1px solid #ccc;
          border-radius: 4px;
          box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
          transition: transform 0.3s ease;
        }

        .string-art-canvas.animating {
          transform: scale(1.02);
        }

        .animation-overlay {
          position: absolute;
          top: 1rem;
          right: 1rem;
          background: rgba(255, 255, 255, 0.9);
          border-radius: 50%;
          width: 3rem;
          height: 3rem;
          display: flex;
          align-items: center;
          justify-content: center;
          animation: bounce 2s infinite;
        }

        .pulse {
          font-size: 1.5rem;
          animation: pulse 1s infinite;
        }

        @keyframes bounce {
          0%, 100% { transform: translateY(0); }
          50% { transform: translateY(-10px); }
        }

        @keyframes pulse {
          0%, 100% { transform: scale(1); }
          50% { transform: scale(1.2); }
        }
      `}</style>
    </div>
  );
};
