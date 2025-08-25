/*!
CSS Types Benchmarks for StyleX CSS Parser.
*/

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use stylex_css_parser::css_types::*;

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

  for input in &color_inputs {
    group.bench_with_input(format!("parse_color_{}", input), input, |b, input| {
      b.iter(|| {
        // Benchmark the parser creation and usage
        let parser = Color::parse();
        black_box(parser.parse(black_box(input)))
      })
    });
  }

  group.finish();
}

fn length_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("Length");

  let length_inputs = [
    "10px",
    "1.5em",
    "100%",
    "50vh",
    "2rem",
    "0",
    "3.14159in",
    "25vw",
    "12pt",
  ];

  for input in &length_inputs {
    group.bench_with_input(format!("parse_length_{}", input), input, |b, input| {
      b.iter(|| {
        let parser = Length::parser();
        black_box(parser.parse(black_box(input)))
      })
    });
  }

  group.finish();
}

fn angle_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("Angle");

  let angle_inputs = [
    "45deg", "1.57rad", "50grad", "0.25turn", "0deg", "360deg", "2π rad",
  ];

  for input in &angle_inputs {
    group.bench_with_input(format!("parse_angle_{}", input), input, |b, input| {
      b.iter(|| {
        let parser = Angle::parser();
        black_box(parser.parse(black_box(input)))
      })
    });
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

  for input in &calc_inputs {
    group.bench_with_input(format!("parse_calc_{}", input), input, |b, input| {
      b.iter(|| {
        let parser = Calc::parse();
        black_box(parser.parse(black_box(input)))
      })
    });
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

  for input in &blend_mode_inputs {
    group.bench_with_input(format!("parse_blend_mode_{}", input), input, |b, input| {
      b.iter(|| {
        let parser = BlendMode::parser();
        black_box(parser.parse(black_box(input)))
      })
    });
  }

  group.finish();
}

fn flex_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("Flex");

  let flex_inputs = ["1fr", "2.5fr", "0fr", "10fr", "0.25fr"];

  for input in &flex_inputs {
    group.bench_with_input(format!("parse_flex_{}", input), input, |b, input| {
      b.iter(|| {
        let parser = Flex::parser();
        black_box(parser.parse(black_box(input)))
      })
    });
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

  for input in &ident_inputs {
    group.bench_with_input(
      format!("parse_custom_ident_{}", input),
      input,
      |b, input| {
        b.iter(|| {
          let parser = CustomIdentifier::parser();
          black_box(parser.parse(black_box(input)))
        })
      },
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

  for input in &position_inputs {
    group.bench_with_input(format!("parse_position_{}", input), input, |b, input| {
      b.iter(|| {
        let parser = Position::parser();
        black_box(parser.parse(black_box(input)))
      })
    });
  }

  group.finish();
}

// Comparative benchmarks that test multiple CSS types for parser performance
fn parser_comparison_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("ParserComparison");

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

  group.bench_function("parse_complex_values", |b| {
    b.iter(|| {
      // Test parsing a variety of CSS values
      let color_parser = Color::parse();
      let length_parser = Length::parser();
      let calc_parser = Calc::parse();

      black_box((
        color_parser.parse("rgba(255, 128, 0, 0.7)"),
        length_parser.parse("calc(100vh - 50px)"),
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
