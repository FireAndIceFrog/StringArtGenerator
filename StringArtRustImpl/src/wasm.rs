//! WebAssembly bindings for string art generation
//! 
//! This module provides JavaScript-friendly interfaces for the string art
//! generation algorithms, enabling real-time streaming of results to web applications.

use crate::abstract_generator::{StringArtConfig, StringArtGenerator};
use crate::greedy_generator::GreedyGenerator;
use crate::utils::Coord;
use crate::error::{Result, StringArtError};
use image::{ImageBuffer, Luma};
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use js_sys::{Array, Function, Promise, Uint8Array};
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
#[derive(Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct WasmStringArtConfig {
    pub num_nails: usize,
    pub image_size: usize,
    pub extract_subject: bool,
    pub remove_shadows: bool,
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
            extract_subject: true,
            remove_shadows: true,
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
            extract_subject: false,
            remove_shadows: false,
            preserve_eyes: false,
            preserve_negative_space: false,
            negative_space_penalty: 0.5,
            negative_space_threshold: 200.0,
        }
    }

    #[wasm_bindgen]
    pub fn preset_balanced() -> WasmStringArtConfig {
        WasmStringArtConfig::new()
    }

    #[wasm_bindgen]
    pub fn preset_high_quality() -> WasmStringArtConfig {
        WasmStringArtConfig {
            num_nails: 1440,
            image_size: 800,
            extract_subject: true,
            remove_shadows: true,
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
            extract_subject: wasm_config.extract_subject,
            remove_shadows: wasm_config.remove_shadows,
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
    pub current_nail: usize,
    pub next_nail: usize,
    pub score: f32,
    pub completion_percent: f32,
}

#[wasm_bindgen]
impl ProgressInfo {
    #[wasm_bindgen(getter)]
    pub fn path_segment(&self) -> Array {
        let segment = Array::new();
        segment.push(&JsValue::from(self.current_nail));
        segment.push(&JsValue::from(self.next_nail));
        segment
    }
}

/// Main WASM interface for string art generation
#[wasm_bindgen]
pub struct StringArtWasm {
    generator: Option<GreedyGenerator>,
    config: WasmStringArtConfig,
    nail_coords: Vec<Coord>,
    current_path: Vec<usize>,
}

#[wasm_bindgen]
impl StringArtWasm {
    /// Create a new StringArtWasm instance from image data
    #[wasm_bindgen(constructor)]
    pub fn new(image_data: &Uint8Array, config: Option<WasmStringArtConfig>) -> Result<StringArtWasm, JsValue> {
        let config = config.unwrap_or_else(WasmStringArtConfig::new);
        
        // Convert Uint8Array to Vec<u8>
        let image_bytes: Vec<u8> = image_data.to_vec();
        
        // Create generator from image bytes
        let generator = Self::create_generator_from_bytes(&image_bytes, &config)
            .map_err(|e| JsValue::from_str(&format!("Failed to create generator: {}", e)))?;
        
        let nail_coords = generator.get_nail_coords().to_vec();
        
        Ok(StringArtWasm {
            generator: Some(generator),
            config,
            nail_coords,
            current_path: Vec::new(),
        })
    }

    /// Get nail coordinates as a JavaScript array
    #[wasm_bindgen]
    pub fn get_nail_coordinates(&self) -> Array {
        let coords_array = Array::new();
        for coord in &self.nail_coords {
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
        let generator = match self.generator.take() {
            Some(gen) => gen,
            None => return Promise::reject(&JsValue::from_str("Generator not initialized")),
        };

        let progress_callback = progress_callback.clone();
        let nail_coords = self.nail_coords.clone();

        future_to_promise(async move {
            let result = Self::generate_with_streaming(
                generator,
                max_lines,
                line_darkness,
                min_improvement_score,
                progress_callback,
                nail_coords,
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
        let path_array = Array::new();
        for nail in &self.current_path {
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
    /// Create generator from image bytes
    fn create_generator_from_bytes(
        image_bytes: &[u8],
        config: &WasmStringArtConfig,
    ) -> Result<GreedyGenerator> {
        // Create temporary file-like interface from bytes
        let img = image::load_from_memory(image_bytes)
            .map_err(|e| StringArtError::ImageProcessingError {
                message: format!("Failed to load image from memory: {}", e),
            })?;

        // Convert to grayscale
        let gray_img = img.to_luma8();
        
        // Save to temporary location for generator
        // Note: In a full implementation, we'd refactor the generator to work directly with image data
        let temp_path = "/tmp/temp_image.png";
        gray_img.save(temp_path)
            .map_err(|e| StringArtError::ImageProcessingError {
                message: format!("Failed to save temporary image: {}", e),
            })?;

        GreedyGenerator::new(temp_path, config.clone().into())
    }

    /// Generate path with streaming updates (async)
    async fn generate_with_streaming(
        mut generator: GreedyGenerator,
        max_lines: usize,
        line_darkness: f32,
        min_improvement_score: f32,
        progress_callback: Function,
        nail_coords: Vec<Coord>,
    ) -> Result<Vec<usize>> {
        // For now, we'll simulate streaming by calling the callback periodically
        // In a full implementation, we'd modify the greedy algorithm to yield control
        
        console::log_1(&"Starting streaming generation...".into());
        
        let path = generator.generate_path(max_lines, line_darkness, min_improvement_score, 0)?;
        
        // Simulate streaming by sending progress updates
        for (i, window) in path.windows(2).enumerate() {
            let progress = ProgressInfo {
                lines_completed: i + 1,
                total_lines: path.len() - 1,
                current_nail: window[0],
                next_nail: window[1],
                score: 0.0, // We'd need to modify the generator to provide real scores
                completion_percent: ((i + 1) as f32 / (path.len() - 1) as f32) * 100.0,
            };

            let progress_js = serde_wasm_bindgen::to_value(&progress)
                .map_err(|e| StringArtError::SerializationError {
                    message: format!("Failed to serialize progress: {}", e),
                })?;

            // Call the progress callback
            let _ = progress_callback.call1(&JsValue::NULL, &progress_js);

            // Yield control every 10 lines to prevent blocking
            if i % 10 == 0 {
                wasm_bindgen_futures::JsFuture::from(
                    js_sys::Promise::resolve(&JsValue::from(0))
                ).await.ok();
            }
        }

        console::log_1(&"Generation complete!".into());
        Ok(path)
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
