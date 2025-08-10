import numpy as np
from PIL import Image, ImageDraw
from skimage.draw import line as get_line_pixels
import math
from tqdm import tqdm # For a nice progress bar

class StringArtGenerator:
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


class GreedyGenerator(StringArtGenerator):
    """
    Implements the greedy search algorithm.
    At each step, it chooses the line that covers the most darkness
    in the current residual image.
    """
    def generate_path(self, num_lines, line_darkness=25, min_improvement_score=10.0):
        """
        Generates the string path using a greedy algorithm.

        Args:
            num_lines (int): The maximum number of lines to draw.
            line_darkness (int): The amount of darkness each line subtracts from the residual.
            min_improvement_score (float): Threshold to stop early if no good lines are found.
        """
        print("Starting greedy path generation...")
        
        # Start at nail 0
        current_nail = 0
        self.path = [current_nail]

        # Use tqdm for a progress bar
        for _ in tqdm(range(num_lines), desc="Placing Strings"):
            best_next_nail = -1
            max_score = -1

            # Find the best next nail to connect to
            for next_nail in range(self.num_nails):
                if next_nail == current_nail:
                    continue

                # Get pixel coordinates for the candidate line
                start_coord = self.nail_coords[current_nail]
                end_coord = self.nail_coords[next_nail]
                rr, cc = get_line_pixels(start_coord[1], start_coord[0], end_coord[1], end_coord[0])

                # Calculate the score (sum of darkness along the line)
                # Ensure we don't go out of bounds
                valid_indices = (rr >= 0) & (rr < self.image_size) & (cc >= 0) & (cc < self.image_size)
                score = self.residual_image[rr[valid_indices], cc[valid_indices]].sum()

                if score > max_score:
                    max_score = score
                    best_next_nail = next_nail

            # Early exit condition
            if max_score < min_improvement_score:
                print("\nNo significant improvement found. Exiting early.")
                break

            # "Draw" the best line by subtracting its darkness from the residual
            start_coord = self.nail_coords[current_nail]
            end_coord = self.nail_coords[best_next_nail]
            rr, cc = get_line_pixels(start_coord[1], start_coord[0], end_coord[1], end_coord[0])
            valid_indices = (rr >= 0) & (rr < self.image_size) & (cc >= 0) & (cc < self.image_size)
            
            # Use np.clip to prevent values from going below zero
            self.residual_image[rr[valid_indices], cc[valid_indices]] = np.clip(
                self.residual_image[rr[valid_indices], cc[valid_indices]] - line_darkness, 
                0, 
                255
            )

            # Move to the new nail and update the path
            current_nail = best_next_nail
            self.path.append(current_nail)
        
        print(f"\nPath generation complete. Total lines: {len(self.path) - 1}")
        return self.path

# --- EXAMPLE USAGE ---
if __name__ == '__main__':
    # Make sure you have an image named 'portrait.jpg' in the same directory,
    # or change the path to your image.
    try:
        # 1. Initialize the generator with your image
        # You can find good public domain portraits on sites like unsplash.com
        generator = GreedyGenerator(image_path='portrait.jpg', num_nails=280)

        # 2. Generate the path (the sequence of nails)
        # More lines = more detail, but longer processing time
        generator.generate_path(num_lines=3000, line_darkness=20)

        # 3. Render the final image from the path
        generator.render_image(output_path='string_art_output.png')

        # 4. (Optional) Save the nail sequence for physical creation
        generator.save_path(output_path='string_art_path.txt')

    except FileNotFoundError:
        print("Error: 'portrait.jpg' not found.")
        print("Please place a portrait image in the same directory or update the path.")