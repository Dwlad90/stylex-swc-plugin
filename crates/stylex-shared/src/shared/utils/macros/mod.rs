//! Utility macros organized by topic.
//!
//! This module contains various utility macros used throughout the StyleX compiler,
//! organized into topic-specific submodules for better maintainability.

pub mod collection_macros;
pub mod conversion_macros;
pub mod error_macros;

// Re-export all macros at the module level for convenient access
#[allow(unused_imports)]
pub use collection_macros::*;
#[allow(unused_imports)]
pub use conversion_macros::*;
#[allow(unused_imports)]
pub use error_macros::*;
