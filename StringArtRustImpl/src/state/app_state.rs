use crate::image_processing::EyeRegion;
use crate::state::config::StringArtConfig;
use crate::utils::Coord;
use ndarray::Array2;
use std::collections::HashMap;

/// Holds the shared state for the string art generation process.
/// This includes the images, masks, and the generated path.
#[derive(Clone)]
pub struct StringArtState {
    pub config: StringArtConfig,
    pub target_image: Array2<f32>,
    pub residual_image: Array2<f32>,
    pub nail_coords: Vec<Coord>,
    pub eye_regions: Vec<EyeRegion>,
    pub eye_protection_mask: Array2<f32>,
    pub negative_space_mask: Array2<f32>,
    pub path: Vec<usize>,
    pub line_pixel_cache: HashMap<(usize, usize), Vec<Coord>>,
}

impl StringArtState {
    pub fn new(
        config: StringArtConfig,
        target_image: Array2<f32>,
        residual_image: Array2<f32>,
        nail_coords: Vec<Coord>,
    ) -> Self {
        let image_size = config.image_size;
        Self {
            config,
            target_image,
            residual_image,
            nail_coords,
            eye_regions: Vec::new(),
            eye_protection_mask: Array2::<f32>::ones((image_size, image_size)),
            negative_space_mask: Array2::<f32>::ones((image_size, image_size)),
            path: Vec::new(),
            line_pixel_cache: HashMap::new(),
        }
    }
}
