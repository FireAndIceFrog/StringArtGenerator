
import numpy as np
from tqdm import tqdm
from skimage.draw import line as get_line_pixels
from .abstract_generator import AbstractGenerator


class ContourGenerator(AbstractGenerator):
    """
    NEW: Scores lines based on the length of continuous dark pixel runs.
    This heavily rewards tracing outlines.
    """
    def generate_path(self, num_lines, line_darkness=25, min_improvement_score=10.0, darkness_threshold=10, save_every=20):
        print("Starting contour-following path generation...")
        current_nail = 0
        self.path = [current_nail]

        for iteration in tqdm(range(num_lines), desc="Contour Search"):
            best_next_nail, max_score = -1, -1

            for next_nail in range(self.num_nails):
                if next_nail == current_nail: continue

                start_coord, end_coord = self.nail_coords[current_nail], self.nail_coords[next_nail]
                rr, cc = get_line_pixels(start_coord[1], start_coord[0], end_coord[1], end_coord[0])
                
                # --- TWO-TIER SCORING LOGIC ---
                # Tier 1: Prioritize lines with fewer dark segments
                # Tier 2: Within each tier, prioritize longer continuous runs
                
                dark_segments = []
                current_run = 0
                continuous_score = 0
                
                for r, c in zip(rr, cc):
                    if 0 <= r < self.image_size and 0 <= c < self.image_size:
                        if self.residual_image[r, c] > darkness_threshold:
                            current_run += 1
                        else:
                            if current_run > 0:
                                dark_segments.append(current_run)
                                continuous_score += current_run ** 2
                                current_run = 0
                
                # Add the last run if it exists
                if current_run > 0:
                    dark_segments.append(current_run)
                    continuous_score += current_run ** 2
                
                # Calculate two-tier score
                num_segments = len(dark_segments)
                
                # Primary factor: fewer segments = higher priority (multiply by large number)
                # Secondary factor: longer continuous runs = higher priority
                segment_penalty = num_segments * 1000000  # Large penalty for more segments
                current_score = continuous_score - segment_penalty
                
                # Alternative approach: boost single-segment lines significantly
                if num_segments == 1:
                    current_score = continuous_score + 10000000  # Big bonus for single segment
                elif num_segments == 0:
                    current_score = 0  # No dark pixels
                else:
                    current_score = continuous_score  # Multi-segment lines get base score
                # --- END TWO-TIER LOGIC ---

                if current_score > max_score:
                    max_score, best_next_nail = current_score, next_nail

            if max_score < min_improvement_score:
                print("\nNo significant improvement found. Exiting early.")
                break

            start_coord, end_coord = self.nail_coords[current_nail], self.nail_coords[best_next_nail]
            rr, cc = get_line_pixels(start_coord[1], start_coord[0], end_coord[1], end_coord[0])
            valid_indices = (rr >= 0) & (rr < self.image_size) & (cc >= 0) & (cc < self.image_size)
            self.residual_image[rr[valid_indices], cc[valid_indices]] = np.clip(
                self.residual_image[rr[valid_indices], cc[valid_indices]] - line_darkness, 0, 255
            )
            current_nail = best_next_nail
            self.path.append(current_nail)
            
            # Save progress every save_every iterations
            if save_every > 0 and (iteration + 1) % save_every == 0:
                self.save_progress()
        
        print(f"\nPath generation complete. Total lines: {len(self.path) - 1}")
        return self.path
