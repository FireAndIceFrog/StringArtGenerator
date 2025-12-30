import { useState, useRef, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import type { AppDispatch } from "../shared/redux/store";
import { setImageUrl, setPreprocessedImageUrl, setImageData } from "../shared/redux/stringArtSlice";
import type { SelfieSegmentation } from "@mediapipe/selfie_segmentation"


const TARGET_SIZE = 1080;
const FILL_COLOUR = "#eaeaeaff";
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

    // Create a working canvas that preserves the source image aspect ratio (max dimension = TARGET_SIZE)
    let workW = img.width;
    let workH = img.height;
    if (Math.max(workW, workH) > TARGET_SIZE) {
        const s = TARGET_SIZE / Math.max(workW, workH);
        workW = Math.round(workW * s);
        workH = Math.round(workH * s);
    }

    const workCanvas = document.createElement("canvas");
    workCanvas.width = workW;
    workCanvas.height = workH;
    const ctx = workCanvas.getContext("2d");
    if (!ctx) {
        setLoading(false);
        return;
    }
    // draw image scaled to work canvas (no letterboxing)
    ctx.fillStyle = FILL_COLOUR;
    ctx.fillRect(0, 0, workW, workH);
    ctx.drawImage(img, 0, 0, workW, workH);

    // Run selfie segmentation on the same workCanvas we'll use for saving
    if (!selfieRef.current) {
        console.warn("SelfieSegmentation not loaded; returning plain workCanvas on white");
        const whiteCanvas = document.createElement("canvas");
        whiteCanvas.width = workW;
        whiteCanvas.height = workH;
        const wctx = whiteCanvas.getContext("2d");
        if (!wctx) {
            setLoading(false);
            return;
        }
        wctx.fillStyle = FILL_COLOUR;
        wctx.fillRect(0, 0, workW, workH);
        wctx.drawImage(workCanvas, 0, 0);
        const dataUrl = whiteCanvas.toDataURL("image/png");
        setPreview(dataUrl);
        setLoading(false);
        return;
    }

    const segmentationResult = await new Promise<any>((resolve) => {
        (async () => {
            selfieRef.current?.onResults((results: any) => {
                resolve(results);
            });
            // Pass the workCanvas (same image) to segmentation
            await selfieRef.current?.send({ image: workCanvas });
        })();
    });

    // Draw mask into a mask canvas matching work dimensions
    const maskCanvas = document.createElement("canvas");
    maskCanvas.width = workW;
    maskCanvas.height = workH;
    const mctx = maskCanvas.getContext("2d");
    if (!mctx) {
        setLoading(false);
        return;
    }

    if (segmentationResult.segmentationMask) {
        mctx.drawImage(segmentationResult.segmentationMask, 0, 0, workW, workH);
    } else if (segmentationResult.multiSegmentationMask) {
        mctx.drawImage(segmentationResult.multiSegmentationMask[0], 0, 0, workW, workH);
    } else if (segmentationResult.mask) {
        const imgData = mctx.createImageData(workW, workH);
        const mask = segmentationResult.mask;
        const [red, green, blue] = FILL_COLOUR.match(/\w\w/g)?.map((c) => parseInt(c, 16)) || [0,0,0];

        for (let i = 0; i < mask.length && i * 4 < imgData.data.length; i++) {
            const a = Math.round(Math.min(1, Math.max(0, mask[i])) * 255);
            imgData.data[i * 4 + 0] = red;
            imgData.data[i * 4 + 1] = green;
            imgData.data[i * 4 + 2] = blue;
            imgData.data[i * 4 + 3] = a;
        }
        mctx.putImageData(imgData, 0, 0);
    } else {
        mctx.fillStyle = FILL_COLOUR;
        mctx.fillRect(0, 0, workW, workH);
    }

    // create final canvas matching work dimensions and composite
    const finalCanvas = document.createElement("canvas");
    finalCanvas.width = workW;
    finalCanvas.height = workH;
    const fctx = finalCanvas.getContext("2d");
    if (!fctx) {
        setLoading(false);
        return;
    }

    fctx.fillStyle = FILL_COLOUR;
    fctx.fillRect(0, 0, workW, workH);
    fctx.drawImage(workCanvas, 0, 0, workW, workH);
    fctx.globalCompositeOperation = 'destination-in';
    fctx.drawImage(maskCanvas, 0, 0, workW, workH);

    // Ensure background (outside the mask) is black by painting black beneath transparent areas
    fctx.globalCompositeOperation = 'destination-over';
    fctx.fillStyle = FILL_COLOUR;
    fctx.fillRect(0, 0, workW, workH);
    fctx.globalCompositeOperation = 'source-over';

    const finalDataUrl = finalCanvas.toDataURL("image/png");
    setPreview(finalDataUrl);
    // Convert canvas to Uint8Array (no fetch needed) and set as imageData so the WASM worker receives the preprocessed image
    try {
        const blob: Blob | null = await new Promise((resolve) => finalCanvas.toBlob((b) => resolve(b)));
        if (blob) {
            const arrayBuffer = await blob.arrayBuffer();
            const u8 = new Uint8Array(arrayBuffer);
            dispatch(setImageData(u8));
        } else {
            console.warn("Canvas.toBlob returned null");
        }
    } catch (err) {
        console.warn("Failed to convert canvas to Uint8Array", err);
    }
    // Automatically set the preprocessed image as the image to process (URL copies for UI)
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
