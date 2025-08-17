use crate::error::Result;
use crate::image_processing::{create_eye_enhancement_mask, detect_eyes};
use crate::state::app_state::StringArtState;
use crate::traits::MaskApplicator;
use std::sync::{Arc, RwLock};

/// Mask applicator for enhancing eye regions.
pub struct EyeEnhancementMask;

impl EyeEnhancementMask {
    pub fn new() -> Self {
        Self
    }
}

impl MaskApplicator for EyeEnhancementMask {
    fn apply(&self, state: Arc<RwLock<StringArtState>>) -> Result<()> {
        let mut state = state.write().unwrap();

        if state.config.preserve_eyes {
            let eyes = detect_eyes(&state.target_image).unwrap_or_else(|e| {
                eprintln!("Warning: Eye detection failed: {}. Eyes will not be enhanced.", e);
                Vec::new()
            });

            let mask = create_eye_enhancement_mask(state.config.image_size, &eyes, &state.target_image);
            state.eye_regions = eyes;
            state.eye_protection_mask = mask;
        }

        Ok(())
    }
}
