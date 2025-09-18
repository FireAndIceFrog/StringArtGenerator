use thiserror::Error;

#[derive(Error, Debug)]
pub enum StringArtError {
    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),
    
    #[cfg(feature = "opencv-support")]
    #[error("OpenCV error: {0}")]
    OpenCvError(#[from] opencv::Error),
    
    #[error("State lock error")]
    StateLock { message: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Invalid parameter: {message}")]
    InvalidParameter { message: String },
    
    #[error("Path generation failed: {reason}")]
    PathGenerationFailed { reason: String },
    
    #[error("No improvement found after {iterations} iterations")]
    NoImprovementFound { iterations: usize },
    
    #[error("Image processing error: {message}")]
    ImageProcessingError { message: String },
    
    #[error("Serialization error: {message}")]
    SerializationError { message: String },
}

pub type Result<T> = std::result::Result<T, StringArtError>;
