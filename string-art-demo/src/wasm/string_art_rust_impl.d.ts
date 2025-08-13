/* tslint:disable */
/* eslint-disable */
export function main(): void;
/**
 * Utility functions for WASM
 */
export function log_to_console(message: string): void;
export function get_version(): string;
/**
 * Test function to verify WASM is working
 */
export function test_wasm(): string;
/**
 * Progress information for streaming updates
 */
export class ProgressInfo {
  private constructor();
  free(): void;
  lines_completed: number;
  total_lines: number;
  current_nail: number;
  next_nail: number;
  score: number;
  completion_percent: number;
  readonly path_segment: Array<any>;
}
/**
 * Main WASM interface for string art generation
 */
export class StringArtWasm {
  free(): void;
  /**
   * Create a new StringArtWasm instance from image data
   */
  constructor(image_data: Uint8Array, config?: WasmStringArtConfig | null);
  /**
   * Get nail coordinates as a JavaScript array
   */
  get_nail_coordinates(): Array<any>;
  /**
   * Get current configuration
   */
  get_config(): WasmStringArtConfig;
  /**
   * Generate string art path with streaming progress updates
   */
  generate_path_streaming(max_lines: number, line_darkness: number, min_improvement_score: number, progress_callback: Function): Promise<any>;
  /**
   * Get the current path as a JavaScript array
   */
  get_current_path(): Array<any>;
  /**
   * Get the total number of nails
   */
  get_nail_count(): number;
  /**
   * Get the image size
   */
  get_image_size(): number;
}
/**
 * Configuration object for WASM interface
 */
export class WasmStringArtConfig {
  free(): void;
  constructor();
  static preset_fast(): WasmStringArtConfig;
  static preset_balanced(): WasmStringArtConfig;
  static preset_high_quality(): WasmStringArtConfig;
  num_nails: number;
  image_size: number;
  extract_subject: boolean;
  remove_shadows: boolean;
  preserve_eyes: boolean;
  preserve_negative_space: boolean;
  negative_space_penalty: number;
  negative_space_threshold: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_wasmstringartconfig_free: (a: number, b: number) => void;
  readonly __wbg_get_wasmstringartconfig_extract_subject: (a: number) => number;
  readonly __wbg_set_wasmstringartconfig_extract_subject: (a: number, b: number) => void;
  readonly __wbg_get_wasmstringartconfig_remove_shadows: (a: number) => number;
  readonly __wbg_set_wasmstringartconfig_remove_shadows: (a: number, b: number) => void;
  readonly __wbg_get_wasmstringartconfig_preserve_eyes: (a: number) => number;
  readonly __wbg_set_wasmstringartconfig_preserve_eyes: (a: number, b: number) => void;
  readonly __wbg_get_wasmstringartconfig_preserve_negative_space: (a: number) => number;
  readonly __wbg_set_wasmstringartconfig_preserve_negative_space: (a: number, b: number) => void;
  readonly __wbg_get_wasmstringartconfig_negative_space_penalty: (a: number) => number;
  readonly __wbg_set_wasmstringartconfig_negative_space_penalty: (a: number, b: number) => void;
  readonly __wbg_get_wasmstringartconfig_negative_space_threshold: (a: number) => number;
  readonly __wbg_set_wasmstringartconfig_negative_space_threshold: (a: number, b: number) => void;
  readonly wasmstringartconfig_new: () => number;
  readonly wasmstringartconfig_preset_fast: () => number;
  readonly wasmstringartconfig_preset_balanced: () => number;
  readonly wasmstringartconfig_preset_high_quality: () => number;
  readonly __wbg_progressinfo_free: (a: number, b: number) => void;
  readonly __wbg_get_progressinfo_lines_completed: (a: number) => number;
  readonly __wbg_set_progressinfo_lines_completed: (a: number, b: number) => void;
  readonly __wbg_get_progressinfo_total_lines: (a: number) => number;
  readonly __wbg_set_progressinfo_total_lines: (a: number, b: number) => void;
  readonly __wbg_get_progressinfo_current_nail: (a: number) => number;
  readonly __wbg_set_progressinfo_current_nail: (a: number, b: number) => void;
  readonly __wbg_get_progressinfo_next_nail: (a: number) => number;
  readonly __wbg_set_progressinfo_next_nail: (a: number, b: number) => void;
  readonly __wbg_get_progressinfo_score: (a: number) => number;
  readonly __wbg_set_progressinfo_score: (a: number, b: number) => void;
  readonly __wbg_get_progressinfo_completion_percent: (a: number) => number;
  readonly __wbg_set_progressinfo_completion_percent: (a: number, b: number) => void;
  readonly progressinfo_path_segment: (a: number) => any;
  readonly __wbg_stringartwasm_free: (a: number, b: number) => void;
  readonly stringartwasm_new: (a: any, b: number) => [number, number, number];
  readonly stringartwasm_get_nail_coordinates: (a: number) => any;
  readonly stringartwasm_get_config: (a: number) => number;
  readonly stringartwasm_generate_path_streaming: (a: number, b: number, c: number, d: number, e: any) => any;
  readonly stringartwasm_get_current_path: (a: number) => any;
  readonly stringartwasm_get_nail_count: (a: number) => number;
  readonly stringartwasm_get_image_size: (a: number) => number;
  readonly log_to_console: (a: number, b: number) => void;
  readonly get_version: () => [number, number];
  readonly test_wasm: () => [number, number];
  readonly __wbg_get_wasmstringartconfig_num_nails: (a: number) => number;
  readonly __wbg_get_wasmstringartconfig_image_size: (a: number) => number;
  readonly main: () => void;
  readonly __wbg_set_wasmstringartconfig_num_nails: (a: number, b: number) => void;
  readonly __wbg_set_wasmstringartconfig_image_size: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_4: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export_6: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly closure84_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure142_externref_shim: (a: number, b: number, c: any, d: any) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
