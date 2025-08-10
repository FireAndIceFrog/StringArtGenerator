import numpy as np
from PIL import Image, ImageDraw
import math
import cv2

class AbstractGenerator:
    """Base class for creating string art.
    
    This class handles image loading, canvas setup, and rendering.
    It's designed to be inherited by specific algorithm implementations.
    """
    def __init__(self, image_path, num_nails=256, image_size=500, extract_subject=True, remove_shadows=True):
        self.num_nails = num_nails
        self.image_size = image_size
        self.path = []

        # 1. Load and prepare the image with improved preprocessing
        processed_image = self._load_and_preprocess_image(image_path, image_size, extract_subject, remove_shadows)
        self.target_image = processed_image

        # 2. Create the residual image (error map)
        # We use a float dtype to allow for negative values during subtraction
        self.residual_image = 255.0 - self.target_image.astype(np.float32)

        # 3. Set up canvas and nail coordinates
        self.center = (image_size // 2, image_size // 2)
        self.radius = image_size // 2 - 5 # Inset nails slightly
        self.nail_coords = self._calculate_nail_coords()

    def _load_and_preprocess_image(self, image_path, image_size, extract_subject, remove_shadows):
        """
        Load and preprocess image with aspect ratio preservation and subject extraction.
        
        Args:
            image_path: Path to input image
            image_size: Target square size
            extract_subject: Whether to extract subject/person from background
            remove_shadows: Whether to reduce shadow effects
        
        Returns:
            numpy array of processed grayscale image
        """
        # Load image and convert to grayscale
        with Image.open(image_path).convert("L") as img:
            original_img = np.array(img)
        
        # Step 1: Preserve aspect ratio - no stretching!
        h, w = original_img.shape
        
        # Create square white canvas
        canvas = np.full((image_size, image_size), 255, dtype=np.uint8)
        
        # Calculate scaling to fit image in canvas while preserving aspect ratio
        scale = min(image_size / w, image_size / h)
        new_w = int(w * scale)
        new_h = int(h * scale)
        
        # Resize image maintaining aspect ratio
        resized_img = cv2.resize(original_img, (new_w, new_h), interpolation=cv2.INTER_LANCZOS4)
        
        # Center the image on white canvas
        start_x = (image_size - new_w) // 2
        start_y = (image_size - new_h) // 2
        canvas[start_y:start_y + new_h, start_x:start_x + new_w] = resized_img
        
        processed_img = canvas
        
        # Step 2: Remove shadows if requested
        if remove_shadows:
            processed_img = self._remove_shadows(processed_img)
        
        # Step 3: Extract subject if requested
        if extract_subject:
            processed_img = self._extract_subject(processed_img)
        
        # Save processed image for reference
        Image.fromarray(processed_img).save('string_art_input.png')
        
        return processed_img

    def _remove_shadows(self, img):
        """
        Reduce shadow effects using histogram equalization and contrast enhancement.
        """
        # Apply CLAHE (Contrast Limited Adaptive Histogram Equalization)
        clahe = cv2.createCLAHE(clipLimit=2.0, tileGridSize=(8, 8))
        enhanced = clahe.apply(img)
        
        # Gamma correction to brighten shadows
        gamma = 1.2
        inv_gamma = 1.0 / gamma
        table = np.array([((i / 255.0) ** inv_gamma) * 255 for i in np.arange(0, 256)]).astype("uint8")
        corrected = cv2.LUT(enhanced, table)
        
        return corrected

    def _extract_subject(self, img):
        """
        Extract subject/person from background using edge-based approach that preserves facial features.
        """
        # Step 1: Apply bilateral filter to reduce noise while preserving edges
        filtered = cv2.bilateralFilter(img, 9, 75, 75)
        
        # Step 2: Use Canny edge detection to find edges
        edges = cv2.Canny(filtered, 50, 150, apertureSize=3)
        
        # Step 3: Dilate edges slightly to connect nearby edges
        kernel = np.ones((3, 3), np.uint8)
        edges_dilated = cv2.dilate(edges, kernel, iterations=1)
        
        # Step 4: Find contours from edges
        contours, _ = cv2.findContours(edges_dilated, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
        
        if contours:
            # Step 5: Find the largest contour (likely the main subject)
            largest_contour = max(contours, key=cv2.contourArea)
            
            # Only proceed if the contour is reasonably large
            if cv2.contourArea(largest_contour) > img.shape[0] * img.shape[1] * 0.05:  # At least 5% of image
                
                # Step 6: Create a gentler mask using GrabCut-like approach
                # Initialize mask
                mask = np.zeros(img.shape[:2], np.uint8)
                
                # Get bounding rectangle of the largest contour
                x, y, w, h = cv2.boundingRect(largest_contour)
                
                # Create initial mask: everything outside bounding rect is background
                # Everything inside is "probably foreground"
                mask.fill(cv2.GC_BGD)  # Background
                mask[y:y+h, x:x+w] = cv2.GC_PR_FGD  # Probably foreground
                
                # Mark the contour area as definite foreground
                cv2.fillPoly(mask, [largest_contour], cv2.GC_FGD)
                
                # Step 7: Use distance transform for smoother transitions
                # Convert mask to binary for distance transform
                binary_mask = (mask >= cv2.GC_PR_FGD).astype(np.uint8) * 255
                
                # Apply distance transform and normalize
                dist_transform = cv2.distanceTransform(binary_mask, cv2.DIST_L2, 5)
                dist_normalized = cv2.normalize(dist_transform, None, 0, 255, cv2.NORM_MINMAX).astype(np.uint8)
                
                # Create smooth mask with gradient falloff
                smooth_mask = cv2.GaussianBlur(dist_normalized, (21, 21), 0)
                
                # Apply threshold to create final mask (more lenient than before)
                _, final_mask = cv2.threshold(smooth_mask, 30, 255, cv2.THRESH_BINARY)
                
                # Step 8: Apply the mask with smooth blending
                result = np.full_like(img, 255)  # White background
                
                # Use the mask to blend the original image
                mask_float = final_mask.astype(np.float32) / 255.0
                result = (result * (1 - mask_float) + img * mask_float).astype(np.uint8)
                
                return result
            else:
                # Contour too small, return original
                return img
        else:
            # No contours found, return original image
            return img

    def _calculate_nail_coords(self):
        """Calculates the (x, y) coordinates for each nail on a circle."""
        coords = []
        for i in range(self.num_nails):
            angle = 2 * math.pi * i / self.num_nails
            x = int(self.center[0] + self.radius * math.cos(angle))
            y = int(self.center[1] + self.radius * math.sin(angle))
            coords.append((x, y))
        return np.array(coords)

    def generate_path(self, *args, **kwargs):
        """Abstract method to be implemented by subclasses."""
        raise NotImplementedError("Each generator must implement its own path-finding algorithm.")

    def render_image(self, output_path, line_color=(0, 0, 0, 235)):
        """Draws the final string art image from the generated path."""
        if not self.path:
            print("Path has not been generated yet. Call generate_path() first.")
            return

        # Create a blank canvas (RGBA to allow for transparent lines)
        canvas = Image.new("RGBA", (self.image_size, self.image_size), "white")
        draw = ImageDraw.Draw(canvas)

        print(f"Rendering image with {len(self.path) - 1} lines...")
        for i in range(len(self.path) - 1):
            start_nail = self.nail_coords[self.path[i]]
            end_nail = self.nail_coords[self.path[i+1]]
            draw.line((tuple(start_nail), tuple(end_nail)), fill=line_color)
        
        canvas.convert("RGB").save(output_path)
        print(f"Image saved to {output_path}")

    def save_path(self, output_path):
        """Saves the sequence of nails to a text file."""
        with open(output_path, 'w') as f:
            f.write(','.join(map(str, self.path)))
        print(f"Path saved to {output_path}")

    def save_progress(self, base_output_path="string_art_progress", save_image=True, save_path=True):
        """Saves the current progress (image and/or path) with the current line count."""
        if not self.path:
            print("No path to save yet.")
            return
        
        line_count = len(self.path) - 1
        
        if save_image:
            progress_image_path = f"{base_output_path}_lines.png"
            self.render_image(progress_image_path)
        
        if save_path:
            progress_path_file = f"{base_output_path}_lines.txt"
            self.save_path(progress_path_file)
        
        print(f"Progress saved at {line_count} lines")
