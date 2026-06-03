import { useRef, useEffect, useState, forwardRef, useImperativeHandle } from 'react';
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
  const hasDrawableStringArt = currentPath.length > 1;
  const shouldShowOriginal = !!imageUrl && (showOriginal || !hasDrawableStringArt);

  useImperativeHandle(ref, () => canvasInternalRef.current as HTMLCanvasElement);

  useEffect(() => {
    const canvas = canvasInternalRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const resetCanvas = (context: CanvasRenderingContext2D) => {
      context.setTransform(1, 0, 0, 1, 0, 0);
      context.globalAlpha = 1;
      context.globalCompositeOperation = 'source-over';
      context.clearRect(0, 0, width, height);
      context.fillStyle = 'white';
      context.fillRect(0, 0, width, height);
    };

    resetCanvas(ctx);

    let isCancelled = false;

    if (shouldShowOriginal) {
      const img = new Image();
      img.onload = () => {
        if (isCancelled) return;

        resetCanvas(ctx);

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
  }, [width, height, nailCoords, currentPath, shouldShowOriginal, imageUrl]);

  return (
    <div className={style['string-art-canvas-container']}>
      <div className={style['canvas-header']}>
        <h3>String Art Preview</h3>
        <div className={style['canvas-controls']}>
          <button
            onClick={() => setShowOriginal(!showOriginal)}
            className={style['toggle-button']}
            disabled={!imageUrl || !hasDrawableStringArt}
          >
            {hasDrawableStringArt && shouldShowOriginal ? 'Show String Art' : 'Show Original'}
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
