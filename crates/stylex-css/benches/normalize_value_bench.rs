//! What the normalizer passes cost per declaration value.
//!
//! `normalize_value` runs once per declaration, so per-value cost at realistic
//! lengths is the number that decides whether a large codebase compiles
//! quickly. Nothing else in the workspace times it: `css_generation_bench`
//! starts after normalization, at the `Pair` the passes produced.
//!
//! Split by what the passes have to *do*, because the three cases allocate for
//! different reasons and a change to one has no reason to move the others.
//! `untouched` is the shape a stylesheet is mostly made of and the only one
//! where a guard against rewriting text with identical text can show; the
//! passes cannot help `rewritten`, which pays for every replacement it makes;
//! and `important` is the one shape that reaches the `!important` planning
//! walk at all.

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use stylex_css::css::normalize_value::normalize_value;
use stylex_structures::stylex_state_options::StyleXStateOptions;

/// Declaration values already spelled the way the passes would spell them.
///
/// Most of a stylesheet is this: single tokens, hex colours, values whose
/// spacing an author wrote canonically to begin with.
fn untouched() -> Vec<(&'static str, &'static str)> {
  vec![
    ("width", "1px"),
    ("color", "red"),
    ("color", "#ff0000"),
    ("display", "block"),
    ("fontWeight", "600"),
    ("border", "1px solid #e0e0e0"),
    ("color", "rgba(0,0,0,.5)"),
    ("width", "var(--spacing)"),
    ("width", "calc(((768px - 100vw) / 2) - 15px)"),
    ("transform", "translate3d(10px,20px,0) rotate(45deg)"),
  ]
}

/// Values every pass has something to do to: whitespace to collapse, a leading
/// zero to drop, a timing to re-spell, a zero dimension to shorten, a camel
/// cased keyword to dashify.
fn rewritten() -> Vec<(&'static str, &'static str)> {
  vec![
    ("width", "1px   solid    red"),
    ("opacity", "0.5"),
    ("transitionDuration", "500ms"),
    ("margin", "0px"),
    ("transform", "translate3d( 10px , 20px , 0 )"),
    ("transitionProperty", "backgroundColor"),
    ("boxShadow", "0 0 0 1px rgba( 0 , 0 , 0 , 0.1 )"),
  ]
}

/// The only shape that reaches `plan_important_removals`, including the nested
/// annotation the plan walk descends into a function to find.
fn important() -> Vec<(&'static str, &'static str)> {
  vec![
    ("color", "red !important"),
    ("width", "calc(1px + 2px) !important"),
  ]
}

/// Times one set under the default options, which is what the parity harness
/// measures against.
///
/// Takes the group rather than opening its own, so the three sets sit under one
/// name and a run reports them side by side.
fn bench_set(
  group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
  name: &str,
  set: &[(&'static str, &'static str)],
) {
  let options = StyleXStateOptions::default();

  group.bench_function(name, |b| {
    b.iter(|| {
      for (key, value) in set {
        black_box(normalize_value(
          black_box(value),
          black_box(key),
          black_box(&options),
        ));
      }
    })
  });
}

fn normalize_value_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("NormalizeValue");

  bench_set(&mut group, "untouched", &untouched());
  bench_set(&mut group, "rewritten", &rewritten());
  bench_set(&mut group, "important", &important());

  group.finish();
}

criterion_group!(normalize_value_benches, normalize_value_benchmarks);

criterion_main!(normalize_value_benches);
