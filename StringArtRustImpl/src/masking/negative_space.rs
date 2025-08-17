use crate::error::Result;
use crate::image_processing::create_negative_space_mask;
use crate::state::app_state::StringArtState;
use crate::traits::mask::MaskApplicator;
use ndarray::Array2;
use std::sync::{Arc, RwLock};

/// Mask applicator for preserving negative space.
pub struct NegativeSpaceMask;

impl NegativeSpaceMask {
    pub fn new() -> Self {
        Self
    }
}

impl MaskApplicator for NegativeSpaceMask {
    fn apply(&self, state: Arc<RwLock<StringArtState>>) -> Result<()> {
        let mut state = state.write().unwrap();

        if state.config.preserve_negative_space {
            let mask = create_negative_space_mask(
                &state.target_image,
                state.config.negative_space_threshold,
            )
            .unwrap_or_else(|e| {
                eprintln!("Warning: Negative space detection failed: {}. Feature disabled.", e);
                Array2::<f32>::ones((state.config.image_size, state.config.image_size))
            });
            state.negative_space_mask = mask;
        }

        Ok(())
    }
}
