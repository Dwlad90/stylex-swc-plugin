//! A sequence inside a folded call is worth its last operand.
//!
//! The other operands are never evaluated, so a name nothing binds may stand
//! there and the fold still answers.

use super::source_evaluation::*;

#[test]
fn a_sequence_in_a_receiver_folds_to_its_last_operand() {
  assert_folds_to_string("[(0, 'red')].join()", "red");
}

#[test]
fn an_operand_before_the_last_is_never_evaluated() {
  assert_folds_to_string("[(missing, 'red')].join()", "red");
  assert_folds_to_string("(missing, 'red').toUpperCase()", "RED");
}

#[test]
fn a_sequence_in_a_callback_folds_to_its_last_operand() {
  assert_folds_to_string("['a', 'b'].map((x) => (missing, x + '!')).join()", "a!,b!");
}

#[test]
fn a_last_operand_that_is_a_sequence_folds_to_its_own_last_operand() {
  assert_folds_to_string("[(0, (1, 'blue'))].join()", "blue");
}
