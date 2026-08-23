//! The order a selector's pseudo keys are read in, and the class name that
//! order hashes.
//!
//! A class name is a hash of the dashed property, the value, and the *modifier*
//! string -- the sorted pseudo keys joined, then the sorted at-rules joined. So
//! the pseudo order is not cosmetic: two compilers that sort the same list
//! differently name different classes for one authored style, and markup built
//! by one names a class the other's stylesheet never defines.
//!
//! `sort_pseudos` sorts *runs*: a pseudo element pins its position, because it
//! names which part of the element the rule targets rather than a state the
//! element is in, and the keys on either side of one sort among themselves.
//! Sorting each pair as it arrives and appending the next key agrees with
//! sorting the run at one and two keys and diverges from three on, which is why
//! nothing before this file measured it -- a pair sorted from either nesting
//! order lands in the same place, and every authored style in the suites nested
//! two.
//!
//! The emitted *selector* is not the sorted list verbatim: the pseudo classes
//! print in sorted order and the pseudo elements print after all of them. Both
//! halves are asserted here, since the hash reads the sorted list and only the
//! rule text shows where the elements went.
//!
//! Every case in this file was measured against `@stylexjs/babel-plugin` 0.19.0
//! under the parity harness's options and agrees with it on class names and rule
//! text -- including the degenerate keys, which neither compiler validates as
//! CSS and both spell into a selector unchanged.
//!
//! The comparator is the *second* thing this file measures, and it is a separate
//! mechanism from the run grouping above: it is visible at two keys, where no
//! grouping question exists at all. Upstream compares with `localeCompare`;
//! `pseudo_comparator` reproduces that ordering over printable ASCII from a
//! weight table read out of `localeCompare` itself rather than from byte order,
//! and hands every other key to root collation. Every case below agrees with
//! 0.19.0 -- letters weighed without their case, symbols below digits below
//! letters whatever their bytes, the symbols not in byte order among themselves,
//! an accented letter beside its base letter, and a character root collation
//! does not weigh carrying no weight here either.
//!
//! The last group used to be the exception. A control character, `DEL` and every
//! non-ASCII character ranked above all of printable ASCII, which cost a class
//! name -- the last divergence in the parity harness that did. Those cases are
//! still here, still measured against 0.19.0, and now agree; what they pin is
//! the behaviour rather than the gap.
//!
//! Every class name quoted below as upstream's was read out of
//! `@stylexjs/babel-plugin` 0.19.0 under the parity harness's options, not
//! inferred.

use crate::utils::{prelude::*, transform::stringify_js, transform::ts_syntax};

/// The transform the generated cases run under.
///
/// Runtime injection is what puts the rule *text* in the output, and the
/// selector is the whole question here -- without it a generated case could only
/// assert the class name, which says two spellings differ without saying how.
/// The `stylex_test!` cases get the same thing from the macro's default arm.
fn injecting_transform(comments: TestComments) -> impl Pass {
  build_test_transform(comments, |b| b.with_runtime_injection())
}

// ──────────────────────────────────────────────
// The reported shape
// ──────────────────────────────────────────────

// Three pseudo-classes nested in an order that is not already alphabetical.
// The leaf selector is `:active:focus:hover` and the class name is `x12rlomf`,
// which is what the reference implementation names it.
stylex_test!(
  three_pseudo_classes_nested_out_of_order,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: {
        zIndex: {
          default: '1',
          ':hover': { default: '1', ':focus': { default: '1', ':active': '1' } },
        },
      },
    });
  "#
);

// All six nesting orders of the same three pseudo-classes, in one module. The
// nesting order is the author's; the class name must not be, so all six leaves
// have to name one class -- and a module that emits it once proves they do,
// because a second spelling would inject a second rule.
#[test]
fn every_nesting_order_of_three_pseudo_classes_names_one_class() {
  let orders = [
    (":hover", ":focus", ":active"),
    (":hover", ":active", ":focus"),
    (":focus", ":hover", ":active"),
    (":focus", ":active", ":hover"),
    (":active", ":hover", ":focus"),
    (":active", ":focus", ":hover"),
  ];

  let namespaces = orders
    .iter()
    .enumerate()
    .map(|(index, (outer, middle, inner))| {
      format!(
        "n{}: {{ zIndex: {{ '{}': {{ '{}': {{ '{}': '1' }} }} }} }},",
        index, outer, middle, inner
      )
    })
    .collect::<Vec<_>>()
    .join("\n");

  let input = format!(
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const styles = stylex.create({{
        {}
      }});
    "#,
    namespaces
  );

  let output = stringify_js(&input, ts_syntax(), |tr| {
    injecting_transform(tr.comments.clone())
  });

  // One rule, named once, for six authored nesting orders.
  assert_eq!(
    output
      .matches(".x12rlomf:active:focus:hover{z-index:1}")
      .count(),
    1,
    "the six nesting orders did not collapse to one rule:\n{}",
    output
  );
  // And no rule spells the leaf in any other order.
  assert!(
    !output.contains(":focus:hover:active"),
    "a leaf still sorted pairwise:\n{}",
    output
  );
  for index in 0..orders.len() {
    assert!(
      output.contains(&format!("\"n{}\"", index)) || output.contains(&format!("n{}:", index)),
      "namespace n{} is missing from the output:\n{}",
      index,
      output
    );
  }
}

// Four, nested in reverse alphabetical order: the run is sorted at whatever
// length it reached, so the length is not a boundary the sort has.
stylex_test!(
  four_pseudo_classes_nested_in_reverse_order,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':last-child': { ':hover': { ':focus': { ':active': 'red' } } } } },
    });
  "#
);

// ──────────────────────────────────────────────
// Where a pseudo element falls
//
// An element pins its position in the *sort*, and prints after every class in
// the selector. Two separate facts, and a run on each side of one is what tells
// them apart.
// ──────────────────────────────────────────────

// Three classes, an element, three more classes. Each side sorts on its own --
// `:active:focus:hover` then `:a:b:c` -- and the element prints last.
stylex_test!(
  a_pseudo_element_splits_two_runs_of_three,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: {
        color: {
          ':hover': {
            ':focus': {
              ':active': { '::before': { ':c': { ':b': { ':a': 'red' } } } },
            },
          },
        },
      },
    });
  "#
);

// The element outermost, so there is one run and it is the whole list after it.
stylex_test!(
  a_leading_pseudo_element_then_three_classes,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { '::before': { ':hover': { ':focus': { ':active': 'red' } } } } },
    });
  "#
);

// The element innermost. The run ends where the element begins, so the three
// classes ahead of it sort and the element still prints last.
stylex_test!(
  three_classes_then_a_trailing_pseudo_element,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':hover': { ':focus': { ':active': { '::before': 'red' } } } } },
    });
  "#
);

// Two adjacent elements between two runs. Neither element moves relative to the
// other, and the run after them is not the run before them: the first key sits
// alone ahead of the pair and does not sort with the two behind it.
stylex_test!(
  two_adjacent_pseudo_elements_between_runs,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: {
        color: {
          ':hover': {
            '::before': { '::first-line': { ':focus': { ':active': 'red' } } },
          },
        },
      },
    });
  "#
);

// The legacy single-colon spelling of a pseudo element. One colon is one colon:
// it reads as a pseudo class, joins the run, and sorts into it -- which is what
// the reference implementation does with it too, so the compatible answer is the
// one that does not special-case the legacy names.
stylex_test!(
  a_legacy_single_colon_element_sorts_as_a_class,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':hover': { ':before': { ':active': 'red' } } } },
    });
  "#
);

// ──────────────────────────────────────────────
// Keys that are neither class nor element
// ──────────────────────────────────────────────

// An attribute selector is not a pseudo element, so it joins the run it sits in
// and sorts with it. `[` sorts after `:`, so it lands last among the three.
stylex_test!(
  an_attribute_selector_sorts_inside_the_run,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':hover': { '[data-x]': { ':active': 'red' } } } },
    });
  "#
);

// A run of nothing but attribute selectors, nested in reverse order.
stylex_test!(
  three_attribute_selectors_nested_in_reverse_order,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { '[data-c]': { '[data-b]': { '[data-a]': 'red' } } } },
    });
  "#
);

// ──────────────────────────────────────────────
// At-rules around the run
// ──────────────────────────────────────────────

// At-rules interleaved between the pseudo-classes. The two kinds are filtered
// out of the key path separately, so an at-rule between two pseudo-classes does
// not split the run -- the three still sort as one, and the at-rules sort into
// their own string.
stylex_test!(
  at_rules_between_pseudo_classes_do_not_split_the_run,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: {
        color: {
          ':hover': {
            '@media (min-width: 1px)': {
              ':focus': { '@supports (color: red)': { ':active': 'red' } },
            },
          },
        },
      },
    });
  "#
);

// ──────────────────────────────────────────────
// Degenerate keys
//
// Neither compiler validates a condition key as CSS: whatever the author wrote
// after the colon is spelled into the selector unchanged, and it is the *sort*
// that has to agree. These are the keys that decide it -- the ones whose first
// bytes are not a letter.
// ──────────────────────────────────────────────

// Functional pseudo-classes carrying commas, spaces and parentheses. They sort
// by their whole text, so `:is(` sorts ahead of `:not(` ahead of `:nth-`.
stylex_test!(
  functional_pseudo_classes_sort_by_their_whole_text,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':not(.a, .b)': { ':nth-child(2n + 1)': { ':is(.x)': 'red' } } } },
    });
  "#
);

// A non-ASCII pseudo name that lands last for a reason that has nothing to do
// with being non-ASCII: root collation weighs `ü` as `u`, so `:ünïcödé` weighs as
// `unicode` and sorts after `:active` and `:hover` on the letters alone. The
// snapshot did not move when the byte ranking was replaced by collation, which is
// what makes this the control case for the three below it.
stylex_test!(
  a_non_ascii_pseudo_name_sorts_last,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':hover': { ':ünïcödé': { ':active': 'red' } } } },
    });
  "#
);

// A CSS escape sequence. The backslash sorts below every letter, so the escaped
// name leads the run.
stylex_test!(
  an_escaped_pseudo_name_leads_the_run,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':hover': { ':\\31 23': { ':active': 'red' } } } },
    });
  "#
);

// A bare colon, which names nothing. It is still a single colon, so it is a
// class, and the empty name sorts ahead of every other -- printing a doubled
// colon into the selector, which is what the reference implementation prints
// too.
stylex_test!(
  a_bare_colon_sorts_first,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':hover': { ':': { ':active': 'red' } } } },
    });
  "#
);

// Three colons. The `::` prefix test is a prefix test, so this reads as a pseudo
// *element*: it splits the run and prints after the classes.
stylex_test!(
  a_triple_colon_reads_as_a_pseudo_element,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':hover': { ':::x': { ':active': 'red' } } } },
    });
  "#
);

// An uppercase pseudo name. Nothing lowercases a condition key, so `:HOVER`
// reaches the comparator spelled as the author wrote it -- and it sorts by the
// letters, not by their bytes: `:active:focus:HOVER`, `xnnn07p`, which is what
// the reference implementation names it. A byte comparison put every uppercase
// letter below every lowercase one and spelled this `:HOVER:active:focus`.
stylex_test!(
  an_uppercase_pseudo_name_sorts_by_its_letters,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':HOVER': { ':focus': { ':active': 'red' } } } },
    });
  "#
);

// ──────────────────────────────────────────────
// Malformed selector syntax
//
// An unclosed bracket, paren or quote makes the key invalid CSS and neither
// compiler notices: the key is a string to sort and a string to print. What is
// pinned here is that a malformed key does not change *where* it sorts.
// ──────────────────────────────────────────────

stylex_test!(
  an_unclosed_attribute_bracket_still_sorts_in_place,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':hover': { '[data-x': { ':active': 'red' } } } },
    });
  "#
);

stylex_test!(
  an_unclosed_functional_paren_still_sorts_in_place,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':hover': { ':not(.a': { ':active': 'red' } } } },
    });
  "#
);

stylex_test!(
  an_unclosed_attribute_quote_still_sorts_in_place,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':hover': { '[data-x="y]': { ':active': 'red' } } } },
    });
  "#
);

// ──────────────────────────────────────────────
// Boundary conditions
//
// Generated rather than written out, because the point is the size. Both are
// measured against Babel 0.19.0 at the same sizes, which accepts them too.
// ──────────────────────────────────────────────

// A run as wide as the evaluator will carry, nested in reverse alphabetical
// order. One run, sorted whole, and the selector is the names in order.
//
// The width is bounded by the nesting ceiling rather than by anything the sort
// has: at 32 levels the evaluator refuses the object before a selector is
// assembled, which is `evaluation_depth_budget.rs`'s question and not this
// file's. 24 leaves room under it.
#[test]
fn a_run_as_wide_as_the_nesting_ceiling_sorts_whole() {
  const WIDTH: usize = 24;

  // `:p00` .. `:p23`, nested from the highest down, so no prefix of the nesting
  // order is already sorted.
  let names: Vec<String> = (0..WIDTH).map(|index| format!(":p{:02}", index)).collect();

  let mut nested = String::from("'red'");
  for name in names.iter() {
    nested = format!("{{ '{}': {} }}", name, nested);
  }

  let input = format!(
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const styles = stylex.create({{ w: {{ color: {} }} }});
    "#,
    nested
  );

  let output = stringify_js(&input, ts_syntax(), |tr| {
    injecting_transform(tr.comments.clone())
  });

  let sorted_selector = names.join("");
  assert!(
    output.contains(&format!("{}{{color:red}}", sorted_selector)),
    "the {WIDTH}-key run did not sort whole:\n{}",
    output
  );
}

// The same width with a pseudo element in the middle. The element splits the run
// in two, so the selector is the first half sorted, the second half sorted, and
// the element after both -- not one sorted list of the whole width.
#[test]
fn a_pseudo_element_splits_a_wide_run() {
  const HALF: usize = 12;

  let leading: Vec<String> = (0..HALF).map(|index| format!(":a{:02}", index)).collect();
  let trailing: Vec<String> = (0..HALF).map(|index| format!(":b{:02}", index)).collect();

  let mut nested = String::from("'red'");
  for name in trailing.iter() {
    nested = format!("{{ '{}': {} }}", name, nested);
  }
  nested = format!("{{ '::before': {} }}", nested);
  for name in leading.iter() {
    nested = format!("{{ '{}': {} }}", name, nested);
  }

  let input = format!(
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const styles = stylex.create({{ w: {{ color: {} }} }});
    "#,
    nested
  );

  let output = stringify_js(&input, ts_syntax(), |tr| {
    injecting_transform(tr.comments.clone())
  });

  let expected = format!(
    "{}{}::before{{color:red}}",
    leading.join(""),
    trailing.join("")
  );
  assert!(
    output.contains(&expected),
    "the element did not split the run:\n{}",
    output
  );
}

// A single pseudo-class name five thousand characters long. The name is sorted
// and hashed whole, so nothing truncates it on the way into the selector.
#[test]
fn a_five_thousand_character_pseudo_name_sorts_whole() {
  let long = "z".repeat(5000);

  let input = format!(
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const styles = stylex.create({{
        w: {{ color: {{ ':hover': {{ ':{}': {{ ':active': 'red' }} }} }} }},
      }});
    "#,
    long
  );

  let output = stringify_js(&input, ts_syntax(), |tr| {
    injecting_transform(tr.comments.clone())
  });

  // `z` is above `a` and `h`, so the long name sorts last of the three.
  assert!(
    output.contains(&format!(":active:hover:{}{{color:red}}", long)),
    "the long pseudo name did not sort last, or did not survive whole"
  );
}

// The same key three times over. Both compilers refuse a repeated condition key
// before the sort is reached, so a run can never hold a duplicate -- which is
// why the sort never has to be stable.
#[test]
#[should_panic(expected = "The same pseudo selector or at-rule cannot be used more than once")]
fn a_repeated_pseudo_class_is_refused_before_the_sort() {
  let input = r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':hover': { ':hover': { ':hover': 'red' } } } },
    });
  "#;

  stringify_js(input, ts_syntax(), |tr| {
    injecting_transform(tr.comments.clone())
  });
}

// Two keys, one of them uppercase. Nothing here is three deep and nothing here
// is a run the grouping fix changed: the pair moved too, in either nesting
// order, which is what makes the comparator a separate mechanism from the
// grouping. `:active:HOVER`, `xyhlusd`, agreeing with the reference
// implementation. Kept beside the three-key case so the two are read together.
stylex_test!(
  two_pseudo_names_differing_only_in_case,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':HOVER': { ':active': 'red' } } },
    });
  "#
);

// ──────────────────────────────────────────────
// The comparator
//
// `pseudo_comparator` is the ASCII half of the ordering `localeCompare`
// reaches: a letter's case is a tiebreak rather than its identity, and the
// tiebreak puts lowercase first. These cases pin both passes, the boundary
// between them, and the half that is not covered.
// ──────────────────────────────────────────────

// The tiebreak alone: two keys whose letters are identical and whose cases are
// not. Nothing separates them until the case pass, and lowercase leads.
stylex_test!(
  one_pseudo_name_in_each_case_sorts_lowercase_first,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':HOVER': { ':hover': 'red' } } },
    });
  "#
);

// A single uppercase letter against its lowercase self, which is the smallest
// input the tiebreak has.
stylex_test!(
  a_single_letter_in_each_case_sorts_lowercase_first,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':A': { ':a': 'red' } } },
    });
  "#
);

// The tiebreak reads the *first* position where the cases differ, not the last
// and not a count of them. `:aBc` and `:AbC` agree on every letter and differ
// in case at all three; position one decides, so the key that is lowercase
// there leads.
stylex_test!(
  the_case_tiebreak_is_decided_by_the_first_position,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':AbC': { ':aBc': 'red' } } },
    });
  "#
);

// Length settles a tie before case does, because a key that runs out of
// characters has run out of letters to weigh: `:a` leads `:aB` even though the
// case difference further along would have put `:aB` first if it were read.
stylex_test!(
  a_shorter_key_leads_its_own_prefix_extension,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':aB': { ':a': 'red' } } },
    });
  "#
);

// An uppercase letter against a character from the block that sits between the
// two ASCII cases -- `[`, `\`, `]`, `^`, `_`, backtick. `:Z` and `:_leading`
// are what decide it, and a byte comparison put `:Z` first because `Z` is
// `0x5A` and `_` is `0x5F`; weighing the letter rather than its byte puts
// `:_leading` first, which is where upstream puts it. `[data-x]` is settled at
// the first character by `:` against `[` and lands last under either
// comparator, which is what makes it the control here.
// `.x1f04poe:_leading:Z[data-x]`, measured. The general form of the same
// statement -- every symbol below every letter, whatever its byte -- is
// `a_symbol_whose_byte_is_above_z_still_sorts_below_it` below.
stylex_test!(
  an_uppercase_letter_sorts_above_the_block_between_the_cases,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':Z': { '[data-x]': { ':_leading': 'red' } } } },
    });
  "#
);

// Digits rank below letters in either pass and in either case, so an uppercase
// letter does not fall below one.
stylex_test!(
  a_digit_sorts_below_an_uppercase_letter,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':A': { ':1': 'red' } } },
    });
  "#
);

// A mixed-case functional pseudo-class. The parenthesis and its contents are
// part of the key's text and weigh with it, so the argument decides when the
// name does not.
stylex_test!(
  a_mixed_case_functional_pseudo_class_sorts_by_its_whole_text,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':NOT(.b)': { ':not(.a)': { ':Is(.c)': 'red' } } } },
    });
  "#
);

// A mixed-case attribute selector, which is not a pseudo class and sorts in the
// run all the same.
stylex_test!(
  a_mixed_case_attribute_selector_sorts_by_its_letters,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { '[DATA-x]': { '[data-X]': { ':hover': 'red' } } } },
    });
  "#
);

// A symbol whose byte is above `z`. The primary pass is a table rather than a
// byte comparison, so `~` weighs below every letter although `0x7E` is above
// every one -- `:~:z`, `x8m0m4e`-shaped, and it is what upstream names too. A
// byte comparison spelled this `:z:~`, which was the pre-existing divergence
// this half of the comparator closes.
stylex_test!(
  a_symbol_whose_byte_is_above_z_still_sorts_below_it,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':z': { ':~': 'red' } } },
    });
  "#
);

// The same statement against the digits: `@` is `0x40` and `1` is `0x31`, so a
// byte comparison put the digit first. Every symbol weighs below every digit.
stylex_test!(
  a_symbol_whose_byte_is_above_a_digit_still_sorts_below_it,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':1': { ':@': 'red' } } },
    });
  "#
);

// The symbols are not in byte order among themselves either. `_` is `0x5F` and
// `-` is `0x2D`, so a byte comparison puts the hyphen first; the table puts the
// underscore first, which is what upstream does. Reachable by an author without
// trying: two attribute selectors on sibling data attributes.
stylex_test!(
  two_attribute_selectors_differing_by_underscore_and_hyphen,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { '[data-x]': { '[data_x]': 'red' } } },
    });
  "#
);

// An attribute selector carrying a matcher, which is ordinary CSS and puts a
// symbol at the position that decides the comparison.
stylex_test!(
  an_attribute_matcher_sorts_by_its_symbol_weight,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { '[ay]': { '[a~=b]': { '[a|=c]': 'red' } } } },
    });
  "#
);

// A control character, which the table does not name. It ranks above every
// character the table does name, where root collation does not weigh it at all
// -- the same rule as the non-ASCII keys below, and the reason an unnamed byte
// keeps a weight of its own rather than sharing one.
stylex_test!(
  a_control_character_pseudo_name_carries_no_weight,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':hover': { ':\u{0001}': 'red' } } },
    });
  "#
);

// The reported shape of the divergence that is now closed. Root collation gives
// `ä` the primary weight of `a`, so `:ä` leads `:z` and the pair names
// `x1enrlzn`. It used to name `x143q076`, because every byte at or above `0x80`
// ranked above every ASCII character; `pre_rule.rs` records what closing that
// cost and what it left.
stylex_test!(
  an_accented_pseudo_name_sorts_beside_its_base_letter,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':ä': { ':z': 'red' } } },
    });
  "#
);

// Case folding is no longer ASCII-only. `:ä` and `:Ä` tie on the letter through
// both the primary and the secondary pass, separate on the tertiary one with the
// lowercase first, and name `xgvn8d` -- the same rule `:a` before `:A` follows.
// They used to sort by their bytes and name `x1th3k6m`.
stylex_test!(
  a_non_ascii_letter_in_each_case_ties_on_the_letter,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':ä': { ':Ä': 'red' } } },
    });
  "#
);

// An emoji, which was the third face of the same divergence and never an
// encoding question. Root collation weighs a symbol *below* every letter, so
// this spells `:\u{1F389}:hover` and names `x1jqz5xw`. It used to land last --
// `:hover:\u{1F389}`, `x17d4qyr` -- and it is kept beside the accented names so
// the rule reads as one: what root collation weighs, this weighs.
stylex_test!(
  a_supplementary_character_pseudo_name_sorts_below_every_letter,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':\u{1F389}': { ':hover': 'red' } } },
    });
  "#
);

// ──────────────────────────────────────────────
// The at-rule comparator, which is a third one
//
// Upstream sorts pseudo keys with `localeCompare` and at-rules with a bare
// `.sort()`. So the at-rules keep the plain comparison: making them
// locale-aware would be a new divergence rather than a fix, and these are what
// says so.
// ──────────────────────────────────────────────

// Two at-rules differing only in case. `A` sorts below `a` here, which is the
// opposite of what the pseudo comparator does with the same pair -- and it is
// what upstream's bare `.sort()` does, so the two comparators disagree on
// purpose.
stylex_test!(
  at_rules_differing_only_in_case_sort_by_their_code_units,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { '@media (MIN-WIDTH: 1px)': 'red', '@media (min-width: 1px)': 'blue' } },
    });
  "#
);

// A non-ASCII at-rule. UTF-8 bytes and UTF-16 code units are both code-point
// order through the basic multilingual plane, so the two encodings agree and so
// do the two compilers.
stylex_test!(
  a_non_ascii_at_rule_sorts_by_its_code_points,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: {
        color: {
          '@supports (--ü: 1)': 'red',
          '@supports (--z: 1)': 'blue',
          '@media (min-width: 1px)': 'green',
        },
      },
    });
  "#
);
