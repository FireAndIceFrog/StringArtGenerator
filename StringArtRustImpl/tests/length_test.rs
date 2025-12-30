use std::f64::consts::PI;

#[test]
fn test_small_deterministic() {
    // num_nails = 4, diameter_m = 1.0, path = [0,1,2,3], slack_pct = 5.0
    let path: Vec<u32> = vec![0, 1, 2, 3];
    let num_nails = 4u32;
    let diameter_m = 1.0f64;
    let slack_pct = 5.0f64;

    // expected: adjacent chord length = 2R*sin(pi/4)
    let r = diameter_m / 2.0;
    let chord = 2.0 * r * (PI / 4.0).sin();
    let expected_no_slack = chord * 3.0; // three segments: 0->1,1->2,2->3
    let expected = expected_no_slack * (1.0 + slack_pct / 100.0);

    let result = string_art_rust_impl::post_processing::compute_length_from_indices(&path, num_nails, diameter_m, slack_pct);
    match result {
        Ok(len) => {
            let diff = (len - expected).abs();
            assert!(diff < 1e-6, "len {} differs from expected {} by {}", len, expected, diff);
        }
        Err(e) => panic!("compute_length_from_indices returned Err: {}", e),
    }
}

#[test]
fn test_two_point_quarter() {
    // num_nails = 100, diameter_m = 0.5, path = [0,25], slack_pct = 5.0
    let path: Vec<u32> = vec![0, 25];
    let num_nails = 100u32;
    let diameter_m = 0.5f64;
    let slack_pct = 5.0f64;

    let r = diameter_m / 2.0;
    let angle = 2.0 * PI * (25.0 / 100.0); // pi/2
    let chord = 2.0 * r * (angle / 2.0).sin();
    let expected = chord * (1.0 + slack_pct / 100.0);

    let result = string_art_rust_impl::post_processing::compute_length_from_indices(&path, num_nails, diameter_m, slack_pct);
    match result {
        Ok(len) => {
            let diff = (len - expected).abs();
            assert!(diff < 1e-6, "len {} differs from expected {} by {}", len, expected, diff);
        }
        Err(e) => panic!("compute_length_from_indices returned Err: {}", e),
    }
}

#[test]
fn test_invalid_index() {
    // path contains index >= num_nails
    let path: Vec<u32> = vec![0, 1, 10];
    let num_nails = 10u32;
    let diameter_m = 1.0f64;
    let slack_pct = 5.0f64;

    let result = string_art_rust_impl::post_processing::compute_length_from_indices(&path, num_nails, diameter_m, slack_pct);
    assert!(result.is_err(), "Expected Err for invalid index, got Ok({:?})", result.ok());
}

#[test]
fn test_sample_path_sanity() {
    // simple sanity check: result is finite and > 0
    let path: Vec<u32> = vec![0, 10, 20, 30];
    let num_nails = 60u32;
    let diameter_m = 0.4f64;
    let slack_pct = 5.0f64;

    let result = string_art_rust_impl::post_processing::compute_length_from_indices(&path, num_nails, diameter_m, slack_pct);
    match result {
        Ok(len) => {
            assert!(len.is_finite() && len > 0.0, "Expected positive finite length, got {}", len);
        }
        Err(e) => panic!("compute_length_from_indices returned Err: {}", e),
    }
}
