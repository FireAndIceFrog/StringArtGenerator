#!/bin/bash

# Build script for WASM package
set -e

echo "🦀 Building WASM package..."

cd StringArtRustImpl

# Build the WASM package with wasm-pack
wasm-pack build --target web --features wasm

echo "✅ WASM package built successfully!"

# Copy the package to the React app's public directory
if [ -d "../string-art-demo/public" ]; then
    echo "📦 Copying WASM files to React app..."
    cp -r pkg/* ../string-art-demo/public/
    echo "✅ WASM files copied to React app!"
fi

echo "🎉 Build complete! You can now use the WASM module in your React app."
echo ""
echo "To test:"
echo "1. Make sure the React dev server is running: cd string-art-demo && npm run dev"
echo "2. Open http://localhost:5173 in your browser"
echo "3. Upload an image and click 'Generate String Art'"
