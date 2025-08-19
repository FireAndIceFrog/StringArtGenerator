use crate::{ Coord };

/// Trait for string art path generators.
/// Implementations of this trait are responsible for calculating the optimal path
/// based on a given image and configuration.
pub trait LinePixelCache: Send + Sync {
    /// Generate the string path using the specific algorithm.
    fn get(&self, nail1: usize, nail2: usize) -> &Vec<Coord>;
}
