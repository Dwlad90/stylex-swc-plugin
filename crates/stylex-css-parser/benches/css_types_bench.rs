/*!
CSS Types Benchmarks for StyleX CSS Parser.
*/

// The allocator the published addon links, so a measurement that allocation
// binds matches what a consumer gets. Rust links a dev-dependency only where a
// target names it, so this line is what makes the choice real.
use swc_malloc as _;

use std::hint::black_box;

use criterion::{
  BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
use stylex_css_parser::{css_types::*, token_parser::TokenParser};

/// Times one parser against one input, after a check that the parser accepts
/// it.
///
/// The check runs once, outside `b.iter`, so it adds nothing to the
/// measurement: the input is the same on every iteration, so one answer speaks
/// for all of them. Without it a parser that starts refusing everything reads
/// as a win, because a refusal is faster than a parse.
///
/// `build` is generic rather than a function pointer, so each call site
/// monomorphizes and the timed closure holds the same direct call it held
/// before this helper existed. The check found three
/// inputs in this file that no parser ever accepted.
fn bench_parse<T: Clone + std::fmt::Debug + 'static>(
  group: &mut BenchmarkGroup<'_, WallTime>,
  name: String,
  input: &'static str,
  build: impl Fn() -> TokenParser<T>,
) {
  assert!(
    build().parse(input).is_ok(),
    "{name} times a refusal: {input:?} is no longer accepted"
  );

  group.bench_function(name, |b| {
    b.iter(|| {
      let parser = build();
      black_box(parser.parse(black_box(input)))
    })
  });
}

fn color_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("Color");

  // Test typical color values that would be parsed
  let color_inputs = [
    "red",
    "#ff0000",
    "#fff",
    "rgb(255, 0, 0)",
    "rgba(255, 0, 0, 0.5)",
    "hsl(0, 100%, 50%)",
    "hsla(0, 100%, 50%, 0.8)",
    "transparent",
    "rebeccapurple",
  ];

  for input in color_inputs {
    bench_parse(
      &mut group,
      format!("parse_color_{}", input),
      input,
      Color::parse,
    );
  }

  group.finish();
}

fn length_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("Length");

  // `0.5cm` stands where `100%` stood. A percentage is not a `<length>`, so
  // that case timed a refusal.
  let length_inputs = [
    "10px",
    "1.5em",
    "0.5cm",
    "50vh",
    "2rem",
    "0",
    "3.14159in",
    "25vw",
    "12pt",
  ];

  for input in length_inputs {
    bench_parse(
      &mut group,
      format!("parse_length_{}", input),
      input,
      Length::parser,
    );
  }

  group.finish();
}

fn angle_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("Angle");

  // `-90deg` stands where `2π rad` stood. `π` is not a CSS number, so that case
  // timed a refusal.
  let angle_inputs = [
    "45deg", "1.57rad", "50grad", "0.25turn", "0deg", "360deg", "-90deg",
  ];

  for input in angle_inputs {
    bench_parse(
      &mut group,
      format!("parse_angle_{}", input),
      input,
      Angle::parser,
    );
  }

  group.finish();
}

fn calc_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("Calc");

  let calc_inputs = [
    "calc(100px + 20px)",
    "calc(50% - 10px)",
    "calc(2 * 3)",
    "calc(100vh / 2)",
    "calc(100% + 20px - 10px)",
    "calc(pi)",
    "calc(e * 2)",
  ];

  for input in calc_inputs {
    bench_parse(
      &mut group,
      format!("parse_calc_{}", input),
      input,
      Calc::parse,
    );
  }

  group.finish();
}

fn blend_mode_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("BlendMode");

  let blend_mode_inputs = [
    "normal",
    "multiply",
    "screen",
    "overlay",
    "darken",
    "lighten",
    "color-dodge",
    "color-burn",
    "hard-light",
    "soft-light",
    "difference",
    "exclusion",
    "hue",
    "saturation",
    "color",
    "luminosity",
  ];

  for input in blend_mode_inputs {
    bench_parse(
      &mut group,
      format!("parse_blend_mode_{}", input),
      input,
      BlendMode::parser,
    );
  }

  group.finish();
}

fn flex_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("Flex");

  let flex_inputs = ["1fr", "2.5fr", "0fr", "10fr", "0.25fr"];

  for input in flex_inputs {
    bench_parse(
      &mut group,
      format!("parse_flex_{}", input),
      input,
      Flex::parser,
    );
  }

  group.finish();
}

fn custom_ident_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("CustomIdent");

  let ident_inputs = [
    "my-custom-ident",
    "button-style",
    "nav-item",
    "CamelCase",
    "snake_case",
    "kebab-case-identifier",
    "veryLongIdentifierNameThatCouldBeUsedInRealWorldScenarios",
  ];

  for input in ident_inputs {
    bench_parse(
      &mut group,
      format!("parse_custom_ident_{}", input),
      input,
      CustomIdentifier::parser,
    );
  }

  group.finish();
}

fn position_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("Position");

  let position_inputs = [
    "left top",
    "center center",
    "right bottom",
    "left",
    "center",
    "right",
    "top",
    "bottom",
    "50% 25%",
    "10px 20px",
  ];

  for input in position_inputs {
    bench_parse(
      &mut group,
      format!("parse_position_{}", input),
      input,
      Position::parser,
    );
  }

  group.finish();
}

// Comparative benchmarks that test multiple CSS types for parser performance
fn parser_comparison_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("ParserComparison");

  // A parser this case builds but never runs would still cost what it costs to
  // build, so the check reads one value through each of the eight.
  assert!(Color::parse().parse("red").is_ok());
  assert!(Length::parser().parse("10px").is_ok());
  assert!(Angle::parser().parse("45deg").is_ok());
  assert!(Calc::parse().parse("calc(1px + 1px)").is_ok());
  assert!(BlendMode::parser().parse("normal").is_ok());
  assert!(Flex::parser().parse("1fr").is_ok());
  assert!(CustomIdentifier::parser().parse("ident").is_ok());
  assert!(Position::parser().parse("left top").is_ok());

  group.bench_function("create_parsers", |b| {
    b.iter(|| {
      // Benchmark the cost of creating parsers
      black_box((
        Color::parse(),
        Length::parser(),
        Angle::parser(),
        Calc::parse(),
        BlendMode::parser(),
        Flex::parser(),
        CustomIdentifier::parser(),
        Position::parser(),
      ))
    })
  });

  // `12.75vmin` stands where `calc(100vh - 50px)` stood. `Length` holds a
  // number and a unit and reads no expression, so that leg timed a refusal;
  // the calc shape the case wants is the third leg below.
  assert!(Color::parse().parse("rgba(255, 128, 0, 0.7)").is_ok());
  assert!(Length::parser().parse("12.75vmin").is_ok());
  assert!(Calc::parse().parse("calc(50% + 2em * 3)").is_ok());

  group.bench_function("parse_complex_values", |b| {
    b.iter(|| {
      // Test parsing a variety of CSS values
      let color_parser = Color::parse();
      let length_parser = Length::parser();
      let calc_parser = Calc::parse();

      black_box((
        color_parser.parse("rgba(255, 128, 0, 0.7)"),
        length_parser.parse("12.75vmin"),
        calc_parser.parse("calc(50% + 2em * 3)"),
      ))
    })
  });

  group.finish();
}

criterion_group!(
  css_types_benches,
  color_benchmarks,
  length_benchmarks,
  angle_benchmarks,
  calc_benchmarks,
  blend_mode_benchmarks,
  flex_benchmarks,
  custom_ident_benchmarks,
  position_benchmarks,
  parser_comparison_benchmarks
);

criterion_main!(css_types_benches);
