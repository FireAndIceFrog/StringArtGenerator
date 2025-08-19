import {
  useState,
  useEffect,
  useCallback,
  useRef,
} from "react";
import type { StringArtConfig } from "../interfaces/stringArtConfig";
import init, {
  StringArtWasm,
  WasmStringArtConfig,
  test_wasm,
  get_version,
  ProgressInfo,
} from "../wasm/string_art_rust_impl";

interface WasmModule {
  StringArtWasm: typeof StringArtWasm;
  WasmStringArtConfig: typeof WasmStringArtConfig;
  test_wasm: typeof test_wasm;
  get_version: typeof get_version;
}

export const useStringArt = () => {
  const [, setWasmModule] = useState<WasmModule | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const settings = useRef({
    num_nails: 500,
    image_size: 500,
    extract_subject: false,
    remove_shadows: false,
    preserve_eyes: true,
    preserve_negative_space: false,
    negative_space_penalty: 5,
    negative_space_threshold: 0.5,
    max_lines: 1000,
    line_darkness: 50,
    min_improvement_score: 15,
    progress_frequency: 300,
  } as StringArtConfig);

  useEffect(() => {
    const loadWasm = async () => {
      try {
        setIsLoading(true);
        setError(null);

        // Initialize the WASM module
        await init();

        // Create the WASM interface
        const wasmInterface: WasmModule = {
          StringArtWasm,
          WasmStringArtConfig,
          test_wasm,
          get_version,
        };

        setWasmModule(wasmInterface);
        console.log("WASM module loaded successfully:", test_wasm());
      } catch (err) {
        console.error("Failed to load WASM module:", err);
        setError(
          err instanceof Error ? err.message : "Failed to load WASM module"
        );
      }
      setTimeout(() => setIsLoading(false), 1000);
    };

    loadWasm();
  }, [setIsLoading, setError, setWasmModule]);

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
          config: settings.current,
          wasmModuleUrl: "../wasm/string_art_rust_impl.js",
        });
      });
    },
    [settings]
  );

  return { isLoading, error, generateStringArt, settings };
};

export type { StringArtConfig, ProgressInfo };
