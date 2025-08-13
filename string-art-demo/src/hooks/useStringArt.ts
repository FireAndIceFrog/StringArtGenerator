import { useState, useEffect, useCallback } from 'react';
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

  const generateStringArt = useCallback(
    async (
      imageData: Uint8Array,
      config: StringArtConfig,
      onProgress: (progress: ProgressInfo) => void,
      onNailCoords?: (coords: Array<[number, number]>) => void
    ): Promise<{ path: number[] | null; nailCoords: Array<[number, number]> }> => {
      if (!wasmModule) {
        throw new Error('WASM module not loaded');
      }

      try {
        // Convert StringArtConfig to WasmStringArtConfig
        const wasmConfig = new wasmModule.WasmStringArtConfig();
        wasmConfig.num_nails = config.num_nails;
        wasmConfig.image_size = config.image_size;
        wasmConfig.extract_subject = config.extract_subject;
        wasmConfig.remove_shadows = config.remove_shadows;
        wasmConfig.preserve_eyes = config.preserve_eyes;
        wasmConfig.preserve_negative_space = config.preserve_negative_space;
        wasmConfig.negative_space_penalty = config.negative_space_penalty;
        wasmConfig.negative_space_threshold = config.negative_space_threshold;

        const generator = new wasmModule.StringArtWasm(imageData, wasmConfig);
        
        // Get nail coordinates from WASM and scale for display
        const wasmNailCoords = generator.get_nail_coordinates();
        const displaySize = 500; // Canvas display size
        const wasmImageSize = config.image_size; // WASM processing size (2000px)
        const scale = displaySize / wasmImageSize;
        
        const scaledNailCoords: Array<[number, number]> = [];
        for (let i = 0; i < wasmNailCoords.length; i++) {
          const coord = wasmNailCoords[i];
          scaledNailCoords.push([
            Math.round(coord[0] * scale),
            Math.round(coord[1] * scale)
          ]);
        }
        
        // Notify about nail coordinates
        if (onNailCoords) {
          onNailCoords(scaledNailCoords);
        }
        
        console.log('🎯 JS: Starting string art generation...');
        
        const path = await generator.generate_path_streaming_with_frequency(
          1000, // max_lines
          25.0, // line_darkness
          10.0, // min_improvement_score
          10, // progress_frequency (fixed: was 10.0, now 10 - integer!)
          (progress: ProgressInfo) => {
            console.log('🎉 JS CALLBACK RECEIVED:', progress);
            console.log('📜 Current Path:', progress.current_path);
            onProgress(progress);
          }
        );
        
        console.log('✅ JS: String art generation completed!');
        
        return { path, nailCoords: scaledNailCoords };
      } catch (err) {
        console.error('String art generation failed:', err);
        setError(err instanceof Error ? err.message : 'Generation failed');
        return { path: null, nailCoords: [] };
      }
    },
    [wasmModule]
  );

  const presets = {
    fast: () => wasmModule?.WasmStringArtConfig.preset_fast() || {} as StringArtConfig,
    balanced: () => wasmModule?.WasmStringArtConfig.preset_balanced() || {} as StringArtConfig,
    highQuality: () => wasmModule?.WasmStringArtConfig.preset_high_quality() || {} as StringArtConfig,
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
