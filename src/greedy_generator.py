import numpy as np
from PIL import Image, ImageDraw
from skimage.draw import line as get_line_pixels
import math
from tqdm import tqdm
from .abstract_generator import AbstractGenerator # For a nice progress bar

class GreedyGenerator(AbstractGenerator):
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

                # Calculate the score (average brightness along the line)
                # This matches the Processing algorithm which uses average brightness of inverted image
                # Ensure we don't go out of bounds
                valid_indices = (rr >= 0) & (rr < self.image_size) & (cc >= 0) & (cc < self.image_size)
                
                if np.sum(valid_indices) > 0:  # Make sure we have valid pixels
                    # Use average brightness (matches Processing: sum[j] = round(sum[j]*1.0/d))
                    score = self.residual_image[rr[valid_indices], cc[valid_indices]].mean()
                else:
                    score = 0

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
