/*!
CSS Properties Benchmarks for StyleX CSS Parser.
These benchmarks test the performance of complex CSS property parsing,
which involves combining multiple CSS types into coherent property values.
*/

// The allocator the published addon links, so a measurement that allocation
// binds matches what a consumer gets. Rust links a dev-dependency only where a
// target names it, so this line is what makes the choice real.
use swc_malloc as _;

use std::hint::black_box;

use criterion::{
  BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
use stylex_css_parser::{properties::*, token_parser::TokenParser};

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
/// before this helper existed. The check found six
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

fn border_radius_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("BorderRadius");

  let border_radius_inputs = [
    "10px",
    "10px 20px",
    "10px 20px 30px",
    "10px 20px 30px 40px",
    "10px / 20px",
    "10px 15px / 20px 25px",
    "50%",
    "1em 2em 3em 4em / 0.5em 1.5em 2.5em 3.5em",
  ];

  for (index, input) in border_radius_inputs.into_iter().enumerate() {
    let suffix = input.replace([' ', '/'], "_");

    bench_parse(
      &mut group,
      format!("individual_border_radius_{}_{}", index, suffix),
      input,
      BorderRadiusIndividual::parser,
    );

    bench_parse(
      &mut group,
      format!("shorthand_border_radius_{}_{}", index, suffix),
      input,
      BorderRadiusShorthand::parser,
    );
  }

  group.finish();
}

fn box_shadow_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("BoxShadow");

  // The colour is not optional, here or in the reference, so the first three
  // shapes carry one. Without it they were refused and the three cases timed
  // the refusal.
  let box_shadow_inputs = [
    "10px 10px red",
    "10px 10px 5px red",
    "10px 10px 5px 2px red",
    "10px 10px 5px 2px rgba(0,0,0,0.3)",
    "inset 10px 10px 5px 2px rgba(0,0,0,0.3)",
    "0 2px 4px rgba(0,0,0,0.1)",
    "0 4px 8px rgba(0,0,0,0.12), 0 2px 4px rgba(0,0,0,0.08)",
    "inset 0 1px 3px rgba(0,0,0,0.3), 0 1px rgba(255,255,255,0.1)",
  ];

  for (index, input) in box_shadow_inputs.into_iter().enumerate() {
    bench_parse(
      &mut group,
      format!("single_box_shadow_{}_{}", index, input.len()),
      input,
      BoxShadow::parser,
    );

    bench_parse(
      &mut group,
      format!("box_shadow_list_{}_{}", index, input.len()),
      input,
      BoxShadowList::parser,
    );
  }

  group.finish();
}

fn transform_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("Transform");

  let transform_inputs = [
    "translate(10px, 20px)",
    "rotate(45deg)",
    "scale(1.5)",
    "scale(2, 0.5)",
    "skew(45deg)",
    "skew(45deg, 30deg)",
    "matrix(1, 0, 0, 1, 10, 20)",
    "translateX(50px)",
    "translateY(100px)",
    "translateZ(25px)",
    "translate3d(10px, 20px, 30px)",
    "rotateX(45deg)",
    "rotateY(90deg)",
    "rotateZ(180deg)",
    "rotate3d(1, 1, 1, 45deg)",
    "scaleX(2)",
    "scaleY(0.5)",
    "scaleZ(1.5)",
    "scale3d(2, 0.5, 1.5)",
    "perspective(1000px)",
    "translate(10px, 20px) rotate(45deg) scale(1.2)",
    "matrix3d(1,0,0,0,0,1,0,0,0,0,1,0,10,20,30,1)",
  ];

  for (index, input) in transform_inputs.into_iter().enumerate() {
    bench_parse(
      &mut group,
      format!(
        "transform_{}_{}",
        index,
        input.split('(').next().unwrap_or("unknown")
      ),
      input,
      Transform::parser,
    );
  }

  group.finish();
}

/// A transform chain of three functions, used by the creation and the display
/// case.
///
/// An empty `Transform` moves an empty `Vec` into a one-field struct, so it
/// allocates nothing and writes nothing: both cases then time about two cycles
/// and report no work. This value makes each case build, and write, what its
/// group name claims.
fn sample_transform() -> Transform {
  use stylex_css_parser::css_types::{
    Angle, Axis, Length, LengthPercentage, RotateXYZ, Scale3d, TransformFunction, Translate3d,
  };

  Transform::new(vec![
    TransformFunction::Translate3d(Translate3d::new(
      LengthPercentage::Length(Length::new(10.0, "px")),
      LengthPercentage::Length(Length::new(20.0, "px")),
      Length::new(30.0, "px"),
    )),
    TransformFunction::RotateXYZ(RotateXYZ::new(Angle::new(45.0, "deg"), Axis::X)),
    TransformFunction::Scale3d(Scale3d::new(1.2, 0.8, 1.5)),
  ])
}

fn property_creation_benchmarks(c: &mut Criterion) {
  use stylex_css_parser::css_types::{Color, Length, LengthPercentage, NamedColor};

  let mut group = c.benchmark_group("PropertyCreation");

  // Each check reads the value the case builds, once, outside `b.iter`. A
  // constructor that stopped filling its fields would otherwise get faster and
  // report a win.
  let border_radius = {
    let length = Length::new(10.0, "px".to_string());
    BorderRadiusIndividual::new(LengthPercentage::Length(length), None)
  };
  assert_eq!(
    border_radius.horizontal,
    LengthPercentage::Length(Length::new(10.0, "px"))
  );

  group.bench_function("border_radius_individual_create", |b| {
    b.iter(|| {
      let length = Length::new(10.0, "px".to_string());
      let lp = LengthPercentage::Length(length);
      black_box(BorderRadiusIndividual::new(lp, None))
    })
  });

  let box_shadow = BoxShadow::new(
    Length::new(10.0, "px".to_string()),
    Length::new(10.0, "px".to_string()),
    Length::new(5.0, "px".to_string()),
    Length::new(2.0, "px".to_string()),
    Color::Named(NamedColor::new("black".to_string())),
    false,
  );
  assert_eq!(box_shadow.blur_radius, Length::new(5.0, "px"));

  group.bench_function("box_shadow_create", |b| {
    b.iter(|| {
      let offset_x = Length::new(10.0, "px".to_string());
      let offset_y = Length::new(10.0, "px".to_string());
      let blur = Length::new(5.0, "px".to_string());
      let spread = Length::new(2.0, "px".to_string());
      let color = Color::Named(NamedColor::new("black".to_string()));

      black_box(BoxShadow::new(
        offset_x, offset_y, blur, spread, color, false,
      ))
    })
  });

  assert_eq!(sample_transform().value.len(), 3);

  group.bench_function("transform_create", |b| {
    b.iter(|| black_box(sample_transform()))
  });

  group.finish();
}

fn property_display_benchmarks(c: &mut Criterion) {
  use stylex_css_parser::css_types::{Color, Length, LengthPercentage, NamedColor};

  let mut group = c.benchmark_group("PropertyDisplay");

  // Each check reads the string the case writes, once, outside `b.iter`. A
  // `Display` impl that stopped writing would otherwise get faster and report a
  // win.
  group.bench_function("border_radius_display", |b| {
    let length = Length::new(10.0, "px".to_string());
    let lp = LengthPercentage::Length(length);
    let border_radius = BorderRadiusIndividual::new(lp, None);
    assert_eq!(border_radius.to_string(), "10px");

    b.iter(|| black_box(border_radius.to_string()))
  });

  group.bench_function("box_shadow_display", |b| {
    let offset_x = Length::new(10.0, "px".to_string());
    let offset_y = Length::new(10.0, "px".to_string());
    let blur = Length::new(5.0, "px".to_string());
    let spread = Length::new(2.0, "px".to_string());
    let color = Color::Named(NamedColor::new("black".to_string()));
    let box_shadow = BoxShadow::new(offset_x, offset_y, blur, spread, color, false);
    assert_eq!(box_shadow.to_string(), "10px 10px 5px 2px black");

    b.iter(|| black_box(box_shadow.to_string()))
  });

  group.bench_function("transform_display", |b| {
    let transform = sample_transform();
    assert_eq!(
      transform.to_string(),
      "translate3d(10px, 20px, 30px) rotateX(45deg) scale3d(1.2, 0.8, 1.5)"
    );

    b.iter(|| black_box(transform.to_string()))
  });

  group.finish();
}

fn complex_property_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("ComplexProperties");

  bench_parse(
    &mut group,
    "complex_border_radius_shorthand".to_string(),
    "10px 20px 30px 40px / 50px 60px 70px 80px",
    BorderRadiusShorthand::parser,
  );

  bench_parse(
    &mut group,
    "complex_box_shadow_list".to_string(),
    "0 2px 4px rgba(0,0,0,0.1), 0 8px 16px rgba(0,0,0,0.2), inset 0 1px 0 rgba(255,255,255,0.1)",
    BoxShadowList::parser,
  );

  bench_parse(
    &mut group,
    "complex_transform_chain".to_string(),
    "translate3d(10px, 20px, 30px) rotateX(45deg) rotateY(30deg) rotateZ(60deg) scale3d(1.2, 0.8, 1.5)",
    Transform::parser,
  );

  group.finish();
}

criterion_group!(
  properties_benches,
  border_radius_benchmarks,
  box_shadow_benchmarks,
  transform_benchmarks,
  property_creation_benchmarks,
  property_display_benchmarks,
  complex_property_benchmarks
);

criterion_main!(properties_benches);
