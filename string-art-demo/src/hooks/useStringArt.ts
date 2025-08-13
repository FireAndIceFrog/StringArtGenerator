import { useState, useEffect, useCallback } from 'react';

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

interface StringArtWasmInstance {
  get_nail_coordinates(): Array<[number, number]>;
  get_config(): StringArtConfig;
  generate_path_streaming(
    max_lines: number,
    line_darkness: number,
    min_improvement_score: number,
    progress_callback: (progress: ProgressInfo) => void
  ): Promise<number[]>;
  get_current_path(): number[];
  get_nail_count(): number;
  get_image_size(): number;
}

interface StringArtWasmConstructor {
  new(imageData: Uint8Array, config?: StringArtConfig): StringArtWasmInstance;
}

interface WasmStringArtConfigStatic {
  new(): StringArtConfig;
  preset_fast(): StringArtConfig;
  preset_balanced(): StringArtConfig;
  preset_high_quality(): StringArtConfig;
}

interface WasmModule {
  StringArtWasm: StringArtWasmConstructor;
  WasmStringArtConfig: WasmStringArtConfigStatic;
  test_wasm(): string;
  get_version(): string;
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
          // Load the WASM module from the public directory
          const wasmModule = await import('/string_art_rust_impl.js');
          await wasmModule.default();
          
          // Create the real WASM interface
          const realWasm: WasmModule = {
            StringArtWasm: wasmModule.StringArtWasm,
            WasmStringArtConfig: wasmModule.WasmStringArtConfig,
            test_wasm: wasmModule.test_wasm,
            get_version: wasmModule.get_version,
          };
          
          setWasmModule(realWasm);
          console.log('Real WASM module loaded:', realWasm.test_wasm());
          return;
        } catch (wasmError) {
          console.warn('Real WASM module not available, using mock:', wasmError);
        }

        // Fallback to mock WASM module for testing
        const mockWasm: WasmModule = {
          StringArtWasm: class MockStringArtWasm implements StringArtWasmInstance {
            constructor(imageData: Uint8Array, _config?: StringArtConfig) {
              console.log('Mock StringArt created with image data:', imageData.length, 'bytes');
            }

            get_nail_coordinates(): Array<[number, number]> {
              // Generate mock circular nail coordinates
              const coords: Array<[number, number]> = [];
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

            get_config(): StringArtConfig {
              return {
                num_nails: 360,
                image_size: 500,
                extract_subject: true,
                remove_shadows: true,
                preserve_eyes: true,
                preserve_negative_space: false,
                negative_space_penalty: 0.5,
                negative_space_threshold: 200.0,
              };
            }

            async generate_path_streaming(
              max_lines: number,
              line_darkness: number,
              min_improvement_score: number,
              progress_callback: (progress: ProgressInfo) => void
            ): Promise<number[]> {
              const path: number[] = [0];
              
              // Simulate path generation with progress updates
              for (let i = 0; i < max_lines; i++) {
                // Simulate some processing time
                await new Promise(resolve => setTimeout(resolve, 10));
                
                const nextNail = Math.floor(Math.random() * 360);
                path.push(nextNail);
                
                const progress: ProgressInfo = {
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

            get_current_path(): number[] {
              return [];
            }

            get_nail_count(): number {
              return 360;
            }

            get_image_size(): number {
              return 500;
            }
          } as StringArtWasmConstructor,
          
          WasmStringArtConfig: {
            new: () => ({
              num_nails: 720,
              image_size: 500,
              extract_subject: true,
              remove_shadows: true,
              preserve_eyes: true,
              preserve_negative_space: false,
              negative_space_penalty: 0.5,
              negative_space_threshold: 200.0,
            }),
            preset_fast: () => ({
              num_nails: 360,
              image_size: 300,
              extract_subject: false,
              remove_shadows: false,
              preserve_eyes: false,
              preserve_negative_space: false,
              negative_space_penalty: 0.5,
              negative_space_threshold: 200.0,
            }),
            preset_balanced: () => ({
              num_nails: 720,
              image_size: 500,
              extract_subject: true,
              remove_shadows: true,
              preserve_eyes: true,
              preserve_negative_space: false,
              negative_space_penalty: 0.5,
              negative_space_threshold: 200.0,
            }),
            preset_high_quality: () => ({
              num_nails: 1440,
              image_size: 800,
              extract_subject: true,
              remove_shadows: true,
              preserve_eyes: true,
              preserve_negative_space: false,
              negative_space_penalty: 0.5,
              negative_space_threshold: 200.0,
            }),
          },
          
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
        const generator = new wasmModule.StringArtWasm(imageData, config);
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
