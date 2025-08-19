import {
  useEffect,
  useCallback,
  useRef,
} from "react";
import { useSelector } from "react-redux";
import type { RootState } from "../redux/store";
import type { StringArtConfig } from "../../../features/shared/interfaces/stringArtConfig";
import { ProgressInfo } from "../../../wasm/string_art_rust_impl";

export const useStringArt = () => {
  const isLoading = useSelector((state: RootState) => state.stringArt.isLoading);
  const error = useSelector((state: RootState) => state.stringArt.error);
  const settings = useSelector((state: RootState) => state.stringArt.settings);
  const workerRef = useRef<Worker | null>(null);

  useEffect(() => {
    // Initialize the service worker
    workerRef.current = new Worker(new URL('../workers/stringArtWorker.ts', import.meta.url), { type: "module" });
    return () => {
      workerRef.current?.terminate();
    };
  }, []);

  const generateStringArt = useCallback(
    async (
      imageData: Uint8Array,
      onProgress: (progress: ProgressInfo) => void,
      onNailCoords?: (coords: Array<[number, number]>) => void
    ): Promise<{
      path: number[] | null;
      nailCoords: Array<[number, number]>;
    }> => {
      if (!workerRef.current) {
        throw new Error("Service worker not initialized");
      }

      return new Promise((resolve, reject) => {
        workerRef.current!.onmessage = (event) => {
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

        workerRef.current!.postMessage({
          imageData,
          config: settings,
        });
      });
    },
    [settings]
  );

  return { isLoading, error, generateStringArt, settings };
};

export type { StringArtConfig, ProgressInfo };
