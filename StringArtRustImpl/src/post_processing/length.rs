use std::f64::consts::PI;

/// Compute total string length (meters) for a path of nail indices.
///
/// Path indices are 0-based. Does not include closing segment.
/// Returns Err(String) if any index is invalid.
pub fn compute_length_from_indices(
    path: &[u32],
    num_nails: u32,
    diameter_m: f64,
    slack_pct: f64,
) -> Result<f64, String> {
    if num_nails == 0 {
        return Err("num_nails must be > 0".to_string());
    }
    if diameter_m <= 0.0 {
        return Err("diameter_m must be > 0".to_string());
    }
    if path.len() < 2 {
        return Ok(0.0);
    }

    let radius = diameter_m / 2.0;
    let two_pi = 2.0 * PI;
    let n = num_nails as f64;

    // helper to compute (x,y) for a nail index
    let coord = |idx: u32| -> (f64, f64) {
        let theta = two_pi * (idx as f64) / n;
        (radius * theta.cos(), radius * theta.sin())
    };

    // validate indices
    for &idx in path {
        if idx >= num_nails {
            return Err(format!("index {} out of range for num_nails {}", idx, num_nails));
        }
    }

    let mut total = 0.0f64;
    for w in path.windows(2) {
        let a = coord(w[0]);
        let b = coord(w[1]);
        let dx = a.0 - b.0;
        let dy = a.1 - b.1;
        total += (dx * dx + dy * dy).sqrt();
    }

    let total_with_slack = total * (1.0 + slack_pct / 100.0);
    Ok(total_with_slack)
}
