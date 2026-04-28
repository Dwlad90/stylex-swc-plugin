use std::{cell::RefCell, hint::black_box, rc::Rc};

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use stylex_styleq::{
  COMPILED_KEY, StyleMap, StyleValue, StyleqInput, StyleqOptions, create_styleq,
};

fn string(value: &str) -> StyleValue {
  StyleValue::string(value)
}

fn compiled(entries: &[(&str, StyleValue)]) -> StyleqInput<StyleValue> {
  let mut style = StyleMap::new();
  style.insert(COMPILED_KEY.to_string(), StyleValue::Bool(true));

  for (key, value) in entries {
    style.insert((*key).to_string(), value.clone());
  }

  StyleqInput::Style(style)
}

fn inline(entries: &[(&str, StyleValue)]) -> StyleqInput<StyleValue> {
  StyleqInput::Style(
    entries
      .iter()
      .map(|(key, value)| ((*key).to_string(), value.clone()))
      .collect(),
  )
}

fn basic_style_fixture_1() -> StyleqInput<StyleValue> {
  compiled(&[
    ("backgroundColor", string("backgroundColor-1")),
    ("color", string("color-1")),
  ])
}

fn basic_style_fixture_2() -> StyleqInput<StyleValue> {
  compiled(&[
    ("backgroundColor", string("backgroundColor-2")),
    ("color", string("color-2")),
  ])
}

fn big_style_fixture() -> StyleqInput<StyleValue> {
  compiled(&[
    ("backgroundColor", string("backgroundColor-3")),
    ("borderColor", string("borderColor-3")),
    ("borderStyle", string("borderStyle-3")),
    ("borderWidth", string("borderWidth-3")),
    ("boxSizing", string("boxSizing-3")),
    ("display", string("display-3")),
    ("listStyle", string("listStyle-3")),
    ("marginBottom", string("marginBottom-3")),
    ("marginInlineEnd", string("marginInlineEnd-3")),
    ("marginInlineStart", string("marginInlineStart-3")),
    ("marginLeft", string("marginLeft-3")),
    ("marginRight", string("marginRight-3")),
    ("marginTop", string("marginTop-3")),
    ("paddingBottom", string("paddingBottom-3")),
    ("paddingInlineEnd", string("paddingInlineEnd-3")),
    ("paddingInlineStart", string("paddingInlineStart-3")),
    ("paddingLeft", string("paddingLeft-3")),
    ("paddingRight", string("paddingRight-3")),
    ("paddingTop", string("paddingTop-3")),
    ("textAlign", string("textAlign-3")),
    ("textDecoration", string("textDecoration-3")),
    ("whiteSpace", string("whiteSpace-3")),
    ("wordWrap", string("wordWrap-3")),
    ("zIndex", string("zIndex-3")),
  ])
}

fn big_style_with_pseudos_fixture() -> StyleqInput<StyleValue> {
  compiled(&[
    ("backgroundColor", string("backgroundColor-4")),
    ("border", string("border-4")),
    ("color", string("color-4")),
    ("cursor", string("cursor-4")),
    ("display", string("display-4")),
    ("fontFamily", string("fontFamily-4")),
    ("fontSize", string("fontSize-4")),
    ("lineHeight", string("lineHeight-4")),
    ("marginEnd", string("marginEnd-4")),
    ("marginStart", string("marginStart-4")),
    ("paddingEnd", string("paddingEnd-4")),
    ("paddingStart", string("paddingStart-4")),
    ("textAlign", string("textAlign-4")),
    ("textDecoration", string("textDecoration-4")),
    (":focus$color", string("focus$color-4")),
    (":focus$textDecoration", string("focus$textDecoration-4")),
    (":active$transform", string("active$transform-4")),
    (":active$transition", string("active$transition-4")),
  ])
}

fn complex_nested_style_fixture() -> StyleqInput<StyleValue> {
  StyleqInput::Nested(vec![
    big_style_fixture(),
    StyleqInput::False,
    StyleqInput::False,
    StyleqInput::False,
    StyleqInput::False,
    StyleqInput::Nested(vec![
      compiled(&[
        ("cursor", string("cursor-a")),
        ("touchAction", string("touchAction-a")),
      ]),
      StyleqInput::False,
      compiled(&[("outline", string("outline-b"))]),
      StyleqInput::Nested(vec![
        compiled(&[
          ("cursor", string("cursor-c")),
          ("touchAction", string("touchAction-c")),
        ]),
        StyleqInput::False,
        StyleqInput::False,
        compiled(&[
          ("textDecoration", string("textDecoration-d")),
          (":focus$textDecoration", string("focus$textDecoration-d")),
        ]),
        StyleqInput::False,
        StyleqInput::Nested(vec![
          big_style_with_pseudos_fixture(),
          compiled(&[
            ("display", string("display-e")),
            ("width", string("width-e")),
          ]),
          StyleqInput::Nested(vec![compiled(&[(
            ":active$transform",
            string("active$transform-f"),
          )])]),
        ]),
      ]),
    ]),
  ])
}

fn merged_inline_fixture() -> [StyleqInput<StyleValue>; 2] {
  [
    inline(&[
      ("backgroundColor", string("blue")),
      ("borderColor", string("blue")),
      ("display", string("block")),
    ]),
    inline(&[
      ("backgroundColor", string("red")),
      ("borderColor", string("red")),
      ("borderStyle", string("solid")),
      ("borderWidth", string("1px")),
      ("boxSizing", string("border-bx")),
      ("display", string("flex")),
      ("listStyle", string("none")),
      ("marginTop", string("0")),
      ("marginEnd", string("0")),
      ("marginBottom", string("0")),
      ("marginStart", string("0")),
      ("paddingTop", string("0")),
      ("paddingEnd", string("0")),
      ("paddingBottom", string("0")),
      ("paddingStart", string("0")),
      ("textAlign", string("start")),
      ("textDecoration", string("none")),
      ("whiteSpace", string("pre")),
      ("zIndex", string("0")),
    ]),
  ]
}

fn styleq_transform() -> stylex_styleq::Styleq<StyleValue> {
  let transform_cache = Rc::new(RefCell::new(Vec::<(
    StyleMap<StyleValue>,
    StyleMap<StyleValue>,
  )>::new()));
  let transform_cache_for_closure = Rc::clone(&transform_cache);

  create_styleq(StyleqOptions {
    transform: Some(Rc::new(move |style: StyleMap<StyleValue>| {
      {
        let transform_cache = transform_cache_for_closure.borrow();
        if let Some((_, cached_style)) = transform_cache
          .iter()
          .find(|(cached_input, _)| cached_input == &style)
        {
          return cached_style.clone();
        }
      }

      let mut flex_style = style.clone();
      flex_style.insert("display".to_string(), string("display-flex"));
      transform_cache_for_closure
        .borrow_mut()
        .push((style, flex_style.clone()));
      flex_style
    })),
    ..Default::default()
  })
}

fn performance_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("styleq");
  let default_styleq = create_styleq(StyleqOptions::default());
  let styleq_no_cache = create_styleq(StyleqOptions {
    disable_cache: true,
    ..Default::default()
  });
  let styleq_no_mix = create_styleq(StyleqOptions {
    disable_mix: true,
    ..Default::default()
  });
  let styleq_transform = styleq_transform();
  let basic_style_1 = basic_style_fixture_1();
  let basic_style_2 = basic_style_fixture_2();
  let big_style = big_style_fixture();
  let complex_nested_style = StyleqInput::Nested(vec![complex_nested_style_fixture()]);
  let small_merge = [basic_style_1.clone(), basic_style_2];

  group.bench_function("small object", |b| {
    b.iter(|| black_box(default_styleq.styleq(black_box(std::slice::from_ref(&basic_style_1)))))
  });

  group.bench_function("small object (cache miss)", |b| {
    b.iter_batched(
      basic_style_fixture_1,
      |fixture| black_box(default_styleq.styleq(black_box(&[fixture]))),
      BatchSize::SmallInput,
    )
  });

  group.bench_function("small object (cache disabled)", |b| {
    b.iter_batched(
      basic_style_fixture_1,
      |fixture| black_box(styleq_no_cache.styleq(black_box(&[fixture]))),
      BatchSize::SmallInput,
    )
  });

  group.bench_function("large object", |b| {
    b.iter(|| black_box(default_styleq.styleq(black_box(std::slice::from_ref(&big_style)))))
  });

  group.bench_function("large object (cache miss)", |b| {
    b.iter_batched(
      big_style_fixture,
      |fixture| black_box(default_styleq.styleq(black_box(&[fixture]))),
      BatchSize::SmallInput,
    )
  });

  group.bench_function("large object (cache disabled)", |b| {
    b.iter_batched(
      big_style_fixture,
      |fixture| black_box(styleq_no_cache.styleq(black_box(&[fixture]))),
      BatchSize::SmallInput,
    )
  });

  group.bench_function("small merge", |b| {
    b.iter(|| black_box(default_styleq.styleq(black_box(&small_merge))))
  });

  group.bench_function("small merge (cache miss)", |b| {
    b.iter_batched(
      || [basic_style_fixture_1(), basic_style_fixture_2()],
      |fixtures| black_box(default_styleq.styleq(black_box(&fixtures))),
      BatchSize::SmallInput,
    )
  });

  group.bench_function("small merge (cache disabled)", |b| {
    b.iter(|| black_box(styleq_no_cache.styleq(black_box(&small_merge))))
  });

  group.bench_function("large merge", |b| {
    b.iter(|| {
      black_box(default_styleq.styleq(black_box(std::slice::from_ref(&complex_nested_style))))
    })
  });

  group.bench_function("large merge (cache disabled)", |b| {
    b.iter(|| {
      black_box(styleq_no_cache.styleq(black_box(std::slice::from_ref(&complex_nested_style))))
    })
  });

  group.bench_function("large merge (transform)", |b| {
    b.iter(|| {
      black_box(styleq_transform.styleq(black_box(std::slice::from_ref(&complex_nested_style))))
    })
  });

  group.bench_function("small inline style", |b| {
    b.iter_batched(
      || inline(&[("backgroundColor", string("red"))]),
      |fixture| black_box(default_styleq.styleq(black_box(&[fixture]))),
      BatchSize::SmallInput,
    )
  });

  group.bench_function("large inline style", |b| {
    b.iter_batched(
      || {
        inline(&[
          ("backgroundColor", string("red")),
          ("borderColor", string("red")),
          ("borderStyle", string("solid")),
          ("borderWidth", string("1px")),
          ("boxSizing", string("border-bx")),
          ("display", string("flex")),
          ("listStyle", string("none")),
          ("marginTop", string("0")),
          ("marginEnd", string("0")),
          ("marginBottom", string("0")),
          ("marginStart", string("0")),
          ("paddingTop", string("0")),
          ("paddingEnd", string("0")),
          ("paddingBottom", string("0")),
          ("paddingStart", string("0")),
          ("textAlign", string("start")),
          ("textDecoration", string("none")),
          ("whiteSpace", string("pre")),
          ("zIndex", string("0")),
        ])
      },
      |fixture| black_box(default_styleq.styleq(black_box(&[fixture]))),
      BatchSize::SmallInput,
    )
  });

  group.bench_function("merged inline style", |b| {
    b.iter_batched(
      merged_inline_fixture,
      |fixtures| black_box(default_styleq.styleq(black_box(&fixtures))),
      BatchSize::SmallInput,
    )
  });

  group.bench_function("merged inline style (mix disabled)", |b| {
    b.iter_batched(
      merged_inline_fixture,
      |fixtures| black_box(styleq_no_mix.styleq(black_box(&fixtures))),
      BatchSize::SmallInput,
    )
  });

  group.finish();
}

criterion_group!(performance_benches, performance_benchmarks);
criterion_main!(performance_benches);
