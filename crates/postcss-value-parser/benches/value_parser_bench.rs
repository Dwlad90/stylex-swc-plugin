//! What the scanner costs per declaration value.
//!
//! The corpus is declaration values as they actually turn up in a compiled
//! file, not synthetic worst cases: the scanner runs once per declaration, so
//! per-value cost at realistic lengths is the number that decides whether a
//! large codebase compiles quickly.
//!
//! `parse` and `stringify` are timed apart because they allocate for different
//! reasons -- `parse` once per token it cuts, `stringify` once for the buffer
//! it fills -- and a change to one has no reason to move the other.

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use postcss_value_parser::{Node, parse, stringify, unit};

/// Declaration values in the proportions a stylesheet holds them: mostly short
/// and simple, with the function-heavy shapes that cost the most represented
/// rather than dominating.
fn corpus() -> Vec<&'static str> {
  vec![
    // The short values that make up most of a stylesheet.
    "1px",
    "0",
    "red",
    "#ff0000",
    "block",
    "1.5",
    "600",
    "100%",
    // Multi-token values.
    "1px solid #e0e0e0",
    "0 auto",
    "12px/1.5 sans-serif",
    "1px !important",
    // Functions, nested and not.
    "rgba(0, 0, 0, 0.5)",
    "var(--spacing)",
    "var(--a, var(--b, 1px))",
    "calc(((768px - 100vw) / 2) - 15px)",
    "translate3d(10px, 20px, 0) rotate(45deg)",
    "clamp(1rem, 2.5vw, 3rem)",
    // The long shapes: strings, urls and comma-separated layers.
    "bold italic 12px/3 'Open Sans', Arial, \"Helvetica Neue\", sans-serif",
    "0 0 0 1px rgba(0, 0, 0, 0.1), 0 2px 4px rgba(0, 0, 0, 0.2)",
    "url(https://example.com/assets/background-image.png)",
    "linear-gradient(to right, #ffffff 0%, #000000 100%)",
  ]
}

fn parse_benchmarks(c: &mut Criterion) {
  let values = corpus();
  let mut group = c.benchmark_group("Parse");

  group.bench_function("corpus", |b| {
    b.iter(|| {
      for value in &values {
        black_box(parse(black_box(value)));
      }
    })
  });

  group.bench_function("short_value", |b| b.iter(|| parse(black_box("1px"))));
  group.bench_function("nested_functions", |b| {
    b.iter(|| parse(black_box("var(--a, var(--b, 1px))")))
  });
  group.bench_function("font_shorthand", |b| {
    b.iter(|| {
      parse(black_box(
        "bold italic 12px/3 'Open Sans', Arial, \"Helvetica Neue\", sans-serif",
      ))
    })
  });

  group.finish();
}

fn stringify_benchmarks(c: &mut Criterion) {
  let values = corpus();
  let trees: Vec<Vec<Node>> = values.iter().map(|value| parse(value)).collect();
  let mut group = c.benchmark_group("Stringify");

  group.bench_function("corpus", |b| {
    b.iter(|| {
      for tree in &trees {
        black_box(stringify(black_box(tree)));
      }
    })
  });

  group.finish();
}

/// The round trip a normalizer actually performs: read a value, then spell the
/// result back out.
fn round_trip_benchmarks(c: &mut Criterion) {
  let values = corpus();
  let mut group = c.benchmark_group("RoundTrip");

  group.bench_function("corpus", |b| {
    b.iter(|| {
      for value in &values {
        black_box(stringify(&parse(black_box(value))));
      }
    })
  });

  group.finish();
}

fn unit_benchmarks(c: &mut Criterion) {
  let words = ["1px", "0", "-12.5rem", "1e3", "100%", "auto", "1E-3px"];
  let mut group = c.benchmark_group("Unit");

  group.bench_function("words", |b| {
    b.iter(|| {
      for word in &words {
        black_box(unit(black_box(word)));
      }
    })
  });

  group.finish();
}

criterion_group!(
  benches,
  parse_benchmarks,
  stringify_benchmarks,
  round_trip_benchmarks,
  unit_benchmarks
);
criterion_main!(benches);
