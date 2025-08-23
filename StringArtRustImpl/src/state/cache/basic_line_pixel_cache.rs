use std::{collections::HashMap, sync::Mutex};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use crate::{traits::LinePixelCache, Coord};
use serde::{Serialize, Deserialize};

#[cfg(not(target_arch = "wasm32"))]
use std::{fs, io};
#[cfg(target_arch = "wasm32")]
use web_sys::XMLHttpRequest;


/// A cache for pre-calculating and storing the pixels for every possible line.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicLinePixelCache {
    cache: HashMap<String, Vec<Coord>>,
}

impl BasicLinePixelCache {
    /// Creates a new cache and pre-calculates all line pixels.
    pub fn new(nail_coords: &[Coord]) -> Self {
        let filename = Self::generate_filename(nail_coords);

        // Attempt to load from cache first
        if let Ok(cached_self) = Self::load_from_json(&filename) {
            return cached_self;
        }

        let num_nails = nail_coords.len();
        let cache = Mutex::new(HashMap::new());

        (0..num_nails).into_par_iter().for_each(|i| {
            let local_results: Vec<(String, Vec<Coord>)> = ((i + 1)..num_nails)
                .map(|j| {
                    let start = nail_coords[i];
                    let end = nail_coords[j];
                    let pixels = Self::get_line_pixels(start, end);
                    (format!("{},{}", i, j), pixels)
                })
                .collect();

            let mut cache_lock = cache.lock().unwrap();
            for (key, value) in local_results {
                cache_lock.insert(key, value);
            }
        });
        let unwrapped_cache = cache.into_inner().unwrap();
        let instance = Self { cache: unwrapped_cache };

        // Save to JSON if not in WASM
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Err(e) = instance.save_to_json(&filename) {
                eprintln!("Failed to save BasicLinePixelCache to JSON: {}", e);
            }
        }

        instance
    }

    /// Generates a unique filename based on the min and max nail coordinates.
    fn generate_filename(nail_coords: &[Coord]) -> String {
        if nail_coords.is_empty() {
            return "empty_nails.json".to_string();
        }

        let min_x = nail_coords.iter().take(2).map(|c| c.x).min().unwrap();
        let min_y = nail_coords.iter().take(2).map(|c| c.y).min().unwrap();
        let max_x = nail_coords.iter().take(2).map(|c| c.x).max().unwrap();
        let max_y = nail_coords.iter().take(2).map(|c| c.y).max().unwrap();

        format!("{min_x},{min_y}|{max_x},{max_y}.json")
    }

    /// Saves the cache to a JSON file.
    #[cfg(not(target_arch = "wasm32"))]
    fn save_to_json(&self, filename: &str) -> io::Result<()> {
        let path = format!("cache/{}", filename);
        let json_string = serde_json::to_string(&self)?;
        fs::write(path, json_string)?;
        Ok(())
    }

    /// Loads the cache from a JSON file or URL.
    fn load_from_json(filename: &str) -> Result<Self, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let path = format!("cache/{}", filename);
            let file_content = fs::read_to_string(path).map_err(|e| e.to_string())?;
            serde_json::from_str(&file_content).map_err(|e| e.to_string())
        }

        #[cfg(target_arch = "wasm32")]
        {
            // Synchronous fetch for WASM in a worker context
            let url = format!("/{}", filename); // Assuming the file is served at the root
            let xhr = XMLHttpRequest::new().map_err(|e| format!("Failed to create XHR: {:?}", e))?;
            xhr.open_with_async("GET", &url, false).map_err(|e| format!("Failed to open XHR: {:?}", e))?;
            xhr.send().map_err(|e| format!("Failed to send XHR: {:?}", e))?;

            if xhr.status() == 200 {
                let text = xhr.response_text().map_err(|e| format!("Failed to get response text: {:?}", e))?;
                serde_json::from_str(&text).map_err(|e| e.to_string())
            } else {
                Err(format!("Failed to load cache from {}: HTTP status {}", url, xhr.status()))
            }
        }
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
            format!("{},{}", nail1, nail2)
        } else {
            format!("{},{}", nail2, nail1)
        };
        self.cache.get(&key).expect("Line pixels should be in cache")
    }
}
