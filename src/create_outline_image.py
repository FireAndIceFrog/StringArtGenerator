import cv2
import numpy as np

def create_outline_image(image_path, output_path, low_threshold=50, high_threshold=150, 
                        max_blur_kernel=21, min_blur_kernel=5, use_gradient_blur=True):
    """
    Converts an image to its "outline" version using Canny edge detection with optional gradient blur.

    Args:
        image_path (str): Path to the input image.
        output_path (str): Path to save the outline image.
        low_threshold (int): First threshold for the hysteresis procedure in Canny.
        high_threshold (int): Second threshold for the hysteresis procedure.
        max_blur_kernel (int): Maximum blur kernel size for top of image (must be odd).
        min_blur_kernel (int): Minimum blur kernel size for bottom of image (must be odd).
        use_gradient_blur (bool): Whether to use gradient blur or uniform blur.
    """
    # 1. Read the image in grayscale
    img = cv2.imread(image_path, cv2.IMREAD_GRAYSCALE)
    if img is None:
        print(f"Error: Could not read image from {image_path}")
        return

    # 2. Apply blur to reduce noise before edge detection
    if use_gradient_blur:
        blurred_img = apply_gradient_blur(img, max_blur_kernel, min_blur_kernel)
    else:
        # Fallback to uniform blur for backward compatibility
        blurred_img = cv2.GaussianBlur(img, (11, 11), 0)

    # 3. Perform Canny edge detection
    edges = cv2.Canny(blurred_img, low_threshold, high_threshold)

    # 4. Invert the image
    # Canny creates white edges on a black background.
    # Our algorithm needs black lines on a white background.
    inverted_edges = 255 - edges

    # 5. Save the final outline image
    cv2.imwrite(output_path, inverted_edges)
    print(f"Outline image saved to {output_path}")


def apply_gradient_blur(img, max_blur_kernel, min_blur_kernel):
    """
    Apply variable Gaussian blur with bell curve pattern: maximum blur at top and bottom edges,
    minimum blur in the center (to preserve facial features).
    
    Args:
        img (numpy.ndarray): Input grayscale image.
        max_blur_kernel (int): Maximum blur kernel size for top and bottom edges.
        min_blur_kernel (int): Minimum blur kernel size for center of image.
    
    Returns:
        numpy.ndarray: Blurred image with bell curve blur effect.
    """
    height, width = img.shape
    result = np.zeros_like(img, dtype=np.float32)
    weight_map = np.zeros_like(img, dtype=np.float32)
    
    # Ensure kernels are odd
    max_blur_kernel = max_blur_kernel if max_blur_kernel % 2 == 1 else max_blur_kernel + 1
    min_blur_kernel = min_blur_kernel if min_blur_kernel % 2 == 1 else min_blur_kernel + 1
    
    # Use more strips for finer gradation and add overlap
    num_strips = 50  # Increased from 20 to 50 for smoother transitions
    overlap_ratio = 0.3  # 30% overlap between strips
    center_y = height / 2
    
    # Calculate strip height with overlap
    base_strip_height = height / (num_strips - (num_strips - 1) * overlap_ratio)
    
    for i in range(num_strips):
        # Calculate strip boundaries with overlap
        y_start = int(i * base_strip_height * (1 - overlap_ratio))
        y_end = int(y_start + base_strip_height)
        
        # Clamp to image boundaries
        y_start = max(0, y_start)
        y_end = min(height, y_end)
        
        if y_start >= y_end:
            continue
            
        # Calculate center position of current strip
        strip_center_y = (y_start + y_end) / 2
        
        # Calculate distance from image center (0 = center, 1 = top/bottom edges)
        distance_from_center = abs(strip_center_y - center_y) / (height / 2)
        
        # Apply bell curve: minimum blur at center, maximum at edges
        # Use quadratic function for smooth transition
        blur_ratio = distance_from_center ** 2
        kernel_size = int(min_blur_kernel + blur_ratio * (max_blur_kernel - min_blur_kernel))
        
        # Ensure kernel size is odd and within bounds
        kernel_size = kernel_size if kernel_size % 2 == 1 else kernel_size + 1
        kernel_size = max(kernel_size, 3)  # Minimum kernel size of 3
        kernel_size = min(kernel_size, max_blur_kernel)  # Don't exceed maximum
        
        # Extract and blur the strip
        strip = img[y_start:y_end, :]
        blurred_strip = cv2.GaussianBlur(strip, (kernel_size, kernel_size), 0)
        
        # Create weight function for smooth blending (Gaussian-like falloff from center)
        strip_height = y_end - y_start
        if strip_height > 1:
            strip_center = strip_height / 2
            y_coords = np.arange(strip_height)
            # Create smooth weight function (higher in center, lower at edges)
            weights = np.exp(-((y_coords - strip_center) / (strip_height / 4)) ** 2)
            weights = weights.reshape(-1, 1)  # Make it a column vector
        else:
            weights = np.ones((strip_height, 1))
        
        # Accumulate weighted results
        result[y_start:y_end, :] += blurred_strip.astype(np.float32) * weights
        weight_map[y_start:y_end, :] += weights
    
    # Normalize by total weights to get final result
    # Avoid division by zero
    weight_map[weight_map == 0] = 1
    result = result / weight_map
    
    # Apply final smoothing to eliminate any remaining artifacts
    result = cv2.GaussianBlur(result.astype(np.uint8), (5, 5), 0)
    
    return result

# Example usage:
if __name__ == '__main__':
    # This will create 'portrait_outline.jpg' from 'portrait.jpg'
    create_outline_image('portrait.jpg', 'portrait_outline.jpg')
