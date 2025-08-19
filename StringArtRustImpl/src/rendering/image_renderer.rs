use crate::error::{Result, StringArtError};
use crate::state::app_state::StringArtState;
use crate::traits::renderer::ImageRenderer as ImageRendererTrait;
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
    fn draw_line(&self, img: &mut RgbImage, start: usize, end: usize, color: (u8, u8, u8)) {
        let (width, height) = img.dimensions();

        let pixel_cache = &self.state.read().unwrap().line_pixel_cache;
        let pixels = pixel_cache.get(start, end);

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

impl ImageRendererTrait for ImageRenderer {
    fn render_to_image(&self, line_color: Option<(u8, u8, u8)>) -> Result<RgbImage> {
        let mut img;
        let color;
        let path;
        {
            let state = match self.state.try_read() {
                Err(_) => return Err(StringArtError::StateLock { message: "Cant get a lock on state".into() }),
                Ok(state) => state,
            };
            let size = state.config.image_size as u32;
            color = line_color.unwrap_or((0, 0, 0));
            img = ImageBuffer::from_pixel(size, size, Rgb([255u8, 255u8, 255u8]));
        
            if state.path.is_empty() {
                return Err(StringArtError::PathGenerationFailed {
                    reason: "No path to render".to_string(),
                });
            }
            path = state.path.clone();
        }
        for window in path.windows(2) {
            self.draw_line(&mut img, window[0], window[1], color);
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
