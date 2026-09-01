#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

//! What a name resolves to, against the declarations the state recorded.
//!
//! The state below this crate records *that* a file declares something. This
//! crate answers the next question: given an identifier or an expression, which
//! declaration binds it, and what does that declaration spell? Both the visitor
//! and the evaluator above ask it, which is why it is a layer of its own rather
//! than a corner of either.
//!
//! [`lookup`] matches a name against a declaration. [`convertors`] reads the
//! declaration back as a string or an expression. Nothing here folds an
//! expression: a conversion that would have to evaluate one belongs above.

pub mod convertors;
pub mod lookup;

#[cfg(test)]
mod tests;
