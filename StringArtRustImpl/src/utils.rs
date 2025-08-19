use ndarray::Array2;
use std::f64::consts::PI;

/// Represents a 2D coordinate
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Calculate nail coordinates around a circle
pub fn calculate_nail_coords(num_nails: usize, center: Coord, radius: i32) -> Vec<Coord> {
    let mut coords = Vec::with_capacity(num_nails);

    for i in 0..num_nails {
        let angle = 2.0 * PI * (i as f64) / (num_nails as f64);
        let x = center.x + (radius as f64 * angle.cos()) as i32;
        let y = center.y + (radius as f64 * angle.sin()) as i32;
        coords.push(Coord::new(x, y));
    }

    coords
}
/// Calculate line score from a pre-computed list of pixels.
pub fn calculate_line_score_from_pixels(
    image: &Array2<f32>,
    pixels: &[Coord],
    enhancement_mask: Option<&Array2<f32>>,
    negative_space_mask: Option<&Array2<f32>>,
    negative_space_penalty: f32,
) -> f32 {
    let (height, width) = image.dim();

    let mut count = 0;
    let mut enhancement_sum = 0.0;
    let mut negative_space_penalty_sum = 0.0;

    for &pixel in pixels {
        // Check bounds
        if pixel.x >= 0 && pixel.x < width as i32 && pixel.y >= 0 && pixel.y < height as i32 {
            let x = pixel.x as usize;
            let y = pixel.y as usize;

            let pixel_value = image[[y, x]];
            count += 1;

            // Apply enhancement mask if provided (eye enhancement)
            if let Some(mask) = enhancement_mask {
                let enhancement_factor = mask[[y, x]];
                enhancement_sum += pixel_value * enhancement_factor;
            } else {
                enhancement_sum += pixel_value;
            }

            // Calculate negative space penalty
            if let Some(neg_mask) = negative_space_mask {
                negative_space_penalty_sum += neg_mask[[y, x]];
            }
        }
    }

    if count == 0 {
        return 0.0;
    }

    // Use enhanced score instead of basic average
    let enhanced_score = enhancement_sum / count as f32;
    let avg_negative_space_penalty = negative_space_penalty_sum / count as f32;

    // Apply negative space penalty
    let final_score = enhanced_score - (avg_negative_space_penalty * negative_space_penalty);

    // Ensure score doesn't go negative
    final_score.max(0.0)
}

/// Apply line darkness from a pre-computed list of pixels.
pub fn apply_line_darkness_from_pixels(
    residual: &mut Array2<f32>,
    pixels: &[Coord],
    darkness: f32,
) {
    let (height, width) = residual.dim();

    for &pixel in pixels {
        if pixel.x >= 0 && pixel.x < width as i32 && pixel.y >= 0 && pixel.y < height as i32 {
            let x = pixel.x as usize;
            let y = pixel.y as usize;

            // Clip to prevent negative values
            residual[[y, x]] = (residual[[y, x]] - darkness).max(0.0);
        }
    }
}
