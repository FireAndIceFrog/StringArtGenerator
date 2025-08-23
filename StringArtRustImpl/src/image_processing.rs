use crate::error::{Result, StringArtError};
use image::{GrayImage, ImageBuffer, Luma};
use ndarray::Array2;
use rayon::iter::*;
use std::collections::HashMap;

#[cfg(feature = "opencv-support")]
use opencv::{
    core::{Mat, Size, CV_8UC1},
    imgproc::{self, CLAHE},
    objdetect::CascadeClassifier,
    prelude::*,
};

/// Eye region information
#[derive(Debug, Clone)]
pub struct EyeRegion {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

/// Preprocesses image data from memory for string art generation
pub fn preprocess_image_from_memory(
    image_data: &[u8],
    target_size: u32,
) -> Result<Array2<f32>> {
    // Load image from memory and convert to grayscale
    let img = image::load_from_memory(image_data)?.to_luma8();
    
    preprocess_image_internal(img, target_size)
}

/// Loads and preprocesses an image for string art generation
pub fn load_and_preprocess_image(
    image_path: &str,
    target_size: u32,
) -> Result<Array2<f32>> {
    // Load image and convert to grayscale
    let img = image::open(image_path)?.to_luma8();
    
    preprocess_image_internal(img, target_size)
}

/// Internal preprocessing logic shared by both file and memory-based functions
fn preprocess_image_internal(
    img: GrayImage,
    target_size: u32,
) -> Result<Array2<f32>> {
    let (width, height) = img.dimensions();
    
    println!("Loaded image: {}x{}", width, height);
    
    // Find the most common color for background
    let background_color = get_most_common_color(&img)?;
    println!("Most common color: {}", background_color);
    
    // Create square canvas with background color
    let mut canvas = ImageBuffer::from_pixel(target_size, target_size, Luma([background_color]));
    
    // Calculate scaling to preserve aspect ratio
    let scale = (target_size as f32 / width as f32).min(target_size as f32 / height as f32);
    let new_width = (width as f32 * scale) as u32;
    let new_height = (height as f32 * scale) as u32;
    
    // Resize image
    let resized = image::imageops::resize(&img, new_width, new_height, image::imageops::FilterType::Lanczos3);
    
    // Center the image on canvas
    let start_x = (target_size - new_width) / 2;
    let start_y = (target_size - new_height) / 2;
    
    for (x, y, pixel) in resized.enumerate_pixels() {
        canvas.put_pixel(start_x + x, start_y + y, *pixel);
    }
    
    let processed = canvas;
    
    // Save processed image for reference (skip in WASM)
    if std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default() != "wasm32" {
        if let Err(e) = processed.save("string_art_input.png") {
            eprintln!("Warning: Could not save processed image: {}", e);
        }
    }
    
    // Convert to ndarray
    let array = image_to_array(&processed);
    Ok(array)
}

/// Find the most common pixel value in the image
fn get_most_common_color(img: &GrayImage) -> Result<u8> {
    let mut color_counts: HashMap<u8, u32> = HashMap::new();
    
    for pixel in img.pixels() {
        *color_counts.entry(pixel[0]).or_insert(0) += 1;
    }
    
    color_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(color, _)| color)
        .ok_or_else(|| StringArtError::InvalidParameter {
            message: "Empty image".to_string(),
        })
}

/// Detect eyes in the image using OpenCV's Haar cascade
#[cfg(feature = "opencv-support")]
pub fn detect_eyes(img: &Array2<f32>) -> Result<Vec<EyeRegion>> {
    let (height, width) = img.dim();
    
    // Convert array to OpenCV Mat
    let mut cv_img = Mat::new_rows_cols_with_default(
        height as i32,
        width as i32,
        CV_8UC1,
        opencv::core::Scalar::all(0.0),
    )?;
    
    for y in 0..height {
        for x in 0..width {
            let val = img[[y, x]] as u8;
            cv_img.at_2d_mut::<u8>(y as i32, x as i32)? = &val;
        }
    }
    
    // Load eye cascade classifier
    let mut eye_cascade = CascadeClassifier::new(&format!(
        "{}/haarcascade_eye.xml",
        std::env::var("OPENCV_HAARCASCADES_PATH").unwrap_or_else(|_| "/usr/share/opencv4/haarcascades".to_string())
    ))?;
    
    if eye_cascade.empty() {
        println!("Warning: Could not load eye cascade classifier. Eyes will not be protected.");
        return Ok(Vec::new());
    }
    
    // Detect eyes
    let mut eyes = opencv::core::Vector::<opencv::core::Rect>::new();
    eye_cascade.detect_multi_scale(
        &cv_img,
        &mut eyes,
        1.1,
        5,
        0,
        Size::new(20, 20),
        Size::new(100, 100),
    )?;
    
    let mut eye_regions = Vec::new();
    for i in 0..eyes.len() {
        let eye = eyes.get(i)?;
        
        // Expand region for better protection
        let padding = ((eye.width.min(eye.height)) / 4).max(10);
        let x_exp = (eye.x - padding).max(0);
        let y_exp = (eye.y - padding).max(0);
        let w_exp = (eye.width + 2 * padding).min(width as i32 - x_exp);
        let h_exp = (eye.height + 2 * padding).min(height as i32 - y_exp);
        
        eye_regions.push(EyeRegion {
            x: x_exp,
            y: y_exp,
            width: w_exp,
            height: h_exp,
        });
    }
    
    println!("Detected {} eye regions", eye_regions.len());
    Ok(eye_regions)
}

/// Fallback eye detection when OpenCV is not available
#[cfg(not(feature = "opencv-support"))]
pub fn detect_eyes(img: &Array2<f32>) -> Result<Vec<EyeRegion>> {
    println!("Using fallback eye detection (no OpenCV support)");
    
    // Use simple computer vision techniques to detect potential eye regions
    detect_eyes_simple(img)
}

/// Simple eye detection without OpenCV
/// Looks for dark circular regions in the upper half of the image
fn detect_eyes_simple(img: &Array2<f32>) -> Result<Vec<EyeRegion>> {
    let (height, width) = img.dim();
    let mut eye_regions = Vec::new();
    
    // Only search in upper 60% of image (where eyes typically are)
    let search_height = (height as f32 * 0.6) as usize;
    
    // Look for dark regions that could be eyes
    let mut dark_regions = find_dark_circular_regions(img, search_height, width);
    
    // Filter and validate potential eye regions
    dark_regions.retain(|region| {
        // Basic size constraints for eyes
        let min_size = (width.min(height) as f32 * 0.03) as i32; // At least 3% of image
        let max_size = (width.min(height) as f32 * 0.15) as i32; // At most 15% of image
        
        region.width >= min_size && region.width <= max_size &&
        region.height >= min_size && region.height <= max_size
    });
    
    // If we found potential eye regions, validate them
    if !dark_regions.is_empty() {
        // Sort by Y position and take up to 2 (most likely to be eyes)
        dark_regions.sort_by_key(|r| r.y);
        
        // Take the first 2 regions that could be eyes
        for region in dark_regions.into_iter().take(2) {
            eye_regions.push(region);
        }
    }
    
    println!("Detected {} potential eye regions using simple detection", eye_regions.len());
    Ok(eye_regions)
}

fn find_dark_circular_regions(img: &Array2<f32>, search_height: usize, width: usize) -> Vec<EyeRegion> {
    // Step 1: Compute average brightness in parallel
    let (total_brightness, pixel_count) = (0..search_height)
        .into_par_iter()
        .map(|y| {
            let mut sum = 0.0;
            for x in 0..width {
                sum += img[[y, x]];
            }
            (sum, width)
        })
        .reduce(|| (0.0, 0), |(sum1, count1), (sum2, count2)| (sum1 + sum2, count1 + count2));

    let avg_brightness = total_brightness / pixel_count as f32;
    let dark_threshold = avg_brightness * 0.7;

    // Step 2: Scan for dark circular regions in parallel
    let step_size = 10;
    let candidates: Vec<EyeRegion> = (step_size..search_height.saturating_sub(step_size))
        .into_par_iter()
        .flat_map(|y| {
            (step_size..width.saturating_sub(step_size))
                .filter_map(move |x| {
                    if img[[y, x]] < dark_threshold {
                        validate_eye_region(img, x, y, dark_threshold, width, search_height)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>() // collect per row to avoid lifetime issues
        })
        .collect();

    // Step 3: Filter out overlapping regions sequentially
    let mut regions = Vec::new();
    for region in candidates {
        if !overlaps_significantly(&region, &regions) {
            regions.push(region);
        }
    }

    regions
}

/// Validate if a point could be the center of an eye region
fn validate_eye_region(
    img: &Array2<f32>,
    center_x: usize,
    center_y: usize,
    dark_threshold: f32,
    width: usize,
    height: usize,
) -> Option<EyeRegion> {
    let min_radius = 8;
    let max_radius = 40;
    
    for radius in min_radius..=max_radius {
        let mut dark_pixels = 0;
        let mut total_pixels = 0;
        
        // Check circular region around center
        for dy in -(radius as i32)..=(radius as i32) {
            for dx in -(radius as i32)..=(radius as i32) {
                let distance = ((dx * dx + dy * dy) as f32).sqrt();
                if distance <= radius as f32 {
                    let x = center_x as i32 + dx;
                    let y = center_y as i32 + dy;
                    
                    if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                        total_pixels += 1;
                        if img[[y as usize, x as usize]] < dark_threshold {
                            dark_pixels += 1;
                        }
                    }
                }
            }
        }
        
        // If this radius has a good concentration of dark pixels, it might be an eye
        let dark_ratio = dark_pixels as f32 / total_pixels as f32;
        if dark_ratio > 0.4 && total_pixels > 50 { // At least 40% dark pixels
            return Some(EyeRegion {
                x: (center_x as i32 - radius as i32).max(0),
                y: (center_y as i32 - radius as i32).max(0),
                width: (radius * 2) as i32,
                height: (radius * 2) as i32,
            });
        }
    }
    
    None
}

/// Check if a region overlaps significantly with existing regions
fn overlaps_significantly(new_region: &EyeRegion, existing_regions: &[EyeRegion]) -> bool {
    for existing in existing_regions {
        let overlap_x = (new_region.x.max(existing.x), 
                        (new_region.x + new_region.width).min(existing.x + existing.width));
        let overlap_y = (new_region.y.max(existing.y), 
                        (new_region.y + new_region.height).min(existing.y + existing.height));
        
        if overlap_x.1 > overlap_x.0 && overlap_y.1 > overlap_y.0 {
            let overlap_area = (overlap_x.1 - overlap_x.0) * (overlap_y.1 - overlap_y.0);
            let new_area = new_region.width * new_region.height;
            
            // If more than 30% overlap, consider it significant
            if overlap_area as f32 / new_area as f32 > 0.3 {
                return true;
            }
        }
    }
    false
}

/// Create eye enhancement mask that BOOSTS scores for dark areas in eye regions
/// This is the opposite of protection - we want MORE lines in dark eye areas
pub fn create_eye_enhancement_mask(
    image_size: usize,
    eye_regions: &[EyeRegion],
    target_image: &Array2<f32>,
) -> Array2<f32> {
    let mut mask = Array2::<f32>::ones((image_size, image_size));
    
    for eye in eye_regions {
        let center_x = eye.x + eye.width / 2;
        let center_y = eye.y + eye.height / 2;
        
        // Calculate average brightness in eye region to determine what's dark/light
        let mut total_brightness = 0.0;
        let mut pixel_count = 0;
        
        for y in eye.y..(eye.y + eye.height).min(image_size as i32) {
            for x in eye.x..(eye.x + eye.width).min(image_size as i32) {
                if y >= 0 && x >= 0 {
                    total_brightness += target_image[[y as usize, x as usize]];
                    pixel_count += 1;
                }
            }
        }
        
        if pixel_count == 0 {
            continue;
        }
        
        let avg_brightness = total_brightness / pixel_count as f32;
        
        for y in eye.y..(eye.y + eye.height).min(image_size as i32) {
            for x in eye.x..(eye.x + eye.width).min(image_size as i32) {
                if y >= 0 && x >= 0 {
                    let ux = x as usize;
                    let uy = y as usize;
                    
                    // Distance from center of eye
                    let dist_x = (x - center_x).abs() as f32 / (eye.width as f32 / 2.0);
                    let dist_y = (y - center_y).abs() as f32 / (eye.height as f32 / 2.0);
                    let dist = (dist_x * dist_x + dist_y * dist_y).sqrt();
                    
                    if dist <= 1.0 {
                        let pixel_brightness = target_image[[uy, ux]];
                        
                        // For dark areas in eyes (pupils, lashes), BOOST the score
                        if pixel_brightness < avg_brightness * 0.8 {
                            let darkness_factor = 1.0 - (pixel_brightness / avg_brightness);
                            let enhancement = 1.0 + (darkness_factor * 3.0); // Up to 4x boost for very dark areas
                            mask[[uy, ux]] = enhancement;
                        }
                        // For very bright areas (highlights), slightly reduce to preserve them
                        else if pixel_brightness > avg_brightness * 1.2 {
                            mask[[uy, ux]] = 0.7; // Reduce chance of lines in highlights
                        }
                    }
                }
            }
        }
    }
    
    mask
}

/// Legacy function name for backward compatibility - now does enhancement instead of protection
pub fn create_eye_protection_mask(
    image_size: usize,
    _eye_regions: &[EyeRegion],
) -> Array2<f32> {
    // Return neutral mask since we need the target image for proper enhancement
    Array2::<f32>::ones((image_size, image_size))
}

/// Detect negative spaces (areas that should remain light) in the image
pub fn detect_negative_spaces(
    img: &Array2<f32>,
    threshold: f32,
) -> Result<Array2<f32>> {
    let (height, width) = img.dim();
    let mut negative_space_mask = Array2::<f32>::zeros((height, width));
    let mut visited = Array2::<bool>::default((height, width));
    
    // Start flood fill from bright areas (potential negative spaces)
    for y in 0..height {
        for x in 0..width {
            if !visited[[y, x]] && img[[y, x]] >= threshold {
                let region_size = flood_fill_region(img, &mut visited, x, y, threshold);
                
                // If region is significant enough, mark as negative space
                let min_region_size = (width * height) / 400; // At least 0.25% of image
                if region_size >= min_region_size {
                    mark_negative_space_region(img, &mut negative_space_mask, x, y, threshold);
                }
            }
        }
    }
    
    // Apply morphological closing to fill small gaps
    let dilated = morphological_dilate(&negative_space_mask, 3);
    let closed = morphological_erode(&dilated, 3);
    
    Ok(closed)
}

/// Flood fill to find connected bright regions
fn flood_fill_region(
    img: &Array2<f32>,
    visited: &mut Array2<bool>,
    start_x: usize,
    start_y: usize,
    threshold: f32,
) -> usize {
    let (height, width) = img.dim();
    let mut stack = vec![(start_x, start_y)];
    let mut count = 0;
    
    while let Some((x, y)) = stack.pop() {
        if x >= width || y >= height || visited[[y, x]] || img[[y, x]] < threshold {
            continue;
        }
        
        visited[[y, x]] = true;
        count += 1;
        
        // Add neighbors to stack
        if x > 0 { stack.push((x - 1, y)); }
        if x + 1 < width { stack.push((x + 1, y)); }
        if y > 0 { stack.push((x, y - 1)); }
        if y + 1 < height { stack.push((x, y + 1)); }
    }
    
    count
}

/// Mark a negative space region in the mask
fn mark_negative_space_region(
    img: &Array2<f32>,
    mask: &mut Array2<f32>,
    start_x: usize,
    start_y: usize,
    threshold: f32,
) {
    let (height, width) = img.dim();
    let mut stack = vec![(start_x, start_y)];
    let mut visited = Array2::<bool>::default((height, width));
    
    while let Some((x, y)) = stack.pop() {
        if x >= width || y >= height || visited[[y, x]] || img[[y, x]] < threshold {
            continue;
        }
        
        visited[[y, x]] = true;
        mask[[y, x]] = 1.0; // Mark as negative space
        
        // Add neighbors to stack
        if x > 0 { stack.push((x - 1, y)); }
        if x + 1 < width { stack.push((x + 1, y)); }
        if y > 0 { stack.push((x, y - 1)); }
        if y + 1 < height { stack.push((x, y + 1)); }
    }
}

/// Simple morphological dilation
fn morphological_dilate(img: &Array2<f32>, radius: usize) -> Array2<f32> {
    let (height, width) = img.dim();
    let mut result = img.clone();
    
    for y in radius..(height - radius) {
        for x in radius..(width - radius) {
            let mut max_val: f32 = 0.0;
            
            for dy in 0..=2*radius {
                for dx in 0..=2*radius {
                    let ny = y + dy - radius;
                    let nx = x + dx - radius;
                    if nx < width && ny < height {
                        max_val = max_val.max(img[[ny, nx]]);
                    }
                }
            }
            
            result[[y, x]] = max_val;
        }
    }
    
    result
}

/// Simple morphological erosion
fn morphological_erode(img: &Array2<f32>, radius: usize) -> Array2<f32> {
    let (height, width) = img.dim();
    let mut result = img.clone();
    
    for y in radius..(height - radius) {
        for x in radius..(width - radius) {
            let mut min_val: f32 = 1.0;
            
            for dy in 0..=2*radius {
                for dx in 0..=2*radius {
                    let ny = y + dy - radius;
                    let nx = x + dx - radius;
                    if nx < width && ny < height {
                        min_val = min_val.min(img[[ny, nx]]);
                    }
                }
            }
            
            result[[y, x]] = min_val;
        }
    }
    
    result
}

/// Create negative space protection mask with gradient falloff
pub fn create_negative_space_mask(
    img: &Array2<f32>,
    threshold: f32,
) -> Result<Array2<f32>> {
    // Detect negative spaces
    let mut mask = detect_negative_spaces(img, threshold)?;
    
    // Apply Gaussian-like falloff around negative spaces
    let (height, width) = mask.dim();
    let falloff_radius = 5;
    
    for y in 0..height {
        for x in 0..width {
            if mask[[y, x]] > 0.0 {
                // Apply falloff to surrounding area
                for dy in -(falloff_radius as i32)..=(falloff_radius as i32) {
                    for dx in -(falloff_radius as i32)..=(falloff_radius as i32) {
                        let ny = y as i32 + dy;
                        let nx = x as i32 + dx;
                        
                        if ny >= 0 && ny < height as i32 && nx >= 0 && nx < width as i32 {
                            let distance = ((dx * dx + dy * dy) as f32).sqrt();
                            if distance <= falloff_radius as f32 {
                                let falloff = (-distance * distance / (2.0 * 2.0 * 2.0)).exp();
                                let protection = 0.3 + 0.7 * (1.0 - falloff); // 0.3 at center, 1.0 at edge
                                mask[[ny as usize, nx as usize]] = mask[[ny as usize, nx as usize]].max(protection);
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(mask)
}

/// Convert GrayImage to ndarray
fn image_to_array(img: &GrayImage) -> Array2<f32> {
    let (width, height) = img.dimensions();
    let mut array = Array2::<f32>::zeros((height as usize, width as usize));
    
    for (x, y, pixel) in img.enumerate_pixels() {
        array[[y as usize, x as usize]] = pixel[0] as f32;
    }
    
    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_most_common_color() {
        let img = ImageBuffer::from_pixel(10, 10, Luma([128u8]));
        let color = get_most_common_color(&img).unwrap();
        assert_eq!(color, 128);
    }

    #[test]
    fn test_image_to_array() {
        let img = ImageBuffer::from_pixel(3, 3, Luma([100u8]));
        let array = image_to_array(&img);
        assert_eq!(array.dim(), (3, 3));
        assert_eq!(array[[0, 0]], 100.0);
    }
}
