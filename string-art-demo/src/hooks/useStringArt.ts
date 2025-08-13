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
    onProgress: (progress: ProgressInfo) => void
  ) => Promise<number[] | null>;
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

        // Try to load the real WASM module first
        try {
          // Initialize the WASM module
          await init();
          
          // Create the real WASM interface
          const realWasm: WasmModule = {
            StringArtWasm,
            WasmStringArtConfig,
            test_wasm,
            get_version,
          };
          
          setWasmModule(realWasm);
          console.log('Real WASM module loaded:', test_wasm());
          return;
        } catch (wasmError) {
          console.warn('Real WASM module not available, using mock:', wasmError);
        }

        // Fallback to mock WASM module for testing
        const mockWasm: WasmModule = {
          StringArtWasm: class MockStringArtWasm {
            constructor(imageData: Uint8Array, _config?: WasmStringArtConfig) {
              console.log('Mock StringArt created with image data:', imageData.length, 'bytes');
            }

            free() {}

            get_nail_coordinates(): any[] {
              // Generate mock circular nail coordinates
              const coords: any[] = [];
              const centerX = 250;
              const centerY = 250;
              const radius = 240;
              const numNails = 360;

              for (let i = 0; i < numNails; i++) {
                const angle = (i / numNails) * 2 * Math.PI;
                const x = centerX + radius * Math.cos(angle);
                const y = centerY + radius * Math.sin(angle);
                coords.push([Math.round(x), Math.round(y)]);
              }
              return coords;
            }

            get_config(): WasmStringArtConfig {
              return new WasmStringArtConfig();
            }

            async generate_path_streaming(
              max_lines: number,
              line_darkness: number,
              min_improvement_score: number,
              progress_callback: Function
            ): Promise<any> {
              const path: number[] = [0];
              
              // Simulate path generation with progress updates
              for (let i = 0; i < max_lines; i++) {
                // Simulate some processing time
                await new Promise(resolve => setTimeout(resolve, 10));
                
                const nextNail = Math.floor(Math.random() * 360);
                path.push(nextNail);
                
                const progress = {
                  lines_completed: i + 1,
                  total_lines: max_lines,
                  current_nail: path[path.length - 2],
                  next_nail: nextNail,
                  score: Math.random() * 100,
                  completion_percent: ((i + 1) / max_lines) * 100,
                  path_segment: [path[path.length - 2], nextNail],
                };
                
                progress_callback(progress);
                
                // Stop early sometimes to simulate real behavior
                if (Math.random() < 0.001) break;
              }
              
              return path;
            }

            get_current_path(): any[] {
              return [];
            }

            get_nail_count(): number {
              return 360;
            }

            get_image_size(): number {
              return 500;
            }
          } as any,
          
          WasmStringArtConfig: class MockWasmStringArtConfig {
            free() {}
            num_nails = 720;
            image_size = 500;
            extract_subject = true;
            remove_shadows = true;
            preserve_eyes = true;
            preserve_negative_space = false;
            negative_space_penalty = 0.5;
            negative_space_threshold = 200.0;

            static preset_fast() {
              const config = new MockWasmStringArtConfig();
              config.num_nails = 360;
              config.image_size = 300;
              config.extract_subject = false;
              config.remove_shadows = false;
              config.preserve_eyes = false;
              return config;
            }

            static preset_balanced() {
              return new MockWasmStringArtConfig();
            }

            static preset_high_quality() {
              const config = new MockWasmStringArtConfig();
              config.num_nails = 1440;
              config.image_size = 800;
              return config;
            }
          } as any,
          
          test_wasm(): string {
            return "Mock WASM module loaded successfully!";
          },
          
          get_version(): string {
            return "0.1.0-mock";
          },
        };

        setWasmModule(mockWasm);
        console.log('WASM module loaded:', mockWasm.test_wasm());
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
      onProgress: (progress: ProgressInfo) => void
    ): Promise<number[] | null> => {
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
        const path = await generator.generate_path_streaming(
          1000, // max_lines
          25.0, // line_darkness
          10.0, // min_improvement_score
          onProgress
        );
        return path;
      } catch (err) {
        console.error('String art generation failed:', err);
        setError(err instanceof Error ? err.message : 'Generation failed');
        return null;
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
