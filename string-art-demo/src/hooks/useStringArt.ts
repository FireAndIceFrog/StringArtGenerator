import { useState, useEffect, useCallback, useRef } from 'react';
import init, { 
  StringArtWasm, 
  WasmStringArtConfig, 
  test_wasm,
  get_version
} from '../wasm/string_art_rust_impl.js';

interface StringArtConfig {
  num_nails: number;
  image_size: number;
  extract_subject: boolean;
  remove_shadows: boolean;
  preserve_eyes: boolean;
  preserve_negative_space: boolean;
  negative_space_penalty: number;
  negative_space_threshold: number;
}

interface ProgressInfo {
  lines_completed: number;
  total_lines: number;
  current_nail: number;
  next_nail: number;
  score: number;
  completion_percent: number;
  path_segment: [number, number];
  current_path: number[]; // Added current_path to match the updated callback
}

interface WasmModule {
  StringArtWasm: typeof StringArtWasm;
  WasmStringArtConfig: typeof WasmStringArtConfig;
  test_wasm: typeof test_wasm;
  get_version: typeof get_version;
}

export interface UseStringArtReturn {
  wasmModule: WasmModule | null;
  isLoading: boolean;
  error: string | null;
  generateStringArt: (
    imageData: Uint8Array,
    config: StringArtConfig,
    onProgress: (progress: ProgressInfo) => void,
    onNailCoords?: (coords: Array<[number, number]>) => void
  ) => Promise<{ path: number[] | null; nailCoords: Array<[number, number]> }>;
  presets: {
    fast: () => StringArtConfig;
    balanced: () => StringArtConfig;
    highQuality: () => StringArtConfig;
  };
}

export const useStringArt = (): UseStringArtReturn => {
  const [wasmModule, setWasmModule] = useState<WasmModule | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

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
        console.log('WASM module loaded successfully:', test_wasm());
        
      } catch (err) {
        console.error('Failed to load WASM module:', err);
        setError(err instanceof Error ? err.message : 'Failed to load WASM module');
      } finally {
        setIsLoading(false);
      }
    };

    loadWasm();
  }, []);

  const workerRef = useRef<Worker | null>(null);

  useEffect(() => {
    // Initialize the service worker
    workerRef.current = new Worker(new URL('../workers/stringArtWorker.ts', import.meta.url), { type: 'module' });
    return () => {
      workerRef.current?.terminate();
    };
  }, []);

  const generateStringArt = useCallback(
    async (
      imageData: Uint8Array,
      config: StringArtConfig,
      onProgress: (progress: ProgressInfo) => void,
      onNailCoords?: (coords: Array<[number, number]>) => void
    ): Promise<{ path: number[] | null; nailCoords: Array<[number, number]> }> => {
      if (!workerRef.current) {
        throw new Error('Service worker not initialized');
      }

      return new Promise((resolve, reject) => {
        workerRef.current!.onmessage = (event) => {
          const { type, data } = event.data;

          if (type === 'nailCoords' && onNailCoords) {
            onNailCoords(data);
          } else if (type === 'progress') {
            onProgress(data);
          } else if (type === 'complete') {
            resolve({ path: data, nailCoords: [] });
          } else if (type === 'error') {
            reject(new Error(data));
          }
        };

        workerRef.current!.postMessage({
          imageData,
          config,
          wasmModuleUrl: '../wasm/string_art_rust_impl.js',
        });
    });
  },
  []
);

  const presets = {
    fast: () => ({ num_nails: 360, image_size: 500, extract_subject: true, remove_shadows: true, preserve_eyes: true, preserve_negative_space: true, negative_space_penalty: 5, negative_space_threshold: 5 }),
    balanced: () => ({ num_nails: 720, image_size: 1000, extract_subject: true, remove_shadows: true, preserve_eyes: true, preserve_negative_space: true, negative_space_penalty: 10, negative_space_threshold: 10 }),
    highQuality: () => ({ num_nails: 1440, image_size: 2000, extract_subject: true, remove_shadows: true, preserve_eyes: true, preserve_negative_space: true, negative_space_penalty: 15, negative_space_threshold: 20 }),
  };

  return {
    wasmModule,
    isLoading,
    error,
    generateStringArt,
    presets,
  };
};

export type { StringArtConfig, ProgressInfo };
