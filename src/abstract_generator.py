import numpy as np
from PIL import Image, ImageDraw
import math

class AbstractGenerator:
    """Base class for creating string art.
    
    This class handles image loading, canvas setup, and rendering.
    It's designed to be inherited by specific algorithm implementations.
    """
    def __init__(self, image_path, num_nails=256, image_size=500):
        self.num_nails = num_nails
        self.image_size = image_size
        self.path = []

        # 1. Load and prepare the image
        with Image.open(image_path).convert("L") as img:
            # Resize and ensure it's a square
            resized_img = img.resize((image_size, image_size))
            self.target_image = np.array(resized_img)
            resized_img.save('string_art_input.png')  # Save the resized image for reference

        # 2. Create the residual image (error map)
        # We use a float dtype to allow for negative values during subtraction
        self.residual_image = 255.0 - self.target_image.astype(np.float32)

        # 3. Set up canvas and nail coordinates
        self.center = (image_size // 2, image_size // 2)
        self.radius = image_size // 2 - 5 # Inset nails slightly
        self.nail_coords = self._calculate_nail_coords()

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
