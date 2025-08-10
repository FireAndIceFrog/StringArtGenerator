import numpy as np
from PIL import Image, ImageDraw
import math
import cv2
import os

class AbstractGenerator:
    """Base class for creating string art.
    
    This class handles image loading, canvas setup, and rendering.
    It's designed to be inherited by specific algorithm implementations.
    """
    def __init__(self, image_path, num_nails=256, image_size=500, extract_subject=True, remove_shadows=True, preserve_eyes=True):
        self.num_nails = num_nails
        self.image_size = image_size
        self.path = []
        self.preserve_eyes = preserve_eyes

        # 1. Load and prepare the image with improved preprocessing
        processed_image = self._load_and_preprocess_image(image_path, image_size, extract_subject, remove_shadows)
        self.target_image = processed_image

        # 2. Detect eyes if eye preservation is enabled
        if self.preserve_eyes:
            self.eye_regions = self._detect_eyes(self.target_image)
            self.eye_protection_mask = self._create_eye_protection_mask()
        else:
            self.eye_regions = []
            self.eye_protection_mask = np.ones((image_size, image_size), dtype=np.float32)

        # 3. Create the residual image (error map)
        # We use a float dtype to allow for negative values during subtraction
        self.residual_image = 255.0 - self.target_image.astype(np.float32)

        # 4. Set up canvas and nail coordinates
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
        
        # Find the most common color in the original image to use as background
        background_color = self._get_most_common_color(original_img)
        
        # Create square canvas with the most common color
        canvas = np.full((image_size, image_size), background_color, dtype=np.uint8)
        
        # Calculate scaling to fit image in canvas while preserving aspect ratio
        scale = min(image_size / w, image_size / h)
        new_w = int(w * scale)
        new_h = int(h * scale)
        
        # Resize image maintaining aspect ratio
        resized_img = cv2.resize(original_img, (new_w, new_h), interpolation=cv2.INTER_LANCZOS4)
        
        # Center the image on canvas with most common color background
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

    def _get_most_common_color(self, img):
        """
        Find the most common color (pixel value) in the image.
        
        Args:
            img: numpy array of grayscale image
            
        Returns:
            int: The most frequently occurring pixel value
        """
        # Get unique values and their counts
        unique_values, counts = np.unique(img, return_counts=True)
        
        # Find the value with the highest count
        most_common_idx = np.argmax(counts)
        most_common_color = unique_values[most_common_idx]
        
        print(f"Most common color in image: {most_common_color} (appears {counts[most_common_idx]} times)")
        
        return int(most_common_color)

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

    def _detect_eyes(self, img):
        """
        Detect eyes in the image using OpenCV's Haar cascade classifiers.
        
        Args:
            img: numpy array of grayscale image
            
        Returns:
            list of tuples: [(x, y, w, h), ...] for each detected eye region
        """
        try:
            # Try to load OpenCV's built-in eye cascade classifier
            eye_cascade = cv2.CascadeClassifier(cv2.data.haarcascades + 'haarcascade_eye.xml')
            
            if eye_cascade.empty():
                print("Warning: Could not load eye cascade classifier. Eyes will not be protected.")
                return []
            
            # Detect eyes
            eyes = eye_cascade.detectMultiScale(
                img,
                scaleFactor=1.1,
                minNeighbors=5,
                minSize=(20, 20),
                maxSize=(100, 100)
            )
            
            eye_regions = []
            for (x, y, w, h) in eyes:
                # Expand the eye region slightly for better protection
                padding = max(10, min(w, h) // 4)
                x_exp = max(0, x - padding)
                y_exp = max(0, y - padding)
                w_exp = min(self.image_size - x_exp, w + 2 * padding)
                h_exp = min(self.image_size - y_exp, h + 2 * padding)
                
                eye_regions.append((x_exp, y_exp, w_exp, h_exp))
            
            print(f"Detected {len(eye_regions)} eye regions")
            return eye_regions
            
        except Exception as e:
            print(f"Warning: Eye detection failed: {e}. Eyes will not be protected.")
            return []

    def _create_eye_protection_mask(self):
        """
        Create a mask that reduces line placement probability in eye regions.
        
        Returns:
            numpy array: Protection mask where 1.0 = normal, 0.0-0.5 = protected areas
        """
        mask = np.ones((self.image_size, self.image_size), dtype=np.float32)
        
        for (x, y, w, h) in self.eye_regions:
            # Create a protection zone around each eye
            # Center of eye gets strongest protection (0.1), fading outward
            center_x, center_y = x + w // 2, y + h // 2
            
            # Create gradient protection - stronger in center, weaker at edges
            for i in range(y, min(y + h, self.image_size)):
                for j in range(x, min(x + w, self.image_size)):
                    # Distance from center of eye
                    dist_x = abs(j - center_x) / (w / 2)
                    dist_y = abs(i - center_y) / (h / 2)
                    dist = np.sqrt(dist_x**2 + dist_y**2)
                    
                    # Protection strength decreases with distance
                    if dist <= 1.0:  # Inside the eye region
                        protection = 0.1 + 0.4 * dist  # 0.1 at center, 0.5 at edge
                        mask[i, j] = min(mask[i, j], protection)
        
        return mask

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
