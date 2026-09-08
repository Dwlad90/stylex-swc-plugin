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

use crate::theme_ref::{VarNaming, var_group_member};

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

/// The two options as one value, and the two ways it is read.
mod var_naming {
  use stylex_structures::stylex_state_options::StyleXStateOptions;

  use crate::state_manager::StateManager;
  use crate::theme_ref::VarNaming;

  fn state(debug: bool, readable_names: bool) -> StateManager {
    StateManager::for_test(
      None,
      StyleXStateOptions::default()
        .with_debug(debug)
        .with_enable_debug_class_names(readable_names),
    )
  }

  /// The pair is read off the project once, and it is the two options as the
  /// project set them.
  #[test]
  fn the_pair_is_read_off_the_project_options() {
    for debug in [false, true] {
      for readable_names in [false, true] {
        assert_eq!(
          VarNaming::of(&state(debug, readable_names)).as_flags(),
          (debug, readable_names)
        );
      }
    }
  }

  /// The pair the engine carries reads back as the pair it was handed, which is
  /// what keeps the engine and the evaluator naming a member the same way.
  #[test]
  fn the_pair_survives_the_trip_through_the_engine() {
    for debug in [false, true] {
      for readable_names in [false, true] {
        assert_eq!(
          VarNaming::from_flags(debug, readable_names).as_flags(),
          (debug, readable_names)
        );
      }
    }
  }
}

/// A reference to a `defineVars` group, as the evaluator holds one.
mod theme_reference {
  use stylex_constants::constants::common::VAR_GROUP_HASH_KEY;
  use stylex_enums::theme_ref::ThemeRefResult;
  use stylex_structures::stylex_state_options::StyleXStateOptions;

  use crate::state_manager::StateManager;
  use crate::theme_ref::{IS_PROXY_KEY, ThemeRef, VarNaming, var_group_member};

  fn plain_state() -> StateManager {
    StateManager::for_test(None, StyleXStateOptions::default())
  }

  fn theme_ref() -> ThemeRef {
    ThemeRef::new("vars.stylex.js", "vars", "x")
  }

  /// The identity is the file and the export joined the way every name derived
  /// from the group is derived.
  #[test]
  fn the_identity_is_the_file_and_the_export() {
    assert_eq!(theme_ref().base_id(), "vars.stylex.js//vars");
  }

  #[test]
  fn the_prefix_is_kept_as_handed_in() {
    assert_eq!(theme_ref().class_name_prefix(), "x");
    assert_eq!(
      ThemeRef::new("vars.stylex.js", "vars", "").class_name_prefix(),
      ""
    );
  }

  /// `toString` is the one key that answers a bare name rather than a `var()`,
  /// and it is the same name `to_string_value` answers.
  #[test]
  fn the_to_string_key_answers_the_group_s_own_name() {
    let mut reference = theme_ref();

    let ThemeRefResult::ToString(name) = reference.get("toString", &plain_state()) else {
      panic!("`toString` did not answer the group's own name");
    };

    // The literal is the group hash of `BASE` under `PREFIX`, pinned at the top
    // of this file against Babel. Reading `to_string_value` for the expectation
    // would let both halves be wrong together.
    assert_eq!(name, "xop34xu");
    assert_eq!(name, reference.to_string_value());
  }

  /// The marker key answers that the value stands in for a group rather than
  /// holding one, which is how every reader tells a reference from an object.
  #[test]
  fn the_proxy_key_answers_that_this_stands_in_for_a_group() {
    let mut reference = theme_ref();

    assert!(matches!(
      reference.get(IS_PROXY_KEY, &plain_state()),
      ThemeRefResult::Proxy
    ));
  }

  /// Every other key answers the variable the derivation names, so a read
  /// through the reference and a read through the derivation agree.
  #[test]
  fn a_member_read_answers_the_variable_the_derivation_names() {
    let mut reference = theme_ref();
    let state = plain_state();

    // The literal is what `var_group_member` is pinned to against Babel at the
    // top of this file. Re-deriving it here would only prove the reference
    // calls the derivation, not that either one is right.
    assert_eq!(
      reference.get("primary", &state).as_css_var(),
      Some("var(--x1ineb92)")
    );
    assert_eq!(
      reference.get("primary", &state).as_css_var(),
      Some(var_group_member(super::BASE, super::PREFIX, "primary", VarNaming::of(&state)).as_str())
    );
  }

  /// The second read of one key answers the first read's own value: the name is
  /// derived once and kept.
  #[test]
  fn a_second_read_of_one_key_answers_the_first_read_s_value() {
    let mut reference = theme_ref();
    let state = plain_state();

    let Some(first) = reference
      .get("primary", &state)
      .as_css_var()
      .map(str::to_string)
    else {
      panic!("the first read named no variable");
    };
    let Some(second) = reference
      .get("primary", &state)
      .as_css_var()
      .map(str::to_string)
    else {
      panic!("the second read named no variable");
    };

    assert_eq!(first, second);
  }

  /// The group's hash key answers a bare name rather than a `var()`, because it
  /// names the group and not a variable of it.
  #[test]
  fn the_group_hash_key_answers_a_bare_name() {
    let mut reference = theme_ref();
    let state = plain_state();

    let answer = reference.get(VAR_GROUP_HASH_KEY, &state);
    let Some(name) = answer.as_css_var() else {
      panic!("the group hash key named nothing");
    };

    assert!(!name.starts_with("var("), "got `{}`", name);
    assert_eq!(name, "xop34xu");
    assert_eq!(name, reference.to_string_value());
  }

  /// A variable the author named is used as written, and reading it twice
  /// answers the same text -- it is derived from nothing, so there is nothing to
  /// keep between the reads.
  #[test]
  fn an_author_named_variable_is_used_as_written() {
    let mut reference = theme_ref();
    let state = plain_state();

    assert_eq!(
      reference.get("--brand-color", &state).as_css_var(),
      Some("var(--brand-color)")
    );
    assert_eq!(
      reference.get("--brand-color", &state).as_css_var(),
      Some("var(--brand-color)")
    );
  }

  /// A clone shares the derived names with the value it was cloned from, which
  /// is what makes the factory that hands out references cheap.
  #[test]
  fn a_clone_shares_the_names_already_derived() {
    let mut reference = theme_ref();
    let state = plain_state();

    let Some(original) = reference
      .get("primary", &state)
      .as_css_var()
      .map(str::to_string)
    else {
      panic!("the read named no variable");
    };

    let mut clone = reference.clone();

    assert_eq!(
      clone.get("primary", &state).as_css_var(),
      Some(original.as_str())
    );
  }

  /// The debug options reach the derivation through the state, so the same key
  /// under a debug project names a readable variable.
  #[test]
  fn the_project_options_reach_the_derivation() {
    let mut reference = theme_ref();
    let debug_state = StateManager::for_test(
      None,
      StyleXStateOptions::default()
        .with_debug(true)
        .with_enable_debug_class_names(true),
    );

    let answer = reference.get("primary", &debug_state);
    let Some(name) = answer.as_css_var() else {
      panic!("the read named no variable");
    };

    assert!(name.starts_with("var(--primary-x"), "got `{}`", name);
  }
}
