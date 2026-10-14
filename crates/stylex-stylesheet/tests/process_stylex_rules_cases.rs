//! Every test of the reference's `__tests__/transform-process-test.js`, by its
//! own name, over the calls it makes: `tests/fixtures/process-stylex-rules-cases.json`
//! holds, per test, each `processStylexRules` call -- the rules handed in (for
//! the cases that compile a fixture, the Babel transform's metadata), the
//! config, and the string the plugin answered, which is the test's inline
//! snapshot. The fixture was recorded by running the test file itself against
//! the published plugin; its header names the version.
//!
//! Each test here asserts that `process_stylex_rules` answers the same string
//! for the same input. The reference's two structural cases hold as well: a
//! Rust caller's rules are borrowed, so `no mutation of rules` is the
//! signature, and the deterministic-order cases run their permutations.

use std::fs;
use std::path::Path;
use std::sync::OnceLock;

use serde::Deserialize;
use stylex_stylesheet::process::{ProcessStylexRulesConfig, Rule, process_stylex_rules};

#[derive(Deserialize)]
struct Fixture {
  // The fixture records the reference commit its cases were recorded from.
  // The tests do not read it.
  #[allow(dead_code)]
  reference: String,
  cases: Vec<Case>,
}

#[derive(Deserialize)]
struct Case {
  name: String,
  calls: Vec<Call>,
}

#[derive(Deserialize)]
struct Call {
  rules: Vec<Rule>,
  config: Option<ProcessStylexRulesConfig>,
  output: String,
}

fn fixture() -> &'static Fixture {
  static FIXTURE: OnceLock<Fixture> = OnceLock::new();

  FIXTURE.get_or_init(|| {
    let path =
      Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/process-stylex-rules-cases.json");
    let text = match fs::read_to_string(&path) {
      Ok(text) => text,
      Err(error) => panic!("cannot read {}: {error}", path.display()),
    };

    match serde_json::from_str(&text) {
      Ok(fixture) => fixture,
      Err(error) => panic!("the fixture is not the shape above: {error}"),
    }
  })
}

fn assert_case(name: &str) {
  let Some(case) = fixture().cases.iter().find(|case| case.name == name) else {
    panic!("the fixture holds no case named {name:?}");
  };

  assert!(!case.calls.is_empty(), "{name:?} makes at least one call");

  for (index, call) in case.calls.iter().enumerate() {
    // `config ?? {}`: a call with no config reads every option as unset.
    let config = call.config.clone().unwrap_or_default();
    let sheet = match process_stylex_rules(&call.rules, &config) {
      Ok(sheet) => sheet,
      Err(error) => panic!("{name:?}, call {index}: {error}"),
    };

    assert_eq!(
      sheet, call.output,
      "{name:?}, call {index}: the sheet differs from the plugin's"
    );
  }
}

#[test]
fn the_fixture_holds_every_test_of_the_reference_file() {
  assert_eq!(fixture().cases.len(), 36);
  assert_eq!(
    fixture()
      .cases
      .iter()
      .map(|case| case.calls.len())
      .sum::<usize>(),
    45
  );
}

/// `no rules`
#[test]
fn no_rules() {
  assert_case("no rules");
}

/// `all rules (useLayers:false)`
#[test]
fn all_rules_uselayers_false() {
  assert_case("all rules (useLayers:false)");
}

/// `all rules (useLayers:true)`
#[test]
fn all_rules_uselayers_true() {
  assert_case("all rules (useLayers:true)");
}

/// `useLayers with before option`
#[test]
fn uselayers_with_before_option() {
  assert_case("useLayers with before option");
}

/// `useLayers with after option`
#[test]
fn uselayers_with_after_option() {
  assert_case("useLayers with after option");
}

/// `useLayers with both before and after`
#[test]
fn uselayers_with_both_before_and_after() {
  assert_case("useLayers with both before and after");
}

/// `useLayers with prefix option`
#[test]
fn uselayers_with_prefix_option() {
  assert_case("useLayers with prefix option");
}

/// `useLayers with prefix, before, and after combined`
#[test]
fn uselayers_with_prefix_before_and_after_combined() {
  assert_case("useLayers with prefix, before, and after combined");
}

/// `useLayers with multi-segment dot-notated prefix (XDS use case)`
#[test]
fn uselayers_with_multi_segment_dot_notated_prefix_xds_use_case() {
  assert_case("useLayers with multi-segment dot-notated prefix (XDS use case)");
}

/// `empty before/after produce standard layer declaration`
#[test]
fn empty_before_after_produce_standard_layer_declaration() {
  assert_case("empty before/after produce standard layer declaration");
}

/// `all rules (legacyDisableLayers:true)`
#[test]
fn all_rules_legacydisablelayers_true() {
  assert_case("all rules (legacyDisableLayers:true)");
}

/// `legacy-expand-shorthands with logical styles polyfill`
#[test]
fn legacy_expand_shorthands_with_logical_styles_polyfill() {
  assert_case("legacy-expand-shorthands with logical styles polyfill");
}

/// `logical float vars are emitted when the float comes from a constant`
#[test]
fn logical_float_vars_are_emitted_when_the_float_comes_from_a_constant() {
  assert_case("logical float vars are emitted when the float comes from a constant");
}

/// `legacy-expand-shorthands duplicates theme selectors for higher precedence`
#[test]
fn legacy_expand_shorthands_duplicates_theme_selectors_for_higher_precedence() {
  assert_case("legacy-expand-shorthands duplicates theme selectors for higher precedence");
}

/// `no mutation of rules`
#[test]
fn no_mutation_of_rules() {
  assert_case("no mutation of rules");
}

/// `useLegacyClassnamesSort: false (default behavior)`
#[test]
fn uselegacyclassnamessort_false_default_behavior() {
  assert_case("useLegacyClassnamesSort: false (default behavior)");
}

/// `useLegacyClassnamesSort: true (legacy behavior)`
#[test]
fn uselegacyclassnamessort_true_legacy_behavior() {
  assert_case("useLegacyClassnamesSort: true (legacy behavior)");
}

/// `sorts min-width with screen and media type`
#[test]
fn sorts_min_width_with_screen_and_media_type() {
  assert_case("sorts min-width with screen and media type");
}

/// `does not misorder negated min-width media queries`
#[test]
fn does_not_misorder_negated_min_width_media_queries() {
  assert_case("does not misorder negated min-width media queries");
}

/// `does not sort a min-width paired with a negated max-width`
#[test]
fn does_not_sort_a_min_width_paired_with_a_negated_max_width() {
  assert_case("does not sort a min-width paired with a negated max-width");
}

/// `sorts min-width breakpoints nested inside another at-rule`
#[test]
fn sorts_min_width_breakpoints_nested_inside_another_at_rule() {
  assert_case("sorts min-width breakpoints nested inside another at-rule");
}

/// `sorts max-width defineConsts breakpoints using real transform metadata`
#[test]
fn sorts_max_width_defineconsts_breakpoints_using_real_transform_metadata() {
  assert_case("sorts max-width defineConsts breakpoints using real transform metadata");
}

/// `sorts min-width defineConsts breakpoints in ascending px order`
#[test]
fn sorts_min_width_defineconsts_breakpoints_in_ascending_px_order() {
  assert_case("sorts min-width defineConsts breakpoints in ascending px order");
}

/// `sorts min-width breakpoints via template literal partial value`
#[test]
fn sorts_min_width_breakpoints_via_template_literal_partial_value() {
  assert_case("sorts min-width breakpoints via template literal partial value");
}

/// `sorts max-width defineConsts breakpoints in descending px order`
#[test]
fn sorts_max_width_defineconsts_breakpoints_in_descending_px_order() {
  assert_case("sorts max-width defineConsts breakpoints in descending px order");
}

/// `sorts CSS Level 4 range syntax (width >= Xpx) as min-width`
#[test]
fn sorts_css_level_4_range_syntax_width_xpx_as_min_width() {
  assert_case("sorts CSS Level 4 range syntax (width >= Xpx) as min-width");
}

/// `range queries (both min and max-width) fall through to existing sort`
#[test]
fn range_queries_both_min_and_max_width_fall_through_to_existing_sort() {
  assert_case("range queries (both min and max-width) fall through to existing sort");
}

/// `sort is a total order across px min- and max-width rules`
#[test]
fn sort_is_a_total_order_across_px_min_and_max_width_rules() {
  assert_case("sort is a total order across px min- and max-width rules");
}

/// `orders the min-width group before the max-width group`
#[test]
fn orders_the_min_width_group_before_the_max_width_group() {
  assert_case("orders the min-width group before the max-width group");
}

/// `sorts unitless zero, any-unit zero, and mixed-case px`
#[test]
fn sorts_unitless_zero_any_unit_zero_and_mixed_case_px() {
  assert_case("sorts unitless zero, any-unit zero, and mixed-case px");
}

/// `does not sort a width whose value is a ratio`
#[test]
fn does_not_sort_a_width_whose_value_is_a_ratio() {
  assert_case("does not sort a width whose value is a ratio");
}

/// `does not sort a unitless non-zero width`
#[test]
fn does_not_sort_a_unitless_non_zero_width() {
  assert_case("does not sort a unitless non-zero width");
}

/// `does not sort rem breakpoints`
#[test]
fn does_not_sort_rem_breakpoints() {
  assert_case("does not sort rem breakpoints");
}

/// `does not sort a query with two bounds on the same side`
#[test]
fn does_not_sort_a_query_with_two_bounds_on_the_same_side() {
  assert_case("does not sort a query with two bounds on the same side");
}

/// `sort is deterministic regardless of input order`
#[test]
fn sort_is_deterministic_regardless_of_input_order() {
  assert_case("sort is deterministic regardless of input order");
}

/// `sort is deterministic with duplicate rules in different input orders`
#[test]
fn sort_is_deterministic_with_duplicate_rules_in_different_input_orders() {
  assert_case("sort is deterministic with duplicate rules in different input orders");
}
