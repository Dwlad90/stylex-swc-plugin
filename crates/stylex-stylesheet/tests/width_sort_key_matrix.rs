//! The width sort key against the reference implementation over a matrix of
//! media-query chains.
//!
//! `fixtures/width-sort-key-matrix.json` holds the at-rule chains: every
//! spelling of a `@media` prelude the StyleX parser treats differently for the
//! width key (px, rem, zero and negative zero, exponents, case, the range
//! syntax both ways, `and` with keywords and other pairs, comma lists, leading
//! and nested `not`, `calc()`, two bounds of one axis, malformed queries), each
//! alone and under a `@supports`, a `@container` and a second `@media`, and a
//! set of contexts whose text leaves ASCII. Beside each chain is
//! the rank `@stylexjs/babel-plugin`'s `processStylexRules` gives it, read
//! off a sort of two copies of every chain (equal keys interleave their
//! copies, different keys do not), `null` where the plugin has no key.
//!
//! The test sorts the chains with this crate's key and comparator and asks
//! that every pair be ordered as the plugin orders it. Regenerate the fixture
//! from the plugin when its version moves; the reader below names the fields.

use std::cmp::Ordering;
use std::fs;
use std::path::Path;

use serde::Deserialize;
use stylex_stylesheet::width_sort_key::{
  WidthSortKey, compare_width_sort_keys, width_sort_key_for_chain,
};

#[derive(Deserialize)]
struct Matrix {
  chains: Vec<Row>,
}

#[derive(Deserialize)]
struct Row {
  preludes: Vec<String>,
  rank: Option<u32>,
}

fn oracle_order(a: Option<u32>, b: Option<u32>) -> Ordering {
  match (a, b) {
    (None, None) => Ordering::Equal,
    (None, Some(_)) => Ordering::Greater,
    (Some(_), None) => Ordering::Less,
    (Some(a), Some(b)) => a.cmp(&b),
  }
}

#[test]
fn every_pair_of_the_matrix_orders_as_the_reference_orders_it() {
  let path =
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/width-sort-key-matrix.json");
  let text = match fs::read_to_string(&path) {
    Ok(text) => text,
    Err(error) => panic!("cannot read {}: {error}", path.display()),
  };
  let matrix: Matrix = match serde_json::from_str(&text) {
    Ok(matrix) => matrix,
    Err(error) => panic!("the fixture is not the shape above: {error}"),
  };

  assert!(
    matrix.chains.len() > 1000,
    "the matrix is the whole generated set"
  );

  let keys: Vec<Option<WidthSortKey>> = matrix
    .chains
    .iter()
    .map(|row| width_sort_key_for_chain(&row.preludes))
    .collect();

  let keyed = keys.iter().filter(|key| key.is_some()).count();
  let keyed_by_oracle = matrix
    .chains
    .iter()
    .filter(|row| row.rank.is_some())
    .count();

  assert_eq!(
    keyed, keyed_by_oracle,
    "the chains with a key are the chains the reference sorts"
  );

  let mut disagreements = Vec::new();

  for (i, a) in matrix.chains.iter().enumerate() {
    for (j, b) in matrix.chains.iter().enumerate().skip(i + 1) {
      let expected = oracle_order(a.rank, b.rank);
      let actual = compare_width_sort_keys(keys[i].as_ref(), keys[j].as_ref());

      if expected != actual {
        disagreements.push(format!(
          "{:?} vs {:?}: reference {expected:?}, this crate {actual:?} ({:?} / {:?})",
          a.preludes.join("{"),
          b.preludes.join("{"),
          keys[i],
          keys[j]
        ));
      }
    }
  }

  assert!(
    disagreements.is_empty(),
    "{} pair(s) disagree with the reference:\n{}",
    disagreements.len(),
    disagreements
      .iter()
      .take(20)
      .cloned()
      .collect::<Vec<String>>()
      .join("\n")
  );
}
