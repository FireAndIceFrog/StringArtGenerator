import init, { compute_length_from_indices_wasm } from '../../../wasm/string_art_rust_impl.js';

export interface WorkerMessage {
    path: Uint32Array, 
    num_nails: number, 
    diameter_m: number, 
    slack_pct: number
}

// Initialize the WASM module
let inited = false;
init().then(() => {
  inited = true;
});

self.onmessage = async (event: MessageEvent<WorkerMessage>) => {
  if (!inited) {
    throw new Error("WASM module not initialized");
  }
  
  const { 
    path, 
    num_nails, 
    diameter_m, 
    slack_pct
 } = event.data

  try {
    
    const length = compute_length_from_indices_wasm(path, num_nails, diameter_m, slack_pct);


    self.postMessage({ type: 'complete', data: length }); // Correct transferable usage
  } catch (error) {
    self.postMessage({ type: 'error', data: (error as Error).message });
  }
};
