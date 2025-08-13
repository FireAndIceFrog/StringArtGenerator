// Quick test script to verify WASM memory-based image processing
import init, { StringArtWasm, WasmStringArtConfig, test_wasm } from './string-art-demo/src/wasm/string_art_rust_impl.js';

async function testWasmMemoryProcessing() {
    try {
        // Initialize WASM
        await init();
        console.log('WASM initialized:', test_wasm());

        // Create a simple test image (red square)
        const canvas = document.createElement('canvas');
        canvas.width = 100;
        canvas.height = 100;
        const ctx = canvas.getContext('2d');
        
        // White background
        ctx.fillStyle = 'white';
        ctx.fillRect(0, 0, 100, 100);
        
        // Black square in center
        ctx.fillStyle = 'black';
        ctx.fillRect(25, 25, 50, 50);
        
        // Convert to image data
        const imageData = ctx.getImageData(0, 0, 100, 100);
        const uint8Array = new Uint8Array(imageData.data.buffer);
        
        // Create configuration
        const config = WasmStringArtConfig.preset_fast();
        config.image_size = 100;
        config.num_nails = 36;
        
        // Test WASM memory processing
        console.log('Creating StringArtWasm with image data...');
        const stringArt = new StringArtWasm(uint8Array, config);
        
        console.log('WASM memory processing test PASSED!');
        console.log('Nail count:', stringArt.get_nail_count());
        console.log('Image size:', stringArt.get_image_size());
        
        return true;
    } catch (error) {
        console.error('WASM memory processing test FAILED:', error);
        return false;
    }
}

// Export for potential use
if (typeof window !== 'undefined') {
    window.testWasmMemoryProcessing = testWasmMemoryProcessing;
}

export { testWasmMemoryProcessing };
