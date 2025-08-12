use crate::error::Result;
use crate::image_processing::{
    create_eye_enhancement_mask, detect_eyes, load_and_preprocess_image, EyeRegion,
};
use crate::utils::{calculate_nail_coords, Coord};
use image::{ImageBuffer, Rgb, RgbImage};
use ndarray::Array2;
use std::fs::File;
use std::io::Write;

/// Configuration for string art generation
#[derive(Debug, Clone)]
pub struct StringArtConfig {
    pub num_nails: usize,
    pub image_size: usize,
    pub extract_subject: bool,
    pub remove_shadows: bool,
    pub preserve_eyes: bool,
    pub preserve_negative_space: bool,
    pub negative_space_penalty: f32,
    pub negative_space_threshold: f32,
}

impl Default for StringArtConfig {
    fn default() -> Self {
        Self {
            num_nails: 720,  // 2 per degree
            image_size: 500,
            extract_subject: true,
            remove_shadows: true,
            preserve_eyes: true,
            preserve_negative_space: false,
            negative_space_penalty: 0.5,
            negative_space_threshold: 200.0,
        }
    }
}

/// Base trait for string art generators
pub trait StringArtGenerator {
    /// Generate the string path using the specific algorithm
    fn generate_path(
        &mut self,
        num_lines: usize,
        line_darkness: f32,
        min_improvement_score: f32,
        save_every: usize,
    ) -> Result<Vec<usize>>;

    /// Get the current path
    fn get_path(&self) -> &[usize];

    /// Get the nail coordinates
    fn get_nail_coords(&self) -> &[Coord];

    /// Get the residual image (for debugging/visualization)
    fn get_residual_image(&self) -> &Array2<f32>;

    /// Render the current path to an image
    fn render_image(&self, output_path: &str, line_color: Option<(u8, u8, u8)>) -> Result<()>;

    /// Save the path to a text file
    fn save_path(&self, output_path: &str) -> Result<()>;

    /// Save current progress (both image and path)
    fn save_progress(&self, base_output_path: &str) -> Result<()>;
}

/// Base implementation for string art generators
pub struct AbstractStringArt {
    pub config: StringArtConfig,
    pub target_image: Array2<f32>,
    pub residual_image: Array2<f32>,
    pub nail_coords: Vec<Coord>,
    pub eye_regions: Vec<EyeRegion>,
    pub eye_protection_mask: Array2<f32>,
    pub negative_space_mask: Array2<f32>,
    pub path: Vec<usize>,
}

impl AbstractStringArt {
    /// Create a new string art generator
    pub fn new(image_path: &str, config: StringArtConfig) -> Result<Self> {
        println!("Initializing string art generator...");
        
        // Load and preprocess the image
        let target_image = load_and_preprocess_image(
            image_path,
            config.image_size as u32,
            config.extract_subject,
            config.remove_shadows,
        )?;

        // Detect eyes if eye preservation is enabled
        let (eye_regions, eye_protection_mask) = if config.preserve_eyes {
            let eyes = detect_eyes(&target_image).unwrap_or_else(|e| {
                eprintln!("Warning: Eye detection failed: {}. Eyes will not be enhanced.", e);
                Vec::new()
            });
            // Use the new enhancement mask that boosts scores for dark areas in eyes
            let mask = create_eye_enhancement_mask(config.image_size, &eyes, &target_image);
            (eyes, mask)
        } else {
            (Vec::new(), Array2::<f32>::ones((config.image_size, config.image_size)))
        };

        // Create negative space mask if enabled
        let negative_space_mask = if config.preserve_negative_space {
            use crate::image_processing::create_negative_space_mask;
            create_negative_space_mask(&target_image, config.negative_space_threshold)
                .unwrap_or_else(|e| {
                    eprintln!("Warning: Negative space detection failed: {}. Feature disabled.", e);
                    Array2::<f32>::ones((config.image_size, config.image_size))
                })
        } else {
            Array2::<f32>::ones((config.image_size, config.image_size))
        };

        // Create residual image (inverted for brightness-based scoring)
        let residual_image = 255.0 - &target_image;

        // Calculate nail coordinates
        let center = Coord::new(
            config.image_size as i32 / 2,
            config.image_size as i32 / 2,
        );
        let radius = config.image_size as i32 / 2 - 5; // Slight inset
        let nail_coords = calculate_nail_coords(config.num_nails, center, radius);

        println!("Setup complete:");
        println!("  - Image size: {}x{}", config.image_size, config.image_size);
        println!("  - Number of nails: {}", config.num_nails);
        println!("  - Eye protection: {} (detected {} regions)", config.preserve_eyes, eye_regions.len());
        println!("  - Negative space preservation: {}", config.preserve_negative_space);

        Ok(Self {
            config,
            target_image,
            residual_image,
            nail_coords,
            eye_regions,
            eye_protection_mask,
            negative_space_mask,
            path: Vec::new(),
        })
    }

    /// Render the string art as an image
    pub fn render_to_image(&self, line_color: Option<(u8, u8, u8)>) -> RgbImage {
        let size = self.config.image_size as u32;
        let color = line_color.unwrap_or((0, 0, 0)); // Default to black
        
        // Create white canvas
        let mut img = ImageBuffer::from_pixel(size, size, Rgb([255u8, 255u8, 255u8]));
        
        // Draw lines between consecutive nails in the path
        for window in self.path.windows(2) {
            let start = self.nail_coords[window[0]];
            let end = self.nail_coords[window[1]];
            
            self.draw_line(&mut img, start, end, color);
        }
        
        img
    }

    /// Draw a line on the image using a simple line drawing algorithm
    fn draw_line(&self, img: &mut RgbImage, start: Coord, end: Coord, color: (u8, u8, u8)) {
        use crate::utils::get_line_pixels;
        
        let pixels = get_line_pixels(start, end);
        let (width, height) = img.dimensions();
        
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

impl StringArtGenerator for AbstractStringArt {
    /// This is abstract - should be overridden by specific implementations
    fn generate_path(
        &mut self,
        _num_lines: usize,
        _line_darkness: f32,
        _min_improvement_score: f32,
        _save_every: usize,
    ) -> Result<Vec<usize>> {
        panic!("generate_path must be implemented by specific generator types");
    }

    fn get_path(&self) -> &[usize] {
        &self.path
    }

    fn get_nail_coords(&self) -> &[Coord] {
        &self.nail_coords
    }

    fn get_residual_image(&self) -> &Array2<f32> {
        &self.residual_image
    }

    fn render_image(&self, output_path: &str, line_color: Option<(u8, u8, u8)>) -> Result<()> {
        if self.path.is_empty() {
            println!("Warning: No path to render. Call generate_path() first.");
            return Ok(());
        }

        println!("Rendering image with {} lines...", self.path.len().saturating_sub(1));
        
        let img = self.render_to_image(line_color);
        img.save(output_path)?;
        
        println!("Image saved to {}", output_path);
        Ok(())
    }

    fn save_path(&self, output_path: &str) -> Result<()> {
        if self.path.is_empty() {
            println!("Warning: No path to save.");
            return Ok(());
        }

        let mut file = File::create(output_path)?;
        
        // Write path as comma-separated values
        let path_str: Vec<String> = self.path.iter().map(|&n| n.to_string()).collect();
        writeln!(file, "{}", path_str.join(","))?;
        
        println!("Path saved to {}", output_path);
        Ok(())
    }

    fn save_progress(&self, base_output_path: &str) -> Result<()> {
        if self.path.is_empty() {
            println!("No path to save yet.");
            return Ok(());
        }

        let line_count = self.path.len().saturating_sub(1);
        
        // Save image
        let image_path = format!("{}_lines.png", base_output_path);
        self.render_image(&image_path, None)?;
        
        // Save path
        let path_file = format!("{}_lines.txt", base_output_path);
        self.save_path(&path_file)?;
        
        println!("Progress saved at {} lines", line_count);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_art_config_default() {
        let config = StringArtConfig::default();
        assert_eq!(config.num_nails, 720);
        assert_eq!(config.image_size, 500);
        assert!(config.extract_subject);
        assert!(config.remove_shadows);
        assert!(config.preserve_eyes);
    }

    #[test]
    fn test_nail_coordinates_calculation() {
        let config = StringArtConfig {
            num_nails: 4,
            image_size: 200,
            ..Default::default()
        };
        
        let center = Coord::new(100, 100);
        let radius = 95;
        let coords = calculate_nail_coords(config.num_nails, center, radius);
        
        assert_eq!(coords.len(), 4);
        // First nail should be at (195, 100) - to the right
        assert_eq!(coords[0], Coord::new(195, 100));
    }
}
