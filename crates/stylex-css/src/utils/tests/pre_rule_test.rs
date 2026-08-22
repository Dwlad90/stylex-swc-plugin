use crate::utils::pre_rule::{sort_at_rules, sort_pseudos};

/// The keys a case is written with, as `sort_pseudos` takes them.
///
/// Only here to keep a case's keys readable as the list it is: the cases below
/// are about the *order* keys come back in, and threading `String::from` through
/// each one buries that under conversion noise.
fn keys(keys: &[&str]) -> Vec<String> {
  keys.iter().map(|key| (*key).to_string()).collect()
}

// ── sort_pseudos ─────────────────────────────────────────────────────

#[test]
fn sort_pseudos_empty() {
  let result = sort_pseudos(&[]);
  assert!(result.is_empty());
}

#[test]
fn sort_pseudos_single_element() {
  let result = sort_pseudos(&[":hover".into()]);
  assert_eq!(result, vec![":hover"]);
}

#[test]
fn sort_pseudos_single_pseudo_element() {
  let result = sort_pseudos(&["::before".into()]);
  assert_eq!(result, vec!["::before"]);
}

#[test]
fn sort_pseudos_two_pseudo_classes_sorted() {
  let result = sort_pseudos(&[":hover".into(), ":active".into()]);
  // Two pseudo-classes should be grouped and sorted
  assert_eq!(result, vec![":active", ":hover"]);
}

#[test]
fn sort_pseudos_pseudo_element_starts_new_group() {
  let result = sort_pseudos(&["::before".into(), ":hover".into()]);
  // ::before starts a group, :hover starts another (since last group
  // has len==1 and starts with ::)
  assert_eq!(result, vec!["::before", ":hover"]);
}

#[test]
fn sort_pseudos_pseudo_element_followed_by_two_classes() {
  let result = sort_pseudos(&["::after".into(), ":hover".into(), ":focus".into()]);
  // ::after → group1, :hover starts group2, :focus joins group2
  // group2 gets sorted
  assert_eq!(result, vec!["::after", ":focus", ":hover"]);
}

#[test]
fn sort_pseudos_multiple_pseudo_elements() {
  let result = sort_pseudos(&["::before".into(), "::after".into()]);
  // Each :: starts its own group (len 1 each)
  assert_eq!(result, vec!["::before", "::after"]);
}

#[test]
fn sort_pseudos_classes_then_element() {
  let result = sort_pseudos(&[":hover".into(), ":focus".into(), "::before".into()]);
  // :hover → group1, :focus joins group1, ::before starts group2
  // group1 sorted, group2 is single
  assert_eq!(result, vec![":focus", ":hover", "::before"]);
}

#[test]
fn sort_pseudos_interleaved() {
  let result = sort_pseudos(&[
    ":hover".into(),
    "::before".into(),
    ":focus".into(),
    ":active".into(),
    "::after".into(),
  ]);
  // :hover → group1 (single), ::before → group2 (single),
  // :focus → group3, :active joins group3 → sorted,
  // ::after → group4
  assert_eq!(
    result,
    vec![":hover", "::before", ":active", ":focus", "::after"]
  );
}

#[test]
fn sort_pseudos_three_pseudo_classes_sort_as_one_run() {
  let result = sort_pseudos(&[":hover".into(), ":focus".into(), ":active".into()]);
  // One run of three, sorted whole -- not a sorted pair with the third
  // appended, which would read `:focus:hover:active`.
  assert_eq!(result, vec![":active", ":focus", ":hover"]);
}

#[test]
fn sort_pseudos_three_pseudo_classes_agree_in_every_nesting_order() {
  let sorted = vec![":active", ":focus", ":hover"];

  for permutation in [
    [":hover", ":focus", ":active"],
    [":hover", ":active", ":focus"],
    [":focus", ":hover", ":active"],
    [":focus", ":active", ":hover"],
    [":active", ":hover", ":focus"],
    [":active", ":focus", ":hover"],
  ] {
    assert_eq!(
      sort_pseudos(&keys(&permutation)),
      sorted,
      "nesting order {:?} sorted differently",
      permutation
    );
  }
}

#[test]
fn sort_pseudos_a_long_run_sorts_whole() {
  let input = keys(&[
    ":last-child",
    ":nth-child(2)",
    ":hover",
    ":focus",
    ":first-child",
    ":active",
    ":disabled",
  ]);

  assert_eq!(
    sort_pseudos(&input),
    vec![
      ":active",
      ":disabled",
      ":first-child",
      ":focus",
      ":hover",
      ":last-child",
      ":nth-child(2)"
    ]
  );
}

#[test]
fn sort_pseudos_an_element_splits_a_run_in_two() {
  let input = keys(&[":hover", ":focus", ":active", "::before", ":c", ":b", ":a"]);

  // Each side of the element sorts on its own; the element itself never moves,
  // because it names a position rather than a state.
  assert_eq!(
    sort_pseudos(&input),
    vec![":active", ":focus", ":hover", "::before", ":a", ":b", ":c"]
  );
}

#[test]
fn sort_pseudos_an_attribute_selector_joins_the_run() {
  let input = keys(&[":hover", "[data-x]", ":active"]);

  // An attribute selector is not a pseudo element, so it sorts with the run it
  // sits in -- and `[` sorts after `:`.
  assert_eq!(sort_pseudos(&input), vec![":active", ":hover", "[data-x]"]);
}

// ── sort_at_rules ────────────────────────────────────────────────────

#[test]
fn sort_at_rules_empty() {
  let result = sort_at_rules(&[]);
  assert!(result.is_empty());
}

#[test]
fn sort_at_rules_single() {
  let result = sort_at_rules(&["@media (max-width: 600px)".into()]);
  assert_eq!(result, vec!["@media (max-width: 600px)"]);
}

#[test]
fn sort_at_rules_default_first() {
  let result = sort_at_rules(&[
    "@media (max-width: 600px)".into(),
    "default".into(),
    "@supports (display: grid)".into(),
  ]);
  assert_eq!(result[0], "default");
}

#[test]
fn sort_at_rules_alphabetical_without_default() {
  let result = sort_at_rules(&[
    "@supports (display: grid)".into(),
    "@media (max-width: 600px)".into(),
  ]);
  assert_eq!(result[0], "@media (max-width: 600px)");
  assert_eq!(result[1], "@supports (display: grid)");
}

#[test]
fn sort_at_rules_already_sorted() {
  let result = sort_at_rules(&[
    "default".into(),
    "@media (max-width: 600px)".into(),
    "@supports (display: grid)".into(),
  ]);
  assert_eq!(result[0], "default");
  assert_eq!(result[1], "@media (max-width: 600px)");
  assert_eq!(result[2], "@supports (display: grid)");
}

#[test]
fn sort_at_rules_multiple_media() {
  let result = sort_at_rules(&[
    "@media (min-width: 800px)".into(),
    "@media (max-width: 600px)".into(),
  ]);
  assert_eq!(result[0], "@media (max-width: 600px)");
  assert_eq!(result[1], "@media (min-width: 800px)");
}

// ── pseudo_comparator ────────────────────────────────────────────────
//
// The comparator a run is sorted with. `sort_pseudos` above is about which keys
// sort together; these are about the order they come back in, taken directly so
// a case does not have to build a run to ask a question about two strings.
//
// Every ordering asserted here was read out of `@stylexjs/babel-plugin` 0.19.0
// under the parity harness's options, including the ones marked as divergent --
// those were measured too, and disagree.

use crate::utils::pre_rule::pseudo_comparator;
use std::cmp::Ordering;

/// `a` before `b`, and `b` after `a` -- both directions, because a comparator
/// that answered `Less` to each of a pair would sort by insertion order and
/// pass every one-directional case.
fn precedes(a: &str, b: &str) {
  assert_eq!(pseudo_comparator(a, b), Ordering::Less, "{a} before {b}");
  assert_eq!(pseudo_comparator(b, a), Ordering::Greater, "{b} after {a}");
}

#[test]
fn a_letter_outranks_its_case() {
  // The primary pass: `HOVER` weighs as `hover`, so it lands between `active`
  // and `italic` rather than below both of them.
  precedes(":active", ":HOVER");
  precedes(":HOVER", ":italic");
}

#[test]
fn case_decides_only_when_the_letters_tie() {
  precedes(":a", ":A");
  precedes(":hover", ":HOVER");
  precedes(":hOver", ":HOver");
}

#[test]
fn the_case_tiebreak_reads_the_first_differing_position() {
  // `:aBc` and `:AbC` differ in case at all three letters. Position one decides
  // and the rest are never read, which is what makes this a tiebreak rather
  // than a count.
  precedes(":aBc", ":AbC");
  precedes(":aBC", ":Abc");
}

#[test]
fn length_settles_a_tie_before_case_does() {
  // A key that runs out of characters has run out of letters to weigh, so the
  // case difference further along is never reached.
  precedes(":a", ":aB");
  precedes(":a", ":ab");
}

#[test]
fn every_symbol_outranks_every_letter_whatever_its_byte() {
  // The primary pass is a table, not a byte comparison, and this is where the
  // two part company hardest: `{ | } ~` are above `z` by byte and below every
  // letter by weight. `[ \ ] ^ _ ` ` happen to agree with their bytes; both
  // groups are here so a reader cannot mistake the agreeing half for the rule.
  for symbol in [
    "[", "\\", "]", "^", "_", "`", "{", "|", "}", "~", "@", "$", "+", "<", "=", ">",
  ] {
    precedes(&format!(":{symbol}"), ":Z");
    precedes(&format!(":{symbol}"), ":z");
    precedes(&format!(":{symbol}"), ":0");
  }
}

#[test]
fn the_symbols_are_not_in_byte_order_among_themselves() {
  // `_` before `-` is the pair that says the table is doing the work: their
  // bytes are 0x5F and 0x2D, so a byte comparison puts them the other way. `$`
  // trailing every other symbol is the same statement at the far end.
  precedes(":_a", ":-a");
  precedes(":~", ":$");
  precedes(":,", ":;");
  precedes("[data_x]", "[data-x]");
}

#[test]
fn a_digit_outranks_nothing_a_letter_outranks() {
  precedes(":1", ":A");
  precedes(":1", ":a");
  precedes(":0", ":9");
  // And every symbol outranks every digit, which byte order gets right below
  // `0x30` and wrong above it.
  precedes(":;", ":1");
  precedes(":@", ":1");
  precedes(":?", ":1");
}

#[test]
fn equal_keys_compare_equal() {
  // Not reachable through `sort_pseudos` -- a repeated condition key is refused
  // before the sort -- but a comparator that answered anything else here would
  // make the sort's result depend on the input order.
  assert_eq!(pseudo_comparator(":hover", ":hover"), Ordering::Equal);
  assert_eq!(pseudo_comparator("", ""), Ordering::Equal);
  assert_eq!(pseudo_comparator(":ä", ":ä"), Ordering::Equal);
}

#[test]
fn an_empty_key_precedes_every_other() {
  precedes("", ":");
  precedes(":", ":a");
}

#[test]
fn the_ordering_is_transitive_across_the_cases_and_the_block() {
  // The three levels the comparator has, chained: punctuation, then digits,
  // then a letter in each case. A comparator whose passes disagreed would break
  // the chain somewhere in the middle and `sort_unstable_by` would be free to
  // produce anything.
  // The bare colon leads on length rather than on weight -- it is a prefix of
  // every other key here, and length settles a tie before anything else does.
  let ascending = [
    ":", ":_", ":-", ":,", ":;", ":!", ":?", ":.", ":@", ":~", ":$", ":0", ":9", ":a", ":A", ":b",
    ":B", ":z", ":Z",
  ];

  for (index, earlier) in ascending.iter().enumerate() {
    for later in &ascending[index + 1..] {
      precedes(earlier, later);
    }
  }
}

#[test]
fn a_sort_over_the_whole_ordering_is_the_ordering() {
  let mut shuffled = keys(&[
    ":Z", ":a", ":~", ":!", ":B", ":0", ":A", ":z", ":_", ":9", ":b", ":",
  ]);
  shuffled.sort_unstable_by(|a, b| pseudo_comparator(a, b));

  assert_eq!(
    shuffled,
    keys(&[
      ":", ":_", ":!", ":~", ":0", ":9", ":a", ":A", ":b", ":B", ":z", ":Z"
    ])
  );
}

#[test]
fn nothing_outside_printable_ascii_is_weighed() {
  // The documented divergence, asserted as it stands rather than as it should
  // be. Root collation gives `ä` the primary weight of `a` and weighs an emoji
  // below every letter; here every byte the table does not name sorts above
  // every one it does, which is one rule producing all of these.
  precedes(":z", ":ä");
  precedes(":hover", ":\u{1F389}");
  // And its two cases are two unrelated keys rather than a tie: `Ä` is `0xC3
  // 0x84` and `ä` is `0xC3 0xA4`, so the byte decides where the case would.
  precedes(":Ä", ":ä");
  // A control character is in the same class, and it is the reason an unnamed
  // byte keeps its own weight rather than sharing one: two of them must not
  // compare `Equal`, which `sort_unstable_by` would be free to resolve either
  // way.
  precedes(":\u{0001}", ":\u{0002}");
  precedes(":hover", ":\u{007f}");
}

#[test]
fn a_run_of_case_differing_keys_sorts_whole() {
  // Through `sort_pseudos` rather than the comparator, so the two are known to
  // be wired together.
  assert_eq!(
    sort_pseudos(&keys(&[":HOVER", ":active", ":Focus"])),
    keys(&[":active", ":Focus", ":HOVER"])
  );
}

#[test]
fn a_pseudo_element_still_pins_its_position_under_the_new_comparator() {
  assert_eq!(
    sort_pseudos(&keys(&[":HOVER", "::before", ":Active", ":hover"])),
    keys(&[":HOVER", "::before", ":Active", ":hover"])
  );
}

// ── the at-rule comparator is a different one ────────────────────────

#[test]
fn at_rules_sort_by_their_bytes_rather_than_by_their_letters() {
  // Upstream sorts pseudo keys with `localeCompare` and at-rules with a bare
  // `.sort()`, so this is the one that must *not* fold case: `M` below `m` is
  // the compatible answer here and the wrong one one function up.
  let result = sort_at_rules(&keys(&[
    "@media (min-width: 1px)",
    "@media (MIN-WIDTH: 1px)",
  ]));

  assert_eq!(
    result,
    keys(&["@media (MIN-WIDTH: 1px)", "@media (min-width: 1px)"])
  );
}

#[test]
fn the_two_comparators_answer_the_same_pair_differently() {
  // Stated as one assertion so the split cannot be quietly undone: whichever
  // comparator either function reaches for, they are not the same one.
  assert_eq!(pseudo_comparator(":a", ":A"), Ordering::Less);
  assert_eq!(sort_at_rules(&keys(&[":a", ":A"])), keys(&[":A", ":a"]));
}

#[test]
fn a_non_ascii_at_rule_sorts_by_its_code_points() {
  // UTF-8 bytes and UTF-16 code units are both code-point order through the
  // basic multilingual plane, so the encoding difference between the two
  // compilers is not reachable here.
  assert_eq!(
    sort_at_rules(&keys(&["@supports (--z: 1)", "@supports (--\u{00fc}: 1)"])),
    keys(&["@supports (--z: 1)", "@supports (--\u{00fc}: 1)"])
  );
}

#[test]
fn default_leads_the_at_rules_from_either_side() {
  // Unreachable from a key path, which filters to `@`-prefixed keys before
  // `sort_at_rules` is called, and asserted anyway: the branch exists and a
  // reader deleting it should have to delete a case that says what it did.
  assert_eq!(
    sort_at_rules(&keys(&["@media (min-width: 1px)", "default"])),
    keys(&["default", "@media (min-width: 1px)"])
  );
  assert_eq!(
    sort_at_rules(&keys(&["default", "@media (min-width: 1px)"])),
    keys(&["default", "@media (min-width: 1px)"])
  );
}

// ── degenerate and hostile inputs ────────────────────────────────────

#[test]
fn a_malformed_key_sorts_where_its_characters_put_it() {
  // Neither compiler validates a condition key as CSS. An unclosed bracket,
  // paren or quote is a string to sort like any other, and the case that would
  // be alarming is one where it moved.
  assert_eq!(
    sort_pseudos(&keys(&[":hover", "[data-x", ":not(.a", "[data-x=\"y]"])),
    keys(&[":hover", ":not(.a", "[data-x", "[data-x=\"y]"])
  );
}

#[test]
fn a_key_of_nothing_but_combining_marks_sorts_above_ascii() {
  // A combining acute with no base character. It is not ASCII, so it is not
  // weighed -- the same rule as every other non-ASCII key, applied to one that
  // is not a letter at all. Divergent: upstream weighs a lone mark below every
  // letter and spells this `:\u{0301}:hover`, `xcdw69q`.
  precedes(":hover", ":\u{0301}");
}

#[test]
fn a_lone_surrogate_cannot_reach_the_comparator() {
  // A `String` cannot hold one, so the shape a JavaScript condition key could
  // carry and this comparator could not is unreachable by construction rather
  // than by a check. Stated as the replacement character, which is what a
  // decoder hands over instead -- and which upstream also sorts last, so this
  // is one non-ASCII key the two agree on.
  precedes(":hover", ":\u{FFFD}");
}

#[test]
fn a_very_long_key_compares_on_its_first_difference() {
  // Linear in the shorter key and short-circuiting, so a long key costs nothing
  // it does not have to. Two five-thousand-character keys differing at position
  // one is the shape that would be slow if it were not.
  let left = format!(":a{}", "z".repeat(5_000));
  let right = format!(":b{}", "z".repeat(5_000));

  precedes(&left, &right);
}

#[test]
fn a_run_of_a_thousand_keys_sorts_whole() {
  // Wider than the nesting ceiling admits, so it is not reachable through a
  // stylesheet -- but `sort_pseudos` is a public function over a slice, and a
  // width limit is not one of the things it has.
  let mut written: Vec<String> = (0..1_000)
    .rev()
    .map(|index| format!(":p{index:04}"))
    .collect();
  let expected: Vec<String> = (0..1_000).map(|index| format!(":p{index:04}")).collect();

  written = sort_pseudos(&written);

  assert_eq!(written, expected);
}
