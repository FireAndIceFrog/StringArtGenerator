//! WebAssembly bindings for string art generation
//! 
//! This module provides JavaScript-friendly interfaces for the string art
//! generation algorithms, enabling real-time streaming of results to web applications.

use crate::error::StringArtError;
use crate::factories::generator_factory::StringArtFactory;
use crate::generators::greedy::GreedyGenerator;
use crate::state::{app_state::StringArtState, config::StringArtConfig};
use crate::traits::generator::StringArtGenerator;
use js_sys::{Array, Function, Promise, Uint8Array};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::console;

// Set up panic hook for better debugging
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

// Use smaller allocator for WASM
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Configuration object for WASM interface
#[derive(Serialize, Deserialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct WasmStringArtConfig {
    pub num_nails: usize,
    pub image_size: usize,
    pub preserve_eyes: bool,
    pub preserve_negative_space: bool,
    pub negative_space_penalty: f32,
    pub negative_space_threshold: f32,
}

#[wasm_bindgen]
impl WasmStringArtConfig {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmStringArtConfig {
        WasmStringArtConfig {
            num_nails: 720,
            image_size: 500,
            preserve_eyes: true,
            preserve_negative_space: false,
            negative_space_penalty: 0.5,
            negative_space_threshold: 200.0,
        }
    }

    #[wasm_bindgen]
    pub fn preset_fast() -> WasmStringArtConfig {
        WasmStringArtConfig {
            num_nails: 360,
            image_size: 300,
            preserve_eyes: false,
            preserve_negative_space: false,
            negative_space_penalty: 0.5,
            negative_space_threshold: 200.0,
        }
    }

    #[wasm_bindgen]
    pub fn preset_balanced() -> WasmStringArtConfig {
        WasmStringArtConfig {
            num_nails: 720,
            image_size: 2000,
            preserve_eyes: true,
            preserve_negative_space: false,
            negative_space_penalty: 0.5,
            negative_space_threshold: 200.0,
        }
    }

    #[wasm_bindgen]
    pub fn preset_high_quality() -> WasmStringArtConfig {
        WasmStringArtConfig {
            num_nails: 1440,
            image_size: 2000,
            preserve_eyes: true,
            preserve_negative_space: false,
            negative_space_penalty: 0.5,
            negative_space_threshold: 200.0,
        }
    }
}

impl From<WasmStringArtConfig> for StringArtConfig {
    fn from(wasm_config: WasmStringArtConfig) -> Self {
        StringArtConfig {
            num_nails: wasm_config.num_nails,
            image_size: wasm_config.image_size,
            preserve_eyes: wasm_config.preserve_eyes,
            preserve_negative_space: wasm_config.preserve_negative_space,
            negative_space_penalty: wasm_config.negative_space_penalty,
            negative_space_threshold: wasm_config.negative_space_threshold,
        }
    }
}

/// Progress information for streaming updates
#[derive(Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct ProgressInfo {
    pub lines_completed: usize,
    pub total_lines: usize,
    pub current_path: Vec<usize>,
    pub score: f32,
    pub completion_percent: f32,
}

/// Main WASM interface for string art generation
#[wasm_bindgen]
pub struct StringArtWasm {
    generator: Option<GreedyGenerator>,
    state: Arc<RwLock<StringArtState>>,
    config: WasmStringArtConfig,
}

#[wasm_bindgen]
impl StringArtWasm {
    /// Create a new StringArtWasm instance from image data
    #[wasm_bindgen(constructor)]
    pub fn new(image_data: &Uint8Array, config: Option<WasmStringArtConfig>) -> Result<StringArtWasm, JsValue> {
        let wasm_config = config.unwrap_or_else(WasmStringArtConfig::new);
        let app_config: StringArtConfig = wasm_config.clone().into();

        let image_bytes: Vec<u8> = image_data.to_vec();

        let (generator, _, state) =
            StringArtFactory::create_from_image_data(&image_bytes, app_config)
                .map_err(|e| JsValue::from_str(&format!("Failed to create generator: {}", e)))?;

        Ok(StringArtWasm {
            generator: Some(generator),
            state,
            config: wasm_config,
        })
    }

    /// Get nail coordinates as a JavaScript array
    #[wasm_bindgen]
    pub fn get_nail_coordinates(&self) -> Array {
        let state = self.state.read().unwrap();
        let coords_array = Array::new();
        for coord in &state.nail_coords {
            let coord_obj = Array::new();
            coord_obj.push(&JsValue::from(coord.x));
            coord_obj.push(&JsValue::from(coord.y));
            coords_array.push(&coord_obj);
        }
        coords_array
    }

    /// Get current configuration
    #[wasm_bindgen]
    pub fn get_config(&self) -> WasmStringArtConfig {
        self.config.clone()
    }

    /// Generate string art path with streaming progress updates
    #[wasm_bindgen]
    pub fn generate_path_streaming(
        &mut self,
        max_lines: usize,
        line_darkness: f32,
        min_improvement_score: f32,
        progress_callback: &Function,
    ) -> Promise {
        self.generate_path_streaming_with_frequency(
            max_lines,
            line_darkness,
            min_improvement_score,
            20, // Default to every 20 steps
            progress_callback,
        )
    }

    /// Generate string art path with configurable streaming frequency
    #[wasm_bindgen]
    pub fn generate_path_streaming_with_frequency(
        &mut self,
        max_lines: usize,
        line_darkness: f32,
        min_improvement_score: f32,
        progress_frequency: usize,
        progress_callback: &Function,
    ) -> Promise {
        let generator = match self.generator.take() {
            Some(gen) => gen,
            None => return Promise::reject(&JsValue::from_str("Generator not initialized")),
        };

        let progress_callback = progress_callback.clone();

        future_to_promise(async move {
            let result = Self::generate_with_streaming(
                generator,
                max_lines,
                line_darkness,
                min_improvement_score,
                progress_frequency,
                progress_callback,
            ).await;

            match result {
                Ok(path) => {
                    let path_array = Array::new();
                    for nail in path {
                        path_array.push(&JsValue::from(nail));
                    }
                    Ok(path_array.into())
                }
                Err(e) => Err(JsValue::from_str(&format!("Generation failed: {}", e))),
            }
        })
    }

    /// Get the current path as a JavaScript array
    #[wasm_bindgen]
    pub fn get_current_path(&self) -> Array {
        let state = self.state.read().unwrap();
        let path_array = Array::new();
        for nail in &state.path {
            path_array.push(&JsValue::from(*nail));
        }
        path_array
    }

    /// Get the total number of nails
    #[wasm_bindgen]
    pub fn get_nail_count(&self) -> usize {
        self.config.num_nails
    }

    /// Get the image size
    #[wasm_bindgen]
    pub fn get_image_size(&self) -> usize {
        self.config.image_size
    }
}

impl StringArtWasm {
    /// Generate path with real streaming updates (async)
    async fn generate_with_streaming(
        mut generator: GreedyGenerator,
        max_lines: usize,
        line_darkness: f32,
        min_improvement_score: f32,
        progress_frequency: usize,
        progress_callback: Function,
    ) -> std::result::Result<Vec<usize>, StringArtError> {
        console::log_1(&"Starting real-time streaming generation...".into());
        
        // Use the new streaming method with real-time callbacks
        console::log_1(&format!("🔧 WASM: Starting generation with progress_frequency={}", progress_frequency).into());
        
        let path = generator.generate_path_with_callback(
            max_lines,
            line_darkness,
            min_improvement_score,
            progress_frequency,
                    |lines_completed, total_lines, current_path, score| {
                        console::log_1(&format!("📡 WASM CALLBACK: lines_completed={}, total_lines={}, current_path_length={}, score={:.2}", 
                            lines_completed, total_lines, current_path.len(), score).into());

                        // Create progress info
                        let progress = ProgressInfo {
                            lines_completed,
                            total_lines,
                            current_path: current_path.to_vec(),
                            score,
                            completion_percent: (lines_completed as f32 / total_lines as f32) * 100.0,
                        };

                console::log_1(&format!("🌊 WASM: Sending progress to JS: {:.1}% complete", progress.completion_percent).into());

                // Serialize and send to JavaScript
                if let Ok(progress_js) = serde_wasm_bindgen::to_value(&progress) {
                    match progress_callback.call1(&JsValue::NULL, &progress_js) {
                        Ok(_) => console::log_1(&"✅ WASM: JavaScript callback completed successfully".into()),
                        Err(e) => console::log_1(&format!("❌ WASM: JavaScript callback failed: {:?}", e).into()),
                    }
                } else {
                    console::log_1(&"❌ WASM: Failed to serialize progress data".into());
                }
            },
        );

        console::log_1(&"Real-time streaming generation complete! Check console for details.".into());
        match path {
            Ok(path) => {
                console::log_1(&format!("✅ WASM: Path generation complete with {} nails", path.len()).into());
                // Store the current path
                Ok(path)
            }
            Err(e) => {
                console::log_1(&format!("❌ WASM: Path generation failed: {}", e).into());
                Err(e)
            }
        }
    }
}

/// Utility functions for WASM
#[wasm_bindgen]
pub fn log_to_console(message: &str) {
    console::log_1(&message.into());
}

#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Test function to verify WASM is working
#[wasm_bindgen]
pub fn test_wasm() -> String {
    "WASM module loaded successfully!".to_string()
}
