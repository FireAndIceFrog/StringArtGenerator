//! Command-line interface for the String Art Rust Implementation
//!
//! This binary provides a convenient command-line interface for generating string art
//! from images using the greedy algorithm.

use clap::{Arg, Command};
use std::process;
use string_art_rust_impl::{
    fast_generator, high_quality_generator, quick_generator, GreedyGenerator, StringArtConfig,
    StringArtGenerator, VERSION,
};

fn main() {
    let matches = Command::new("string_art")
        .version(VERSION)
        .author("String Art Generator Team")
        .about("Generate string art from images using the greedy algorithm")
        .arg(
            Arg::new("input")
                .help("Input image file path")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output image file path")
                .default_value("string_art_output.png"),
        )
        .arg(
            Arg::new("path-output")
                .short('p')
                .long("path")
                .value_name("FILE")
                .help("Output path file (nail sequence)")
                .default_value("string_art_path.txt"),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .value_name("NUMBER")
                .help("Maximum number of lines to generate")
                .default_value("5000"),
        )
        .arg(
            Arg::new("nails")
                .short('n')
                .long("nails")
                .value_name("NUMBER")
                .help("Number of nails around the circle")
                .default_value("720"),
        )
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .value_name("PIXELS")
                .help("Canvas size (width and height)")
                .default_value("500"),
        )
        .arg(
            Arg::new("darkness")
                .short('d')
                .long("darkness")
                .value_name("VALUE")
                .help("Line darkness value (how much each line reduces brightness)")
                .default_value("400.0"),
        )
        .arg(
            Arg::new("min-score")
                .short('m')
                .long("min-score")
                .value_name("VALUE")
                .help("Minimum improvement score to continue")
                .default_value("10.0"),
        )
        .arg(
            Arg::new("save-every")
                .long("save-every")
                .value_name("NUMBER")
                .help("Save progress every N iterations (0 to disable)")
                .default_value("20"),
        )
        .arg(
            Arg::new("preset")
                .long("preset")
                .value_name("NAME")
                .help("Use a preset configuration")
                .value_parser(["quick", "fast", "high-quality"])
                .conflicts_with_all(&["nails", "size"]),
        )
        .arg(
            Arg::new("no-subject-extraction")
                .long("no-subject-extraction")
                .help("Disable automatic subject extraction")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-shadow-removal")
                .long("no-shadow-removal")
                .help("Disable shadow removal preprocessing")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-eye-protection")
                .long("no-eye-protection")
                .help("Disable eye detection and protection")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("preserve-text")
                .long("preserve-text")
                .help("Enable negative space preservation for better text/character rendering")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("negative-space-penalty")
                .long("negative-space-penalty")
                .value_name("VALUE")
                .help("Penalty weight for lines crossing negative spaces (0.0-1.0)")
                .default_value("0.5"),
        )
        .arg(
            Arg::new("negative-space-threshold")
                .long("negative-space-threshold")
                .value_name("VALUE")
                .help("Brightness threshold for detecting negative spaces (0-255)")
                .default_value("200.0"),
        )
        .arg(
            Arg::new("line-color")
                .long("line-color")
                .value_name("R,G,B")
                .help("Line color as RGB values (e.g., 0,0,0 for black)")
                .default_value("0,0,0"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Parse arguments
    let input_path = matches.get_one::<String>("input").unwrap();
    let output_path = matches.get_one::<String>("output").unwrap();
    let path_output = matches.get_one::<String>("path-output").unwrap();
    let verbose = matches.get_flag("verbose");

    if verbose {
        println!("String Art Generator v{}", VERSION);
        println!("Input: {}", input_path);
        println!("Output: {}", output_path);
        println!("Path output: {}", path_output);
    }

    // Parse numeric parameters
    let lines = parse_or_exit::<usize>(matches.get_one::<String>("lines").unwrap(), "lines");
    let darkness = parse_or_exit::<f32>(matches.get_one::<String>("darkness").unwrap(), "darkness");
    let min_score = parse_or_exit::<f32>(matches.get_one::<String>("min-score").unwrap(), "min-score");
    let save_every = parse_or_exit::<usize>(matches.get_one::<String>("save-every").unwrap(), "save-every");

    // Parse line color
    let line_color = parse_color(matches.get_one::<String>("line-color").unwrap());

    // Create generator based on preset or custom configuration
    let generator = if let Some(preset) = matches.get_one::<String>("preset") {
        match preset.as_str() {
            "quick" => quick_generator(input_path),
            "fast" => fast_generator(input_path),
            "high-quality" => high_quality_generator(input_path),
            _ => unreachable!("clap should prevent this"),
        }
    } else {
        // Custom configuration
        let nails = parse_or_exit::<usize>(matches.get_one::<String>("nails").unwrap(), "nails");
        let size = parse_or_exit::<usize>(matches.get_one::<String>("size").unwrap(), "size");

        // Parse negative space parameters
        let negative_space_penalty = parse_or_exit::<f32>(
            matches.get_one::<String>("negative-space-penalty").unwrap(),
            "negative-space-penalty"
        );
        let negative_space_threshold = parse_or_exit::<f32>(
            matches.get_one::<String>("negative-space-threshold").unwrap(),
            "negative-space-threshold"
        );

        let config = StringArtConfig {
            num_nails: nails,
            image_size: size,
            extract_subject: !matches.get_flag("no-subject-extraction"),
            remove_shadows: !matches.get_flag("no-shadow-removal"),
            preserve_eyes: !matches.get_flag("no-eye-protection"),
            preserve_negative_space: matches.get_flag("preserve-text"),
            negative_space_penalty,
            negative_space_threshold,
        };

        if verbose {
            println!("Configuration:");
            println!("  Nails: {}", config.num_nails);
            println!("  Canvas size: {}x{}", config.image_size, config.image_size);
            println!("  Subject extraction: {}", config.extract_subject);
            println!("  Shadow removal: {}", config.remove_shadows);
            println!("  Eye protection: {}", config.preserve_eyes);
            println!("  Negative space preservation: {}", config.preserve_negative_space);
            if config.preserve_negative_space {
                println!("    Penalty: {}", config.negative_space_penalty);
                println!("    Threshold: {}", config.negative_space_threshold);
            }
        }

        GreedyGenerator::new(input_path, config)
    };

    let mut generator = match generator {
        Ok(gen) => gen,
        Err(e) => {
            eprintln!("Error creating generator: {}", e);
            process::exit(1);
        }
    };

    if verbose {
        println!("Generation parameters:");
        println!("  Max lines: {}", lines);
        println!("  Line darkness: {}", darkness);
        println!("  Min improvement score: {}", min_score);
        println!("  Save every: {} iterations", save_every);
        println!();
    }

    // Generate the path
    println!("Starting string art generation...");
    let start_time = std::time::Instant::now();

    match generator.generate_path(lines, darkness, min_score, save_every) {
        Ok(path) => {
            let duration = start_time.elapsed();
            println!("✓ Path generation completed in {:.2?}", duration);
            println!("✓ Generated {} lines", path.len().saturating_sub(1));

            // Save the rendered image
            if let Err(e) = generator.render_image(output_path, line_color) {
                eprintln!("Warning: Failed to save image: {}", e);
            } else {
                println!("✓ Image saved to {}", output_path);
            }

            // Save the path
            if let Err(e) = generator.save_path(path_output) {
                eprintln!("Warning: Failed to save path: {}", e);
            } else {
                println!("✓ Path saved to {}", path_output);
            }

            println!("String art generation completed successfully!");
        }
        Err(e) => {
            eprintln!("Error during generation: {}", e);
            process::exit(1);
        }
    }
}

/// Parse a string to a numeric type or exit with an error message
fn parse_or_exit<T: std::str::FromStr>(value: &str, param_name: &str) -> T
where
    T::Err: std::fmt::Display,
{
    match value.parse() {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error parsing {}: {} ({})", param_name, e, value);
            process::exit(1);
        }
    }
}

/// Parse color from R,G,B string format
fn parse_color(color_str: &str) -> Option<(u8, u8, u8)> {
    let parts: Vec<&str> = color_str.split(',').collect();
    if parts.len() != 3 {
        eprintln!("Warning: Invalid color format '{}'. Using default black.", color_str);
        return Some((0, 0, 0));
    }

    let r = parse_or_exit::<u8>(parts[0], "red component");
    let g = parse_or_exit::<u8>(parts[1], "green component");
    let b = parse_or_exit::<u8>(parts[2], "blue component");

    Some((r, g, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color() {
        assert_eq!(parse_color("0,0,0"), Some((0, 0, 0)));
        assert_eq!(parse_color("255,128,64"), Some((255, 128, 64)));
    }

    #[test]
    fn test_parse_color_invalid() {
        // Should return default black for invalid input
        assert_eq!(parse_color("invalid"), Some((0, 0, 0)));
    }
}
