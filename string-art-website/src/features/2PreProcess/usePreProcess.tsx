import { useState, useRef, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import type { AppDispatch } from "../shared/redux/store";
import { setImageUrl, setPreprocessedImageUrl } from "../shared/redux/stringArtSlice";
import type { SelfieSegmentation } from "@mediapipe/selfie_segmentation"


const TARGET_SIZE = 1080;

declare global {
  interface Window {
    SelfieSegmentation?: any;
  }
}


export const usePreProcess = () => {
    const dispatch = useDispatch<AppDispatch>();
    const imageUrl = useSelector((state: any) => state.stringArt.imageUrl || state.stringArt.preprocessedImageUrl);
    const [loading, setLoading] = useState(false);
    const [preview, setPreview] = useState<string | null>(null);
    const selfieRef = useRef<SelfieSegmentation | null>(null);

    useEffect(() => {
        // Lazy load SelfieSegmentation (cdn) — we'll use it after we have a crop
        import("@mediapipe/selfie_segmentation")
        .then((SelfieSegmentationModule) => {
            const SelfieSegmentation  = SelfieSegmentationModule.SelfieSegmentation
            selfieRef.current = new SelfieSegmentation({
            locateFile: (file: string) =>
                `https://cdn.jsdelivr.net/npm/@mediapipe/selfie_segmentation/${file}`,
            });
            selfieRef.current.setOptions({ modelSelection: 0, selfieMode: true});
        })
        .catch((err) => {
            console.warn("Failed to load SelfieSegmentation", err);
        });
        // FaceDetector (tasks-vision) is relatively heavy and may already be used elsewhere;
        // for simplicity we'll dynamically import it from the same CDN used in the demo if needed.
        // If project has a central copy, refactor to reuse it.
        // We do not eagerly instantiate tasks-vision FaceDetector here to keep initial load small.
    }, []);

    async function processImage() {
    if (!imageUrl) return;
    setLoading(true);
    setPreview(null);

    // Load image element
    const img = new Image();
    img.crossOrigin = "anonymous";
    img.src = imageUrl;
    await new Promise((res, rej) => {
        img.onload = res;
        img.onerror = rej;
    });

    const cropCanvas = document.createElement("canvas");
    cropCanvas.width = TARGET_SIZE;
    cropCanvas.height = TARGET_SIZE;
    const ctx = cropCanvas.getContext("2d");
    if (!ctx) {
        setLoading(false);
        return;
    }
    // draw full image to canvas preserving aspect ratio (letterbox)
    const scale = Math.min(TARGET_SIZE / img.width, TARGET_SIZE / img.height);
    const drawWidth = Math.round(img.width * scale);
    const drawHeight = Math.round(img.height * scale);
    const dx = Math.round((TARGET_SIZE - drawWidth) / 2);
    const dy = Math.round((TARGET_SIZE - drawHeight) / 2);
    // fill background white to avoid transparent borders
    ctx.fillStyle = "#ffffff";
    ctx.fillRect(0, 0, TARGET_SIZE, TARGET_SIZE);
    ctx.drawImage(img, 0, 0, img.width, img.height, dx, dy, drawWidth, drawHeight);

    // Run selfie segmentation on the cropped canvas
    if (!selfieRef.current) {
        console.warn("SelfieSegmentation not loaded; returning plain crop on white");
        // Composite crop on white and return
        const whiteCanvas = document.createElement("canvas");
        whiteCanvas.width = TARGET_SIZE;
        whiteCanvas.height = TARGET_SIZE;
        const wctx = whiteCanvas.getContext("2d");
        if (!wctx) {
        setLoading(false);
        return;
        }
        wctx.fillStyle = "#ffffff";
        wctx.fillRect(0, 0, TARGET_SIZE, TARGET_SIZE);
        wctx.drawImage(cropCanvas, 0, 0);
        const dataUrl = whiteCanvas.toDataURL("image/png");
        setPreview(dataUrl);
        setLoading(false);
        return;
    }

    // SelfieSegmentation in browser: use onResults pattern; we will send the crop canvas as an ImageBitmap
    const segmentationResult = await new Promise<any>((resolve) => {
        (async () => {
        selfieRef.current.onResults((results: any) => {
            resolve(results);
        });
        // selfieRef expects an HTMLVideoElement/Image/Canvas; pass the cropCanvas
        await selfieRef.current.send({ image: cropCanvas });
        })();
    });

    // segmentationResult.segmentationMask is an HTMLCanvas or ImageData depending on implementation;
    // To be robust, draw the mask onto a temp canvas.
    const maskCanvas = document.createElement("canvas");
    maskCanvas.width = TARGET_SIZE;
    maskCanvas.height = TARGET_SIZE;
    const mctx = maskCanvas.getContext("2d");
    if (!mctx) {
        setLoading(false);
        return;
    }

    if (segmentationResult.segmentationMask) {
        // draw mask to canvas
        mctx.drawImage(segmentationResult.segmentationMask, 0, 0, TARGET_SIZE, TARGET_SIZE);
    } else if (segmentationResult.multiSegmentationMask) {
        mctx.drawImage(segmentationResult.multiSegmentationMask[0], 0, 0, TARGET_SIZE, TARGET_SIZE);
    } else if (segmentationResult.mask) {
        // mask as float array: convert to ImageData
        const imgData = mctx.createImageData(TARGET_SIZE, TARGET_SIZE);
        const mask = segmentationResult.mask; // assume Float32Array or array of [0..1]
        for (let i = 0; i < mask.length && i * 4 < imgData.data.length; i++) {
        const a = Math.round(Math.min(1, Math.max(0, mask[i])) * 255);
        imgData.data[i * 4 + 0] = 255;
        imgData.data[i * 4 + 1] = 255;
        imgData.data[i * 4 + 2] = 255;
        imgData.data[i * 4 + 3] = a;
        }
        mctx.putImageData(imgData, 0, 0);
    } else {
        // unknown mask format, fallback
        mctx.fillStyle = "#ffffff";
        mctx.fillRect(0, 0, TARGET_SIZE, TARGET_SIZE);
    }

    // Optional: feather edges by applying a small blur filter to mask
    // create final canvas: draw white background, then draw image masked by alpha
    const finalCanvas = document.createElement("canvas");
    finalCanvas.width = TARGET_SIZE;
    finalCanvas.height = TARGET_SIZE;
    const fctx = finalCanvas.getContext("2d");
    if (!fctx) {
        setLoading(false);
        return;
    }

    // Composite: draw white background and keep only foreground using the mask
    // Paint white background first
    fctx.fillStyle = "#ffffff";
    fctx.fillRect(0, 0, TARGET_SIZE, TARGET_SIZE);
    // Draw the crop image on top
    fctx.drawImage(cropCanvas, 0, 0, TARGET_SIZE, TARGET_SIZE);
    // Use mask to keep only foreground pixels (mask white = foreground)
    fctx.globalCompositeOperation = 'destination-in';
    fctx.drawImage(maskCanvas, 0, 0, TARGET_SIZE, TARGET_SIZE);
    // Restore default composite mode
    fctx.globalCompositeOperation = 'source-over';

    // Convert final canvas to data URL
    const finalDataUrl = finalCanvas.toDataURL("image/png");
    setPreview(finalDataUrl);
    // Automatically set the preprocessed image as the image to process
    dispatch(setImageUrl(finalDataUrl));
    dispatch(setPreprocessedImageUrl(finalDataUrl));
    setLoading(false);
    }

    useEffect(() => {
    if (imageUrl && !preview && !loading) {
        processImage();
    }
    // eslint-disable-next-line
    }, [imageUrl]);


    return preview;
}