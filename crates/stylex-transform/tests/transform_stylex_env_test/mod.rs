//! `stylex.env.<name>` read by each call that folds a StyleX argument.
//!
//! Split by what the call declares -- a rule or a variable -- because the two
//! families reach the environment by different routes: a rule call binds no
//! namespace name and reads the entry off what the namespace has, and a
//! variable-declaring call binds the name and carries the entry in its fold.

use crate::utils::prelude::*;
use stylex_ast::ast::convertors::create_string_expr;

mod rule_calls;
mod variable_producers;

/// One environment value, under the name every case in this suite reads.
pub(crate) fn brand_env() -> IndexMap<String, EnvEntry> {
  let mut env = IndexMap::new();

  env.insert(
    "brandPrimary".to_string(),
    EnvEntry::Expr(create_string_expr("#123456")),
  );

  env
}
