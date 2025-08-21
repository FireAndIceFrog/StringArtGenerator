import React, { useRef, useEffect, useState, forwardRef, useImperativeHandle } from 'react';
import style from './style.module.css'
import type { StringArtState } from '../../../shared/redux/stringArtSlice';
import { useSelector } from 'react-redux';

interface StringArtCanvasProps {
  width: number;
  height: number;
}

export const StringArtCanvas = forwardRef<HTMLCanvasElement, StringArtCanvasProps>(({
  width,
  height,
}, ref) => {
  const { imageUrl, isGenerating: isAnimating, currentPath, nailCoords } = useSelector(
    (state: { stringArt: StringArtState }) => state.stringArt
  );
  const canvasInternalRef = useRef<HTMLCanvasElement>(null);
  const [showOriginal, setShowOriginal] = useState(false);

  useImperativeHandle(ref, () => canvasInternalRef.current as HTMLCanvasElement);

  useEffect(() => {
    const canvas = canvasInternalRef.current;
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
    <div className={style['string-art-canvas-container']}>
      <div className={style['canvas-header']}>
        <h3>String Art Preview</h3>
        <div className={style['canvas-controls']}>
          <button
            onClick={() => setShowOriginal(!showOriginal)}
            className={style['toggle-button']}
            disabled={!imageUrl}
          >
            {showOriginal ? 'Show String Art' : 'Show Original'}
          </button>
        </div>
      </div>
      
      <div className="canvas-wrapper">
        <canvas
          ref={canvasInternalRef}
          width={width}
          height={height}
          className={`${style['string-art-canvas']} ${isAnimating ? style['animating'] : ''}`}
        />
      </div>
    </div>
  );
});
