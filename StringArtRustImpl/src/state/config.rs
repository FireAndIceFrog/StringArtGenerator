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
