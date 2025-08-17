use crate::error::Result;
use crate::state::app_state::StringArtState;
use std::sync::{Arc, RwLock};

/// Trait for applying a mask to the string art state.
/// Implementations of this trait will modify the state, for example by updating
/// the eye protection or negative space masks.
pub trait MaskApplicator {
    /// Applies the mask to the given string art state.
    fn apply(&self, state: Arc<RwLock<StringArtState>>) -> Result<()>;
}
