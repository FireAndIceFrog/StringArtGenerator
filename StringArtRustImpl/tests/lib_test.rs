use std::path::PathBuf;
use tempfile::tempdir;
use image::{ImageBuffer, Luma};
use string_art_rust_impl::{quick_generator, high_quality_generator, fast_generator, default_config};

fn create_test_image() -> (tempfile::TempDir, String) {
    let dir = tempdir().unwrap();
    let image_path = dir.path().join("test.png");
    
    let img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_fn(100, 100, |x, y| {
        if x > 30 && x < 70 && y > 30 && y < 70 {
            Luma([50u8]) // Dark square in center
        } else {
            Luma([200u8]) // Light background
        }
    });
    
    img.save(&image_path).unwrap();
    (dir, image_path.to_string_lossy().to_string())
}

#[test]
fn test_quick_generator() {
    let (_dir, image_path) = create_test_image();
    let result = quick_generator(&image_path);
    assert!(result.is_ok());
}

#[test]
fn test_high_quality_generator() {
    let (_dir, image_path) = create_test_image();
    let result = high_quality_generator(&image_path);
    assert!(result.is_ok());
}

#[test]
fn test_fast_generator() {
    let (_dir, image_path) = create_test_image();
    let result = fast_generator(&image_path);
    assert!(result.is_ok());
}

#[test]
fn test_default_config() {
    let config = default_config();
    assert_eq!(config.num_nails, 720);
    assert_eq!(config.image_size, 500);
    assert!(config.preserve_eyes);
}
