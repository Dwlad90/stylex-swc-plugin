use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use stylex_css::css::common::{generate_css_rule, inline_style_to_css_string};
use stylex_enums::style_resolution::StyleResolution;
use stylex_structures::{pair::Pair, stylex_state_options::StyleXStateOptions};

fn ltr_only_pairs() -> Vec<Pair> {
  vec![
    Pair::new("backgroundColor", "red"),
    Pair::new("borderRadius", "8px"),
    Pair::new("display", "flex"),
    Pair::new("opacity", "0.8"),
    Pair::new("transform", "scale(1)"),
  ]
}

fn rtl_flippable_pairs() -> Vec<Pair> {
  vec![
    Pair::new("margin-inline-start", "8px"),
    Pair::new("padding-inline-end", "16px"),
    Pair::new("background-position", "start center"),
    Pair::new("cursor", "w-resize"),
    Pair::new("box-shadow", "4px 2px 8px rgba(0, 0, 0, 0.2)"),
  ]
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
  let ltr_pairs = ltr_only_pairs();
  let rtl_pairs = rtl_flippable_pairs();

  group.bench_function("ltr_only_pairs", |b| {
    b.iter(|| inline_style_to_css_string(black_box(&ltr_pairs)))
  });

  group.bench_function("rtl_flippable_pairs", |b| {
    b.iter(|| inline_style_to_css_string(black_box(&rtl_pairs)))
  });

  group.finish();
}

fn generate_css_rule_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("GenerateCssRule");
  let options = rtl_options();

  let ltr_values = vec!["red".to_string(), "blue".to_string(), "#fff".to_string()];
  group.bench_function("ltr_only", |b| {
    b.iter_batched(
      || {
        (
          Vec::<String>::new(),
          Vec::<String>::new(),
          Vec::<String>::new(),
        )
      },
      |(mut pseudos, mut at_rules, mut const_rules)| {
        generate_css_rule(
          black_box("x1abcde"),
          black_box("background-color"),
          black_box(&ltr_values),
          black_box(&mut pseudos),
          black_box(&mut at_rules),
          black_box(&mut const_rules),
          black_box(&options),
        )
      },
      BatchSize::SmallInput,
    )
  });

  let rtl_values = vec!["12px".to_string(), "24px".to_string(), "36px".to_string()];
  group.bench_function("rtl_flippable", |b| {
    b.iter_batched(
      || {
        (
          Vec::<String>::new(),
          Vec::<String>::new(),
          Vec::<String>::new(),
        )
      },
      |(mut pseudos, mut at_rules, mut const_rules)| {
        generate_css_rule(
          black_box("x2abcde"),
          black_box("margin-inline-start"),
          black_box(&rtl_values),
          black_box(&mut pseudos),
          black_box(&mut at_rules),
          black_box(&mut const_rules),
          black_box(&options),
        )
      },
      BatchSize::SmallInput,
    )
  });

  let hover_values = vec!["underline".to_string(), "none".to_string()];
  group.bench_function("hover_pseudo", |b| {
    b.iter_batched(
      || {
        (
          vec![":hover".to_string()],
          Vec::<String>::new(),
          Vec::<String>::new(),
        )
      },
      |(mut pseudos, mut at_rules, mut const_rules)| {
        generate_css_rule(
          black_box("x3abcde"),
          black_box("text-decoration"),
          black_box(&hover_values),
          black_box(&mut pseudos),
          black_box(&mut at_rules),
          black_box(&mut const_rules),
          black_box(&options),
        )
      },
      BatchSize::SmallInput,
    )
  });

  let media_values = vec!["scale(1.1)".to_string(), "scale(0.98)".to_string()];
  group.bench_function("media_at_rule", |b| {
    b.iter_batched(
      || {
        (
          vec![":hover".to_string()],
          vec!["@media (hover: hover)".to_string()],
          Vec::<String>::new(),
        )
      },
      |(mut pseudos, mut at_rules, mut const_rules)| {
        generate_css_rule(
          black_box("x4abcde"),
          black_box("transform"),
          black_box(&media_values),
          black_box(&mut pseudos),
          black_box(&mut at_rules),
          black_box(&mut const_rules),
          black_box(&options),
        )
      },
      BatchSize::SmallInput,
    )
  });

  group.finish();
}

criterion_group!(
  css_generation_benches,
  inline_style_benchmarks,
  generate_css_rule_benchmarks
);

criterion_main!(css_generation_benches);
