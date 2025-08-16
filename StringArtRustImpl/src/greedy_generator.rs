use crate::abstract_generator::{AbstractStringArt, StringArtConfig, StringArtGenerator};
use crate::error::{Result, StringArtError};
use crate::utils::{apply_line_darkness, calculate_line_score, calculate_line_score_with_negative_space};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use rayon::iter::IntoParallelIterator;

/// Greedy algorithm implementation for string art generation
#[derive(Clone)]
pub struct GreedyGenerator {
    base: AbstractStringArt,
}

impl GreedyGenerator {
    /// Create a new greedy generator from image data in memory
    pub fn from_image_data(image_data: &[u8], config: StringArtConfig) -> Result<Self> {
        let base = AbstractStringArt::from_image_data(image_data, config)?;
        Ok(Self { base })
    }

    /// Create a new greedy generator
    pub fn new(image_path: &str, config: StringArtConfig) -> Result<Self> {
        let base = AbstractStringArt::new(image_path, config)?;
        Ok(Self { base })
    }

    /// Create with default configuration
    pub fn new_default(image_path: &str) -> Result<Self> {
        Self::new(image_path, StringArtConfig::default())
    }

    /// Create with custom number of nails and image size
    pub fn new_with_params(
        image_path: &str,
        num_nails: usize,
        image_size: usize,
    ) -> Result<Self> {
        let config = StringArtConfig {
            num_nails,
            image_size,
            ..Default::default()
        };
        Self::new(image_path, config)
    }

    /// Find the best next nail using parallel processing for performance
    fn find_best_next_nail(
        &self,
        current_nail: usize,
        min_improvement_score: f32,
    ) -> Option<(usize, f32)> {
        // Use parallel iterator for better performance
        let scores: Vec<(usize, f32)> = (0..self.base.config.num_nails)
            .into_par_iter()
            .filter(|&next_nail| next_nail != current_nail)
            .map(|next_nail| {
                let start = self.base.nail_coords[current_nail];
                let end = self.base.nail_coords[next_nail];
                
                let score = if self.base.config.preserve_negative_space {
                    // Use negative space aware scoring
                    calculate_line_score_with_negative_space(
                        &self.base.residual_image,
                        start,
                        end,
                        Some(&self.base.eye_protection_mask),
                        Some(&self.base.negative_space_mask),
                        self.base.config.negative_space_penalty,
                    )
                } else {
                    // Use standard scoring
                    calculate_line_score(
                        &self.base.residual_image,
                        start,
                        end,
                        Some(&self.base.eye_protection_mask),
                    )
                };
                
                (next_nail, score)
            })
            .collect();

        // Find the best score
        scores
            .into_iter()
            .filter(|(_, score)| *score >= min_improvement_score)
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }
}

impl StringArtGenerator for GreedyGenerator {
    fn generate_path(
        &mut self,
        num_lines: usize,
        line_darkness: f32,
        min_improvement_score: f32,
        save_every: usize,
    ) -> Result<Vec<usize>> {
        // Use the callback method with a progress bar for CLI usage
        let progress_bar = ProgressBar::new(num_lines as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("##-"),
        );
        let mut closure_self =  self.clone();
        closure_self.base.path.clear();
        let result = self.generate_path_with_callback(
            num_lines,
            line_darkness, 
            min_improvement_score,
            save_every, 
            |lines_completed, _total_lines, current_path, score| {
                progress_bar.set_message(format!(
                    "String {}: path {:?} (score: {:.2})",
                    lines_completed,
                    current_path,
                    score
                ));
                progress_bar.set_position(lines_completed as u64);

                // Save progress periodically outside the callback
                let base_output_path = "string_art_progress";
                closure_self.base.path.extend_from_slice(current_path);
                if let Err(e) = closure_self.save_progress(base_output_path) {
                    eprintln!("Warning: Failed to save progress: {}", e);
                } else {
                    println!("Progress saved");
                }
            }
        );

        progress_bar.finish_with_message("Path generation complete");
        result
    }

    fn generate_path_with_callback<F>(
        &mut self,
        num_lines: usize,
        line_darkness: f32,
        min_improvement_score: f32,
        progress_frequency: usize,
        mut callback: F,
    ) -> Result<Vec<usize>>
    where
        F: FnMut(usize, usize, &[usize], f32), // lines_completed, total, current_nail, next_nail, score
    {
        println!("Starting greedy path generation with streaming...");
        println!("Parameters:");
        println!("  - Max lines: {}", num_lines);
        println!("  - Line darkness: {}", line_darkness);
        println!("  - Min improvement score: {}", min_improvement_score);
        println!("  - Progress frequency: every {} iterations", progress_frequency);

        // Validate parameters
        if num_lines == 0 {
            return Err(StringArtError::InvalidParameter {
                message: "num_lines must be greater than 0".to_string(),
            });
        }

        if line_darkness <= 0.0 {
            return Err(StringArtError::InvalidParameter {
                message: "line_darkness must be positive".to_string(),
            });
        }

        // Start at nail 0
        let mut current_nail = 0;
        self.base.path = vec![current_nail];

        let mut consecutive_failures = 0;
        let max_consecutive_failures = 3;
        let mut start_time = std::time::Instant::now();

        for iteration in 0..num_lines {
            // Find the best next nail
            match self.find_best_next_nail(current_nail, min_improvement_score) {
                Some((best_next_nail, max_score)) => {
                    // Reset failure counter
                    consecutive_failures = 0;

                    // Apply the line darkness to the residual image
                    let start = self.base.nail_coords[current_nail];
                    let end = self.base.nail_coords[best_next_nail];
                    
                    apply_line_darkness(
                        &mut self.base.residual_image,
                        start,
                        end,
                        line_darkness,
                    );

                    // Update current position and path
                    current_nail = best_next_nail;
                    self.base.path.push(current_nail);

                    // Call progress callback every N iterations
                    if iteration % progress_frequency == 0 || iteration == num_lines - 1 {
                        let lines_completed = iteration + 1;
                        println!("🚀 CALLING CALLBACK: iteration={}, lines_completed={}, current_nail={}, next_nail={}, score={:.2}", 
                            iteration, lines_completed, self.base.path[self.base.path.len() - 2], current_nail, max_score);
                        let recent_changes = &self.base.path[self.base.path.len().saturating_sub(progress_frequency)..];
                        callback(
                            lines_completed,
                            num_lines,
                            recent_changes, // Pass only the last changes
                            max_score,
                        );
                        let end_time = std::time::Instant::now();
                        let duration = end_time.duration_since(start_time);
                        println!("✅ CALLBACK COMPLETED in {:?}", duration);
                        start_time = end_time; // Reset start time for next iteration
                    }
                }
                None => {
                    consecutive_failures += 1;
                    println!("❌ No improvement found for iteration {}", iteration);

                    if consecutive_failures >= max_consecutive_failures {
                        println!("Stopped early: No improvement for {} consecutive iterations", consecutive_failures);
                        break;
                    }
                }
            }
        }

        let total_lines = self.base.path.len().saturating_sub(1);
        println!("Path generation complete. Total lines: {}", total_lines);

        if total_lines == 0 {
            return Err(StringArtError::PathGenerationFailed {
                reason: "No valid lines could be generated".to_string(),
            });
        }

        Ok(self.base.path.clone())
    }

    fn get_path(&self) -> &[usize] {
        self.base.get_path()
    }

    fn get_nail_coords(&self) -> &[crate::utils::Coord] {
        self.base.get_nail_coords()
    }

    fn get_residual_image(&self) -> &ndarray::Array2<f32> {
        self.base.get_residual_image()
    }

    fn render_image(&mut self, output_path: &str, line_color: Option<(u8, u8, u8)>) -> Result<()> {
        self.base.render_image(output_path, line_color)
    }

    fn save_path(&mut self, output_path: &str) -> Result<()> {
        self.base.save_path(output_path)
    }

    fn save_progress(&mut self, base_output_path: &str) -> Result<()> {
        self.base.save_progress(base_output_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::Coord;
    use ndarray::Array2;
    use std::fs;
    use tempfile::tempdir;

    fn create_test_image() -> String {
        let dir = tempdir().unwrap();
        let image_path = dir.path().join("test.png");
        
        // Create a simple 100x100 black square on white background
        let img = image::ImageBuffer::from_fn(100, 100, |x, y| {
            if x > 25 && x < 75 && y > 25 && y < 75 {
                image::Luma([0u8]) // Black square in center
            } else {
                image::Luma([255u8]) // White background
            }
        });
        
        img.save(&image_path).unwrap();
        image_path.to_string_lossy().to_string()
    }

    #[test]
    fn test_greedy_generator_creation() {
        let image_path = create_test_image();
        let config = StringArtConfig {
            num_nails: 10,
            image_size: 100,
            extract_subject: false,
            remove_shadows: false,
            preserve_eyes: false,
            preserve_negative_space: false,
            negative_space_penalty: 0.5,
            negative_space_threshold: 200.0,
        };
        
        let generator = GreedyGenerator::new(&image_path, config);
        assert!(generator.is_ok());
        
        let gen = generator.unwrap();
        assert_eq!(gen.get_nail_coords().len(), 10);
        assert_eq!(gen.get_path().len(), 0); // No path generated yet
    }

    #[test]
    fn test_greedy_path_generation() {
        let image_path = create_test_image();
        let config = StringArtConfig {
            num_nails: 8,
            image_size: 100,
            extract_subject: false,
            remove_shadows: false,
            preserve_eyes: false,
            preserve_negative_space: false,
            negative_space_penalty: 0.5,
            negative_space_threshold: 200.0,
        };
        
        let mut generator = GreedyGenerator::new(&image_path, config).unwrap();
        
        // Generate a short path
        let result = generator.generate_path(5, 25.0, 1.0, 0);
        assert!(result.is_ok());
        
        let path = result.unwrap();
        assert!(!path.is_empty());
        assert_eq!(path[0], 0); // Should start at nail 0
    }

    #[test]
    fn test_invalid_parameters() {
        let image_path = create_test_image();
        let mut generator = GreedyGenerator::new_default(&image_path).unwrap();
        
        // Test invalid num_lines
        let result = generator.generate_path(0, 25.0, 1.0, 0);
        assert!(result.is_err());
        
        // Test invalid line_darkness
        let result = generator.generate_path(10, -1.0, 1.0, 0);
        assert!(result.is_err());
    }
}
