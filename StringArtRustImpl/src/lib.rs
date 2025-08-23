//! # String Art Rust Implementation
//!
//! A high-performance Rust implementation of the greedy algorithm for automated string art generation.
//! This library provides tools for converting images into string art paths suitable for automated
//! string art machines.
//!
//! ## Features
//!
//! - **Greedy Algorithm**: Implements the greedy search algorithm for optimal string placement
//! - **Image Preprocessing**: Advanced image processing including subject extraction and shadow removal
//! - **Eye Protection**: Automatically detects and protects facial features from string placement
//! - **Performance Optimized**: Uses parallel processing and efficient algorithms
//! - **Progress Tracking**: Real-time progress bars and intermediate result saving
//! - **Configurable**: Highly configurable parameters for different use cases
//!
//! ## Basic Usage
//!
//! ```rust,no_run
//! use string_art_rust_impl::{StringArtFactory, StringArtConfig, StringArtGenerator, ImageRenderer};
//!
//! // Create a generator with default settings
//! let (mut generator, renderer, _) = StringArtFactory::create_from_image("portrait.jpg", StringArtConfig::default())?;
//!
//! // Generate string art path
//! let path = generator.generate_path(5000, 25.0, 10.0)?;
//!
//! // Render and save the result
//! renderer.save_image("string_art_output.png", None)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Advanced Configuration
//!
//! ```rust,no_run
//! use string_art_rust_impl::{StringArtFactory, StringArtConfig, StringArtGenerator, ImageRenderer};
//!
//! let config = StringArtConfig {
//!     num_nails: 720,
//!     image_size: 500,
//!     preserve_eyes: true,
//!     ..Default::default()
//! };
//!
//! let (mut generator, renderer, _) = StringArtFactory::create_from_image("portrait.jpg", config)?;
//! let path = generator.generate_path(5000, 25.0, 10.0)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod error;
pub mod factories;
pub mod generators;
pub mod image_processing;
pub mod masking;
pub mod rendering;
pub mod state;
pub mod traits;
pub mod utils;

#[cfg(feature = "wasm")]
pub mod wasm;

// Re-export main types for easier use
pub use error::{Result, StringArtError};
pub use factories::generator_factory::StringArtFactory;
pub use generators::greedy::GreedyGenerator;
pub use image_processing::EyeRegion;
pub use rendering::image_renderer::ImageRenderer;
pub use state::config::StringArtConfig;
pub use traits::generator::StringArtGenerator;
pub use traits::renderer::ImageRenderer as ImageRendererTrait;
pub use utils::Coord;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default configuration for string art generation
///
/// This provides sensible defaults for most use cases:
/// - 720 nails (2 per degree)
/// - 500x500 pixel canvas
/// - Subject extraction enabled
/// - Shadow removal enabled
/// - Eye protection enabled
pub fn default_config() -> StringArtConfig {
    StringArtConfig::default()
}

/// Create a quick string art generator with minimal configuration
///
/// This is a convenience function for simple use cases where you just want
/// to generate string art with default settings.
///
/// # Example
///
/// ```rust,no_run
/// use string_art_rust_impl::{quick_generator, StringArtGenerator};
///
/// let (mut generator, renderer, _) = quick_generator("portrait.jpg")?;
/// let path = generator.generate_path(1000, 25.0, 10.0)?;
/// renderer.save_image("output.png", None)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn quick_generator(
    image_path: &str,
) -> Result<(GreedyGenerator, ImageRenderer, std::sync::Arc<std::sync::RwLock<state::app_state::StringArtState>>)> {
    StringArtFactory::create_from_image(image_path, StringArtConfig::default())
}

/// Create a string art generator optimized for high quality results
///
/// This configuration prioritizes quality over speed:
/// - Higher resolution (800x800)
/// - More nails (1440)
/// - All preprocessing features enabled
///
/// # Example
///
/// ```rust,no_run
/// use string_art_rust_impl::{high_quality_generator, StringArtGenerator};
///
/// let (mut generator, _, _) = high_quality_generator("portrait.jpg")?;
/// let path = generator.generate_path(10000, 20.0, 5.0)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn high_quality_generator(
    image_path: &str,
) -> Result<(GreedyGenerator, ImageRenderer, std::sync::Arc<std::sync::RwLock<state::app_state::StringArtState>>)> {
    let config = StringArtConfig {
        num_nails: 1440,
        image_size: 800,
        preserve_eyes: true,
        ..Default::default()
    };
    StringArtFactory::create_from_image(image_path, config)
}

/// Create a string art generator optimized for speed
///
/// This configuration prioritizes speed over quality:
/// - Lower resolution (300x300)
/// - Fewer nails (360)
/// - Minimal preprocessing
///
/// # Example
///
/// ```rust,no_run
/// use string_art_rust_impl::{fast_generator, StringArtGenerator};
///
/// let (mut generator, _, _) = fast_generator("portrait.jpg")?;
/// let path = generator.generate_path(1000, 30.0, 15.0)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn fast_generator(
    image_path: &str,
) -> Result<(GreedyGenerator, ImageRenderer, std::sync::Arc<std::sync::RwLock<state::app_state::StringArtState>>)> {
    let config = StringArtConfig {
        num_nails: 360,
        image_size: 300,
        preserve_eyes: false,
        ..Default::default()
    };
    StringArtFactory::create_from_image(image_path, config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_image() -> String {
        let dir = tempdir().unwrap();
        let image_path = dir.path().join("test.png");
        
        let img = image::ImageBuffer::from_fn(100, 100, |x, y| {
            if x > 30 && x < 70 && y > 30 && y < 70 {
                image::Luma([50u8]) // Dark square in center
            } else {
                image::Luma([200u8]) // Light background
            }
        });
        
        img.save(&image_path).unwrap();
        image_path.to_string_lossy().to_string()
    }

    #[test]
    fn test_quick_generator() {
        let image_path = create_test_image();
        let result = quick_generator(&image_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_high_quality_generator() {
        let image_path = create_test_image();
        let result = high_quality_generator(&image_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fast_generator() {
        let image_path = create_test_image();
        let result = fast_generator(&image_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_default_config() {
        let config = default_config();
        assert_eq!(config.num_nails, 720);
        assert_eq!(config.image_size, 500);
        assert!(config.preserve_eyes);
    }
}
