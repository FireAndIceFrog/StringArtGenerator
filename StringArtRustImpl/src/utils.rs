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

/// Get line pixels using Bresenham's line algorithm
/// Returns a vector of coordinates representing the pixels on the line
pub fn get_line_pixels(start: Coord, end: Coord) -> Vec<Coord> {
    let mut pixels = Vec::new();
    
    let dx = (end.x - start.x).abs();
    let dy = (end.y - start.y).abs();
    let sx = if start.x < end.x { 1 } else { -1 };
    let sy = if start.y < end.y { 1 } else { -1 };
    let mut err = dx - dy;
    
    let mut x = start.x;
    let mut y = start.y;
    
    loop {
        pixels.push(Coord::new(x, y));
        
        if x == end.x && y == end.y {
            break;
        }
        
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
    
    pixels
}

/// Calculate the average value along a line in an image
pub fn calculate_line_score(
    image: &Array2<f32>,
    start: Coord,
    end: Coord,
    protection_mask: Option<&Array2<f32>>,
) -> f32 {
    calculate_line_score_with_negative_space(image, start, end, protection_mask, None, 0.0)
}

/// Calculate line score with negative space awareness and eye enhancement
pub fn calculate_line_score_with_negative_space(
    image: &Array2<f32>,
    start: Coord,
    end: Coord,
    enhancement_mask: Option<&Array2<f32>>,
    negative_space_mask: Option<&Array2<f32>>,
    negative_space_penalty: f32,
) -> f32 {
    let pixels = get_line_pixels(start, end);
    let (height, width) = image.dim();
    
    let mut count = 0;
    let mut enhancement_sum = 0.0;
    let mut negative_space_penalty_sum = 0.0;
    
    for pixel in pixels {
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

/// Apply line darkness to the residual image
pub fn apply_line_darkness(
    residual: &mut Array2<f32>,
    start: Coord,
    end: Coord,
    darkness: f32,
) {
    let pixels = get_line_pixels(start, end);
    let (height, width) = residual.dim();
    
    for pixel in pixels {
        if pixel.x >= 0 && pixel.x < width as i32 && pixel.y >= 0 && pixel.y < height as i32 {
            let x = pixel.x as usize;
            let y = pixel.y as usize;
            
            // Clip to prevent negative values
            residual[[y, x]] = (residual[[y, x]] - darkness).max(0.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array2;

    #[test]
    fn test_calculate_nail_coords() {
        let coords = calculate_nail_coords(4, Coord::new(100, 100), 50);
        assert_eq!(coords.len(), 4);
        
        // Check first coordinate (0 degrees)
        assert_eq!(coords[0], Coord::new(150, 100));
    }

    #[test]
    fn test_get_line_pixels() {
        let pixels = get_line_pixels(Coord::new(0, 0), Coord::new(2, 2));
        assert_eq!(pixels, vec![
            Coord::new(0, 0),
            Coord::new(1, 1),
            Coord::new(2, 2)
        ]);
    }

    #[test]
    fn test_calculate_line_score() {
        let mut image = Array2::<f32>::zeros((3, 3));
        image[[1, 1]] = 100.0;
        
        let score = calculate_line_score(
            &image,
            Coord::new(0, 0),
            Coord::new(2, 2),
            None,
        );
        
        // Should be average of values along diagonal
        assert!((score - 33.333_332).abs() < 1e-5);
    }
}
