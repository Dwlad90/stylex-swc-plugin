//! How a member read off a `defineVars` group is named.
//!
//! Two readers derive a member's name from the same three facts — the group's
//! identity, the key, and the two debug options — and they run in different
//! places: the evaluator's own lookup, and the compile-time engine, which holds
//! the identity and not the group. So the derivation is one function and its
//! answers are asserted here rather than through either caller.
//!
//! Every expected value below is what `@stylexjs/babel-plugin` 0.19.0 derives
//! for the same key under the same options.

use crate::shared::structures::theme_ref::{VarNaming, var_group_member};

/// The two options as a pair, so a case says which spelling it is asking about.
fn naming(debug: bool, readable_names: bool) -> VarNaming {
  VarNaming::from_flags(debug, readable_names)
}

const BASE: &str = "vars.stylex.js//vars";
const PREFIX: &str = "x";

/// The ordinary read: a `var()` naming a variable hashed from the group and the
/// key together.
#[test]
fn a_named_member_is_a_variable_hashed_from_the_group_and_the_key() {
  assert_eq!(
    var_group_member(BASE, PREFIX, "primary", naming(false, false)),
    "var(--x1ineb92)"
  );
}

/// Two keys of one group are two variables, which is the whole of what the
/// derivation is for.
#[test]
fn two_keys_of_one_group_name_two_variables() {
  assert_ne!(
    var_group_member(BASE, PREFIX, "primary", naming(false, false)),
    var_group_member(BASE, PREFIX, "secondary", naming(false, false))
  );
}

/// The same key under two groups is two variables, so a name is a property of
/// the group as much as of the key.
#[test]
fn one_key_under_two_groups_names_two_variables() {
  assert_ne!(
    var_group_member(BASE, PREFIX, "primary", naming(false, false)),
    var_group_member(
      "other.stylex.js//other",
      PREFIX,
      "primary",
      naming(false, false)
    )
  );
}

/// A dotted key is one key: a group whose members are groups names one variable
/// per path rather than one per level.
#[test]
fn a_dotted_key_names_one_variable_for_the_whole_path() {
  assert_eq!(
    var_group_member(BASE, PREFIX, "brand.primary", naming(false, false)),
    "var(--x1tr9ywo)"
  );
}

/// A key an author spelled as a variable of their own is used exactly as
/// written, hashed or prefixed by nothing.
#[test]
fn a_key_spelled_as_a_variable_is_used_as_written() {
  assert_eq!(
    var_group_member(BASE, PREFIX, "--custom", naming(false, false)),
    "var(--custom)"
  );

  // Under debug too: there is no hash to make readable.
  assert_eq!(
    var_group_member(BASE, PREFIX, "--custom", naming(true, true)),
    "var(--custom)"
  );
}

/// The group's own key answers a bare name rather than a `var()`, because it is
/// the group's hash and not a variable anything reads.
#[test]
fn the_group_s_own_key_answers_a_bare_hash() {
  let hash = var_group_member(BASE, PREFIX, "__varGroupHash__", naming(false, false));

  assert_eq!(hash, "xop34xu");

  // And stays bare under debug, where every other key gains a readable prefix.
  assert_eq!(
    var_group_member(BASE, PREFIX, "__varGroupHash__", naming(true, true)),
    hash
  );
}

/// Debug naming puts the key in front of the hash, so a variable in a stylesheet
/// says which token it is.
#[test]
fn a_debug_name_carries_the_key_in_front_of_the_hash() {
  assert_eq!(
    var_group_member(BASE, PREFIX, "primary", naming(true, true)),
    "var(--primary-x1ineb92)"
  );
}

/// Both options together, and neither alone: the readable half is what the
/// second one turns on.
#[test]
fn a_readable_name_needs_both_options() {
  let plain = var_group_member(BASE, PREFIX, "primary", naming(false, false));

  assert_eq!(
    var_group_member(BASE, PREFIX, "primary", naming(true, false)),
    plain
  );
  assert_eq!(
    var_group_member(BASE, PREFIX, "primary", naming(false, true)),
    plain
  );
}

/// A key that is not a plain name is made into one for the readable half — every
/// character that is not a letter or a digit becomes an underscore — while the
/// hash still stands for the key as written.
#[test]
fn a_debug_name_makes_the_key_safe_to_write_as_a_variable() {
  assert_eq!(
    var_group_member(BASE, PREFIX, "brand.primary", naming(true, true)),
    "var(--brand_primary-x1tr9ywo)"
  );

  let named = var_group_member(BASE, PREFIX, "a b&c", naming(true, true));

  assert!(
    named.starts_with("var(--a_b_c-x"),
    "expected every character that is not a letter or a digit to be an underscore, got `{}`",
    named
  );
}

/// A key beginning with a digit gains a leading underscore, because a CSS custom
/// property may not start with one.
#[test]
fn a_debug_name_of_a_numeric_key_gains_a_leading_underscore() {
  let named = var_group_member(BASE, PREFIX, "0", naming(true, true));

  assert!(
    named.starts_with("var(--_0-"),
    "expected a numeric key to be prefixed, got `{}`",
    named
  );
}

/// The prefix is the project's own and prefixes every name the derivation
/// answers, the readable ones included.
#[test]
fn the_class_name_prefix_is_carried_into_every_name() {
  assert!(
    var_group_member(BASE, "zz", "primary", naming(false, false)).starts_with("var(--zz"),
    "expected the project's prefix in front of the hash"
  );

  assert!(
    var_group_member(BASE, "zz", "primary", naming(true, true)).starts_with("var(--primary-zz"),
    "expected the project's prefix behind the readable key"
  );
}

/// An empty key is a key: it names the group's own identity without being the
/// group's own hash, which is what says the two branches are separate.
#[test]
fn an_empty_key_is_still_a_key() {
  let named = var_group_member(BASE, PREFIX, "", naming(false, false));

  assert!(named.starts_with("var(--x"), "got `{}`", named);
  assert_ne!(named, "var(--xop34xu)");
}

/// A key far longer than anything an author writes still answers one name, and a
/// different one per key: the derivation hashes rather than copies.
#[test]
fn an_enormous_key_still_answers_one_name() {
  let long = "k".repeat(100_000);
  let named = var_group_member(BASE, PREFIX, &long, naming(false, false));

  assert!(
    named.len() < 32,
    "expected a hash, got {} bytes",
    named.len()
  );
  assert_ne!(
    named,
    var_group_member(BASE, PREFIX, &format!("{}k", long), naming(false, false))
  );
}

/// A key outside ASCII is hashed as written and made safe for the readable half
/// character by character.
#[test]
fn a_non_ascii_key_is_hashed_as_written_and_made_safe_to_read() {
  let plain = var_group_member(BASE, PREFIX, "ключ", naming(false, false));

  assert!(plain.starts_with("var(--x"), "got `{}`", plain);

  let readable = var_group_member(BASE, PREFIX, "ключ", naming(true, true));

  assert!(
    readable.starts_with("var(--____-"),
    "expected one underscore per non-ASCII character, got `{}`",
    readable
  );
}
