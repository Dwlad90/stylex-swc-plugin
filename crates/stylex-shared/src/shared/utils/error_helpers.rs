//! Error handling helper macros for consistent error handling across the workspace.
//!
//! This module re-exports all macros from the organized macro submodules for backward compatibility.
//! For documentation on individual macros, see the specific macro modules:
//! - `macros::error_macros` - Error handling patterns
//! - `macros::conversion_macros` - Type conversion patterns
//! - `macros::collection_macros` - Collection and iteration patterns

// Re-export all macros for backward compatibility
#[allow(unused_imports)]
pub use super::macros::collection_macros::*;
#[allow(unused_imports)]
pub use super::macros::conversion_macros::*;
#[allow(unused_imports)]
pub use super::macros::error_macros::*;
