import { useTranslation } from "react-i18next";
import { usePreProcess } from "./usePreProcess";

/**
 * Minimal PreProcess screen that:
 * - loads MediaPipe FaceDetector (tasks-vision) and SelfieSegmentation (cdn)
 * - detects best face bbox, expands & squares it
 * - crops + resizes to TARGET
 * - runs selfie segmentation on the crop and composites on white
 * - produces a PNG base64 and allows "Use for generation" which dispatches to store
 *
 * NOTE: This file keeps helpers inline for simplicity. Future refactor should move heavy code
 * to a service + WebWorker/OffscreenCanvas.
 */


export default function PreProcessScreen() {
  const i18next = useTranslation();
  const preview = usePreProcess();

  return (
    <main className="upload-section controls-section">
        <div style={{ marginTop: 12, marginLeft: "auto", marginRight: "auto" }}>
          {preview ? (
            <div>
              <p>{i18next.t("PreProcessBlurb")}</p>
              <img src={preview} alt="preview" style={{ maxWidth: 512}} />
            </div>
          ) : (
            <div>
              <p>{i18next.t("Processing...")}</p>
            </div>
          )}
        </div>
    </main>
  );
}
