import numpy as np
from PIL import Image, ImageDraw
from skimage.draw import line as get_line_pixels
import math
from tqdm import tqdm

from src.create_outline_image import create_outline_image
from src.greedy_generator import GreedyGenerator # Original algorithm implementation

# --- EXAMPLE USAGE ---
if __name__ == '__main__':
    # Make sure you have an image named 'portrait.jpg' in the same directory,
    # or change the path to your image.
    try:

        create_outline_image('portrait.jpg', 'portrait_outline.jpg', 
                           low_threshold=25, high_threshold=50,
                           max_blur_kernel=55, min_blur_kernel=11, 
                           use_gradient_blur=True)

        # 1. Initialize the generator with your image
        # You can find good public domain portraits on sites like unsplash.com
        generator = GreedyGenerator(image_path='portrait_outline.jpg', num_nails=720, image_size=500)

        # 2. Generate the path (the sequence of nails)
        # More lines = more detail, but longer processing time
        generator.generate_path(num_lines=1000, line_darkness=30)

        # 3. Render the final image from the path
        generator.render_image(output_path='string_art_output.png')

        # 4. (Optional) Save the nail sequence for physical creation
        generator.save_path(output_path='string_art_path.txt')

    except FileNotFoundError:
        print("Error: 'portrait.jpg' not found.")
        print("Please place a portrait image in the same directory or update the path.")
