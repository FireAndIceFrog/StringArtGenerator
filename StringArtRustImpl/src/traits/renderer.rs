use crate::error::Result;
use image::RgbImage;

/// Trait for rendering the string art path to an image.
pub trait ImageRenderer {
    /// Renders the current path to an image buffer.
    fn render_to_image(&self, line_color: Option<(u8, u8, u8)>) -> Result<RgbImage>;

    /// Renders the image and saves it to the specified output path.
    fn save_image(&self, output_path: &str, line_color: Option<(u8, u8, u8)>) -> Result<()>;
}
