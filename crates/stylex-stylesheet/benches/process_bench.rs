//! What the stylesheet step costs on the rules of a large build.
//!
//! `processStylexRules` runs once per build, over every rule every module
//! emitted, so its cost grows with the size of the application. Three sizes
//! show how it grows: a small application, a large one, and the largest the
//! benchmark corpus holds. Four shapes show which part of the step costs the
//! most: plain rules, rules that read constants, rules with an RTL text, and
//! the legacy class-name sort.
//!
//! Each subject is checked once, outside the measured loop, so a change that
//! breaks the sheet stops the bench instead of making it faster.

// The allocator the published addon links, so a measurement that allocation
// binds matches what a consumer gets.
use swc_malloc as _;

use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use stylex_js::value::JsValue;
use stylex_stylesheet::process::{ProcessStylexRulesConfig, Rule, RuleStyle, process_stylex_rules};

/// Rules per build, five times apart, so a linear cost reads differently from
/// an `n log n` one.
const SIZES: [usize; 3] = [1_000, 10_000, 50_000];

/// The properties a rule declares, in rotation. Rules for the same property
/// make the comparator go past the property name to the width key and text.
const PROPERTIES: [&str; 8] = [
  "color",
  "margin-top",
  "padding-left",
  "display",
  "width",
  "background-color",
  "font-size",
  "border-radius",
];

/// The at-rule a rule sits in, in rotation: none, or one of three widths.
const WRAPPERS: [Option<&str>; 4] = [
  None,
  Some("@media (min-width: 768px)"),
  Some("@media (max-width: 1024px)"),
  Some("@supports (display: grid)"),
];

/// The shapes of rules a build emits.
#[derive(Clone, Copy)]
enum Shape {
  /// Style rules only.
  Plain,
  /// One rule in fifty declares a constant, and one in ten reads one.
  Constants,
  /// One rule in five has an RTL text.
  Rtl,
  /// Plain rules under `useLegacyClassnamesSort`.
  Legacy,
}

impl Shape {
  fn name(self) -> &'static str {
    match self {
      Shape::Plain => "plain",
      Shape::Constants => "constants",
      Shape::Rtl => "rtl",
      Shape::Legacy => "legacy",
    }
  }
}

/// A class name like the ones the compiler writes: `x` and a base-36 hash.
fn class_name(index: usize) -> String {
  let mut hash = index.wrapping_mul(2_654_435_761) % 2_176_782_336;
  let mut name = String::from("x");

  for _ in 0..6 {
    let digit = (hash % 36) as u32;
    name.push(char::from_digit(digit, 36).unwrap_or('0'));
    hash /= 36;
  }

  name
}

fn text(class_name: &str, index: usize, value: &str) -> String {
  let property = PROPERTIES[index % PROPERTIES.len()];
  let declaration = format!(".{class_name}{{{property}:{value}}}");

  match WRAPPERS[(index / PROPERTIES.len()) % WRAPPERS.len()] {
    Some(wrapper) => format!("{wrapper}{{{declaration}}}"),
    None => declaration,
  }
}

fn style(ltr: String, rtl: Option<String>) -> RuleStyle {
  RuleStyle {
    ltr: JsValue::String(ltr),
    rtl: rtl.map_or(JsValue::Null, JsValue::String),
    ..RuleStyle::default()
  }
}

fn constant(name: String, value: String) -> RuleStyle {
  RuleStyle {
    const_key: JsValue::String(name),
    const_val: JsValue::String(value),
    ..RuleStyle::default()
  }
}

/// `size` rules of one shape, with priorities spread over four levels.
fn rules_of(shape: Shape, size: usize) -> Vec<Rule> {
  (0..size)
    .map(|index| {
      let class_name = class_name(index);
      let priority = [3000.0, 3500.0, 4000.0, 1000.0][index % 4];

      let style = match shape {
        Shape::Constants if index % 50 == 0 => {
          return Rule(
            class_name.as_str().into(),
            constant(class_name.clone(), format!("{}px", index % 97)),
            0.0,
          );
        },
        Shape::Constants if index % 10 == 5 => {
          let referenced = class_name_of_constant(index);
          style(
            text(&class_name, index, &format!("var(--{referenced})")),
            None,
          )
        },
        Shape::Rtl if index % 5 == 0 => style(
          text(&class_name, index, "left"),
          Some(text(&class_name, index, "right")),
        ),
        _ => style(text(&class_name, index, &format!("{}px", index % 13)), None),
      };

      Rule(class_name.as_str().into(), style, priority)
    })
    .collect()
}

/// The class name of the constant declared closest before `index`.
fn class_name_of_constant(index: usize) -> String {
  class_name(index - index % 50)
}

fn config_of(shape: Shape) -> ProcessStylexRulesConfig {
  ProcessStylexRulesConfig {
    use_legacy_classnames_sort: matches!(shape, Shape::Legacy),
    ..ProcessStylexRulesConfig::default()
  }
}

fn bench_process_stylex_rules(c: &mut Criterion) {
  let mut group = c.benchmark_group("process_stylex_rules");

  for shape in [Shape::Plain, Shape::Constants, Shape::Rtl, Shape::Legacy] {
    for size in SIZES {
      let rules = rules_of(shape, size);
      let config = config_of(shape);

      // The sheet holds every rule that is not a constant at least once.
      let sheet = match process_stylex_rules(&rules, &config) {
        Ok(sheet) => sheet,
        Err(error) => panic!("{} at {size}: {error}", shape.name()),
      };
      assert!(
        sheet.lines().count() >= size * 9 / 10,
        "the sheet is too short"
      );

      group.bench_with_input(BenchmarkId::new(shape.name(), size), &rules, |b, rules| {
        b.iter(|| process_stylex_rules(black_box(rules), black_box(&config)))
      });
    }
  }

  group.finish();
}

criterion_group!(benches, bench_process_stylex_rules);
criterion_main!(benches);
