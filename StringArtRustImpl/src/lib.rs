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
//! use string_art_rust_impl::{StringArtFactory, StringArtConfig, StringArtGenerator, ImageRenderer, ImageRendererTrait};
//!
//! // Create a generator with default settings
//! let (mut generator, renderer, _) = StringArtFactory::create_from_image("portrait.jpg", StringArtConfig::default())?;
//!
//! // Generate string art path (progress_frequency argument added)
//! let path = generator.generate_path(5000, 25.0, 10.0, 100)?;
//!
//! // Render and save the result (bring trait into scope via the import above)
//! renderer.save_image("string_art_output.png", None)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Advanced Configuration
//!
//! ```rust,no_run
//! use string_art_rust_impl::{StringArtFactory, StringArtConfig, StringArtGenerator, ImageRenderer, ImageRendererTrait};
//!
//! let config = StringArtConfig {
//!     num_nails: 720,
//!     image_size: 500,
//!     preserve_eyes: true,
//!     ..Default::default()
//! };
//!
//! let (mut generator, renderer, _) = StringArtFactory::create_from_image("portrait.jpg", config)?;
//! let path = generator.generate_path(5000, 25.0, 10.0, 100)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Convenience Generators
//!
//! ```rust,no_run
//! use string_art_rust_impl::{quick_generator, StringArtGenerator, ImageRendererTrait};
//!
//! let (mut generator, renderer, _) = quick_generator("portrait.jpg")?;
//! let path = generator.generate_path(1000, 25.0, 10.0, 100)?;
//! renderer.save_image("output.png", None)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ```rust,no_run
//! use string_art_rust_impl::{high_quality_generator, StringArtGenerator};
//!
//! let (mut generator, _, _) = high_quality_generator("portrait.jpg")?;
//! let path = generator.generate_path(10000, 20.0, 5.0, 100)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ```rust,no_run
//! use string_art_rust_impl::{fast_generator, StringArtGenerator};
//!
//! let (mut generator, _, _) = fast_generator("portrait.jpg")?;
//! let path = generator.generate_path(1000, 30.0, 15.0, 100)?;
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
pub mod post_processing;

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
pub fn quick_generator(
    image_path: &str,
) -> Result<(GreedyGenerator, ImageRenderer, std::sync::Arc<std::sync::RwLock<state::app_state::StringArtState>>)> {
    StringArtFactory::create_from_image(image_path, StringArtConfig::default())
}

/// Create a string art generator optimized for high quality results
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
