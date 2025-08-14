import init, { StringArtWasm, WasmStringArtConfig } from '../wasm/string_art_rust_impl.js';

interface WorkerMessage {
  imageData: Uint8Array;
  config: WasmStringArtConfig;
}

interface ProgressInfo {
  lines_completed: number;
  total_lines: number;
  current_nail: number;
  next_nail: number;
  score: number;
  completion_percent: number;
  path_segment: [number, number];
  current_path: number[];
}

self.onmessage = async (event: MessageEvent<WorkerMessage>) => {
  const { imageData, config } = event.data;

  try {
    // Initialize the WASM module
    await init();

    // Convert config to WASM-compatible format
    const wasmConfig = new WasmStringArtConfig();
    Object.assign(wasmConfig, config);

    const generator = new StringArtWasm(imageData, wasmConfig);

    // Get nail coordinates
    const wasmNailCoords = generator.get_nail_coordinates();
    const displaySize = 500; // Canvas display size
    const scale = displaySize / config.image_size;
    const scaledNailCoords = wasmNailCoords.map(([x, y]) => [Math.round(x * scale), Math.round(y * scale)]);

    // Send nail coordinates back to the main thread
    self.postMessage({ type: 'nailCoords', data: scaledNailCoords });

    // Generate the string art path with progress updates
    const path = await generator.generate_path_streaming_with_frequency(
      2000, // max_lines
      100.0, // line_darkness
      10.0, // min_improvement_score
      300, // progress_frequency
      (progress: ProgressInfo) => {
        const { current_path } = progress;
        self.postMessage({ type: 'progress', data: { ...progress, current_path } });
      }
    );

    // Send the final path back to the main thread
    const pathArray = new Uint8Array(path); // Ensure path is a transferable object
    self.postMessage({ type: 'complete', data: pathArray }, { transfer: [pathArray.buffer] }); // Correct transferable usage
  } catch (error) {
    self.postMessage({ type: 'error', data: (error as Error).message });
  }
};
