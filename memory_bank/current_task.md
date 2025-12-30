Plan to implement post-processing (tests first)

Where to put it

- New crate module: StringArtRustImpl/src/post_processing/

  - mod.rs — public API
  - length.rs — implementation + unit tests

Function signature (internal)

- fn compute_length_from_indices(path: &[u32], num_nails: u32, diameter_m: f64, slack_pct: f64) -> Result<f64, String>

  - note: include_closing = false per your choice; caller will pass slack_pct (default 5.0)
  - use 0-based indices (you confirmed)

Tests to add (unit tests in length.rs)

- Test 1 (small deterministic): num_nails = 4, diameter_m = 1.0, path = [0,1,2,3]
  - compute expected using chord formula (adjacent chord length = 2R*sin(pi/4)), sum first 3 segments, apply 5% slack — assert approx equal
- Test 2 (simple two-point): num_nails = 100, diameter_m = 0.5, path = [0,25]
  - expected = single chord length between opposite quarter points — assert approx equal
- Test 3 (invalid index): path contains index >= num_nails — assert Err returned
- Test 4 (sample path): use your sample path and assert result is finite and > 0 (sanity); optionally log computed value for manual verification

Implementation notes

- Convert index -> angle theta = 2π * (index as f64) / (num_nails as f64)
- Radius = diameter_m / 2.0
- Convert to (x,y) = (R*cosθ, R*sinθ)
- Sum Euclidean distances for consecutive window pairs (no closing unless include_closing true)
- Multiply by (1.0 + slack_pct/100.0)
- Return f64 (no rounding in Rust function); rounding to 0.2 m can be done on UI side or another helper function

WASM export

- After tests pass, add a wasm_bindgen wrapper in src/wasm.rs:

  - #[wasm_bindgen] pub fn compute_length_from_indices_wasm(path: Vec, num_nails:u32, diameter_m:f64, slack_pct:f64) -> f64
  - unwrap or map errors to NaN / negative indicator as needed

UI

- Later: expose inputs for diameter (meters) and slack (%), call wasm export, round to 0.2 m for display.
