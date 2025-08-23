import type { StringArtConfig } from "../interfaces/stringArtConfig";
import { ProgressInfo } from "../../../wasm/string_art_rust_impl";

const worker = new Worker(
  new URL("../workers/stringArtWorker.ts", import.meta.url),
  { type: "module" }
);

export async function generateStringArt(
  settings: StringArtConfig,
  imageData: Uint8Array,
  onProgress: (progress: ProgressInfo) => void,
  onNailCoords?: (coords: Array<[number, number]>) => void
): Promise<{
  path: number[] | null;
  nailCoords: Array<[number, number]>;
}> {
  if (!worker) {
    throw new Error("Service worker not initialized");
  }

  return new Promise((resolve, reject) => {
    worker.onmessage = (event) => {
      const { type, data } = event.data;

      if (type === "nailCoords" && onNailCoords) {
        onNailCoords(data);
      } else if (type === "progress") {
        onProgress(data);
      } else if (type === "complete") {
        resolve({ path: data, nailCoords: [] });
      } else if (type === "error") {
        reject(new Error(data));
      }
    };

    worker.postMessage({
      imageData,
      config: settings,
    });
  });
}
