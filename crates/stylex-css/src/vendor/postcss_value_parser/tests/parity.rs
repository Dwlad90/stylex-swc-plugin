//! Does this agree with the JavaScript it stands in for?
//!
//! Every expectation here is that JavaScript's own answer, printed by
//! `scripts/generate-value-parser-cases.mjs` into `cases.rs`. Nothing in this
//! file is written by eye, and nothing in it should be: a hand-edited
//! expectation here is just a divergence with a test blessing it.
//!
//! Regenerate the table after adding an input to the generator:
//!
//! ```sh
//! pnpm run --filter=@stylexswc/css generate:value-parser-cases
//! ```

use crate::vendor::postcss_value_parser::{Dimension, ValueParser, parse, stringify, unit};

use super::cases::{PARSER_CASES, STRESS_CASES, UNIT_CASES};
use super::dump::dump;

#[test]
fn parses_every_value_into_the_same_tree() {
  for case in PARSER_CASES {
    assert_eq!(
      dump(&parse(case.input)),
      case.ast,
      "tree differs for {:?}",
      case.input
    );
  }
}

#[test]
fn serializes_every_value_into_the_same_text() {
  for case in PARSER_CASES {
    assert_eq!(
      stringify(&parse(case.input)),
      case.output,
      "serialized text differs for {:?}",
      case.input
    );
  }
}

/// The property the rest of the normalization effort rests on: what no
/// normalizer rewrites comes back exactly as the author wrote it — hex
/// spelling, letter case, quote character, exponent and whitespace positions
/// included.
///
/// Four corpus values are excluded, and they are the exception that proves
/// the agreement is real rather than merely plausible. `/*/` closes the comment
/// it opens, so a value containing one comes back changed. Matching that is the
/// requirement; improving on it would change class names.
#[test]
fn round_trips_every_value_byte_for_byte() {
  let mut quirks = Vec::new();

  for case in PARSER_CASES {
    match case.output == case.input {
      true => assert_eq!(
        stringify(&parse(case.input)),
        case.input,
        "round trip differs for {:?}",
        case.input
      ),
      false => quirks.push(case.input),
    }
  }

  assert_eq!(
    quirks,
    vec!["/*/ x */ 1px", "1px /*/ y */", "/*/", "/*/ x */"]
  );
}

#[test]
fn survives_values_at_the_sizes_where_scanning_stops_being_about_css() {
  for case in STRESS_CASES {
    assert_eq!(
      stringify(&parse(case.input)),
      case.output,
      "serialized text differs for the {} case",
      case.label
    );
  }
}

#[test]
fn splits_every_word_into_the_same_number_and_unit() {
  for (input, expected) in UNIT_CASES {
    let expected = expected.map(|(number, unit)| Dimension {
      number: number.to_owned(),
      unit: unit.to_owned(),
    });

    assert_eq!(unit(input), expected, "unit split differs for {input:?}");
  }
}

/// The entry point ties the three together, so it is measured against the same
/// answers rather than trusted to call them correctly.
#[test]
fn the_entry_point_parses_and_serializes_like_its_parts() {
  for case in PARSER_CASES {
    let parsed = ValueParser::new(case.input);

    assert_eq!(
      dump(&parsed.nodes),
      case.ast,
      "tree differs for {:?}",
      case.input
    );
    assert_eq!(
      parsed.to_string(),
      case.output,
      "serialized text differs for {:?}",
      case.input
    );
  }
}
