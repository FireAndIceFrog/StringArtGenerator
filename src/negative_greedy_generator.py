import numpy as np
from PIL import Image, ImageDraw
from skimage.draw import line as get_line_pixels
import math
from tqdm import tqdm
from .abstract_generator import AbstractGenerator

class NegativeGreedyGenerator(AbstractGenerator):
    """
    Implements the negative space greedy search algorithm.
    Instead of drawing facial features, this creates strings in the background/light areas,
    allowing the face to emerge as negative space (silhouette effect).
    """
    
    def __init__(self, image_path, num_nails=256, image_size=500, extract_subject=True, remove_shadows=True):
        # Initialize using parent class
        super().__init__(image_path, num_nails, image_size, extract_subject, remove_shadows)
        
        # Override the residual image for negative space approach
        # Instead of targeting dark areas, we target light areas
        self.residual_image = self.target_image.astype(np.float32)
        
        print("NegativeGreedyGenerator initialized - targeting light areas for negative space effect")
    
    def generate_path(self, num_lines, line_darkness=25, min_improvement_score=10.0, save_every=20):
        """
        Generates the string path using a greedy algorithm targeting negative space.
        
        Args:
            num_lines (int): The maximum number of lines to draw.
            line_darkness (int): The amount of brightness each line subtracts from the residual.
            min_improvement_score (float): Threshold to stop early if no good lines are found.
            save_every (int): Save progress every N lines (0 to disable)
        """
        print("Starting negative space greedy path generation...")
        print("This will create the face as negative space surrounded by strings...")
        
        # Start at nail 0
        current_nail = 0
        self.path = [current_nail]

        # Use tqdm for a progress bar
        for iteration in tqdm(range(num_lines), desc="Placing Background Strings"):
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
                # High brightness = light background areas we want to fill with strings
                # Ensure we don't go out of bounds
                valid_indices = (rr >= 0) & (rr < self.image_size) & (cc >= 0) & (cc < self.image_size)
                
                if np.sum(valid_indices) > 0:  # Make sure we have valid pixels
                    # Use average brightness - higher values mean lighter background areas
                    base_score = self.residual_image[rr[valid_indices], cc[valid_indices]].mean()
                    
                    # Apply eye protection mask to reduce scores in eye regions
                    protection_weights = self.eye_protection_mask[rr[valid_indices], cc[valid_indices]]
                    score = base_score * protection_weights.mean()
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
            
            # Subtract darkness from light areas (we're filling in the background)
            self.residual_image[rr[valid_indices], cc[valid_indices]] = np.clip(
                self.residual_image[rr[valid_indices], cc[valid_indices]] - line_darkness, 
                0, 
                255
            )

            # Move to the new nail and update the path
            current_nail = best_next_nail
            self.path.append(current_nail)

            # Save progress every save_every iterations
            if save_every > 0 and (iteration + 1) % save_every == 0:
                self.save_progress(base_output_path="string_art_progress_lines")
        
        print(f"\nNegative space path generation complete. Total lines: {len(self.path) - 1}")
        print("The face should now appear as clean negative space surrounded by string patterns.")
        return self.path
