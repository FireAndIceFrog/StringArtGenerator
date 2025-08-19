use std::{collections::HashMap, sync::Mutex};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use crate::{traits::LinePixelCache, Coord};


/// A cache for pre-calculating and storing the pixels for every possible line.
#[derive(Debug, Clone)]
pub struct BasicLinePixelCache {
    cache: HashMap<(usize, usize), Vec<Coord>>,
}

impl BasicLinePixelCache {
    /// Creates a new cache and pre-calculates all line pixels.
    pub fn new(nail_coords: &[Coord]) -> Self {
        let num_nails = nail_coords.len();
        let cache = Mutex::new(HashMap::new());

        (0..num_nails).into_par_iter().for_each(|i| {
            let local_results: Vec<((usize, usize), Vec<Coord>)> = ((i + 1)..num_nails)
                .map(|j| {
                    let start = nail_coords[i];
                    let end = nail_coords[j];
                    let pixels = Self::get_line_pixels(start, end);
                    ((i, j), pixels)
                })
                .collect();

            let mut cache_lock = cache.lock().unwrap();
            for (key, value) in local_results {
                cache_lock.insert(key, value);
            }
        });
        let unwrapped_cache = cache.into_inner().unwrap();
        Self { cache: unwrapped_cache }
    }

    /// Traverse line pixels using Bresenham's line algorithm and apply a closure.
    /// This avoids allocating a vector for the pixels.
    fn get_line_pixels(start: Coord, end: Coord) -> Vec<Coord> {
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
}


impl LinePixelCache for BasicLinePixelCache {
    fn get(&self, nail1: usize, nail2: usize) -> &Vec<Coord> {
        let key = if nail1 < nail2 {
            (nail1, nail2)
        } else {
            (nail2, nail1)
        };
        self.cache.get(&key).expect("Line pixels should be in cache")
    }
}