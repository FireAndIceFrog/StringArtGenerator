# Specification: GPU-Accelerated String Art Generation with `wgpu`

## Objective
Enhance the performance of the string art generation algorithm by leveraging GPU acceleration using `wgpu`. Ensure the solution works seamlessly for both the command-line and web environments.

## Design Goals
1. **Performance**: Utilize GPU parallelism to accelerate computationally intensive tasks like scoring and image updates.
2. **Compatibility**: Maintain compatibility with both the command-line (native Rust) and web (WASM) environments.
3. **Incremental Development**: Implement the improvements in small, testable steps to ensure stability and correctness.
4. **Fallback Support**: Provide a CPU-based fallback for systems without GPU support.

---

## Key Components
1. **GPU Compute Pipeline**:
   - Use `wgpu` to create compute shaders for scoring and image updates.
   - Transfer data (e.g., nail coordinates, residual image) to GPU buffers for parallel processing.

2. **Shader Functions**:
   - **Scoring Shader**: Calculate line scores in parallel for all possible nail pairs.
   - **Image Update Shader**: Apply line darkness to the residual image in parallel.

3. **Data Flow**:
   - Input: Nail coordinates, residual image, and configuration parameters.
   - Output: Updated residual image and selected path.

4. **Integration**:
   - **Command-Line**: Use `wgpu` directly in the Rust implementation.
   - **Web**: Compile the `wgpu`-based implementation to WASM and integrate it with the existing frontend.

5. **Testing and Benchmarking**:
   - Compare the performance of the GPU-accelerated implementation with the current CPU-based approach.
   - Test on various hardware configurations to ensure stability and scalability.

---

## Incremental Improvement Plan

### Step 1: Setup `wgpu` in the Rust Project
- Add `wgpu` as a dependency in `Cargo.toml`.
- Initialize the GPU device and queue in the Rust implementation.
- Create a basic compute pipeline to verify GPU functionality.

### Step 2: Implement Scoring Shader
- Write a compute shader to calculate line scores in parallel.
- Transfer nail coordinates and residual image to GPU buffers.
- Dispatch the compute shader and retrieve the scores.

### Step 3: Integrate Scoring Shader with the Algorithm
- Replace the CPU-based scoring function with the GPU-accelerated version.
- Ensure the integration works for both the command-line and web environments.

### Step 4: Implement Image Update Shader
- Write a compute shader to apply line darkness to the residual image in parallel.
- Transfer the updated residual image back to the CPU.

### Step 5: Optimize Data Flow
- Batch multiple lines for processing in a single GPU dispatch.
- Minimize CPU-GPU communication by keeping intermediate results on the GPU.

### Step 6: Add Fallback Support
- Implement a CPU-based fallback for systems without GPU support.
- Automatically detect GPU availability and switch between implementations.

### Step 7: Compile to WASM
- Compile the `wgpu`-based implementation to WASM.
- Integrate the WASM module with the existing frontend.

### Step 8: Testing and Benchmarking
- Test the implementation on various hardware configurations.
- Benchmark the performance improvements for different presets (`fast`, `balanced`, `highQuality`).

### Step 9: Documentation and Deployment
- Document the GPU-accelerated implementation and its usage.
- Deploy the updated version to both the command-line and web environments.

---

## Notes
- The GPU-accelerated implementation is expected to significantly reduce the time complexity of scoring and image updates.
- The fallback mechanism ensures compatibility across all devices, including those without GPU support.
- Incremental development allows for thorough testing and validation at each step.
