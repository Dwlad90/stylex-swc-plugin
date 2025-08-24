/*!
CSS Properties Benchmarks for StyleX CSS Parser.
These benchmarks test the performance of complex CSS property parsing,
which involves combining multiple CSS types into coherent property values.
*/

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stylex_css_parser::properties::*;

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

  for input in &border_radius_inputs {
    group.bench_with_input(
      format!(
        "individual_border_radius_{}",
        input.replace([' ', '/'], "_")
      ),
      input,
      |b, input| {
        b.iter(|| {
          let parser = BorderRadiusIndividual::parser();
          black_box(parser.parse(black_box(input)))
        })
      },
    );

    group.bench_with_input(
      format!("shorthand_border_radius_{}", input.replace([' ', '/'], "_")),
      input,
      |b, input| {
        b.iter(|| {
          let parser = BorderRadiusShorthand::parser();
          black_box(parser.parse(black_box(input)))
        })
      },
    );
  }

  group.finish();
}

fn box_shadow_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("BoxShadow");

  let box_shadow_inputs = [
    "10px 10px",
    "10px 10px 5px",
    "10px 10px 5px 2px",
    "10px 10px 5px 2px rgba(0,0,0,0.3)",
    "inset 10px 10px 5px 2px rgba(0,0,0,0.3)",
    "0 2px 4px rgba(0,0,0,0.1)",
    "0 4px 8px rgba(0,0,0,0.12), 0 2px 4px rgba(0,0,0,0.08)",
    "inset 0 1px 3px rgba(0,0,0,0.3), 0 1px rgba(255,255,255,0.1)",
  ];

  for input in &box_shadow_inputs {
    group.bench_with_input(
      format!("single_box_shadow_{}", input.len()),
      input,
      |b, input| {
        b.iter(|| {
          let parser = BoxShadow::parser();
          black_box(parser.parse(black_box(input)))
        })
      },
    );

    group.bench_with_input(
      format!("box_shadow_list_{}", input.len()),
      input,
      |b, input| {
        b.iter(|| {
          let parser = BoxShadowList::parser();
          black_box(parser.parse(black_box(input)))
        })
      },
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

  for input in &transform_inputs {
    group.bench_with_input(
      format!("transform_{}", input.split('(').next().unwrap_or("unknown")),
      input,
      |b, input| {
        b.iter(|| {
          let parser = Transform::parser();
          black_box(parser.parse(black_box(input)))
        })
      },
    );
  }

  group.finish();
}

fn property_creation_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("PropertyCreation");

  group.bench_function("border_radius_individual_create", |b| {
    b.iter(|| {
      use stylex_css_parser::css_types::{Length, LengthPercentage};
      let length = Length::new(10.0, "px".to_string());
      let lp = LengthPercentage::Length(length);
      black_box(BorderRadiusIndividual::new(lp, None))
    })
  });

  group.bench_function("box_shadow_create", |b| {
    b.iter(|| {
      use stylex_css_parser::css_types::{Color, Length, NamedColor};
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

  group.bench_function("transform_create", |b| {
    b.iter(|| black_box(Transform::new(vec![])))
  });

  group.finish();
}

fn property_display_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("PropertyDisplay");

  group.bench_function("border_radius_display", |b| {
    use stylex_css_parser::css_types::{Length, LengthPercentage};
    let length = Length::new(10.0, "px".to_string());
    let lp = LengthPercentage::Length(length);
    let border_radius = BorderRadiusIndividual::new(lp, None);

    b.iter(|| black_box(border_radius.to_string()))
  });

  group.bench_function("box_shadow_display", |b| {
    use stylex_css_parser::css_types::{Color, Length, NamedColor};
    let offset_x = Length::new(10.0, "px".to_string());
    let offset_y = Length::new(10.0, "px".to_string());
    let blur = Length::new(5.0, "px".to_string());
    let spread = Length::new(2.0, "px".to_string());
    let color = Color::Named(NamedColor::new("black".to_string()));
    let box_shadow = BoxShadow::new(offset_x, offset_y, blur, spread, color, false);

    b.iter(|| black_box(box_shadow.to_string()))
  });

  group.bench_function("transform_display", |b| {
    let transform = Transform::new(vec![]);

    b.iter(|| black_box(transform.to_string()))
  });

  group.finish();
}

fn complex_property_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("ComplexProperties");

  group.bench_function("complex_border_radius_shorthand", |b| {
    let input = "10px 20px 30px 40px / 50px 60px 70px 80px";
    b.iter(|| {
      let parser = BorderRadiusShorthand::parser();
      black_box(parser.parse(black_box(input)))
    })
  });

  group.bench_function("complex_box_shadow_list", |b| {
    let input =
      "0 2px 4px rgba(0,0,0,0.1), 0 8px 16px rgba(0,0,0,0.2), inset 0 1px 0 rgba(255,255,255,0.1)";
    b.iter(|| {
      let parser = BoxShadowList::parser();
      black_box(parser.parse(black_box(input)))
    })
  });

  group.bench_function("complex_transform_chain", |b| {
        let input = "translate3d(10px, 20px, 30px) rotateX(45deg) rotateY(30deg) rotateZ(60deg) scale3d(1.2, 0.8, 1.5)";
        b.iter(|| {
            let parser = Transform::parser();
            black_box(parser.parse(black_box(input)))
        })
    });

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
