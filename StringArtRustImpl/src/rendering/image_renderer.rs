use crate::error::{Result, StringArtError};
use crate::state::app_state::StringArtState;
use crate::traits::renderer::ImageRenderer as ImageRendererTrait;
use crate::utils::Coord;
use image::{ImageBuffer, Rgb, RgbImage};
use std::sync::{Arc, RwLock};

/// Struct responsible for rendering the string art path to an image.
pub struct ImageRenderer {
    state: Arc<RwLock<StringArtState>>,
}

impl ImageRenderer {
    /// Creates a new ImageRenderer with a shared reference to the application state.
    pub fn new(state: Arc<RwLock<StringArtState>>) -> Self {
        Self { state }
    }

    /// Draws a line on the image.
    fn draw_line(&self, img: &mut RgbImage, start: Coord, end: Coord, color: (u8, u8, u8)) {
        let (width, height) = img.dimensions();
        let mut state = self.state.write().unwrap();

        let start_idx = state.nail_coords.iter().position(|&c| c == start);
        let end_idx = state.nail_coords.iter().position(|&c| c == end);

        if let (Some(i), Some(j)) = (start_idx, end_idx) {
            let key = (i, j);
            if !state.line_pixel_cache.contains_key(&key) {
                let computed = crate::utils::get_line_pixels(start, end);
                state.line_pixel_cache.insert(key, computed);
            }
            let pixels = state.line_pixel_cache.get(&key).unwrap();

            for pixel in pixels {
                if pixel.x >= 0 && pixel.x < width as i32 && pixel.y >= 0 && pixel.y < height as i32 {
                    img.put_pixel(
                        pixel.x as u32,
                        pixel.y as u32,
                        Rgb([color.0, color.1, color.2]),
                    );
                }
            }
        }
    }
}

impl ImageRendererTrait for ImageRenderer {
    fn render_to_image(&self, line_color: Option<(u8, u8, u8)>) -> Result<RgbImage> {
        let state = self.state.read().unwrap();
        let size = state.config.image_size as u32;
        let color = line_color.unwrap_or((0, 0, 0));

        let mut img = ImageBuffer::from_pixel(size, size, Rgb([255u8, 255u8, 255u8]));

        if state.path.is_empty() {
            return Err(StringArtError::PathGenerationFailed {
                reason: "No path to render".to_string(),
            });
        }

        for window in state.path.windows(2) {
            let start = state.nail_coords[window[0]];
            let end = state.nail_coords[window[1]];
            self.draw_line(&mut img, start, end, color);
        }

        Ok(img)
    }

    fn save_image(&self, output_path: &str, line_color: Option<(u8, u8, u8)>) -> Result<()> {
        let img = self.render_to_image(line_color)?;
        img.save(output_path)?;
        println!("Image saved to {}", output_path);
        Ok(())
    }
}
