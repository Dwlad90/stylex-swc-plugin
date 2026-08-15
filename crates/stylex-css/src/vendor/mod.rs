//! Third-party code, carried here rather than depended on.
//!
//! Everything under here is somebody else's work, kept visibly separate from
//! this crate's own logic and left reading the way it was written. A module in
//! here that looks unidiomatic by this repository's standards is not an
//! oversight: its behaviour is observable in a StyleX class name, so it is
//! preserved character for character rather than tidied.
//!
//! Each module carries its own copyright notice and licence.

pub mod postcss_value_parser;
