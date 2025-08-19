import type { WasmStringArtConfig } from "../../../wasm/string_art_rust_impl";

export type StringArtConfig = Pick<WasmStringArtConfig,  keyof {
  num_nails: number;
  image_size: number;
  extract_subject: boolean;
  remove_shadows: boolean;
  preserve_eyes: boolean;
  preserve_negative_space: boolean;
  negative_space_penalty: number;
  negative_space_threshold: number;
}> & {
    max_lines: number, 
    line_darkness: number, 
    min_improvement_score: number, 
    progress_frequency: number,
}