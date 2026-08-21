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
//! CSS and both spell into a selector unchanged. The two exceptions are the
//! case-differing keys at the end, which diverge for a reason the grouping does
//! not reach: the *comparator*. Upstream compares with `localeCompare`, which
//! orders `:HOVER` after `:active`; this compiler compares bytes, which orders
//! it before. That is a second, independent divergence in the same function,
//! pre-existing and visible at two keys as well as three, and it is measured
//! here rather than routed around -- issue 32 of this effort owns it.

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

// A non-ASCII pseudo name. `ü` is above every ASCII letter, so it sorts last --
// and the sort is over the key's bytes, which for a name whose ASCII prefix
// already decides the comparison is the same answer either compiler reaches.
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

// An uppercase pseudo name -- the one shape in this file where the two
// compilers disagree, and the grouping is not why. Nothing lowercases a
// condition key, and the comparison is over bytes, where every uppercase letter
// sorts below every lowercase one: `:HOVER:active:focus` here against
// `:active:focus:HOVER` upstream, `x17ymi95` against `xnnn07p`. The run is
// grouped identically on both sides; what differs is `localeCompare` against a
// byte comparison, which `two_pseudo_names_differing_only_in_case` below shows
// at two keys, where no grouping question exists at all. Recorded so the
// divergence has a name and a place, and reports as a changed verdict the day
// the comparator moves.
stylex_test!(
  an_uppercase_pseudo_name_sorts_by_its_bytes,
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
// is a run this ticket's grouping changed: the pair diverges too, in either
// nesting order, which is what makes the comparator a separate mechanism from
// the grouping. Kept beside the three-key case so the two are read together.
stylex_test!(
  two_pseudo_names_differing_only_in_case,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      w: { color: { ':HOVER': { ':active': 'red' } } },
    });
  "#
);
