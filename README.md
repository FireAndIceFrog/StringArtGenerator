# 🎨 String Art Generator - WebAssembly + React

A real-time string art generator that converts images into beautiful string art patterns. Built with Rust compiled to WebAssembly for high-performance image processing, and React for an interactive web interface.

## ✨ Features

- **Real-time Generation**: Watch string art being created line by line
- **WebAssembly Performance**: Fast image processing using Rust compiled to WASM
- **Interactive UI**: Drag & drop image upload with live preview
- **Multiple Quality Presets**: Fast, Balanced, and High Quality modes
- **Real-time Progress**: Live updates showing generation progress
- **Canvas Visualization**: Interactive canvas with string path visualization
- **Image Comparison**: Toggle between original image and string art result

## 🏗️ Architecture

### Rust WASM Module (`StringArtRustImpl/`)
- **Core Algorithm**: Greedy string art path generation
- **Image Processing**: Advanced preprocessing with subject extraction, shadow removal
- **WASM Interface**: Streaming progress callbacks for real-time updates
- **Configuration**: Flexible configuration system with presets

### React Frontend (`string-art-demo/`)
- **Modern React**: TypeScript, Hooks, and functional components
- **Real-time Updates**: Live canvas updates during generation
- **Responsive Design**: Mobile-friendly interface
- **Progress Tracking**: Detailed progress information with completion percentage

## 🚀 Quick Start

### Prerequisites
- Rust (latest stable)
- Node.js (v20.19.0+ or v22.12.0+)
- `wasm-pack` (install with `cargo install wasm-pack`)

### Installation

1. **Clone the repository**
   ```bash
   git clone <your-repo-url>
   cd StringArt
   ```

2. **Install React dependencies**
   ```bash
   cd string-art-demo
   npm install
   cd ..
   ```

3. **Build the WASM package**
   ```bash
   ./build-wasm.sh
   ```

4. **Start the development server**
   ```bash
   cd string-art-demo
   npm run dev
   ```

5. **Open your browser**
   Navigate to `http://localhost:5173`

## 🔧 Development

### Building WASM Module

The WASM module is built using `wasm-pack` with the following command:

```bash
cd StringArtRustImpl
wasm-pack build --target web --features wasm --out-dir ../string-art-wasm
```

Or use the provided build script:
```bash
./build-wasm.sh
```

### Project Structure

```
StringArt/
├── StringArtRustImpl/          # Rust WASM module
│   ├── src/
│   │   ├── lib.rs             # Main library entry point
│   │   ├── wasm.rs            # WASM interface and bindings
│   │   ├── greedy_generator.rs # Core string art algorithm
│   │   ├── image_processing.rs # Image preprocessing
│   │   └── ...
│   └── Cargo.toml             # Rust dependencies
├── string-art-demo/           # React frontend
│   ├── src/
│   │   ├── App.tsx            # Main React component
│   │   ├── hooks/
│   │   │   └── useStringArt.ts # WASM integration hook
│   │   └── components/
│   │       ├── ImageUploader.tsx
│   │       └── StringArtCanvas.tsx
│   └── package.json           # Node.js dependencies
├── build-wasm.sh             # Build script
└── README.md                 # This file
```

### WASM Interface

The Rust code exposes these main interfaces to JavaScript:

```rust
// Main string art generator
#[wasm_bindgen]
pub struct StringArtWasm {
    // ... implementation
}

#[wasm_bindgen]
impl StringArtWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(image_data: &[u8], config: Option<WasmStringArtConfig>) -> Result<StringArtWasm, JsValue>;
    
    pub fn generate_path_streaming(
        &mut self,
        max_lines: usize,
        line_darkness: f64,
        min_improvement_score: f64,
        progress_callback: &js_sys::Function,
    ) -> Result<Vec<usize>, JsValue>;
    
    pub fn get_nail_coordinates(&self) -> Vec<JsValue>;
    // ... more methods
}
```

### Configuration Options

```typescript
interface StringArtConfig {
  num_nails: number;              // Number of nails around the circle
  image_size: number;             // Target image size for processing
  extract_subject: boolean;       // Extract main subject from background
  remove_shadows: boolean;        // Remove shadows from the image
  preserve_eyes: boolean;         // Preserve eye details
  preserve_negative_space: boolean; // Preserve negative space
  negative_space_penalty: number; // Penalty for drawing in negative space
  negative_space_threshold: number; // Threshold for negative space detection
}
```

### Presets

- **Fast**: 360 nails, basic processing (quick results)
- **Balanced**: 720 nails, full processing (good quality/speed balance)
- **High Quality**: 1440 nails, full processing (best quality)

## 🧪 Usage

1. **Upload an Image**: Drag and drop or click to select an image file
2. **Choose Quality**: Select from Fast, Balanced, or High Quality presets
3. **Generate**: Click "Generate String Art" to start the process
4. **Watch Progress**: Monitor real-time progress with live canvas updates
5. **View Results**: Toggle between original image and string art result

## 🔨 Implementation Details

### Algorithm
- **Greedy Path Finding**: Iteratively selects the next nail that maximizes improvement
- **Image Preprocessing**: Subject extraction, shadow removal, contrast enhancement
- **Real-time Updates**: Streaming progress callbacks for live visualization

### Performance
- **WASM Compilation**: Rust compiled to WebAssembly for near-native performance
- **Streaming Results**: Real-time path updates without blocking the UI
- **Efficient Canvas**: Optimized canvas rendering for smooth animations

### Browser Compatibility
- Modern browsers with WebAssembly support
- Chrome, Firefox, Safari, Edge (latest versions)

## 📝 API Reference

### React Hook: `useStringArt`

```typescript
const {
  wasmModule,           // Loaded WASM module
  isLoading,           // Loading state
  error,               // Error state
  generateStringArt,   // Function to generate string art
  presets,             // Configuration presets
} = useStringArt();
```

### Progress Callback

```typescript
interface ProgressInfo {
  lines_completed: number;     // Number of lines completed
  total_lines: number;         // Total number of lines to generate
  current_nail: number;        // Current nail index
  next_nail: number;           // Next nail index
  score: number;               // Current improvement score
  completion_percent: number;  // Completion percentage (0-100)
  path_segment: [number, number]; // Current path segment
}
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by the mathematical beauty of string art
- Built with modern web technologies for educational and artistic purposes
- Thanks to the Rust and WebAssembly communities for excellent tooling

## 🐛 Troubleshooting

### Common Issues

1. **WASM Module Not Loading**
   - Ensure you've built the WASM package: `./build-wasm.sh`
   - Check browser console for CORS or loading errors

2. **Build Errors**
   - Verify Rust and wasm-pack are installed
   - Update dependencies: `cargo update` and `npm update`

3. **Performance Issues**
   - Try the "Fast" preset for quicker results
   - Ensure your browser supports WebAssembly

### Development Tips

- Use browser dev tools to monitor WASM module loading
- Check the Network tab for WASM file loading issues
- Use the Console to see progress callback outputs

## 📧 Support

For questions, issues, or contributions, please open an issue on GitHub or contact the maintainers.

---

**Happy String Art Creating! 🎨**
