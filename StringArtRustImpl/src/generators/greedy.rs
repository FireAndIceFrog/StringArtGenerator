use crate::error::{Result, StringArtError};
use crate::state::app_state::StringArtState;
use crate::traits::StringArtGenerator;
use crate::utils::{
    apply_line_darkness_from_pixels, calculate_line_score_from_pixels,
};
use crate::{ImageRenderer, ImageRendererTrait};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::{Arc, RwLock};
use std::time::Instant;

/// Greedy algorithm implementation for string art generation.
pub struct GreedyGenerator {
    state: Arc<RwLock<StringArtState>>,
}

impl GreedyGenerator {
    /// Creates a new GreedyGenerator with a shared reference to the application state.
    pub fn new(state: Arc<RwLock<StringArtState>>) -> Self {
        Self { state }
    }

    /// Finds the best next nail to connect to, using parallel processing for performance.
    fn find_best_next_nail(
        &self,
        current_nail: usize,
        min_improvement_score: f32,
    ) -> Option<(usize, f32)> {
        let state = self.state.read().unwrap();
        let num_nails = state.config.num_nails;

        return (0..num_nails)
            .into_par_iter()
            .filter(|&next_nail| next_nail != current_nail)
            .map(|next_nail| {
                let pixels = state.line_pixel_cache.get(current_nail, next_nail);

                let score = calculate_line_score_from_pixels(
                    &state.residual_image,
                    pixels,
                    Some(&state.eye_protection_mask),
                    if state.config.preserve_negative_space {
                        Some(&state.negative_space_mask)
                    } else {
                        None
                    },
                    state.config.negative_space_penalty,
                );
                (next_nail, score)
            })
            .filter(|(_, score)| *score >= min_improvement_score)
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    }
}

impl StringArtGenerator for GreedyGenerator {
    fn generate_path(
        &mut self,
        num_lines: usize,
        line_darkness: f32,
        min_improvement_score: f32,
        progress_frequency: usize,
    ) -> Result<Vec<usize>> {
        let progress_bar = ProgressBar::new(num_lines as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("##-"),
        );
        let mut start = Instant::now();
        let image_renderer = ImageRenderer::new(self.state.clone());
        
        self.generate_path_with_callback(num_lines, line_darkness, min_improvement_score, progress_frequency, |lines_completed, _total_lines, _, score| {
            let time_to_find_line = start.elapsed();
            match image_renderer.save_image("processing.jpg", Some((0, 0, 0))) {
                Err(_) => {
                    eprintln!("Error saving image at line {}", lines_completed);
                },
                Ok(_) => {
                    let time_to_save = start.elapsed() - time_to_find_line;
                    progress_bar.set_message(format!(
                        "String {}: (score: {:.2}, find time: {:.2?}, save time: {:.2?})",
                        lines_completed,
                        score,
                        time_to_find_line,
                        time_to_save
                    ));
                    progress_bar.set_position(lines_completed as u64);
                    start = Instant::now();
                }
            }
        })
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
        F: FnMut(usize, usize, &[usize], f32),
    {
        if num_lines == 0 || line_darkness <= 0.0 {
            return Err(StringArtError::InvalidParameter {
                message: "num_lines and line_darkness must be positive".to_string(),
            });
        }

        let mut current_nail = 0;
        {
            let mut state = self.state.write().unwrap();
            state.path = vec![current_nail];
        }

        let mut consecutive_failures = 0;
        let max_consecutive_failures = 3;

        for iteration in 0..num_lines {
            match self.find_best_next_nail(current_nail, min_improvement_score) {
                Some((best_next_nail, max_score)) => {
                    let recent_changes: Vec<usize>;
                    {
                        consecutive_failures = 0;
                        let mut state = self.state.write().unwrap();
                        let pixels = state
                            .line_pixel_cache
                            .get(current_nail, best_next_nail)
                            .clone();

                        let start_time = Instant::now();
                        apply_line_darkness_from_pixels(
                            &mut state.residual_image,
                            &pixels,
                            line_darkness,
                        );
                        print!("Time to apply line darkness: {:?}", start_time.elapsed());
                        current_nail = best_next_nail;
                        state.path.push(current_nail);
                        recent_changes = state.path[state.path.len().saturating_sub(progress_frequency)..].to_vec();
                    }
                    if iteration % progress_frequency == 0 || iteration == num_lines - 1 {
                        let lines_completed = iteration + 1;
                        callback(lines_completed, num_lines, &recent_changes.clone(), max_score);
                    }
                }
                None => {
                    consecutive_failures += 1;
                    if consecutive_failures >= max_consecutive_failures {
                        println!("Stopped early: No improvement for {} consecutive iterations", consecutive_failures);
                        break;
                    }
                }
            }
        }

        let final_path = self.state.read().unwrap().path.clone();
        if final_path.len() <= 1 {
            return Err(StringArtError::PathGenerationFailed {
                reason: "No valid lines could be generated".to_string(),
            });
        }

        Ok(final_path)
    }
}
