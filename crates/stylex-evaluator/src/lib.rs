#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

//! The JavaScript evaluator: what an authored expression folds to, or why it
//! cannot.
//!
//! Every `stylex.*` call reaches the stylesheet through a fold of the
//! expressions the author wrote, and a fold either answers with a value or
//! refuses. Refusing is a normal answer here rather than a failure -- a value
//! that cannot be known at compile time becomes an inline style instead -- so
//! nothing on this path may abort the process, and that is what shapes the crate
//! from the bottom up.
//!
//! [`growable_stack`] is the bottom: every descent a fold runs is given room of
//! its own, because a descent that overflows the thread it started on aborts
//! from inside an evaluation whose whole contract is that it may fail.

pub mod growable_stack;

#[cfg(test)]
mod tests;
