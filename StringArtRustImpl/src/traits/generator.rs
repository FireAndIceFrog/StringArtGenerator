use crate::error::Result;

/// Trait for string art path generators.
/// Implementations of this trait are responsible for calculating the optimal path
/// based on a given image and configuration.
pub trait StringArtGenerator {
    /// Generate the string path using the specific algorithm.
    fn generate_path(
        &mut self,
        num_lines: usize,
        line_darkness: f32,
        min_improvement_score: f32,
        progress_frequency: usize,
    ) -> Result<Vec<usize>>;

    /// Generate the string path with real-time progress callbacks.
    fn generate_path_with_callback<F>(
        &mut self,
        num_lines: usize,
        line_darkness: f32,
        min_improvement_score: f32,
        progress_frequency: usize,
        callback: F,
    ) -> Result<Vec<usize>>
    where
        F: FnMut(usize, usize, &[usize], f32); // lines_completed, total, current_path, score
}
