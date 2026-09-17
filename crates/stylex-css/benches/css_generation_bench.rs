// The allocator the published addon links, so a measurement that allocation
// binds matches what a consumer gets. Rust links a dev-dependency only where a
// target names it, so this line is what makes the choice real.
use swc_malloc as _;

use std::borrow::Cow;
use std::hint::black_box;

use criterion::{
  BatchSize, BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
use stylex_css::css::common::{generate_css_rule, inline_style_to_css_string};
use stylex_enums::style_resolution::StyleResolution;
use stylex_structures::{pair::PairCow, stylex_state_options::StyleXStateOptions};
use stylex_types::structures::injectable_style::InjectableStyle;

/// The five declarations the inline-style cases serialize, spelled camel case.
///
/// `kebab_case` has to build a new string for each of these keys, which is the
/// cost this set prices.
fn converted_key_pairs() -> Vec<PairCow<'static>> {
  declarations(&[
    ("marginInlineStart", "8px"),
    ("paddingInlineEnd", "16px"),
    ("backgroundPosition", "start center"),
    ("borderRadius", "8px"),
    ("boxShadow", "4px 2px 8px rgba(0, 0, 0, 0.2)"),
  ])
}

/// The same five declarations, already spelled in kebab case.
///
/// `kebab_case` borrows each of these keys and converts nothing, so the pair of
/// cases separates one variable: whether the key needs converting.
///
/// The two cases used to be named for text direction. Nothing here flips:
/// `inline_style_to_css_string` takes no options, so it cannot. The
/// right-to-left case was also the faster of the two, which no reading of
/// flipping explains -- flipping is extra work. What separated them was always
/// the key spelling, and now the names say so. Flipping is priced by
/// `GenerateCssRule/rtl_flippable` below, which passes the options that turn it
/// on.
fn borrowed_key_pairs() -> Vec<PairCow<'static>> {
  declarations(&[
    ("margin-inline-start", "8px"),
    ("padding-inline-end", "16px"),
    ("background-position", "start center"),
    ("border-radius", "8px"),
    ("box-shadow", "4px 2px 8px rgba(0, 0, 0, 0.2)"),
  ])
}

/// The declarations a case serializes, each half borrowed the way the caller
/// that builds them holds them.
fn declarations(pairs: &[(&'static str, &'static str)]) -> Vec<PairCow<'static>> {
  pairs
    .iter()
    .map(|(key, value)| PairCow {
      key: Cow::Borrowed(*key),
      value: Cow::Borrowed(*value),
    })
    .collect()
}

fn rtl_options() -> StyleXStateOptions {
  let mut options = StyleXStateOptions::default();
  options.core.style_resolution = StyleResolution::LegacyExpandShorthands;
  options.core.enable_logical_styles_polyfill = true;
  options.core.enable_legacy_value_flipping = true;
  options
}

fn inline_style_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("InlineStyleToCssString");
  let converted = converted_key_pairs();
  let borrowed = borrowed_key_pairs();

  // Both sets must serialize to the same text, which is what makes the two
  // numbers comparable, and neither may serialize to nothing. The check runs
  // once, outside `b.iter`: the input is fixed, so one answer speaks for every
  // iteration, and a serializer that started writing nothing would otherwise
  // read as a win.
  let expected = concat!(
    "margin-inline-start:8px;padding-inline-end:16px;",
    "background-position:start center;border-radius:8px;",
    "box-shadow:4px 2px 8px rgba(0, 0, 0, 0.2)"
  );
  assert_eq!(inline_style_to_css_string(&converted), expected);
  assert_eq!(inline_style_to_css_string(&borrowed), expected);

  group.bench_function("converted_keys", |b| {
    b.iter(|| inline_style_to_css_string(black_box(&converted)))
  });

  group.bench_function("borrowed_keys", |b| {
    b.iter(|| inline_style_to_css_string(black_box(&borrowed)))
  });

  group.finish();
}

/// One `generate_css_rule` call: the arguments, and the answer it must give.
///
/// The four cases below differ only in these fields, so spelling them once
/// keeps the check and the timed closure from drifting apart.
struct RuleCase {
  class_name: &'static str,
  key: &'static str,
  values: Vec<String>,
  pseudos: Vec<String>,
  at_rules: Vec<String>,
  /// Whether this key, under these options, also produces a right-to-left
  /// rule. That is the flipping work, and it is the first thing a broken
  /// subject would stop doing.
  flips: bool,
}

impl RuleCase {
  fn new(class_name: &'static str, key: &'static str, values: &[&str], flips: bool) -> Self {
    Self {
      class_name,
      key,
      values: values.iter().map(|value| value.to_string()).collect(),
      pseudos: Vec::new(),
      at_rules: Vec::new(),
      flips,
    }
  }

  fn with_pseudos(mut self, pseudos: &[&str]) -> Self {
    self.pseudos = pseudos.iter().map(|pseudo| pseudo.to_string()).collect();
    self
  }

  fn with_at_rules(mut self, at_rules: &[&str]) -> Self {
    self.at_rules = at_rules.iter().map(|at_rule| at_rule.to_string()).collect();
    self
  }

  /// The call the timed closure makes, with the buffers it is handed.
  fn call(
    &self,
    pseudos: &mut [String],
    at_rules: &mut [String],
    const_rules: &mut [String],
    options: &StyleXStateOptions,
  ) -> InjectableStyle {
    generate_css_rule(
      black_box(self.class_name),
      black_box(self.key),
      black_box(&self.values),
      black_box(pseudos),
      black_box(at_rules),
      black_box(const_rules),
      black_box(options),
    )
  }
}

/// Times one case, after a check that the call produced the rule it exists to
/// time.
///
/// The check runs once, outside `b.iter`, so it adds nothing to the
/// measurement: every argument is fixed, so one answer speaks for every
/// iteration. Without it a generator that started returning an empty rule, or
/// one that stopped flipping, would read as a win.
fn bench_rule(
  group: &mut BenchmarkGroup<'_, WallTime>,
  name: &str,
  case: &RuleCase,
  options: &StyleXStateOptions,
) {
  let rule = case.call(
    &mut case.pseudos.clone(),
    &mut case.at_rules.clone(),
    &mut Vec::new(),
    options,
  );

  // The key itself is not asserted: the logical-property polyfill rewrites
  // some keys, so `margin-inline-start` reaches the output as `margin-left`.
  // What every case must produce is a rule that selects its own class and
  // carries a declaration.
  assert!(
    rule.ltr.contains(case.class_name) && rule.ltr.contains(':'),
    "{name} produced no rule for {}",
    case.key
  );
  assert_eq!(
    rule.rtl.is_some(),
    case.flips,
    "{name} no longer flips the way the case was written to time"
  );

  group.bench_function(name, |b| {
    b.iter_batched(
      || {
        (
          case.pseudos.clone(),
          case.at_rules.clone(),
          Vec::<String>::new(),
        )
      },
      |(mut pseudos, mut at_rules, mut const_rules)| {
        case.call(&mut pseudos, &mut at_rules, &mut const_rules, options)
      },
      BatchSize::SmallInput,
    )
  });
}

fn generate_css_rule_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("GenerateCssRule");
  let options = rtl_options();

  bench_rule(
    &mut group,
    "ltr_only",
    &RuleCase::new(
      "x1abcde",
      "background-color",
      &["red", "blue", "#fff"],
      false,
    ),
    &options,
  );

  bench_rule(
    &mut group,
    "rtl_flippable",
    &RuleCase::new(
      "x2abcde",
      "margin-inline-start",
      &["12px", "24px", "36px"],
      true,
    ),
    &options,
  );

  bench_rule(
    &mut group,
    "hover_pseudo",
    &RuleCase::new("x3abcde", "text-decoration", &["underline", "none"], false)
      .with_pseudos(&[":hover"]),
    &options,
  );

  bench_rule(
    &mut group,
    "media_at_rule",
    &RuleCase::new(
      "x4abcde",
      "transform",
      &["scale(1.1)", "scale(0.98)"],
      false,
    )
    .with_pseudos(&[":hover"])
    .with_at_rules(&["@media (hover: hover)"]),
    &options,
  );

  group.finish();
}

criterion_group!(
  css_generation_benches,
  inline_style_benchmarks,
  generate_css_rule_benchmarks
);

criterion_main!(css_generation_benches);
