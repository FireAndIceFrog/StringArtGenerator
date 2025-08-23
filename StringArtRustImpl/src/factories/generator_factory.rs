use crate::error::Result;
use crate::generators::greedy::GreedyGenerator;
use crate::image_processing::{load_and_preprocess_image, preprocess_image_from_memory};
use crate::masking::{eye_enhancement::EyeEnhancementMask, negative_space::NegativeSpaceMask};
use crate::rendering::image_renderer::ImageRenderer;
use crate::state::{app_state::StringArtState, config::StringArtConfig};
use crate::traits::{MaskApplicator};
use crate::utils::calculate_nail_coords;
use std::sync::{Arc, RwLock};

/// Factory for creating and initializing the string art generator and its components.
pub struct StringArtFactory;

impl StringArtFactory {
    /// Creates a new string art session from an image path and configuration.
    pub fn create_from_image(
        image_path: &str,
        config: StringArtConfig,
    ) -> Result<(
        GreedyGenerator,
        ImageRenderer,
        Arc<RwLock<StringArtState>>,
    )> {
        let target_image = load_and_preprocess_image(
            image_path,
            config.image_size as u32,
        )?;

        let residual_image = 255.0 - &target_image;
        let center = crate::utils::Coord::new(config.image_size as i32 / 2, config.image_size as i32 / 2);
        let radius = config.image_size as i32 / 2 - 5;
        let nail_coords = calculate_nail_coords(config.num_nails, center, radius);

        let state = Arc::new(RwLock::new(StringArtState::new(
            config.clone(),
            target_image,
            residual_image,
            nail_coords,
        )));

        if config.preserve_eyes {
            let eye_mask = EyeEnhancementMask::new();
            eye_mask.apply(state.clone())?;
        }

        if config.preserve_negative_space {
            let negative_space_mask = NegativeSpaceMask::new();
            negative_space_mask.apply(state.clone())?;
        }

        let generator = GreedyGenerator::new(state.clone());
        let renderer = ImageRenderer::new(state.clone());

        Ok((generator, renderer, state))
    }

    /// Creates a new string art session from image data in memory and configuration.
    pub fn create_from_image_data(
        image_data: &[u8],
        config: StringArtConfig,
    ) -> Result<(
        GreedyGenerator,
        ImageRenderer,
        Arc<RwLock<StringArtState>>,
    )> {
        let target_image = preprocess_image_from_memory(
            image_data,
            config.image_size as u32,
        )?;

        let residual_image = 255.0 - &target_image;
        let center = crate::utils::Coord::new(config.image_size as i32 / 2, config.image_size as i32 / 2);
        let radius = config.image_size as i32 / 2 - 5;
        let nail_coords = calculate_nail_coords(config.num_nails, center, radius);

        let state = Arc::new(RwLock::new(StringArtState::new(
            config.clone(),
            target_image,
            residual_image,
            nail_coords,
        )));

        if config.preserve_eyes {
            let eye_mask = EyeEnhancementMask::new();
            eye_mask.apply(state.clone())?;
        }

        if config.preserve_negative_space {
            let negative_space_mask = NegativeSpaceMask::new();
            negative_space_mask.apply(state.clone())?;
        }

        let generator = GreedyGenerator::new(state.clone());
        let renderer = ImageRenderer::new(state.clone());

        Ok((generator, renderer, state))
    }
}
