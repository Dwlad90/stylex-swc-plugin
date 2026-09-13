/*!
Token Parser Benchmarks for StyleX CSS Parser.

These benchmarks test the performance of our token parser combinators
and compare with simple parsing approaches.

Provides benchmarks for token parsing functionality and performance.
*/

// The allocator the published addon links, so a measurement that allocation
// binds matches what a consumer gets. Rust links a dev-dependency only where a
// target names it, so this line is what makes the choice real.
use swc_malloc as _;

use std::{fmt::Debug, hint::black_box};

use criterion::{
  BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
use stylex_css_parser::{
  CssParseError,
  token_parser::{Either, TokenParser},
  token_types::TokenList,
};

/// Times one parser, after a check that it still answers what it answered when
/// the case was written.
///
/// The check runs once, outside `b.iter`, so it adds nothing to the
/// measurement: the parser is built from literals and reads the same input on
/// every iteration, so one answer speaks for all of them.
///
/// `expected` carries the refusals too. Three cases in this file time an error
/// path on purpose, and their names say so; each one states `Err(())` here, so
/// a parser that starts answering `Ok` cannot pass as a win.
fn bench_answer<T: Clone + Debug + PartialEq + 'static>(
  group: &mut BenchmarkGroup<'_, WallTime>,
  name: &str,
  expected: Result<T, ()>,
  run: fn() -> Result<T, CssParseError>,
) {
  assert_eq!(
    run().map_err(|_| ()),
    expected,
    "{name} no longer answers what the case was written to time"
  );

  group.bench_function(name, |b| b.iter(|| black_box(run())));
}

fn basic_parser_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("BasicParsers");

  bench_answer(&mut group, "always_parser", Ok(42), || {
    let parser = TokenParser::always(black_box(42));
    parser.parse("anything")
  });

  bench_answer(&mut group, "never_parser", Err(()), || {
    let parser = TokenParser::<i32>::never();
    parser.parse("anything")
  });

  bench_answer(&mut group, "optional_parser", Ok(Some(42)), || {
    let parser = TokenParser::always(42).optional();
    parser.parse("test")
  });

  group.finish();
}

fn combinator_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("Combinators");

  bench_answer(&mut group, "map_transformation", Ok(10), || {
    let parser = TokenParser::always(black_box(5)).map(|x| x * 2, Some("double"));
    parser.parse("test")
  });

  bench_answer(&mut group, "flat_map_chaining", Ok(15), || {
    let parser =
      TokenParser::always(black_box(5)).flat_map(|x| TokenParser::always(x * 3), Some("triple"));
    parser.parse("test")
  });

  bench_answer(&mut group, "where_clause_filtering", Ok(10), || {
    let parser = TokenParser::always(black_box(10)).where_fn(|&x| x > 5, Some("greater_than_5"));
    parser.parse("test")
  });

  bench_answer(&mut group, "or_combination", Ok(Either::Left(1)), || {
    let parser1 = TokenParser::always(1);
    let parser2 = TokenParser::always(2);
    let combined = parser1.or(parser2);
    combined.parse("test")
  });

  group.finish();
}

fn sequence_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("Sequences");

  bench_answer(
    &mut group,
    "simple_sequence",
    Ok(vec!["a".to_string(), "b".to_string(), "c".to_string()]),
    || {
      let parser = TokenParser::<String>::sequence(vec![
        TokenParser::always("a".to_string()),
        TokenParser::always("b".to_string()),
        TokenParser::always("c".to_string()),
      ]);
      parser.parse("test")
    },
  );

  bench_answer(
    &mut group,
    "long_sequence",
    Ok((0..10).map(|i| format!("item_{}", i)).collect::<Vec<_>>()),
    || {
      let parsers: Vec<_> = (0..10)
        .map(|i| TokenParser::always(format!("item_{}", i)))
        .collect();
      let parser = TokenParser::<String>::sequence(parsers);
      parser.parse("test")
    },
  );

  bench_answer(&mut group, "one_of_small", Ok("first".to_string()), || {
    let parser = TokenParser::one_of(vec![
      TokenParser::always("first".to_string()),
      TokenParser::always("second".to_string()),
      TokenParser::always("third".to_string()),
    ]);
    parser.parse("test")
  });

  bench_answer(
    &mut group,
    "one_of_large",
    Ok("option_0".to_string()),
    || {
      let parsers: Vec<_> = (0..20)
        .map(|i| TokenParser::always(format!("option_{}", i)))
        .collect();
      let parser = TokenParser::one_of(parsers);
      parser.parse("test")
    },
  );

  group.finish();
}

/// Both cases here run a parser that can never match. That is deliberate, and
/// the names say so, so each one states the answer it expects rather than
/// dropping it.
fn repetition_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("Repetition");

  bench_answer(&mut group, "zero_or_more_empty", Ok(vec![]), || {
    let parser = TokenParser::zero_or_more(TokenParser::<String>::never());
    parser.parse("")
  });

  bench_answer(
    &mut group,
    "one_or_more_failure",
    Err::<Vec<String>, ()>(()),
    || {
      let parser = TokenParser::one_or_more(TokenParser::<String>::never());
      parser.parse("test")
    },
  );

  group.finish();
}

fn token_list_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("TokenList");

  let css_samples = [
    "color: red;",
    "margin: 10px 20px;",
    "transform: rotate(45deg) scale(1.2);",
    "background: linear-gradient(to right, #ff0000, #00ff00);",
    "font-family: 'Helvetica Neue', Arial, sans-serif;",
    "box-shadow: 0 2px 4px rgba(0,0,0,0.1), 0 8px 16px rgba(0,0,0,0.1);",
  ];

  for (i, sample) in css_samples.into_iter().enumerate() {
    // A tokenizer that stopped cutting tokens would report a win, so each pair
    // of cases states the count its sample produces. The count is taken once,
    // outside `b.iter`.
    let expected_tokens = consume_all(sample);
    assert!(
      expected_tokens > 0,
      "sample {i} tokenizes to nothing, so both cases would time an empty walk"
    );

    group.bench_with_input(format!("tokenize_css_{}", i), sample, |b, sample| {
      b.iter(|| {
        let token_list = TokenList::new(black_box(sample));
        black_box(token_list)
      })
    });

    group.bench_with_input(format!("consume_tokens_{}", i), sample, |b, sample| {
      b.iter(|| {
        let mut token_list = TokenList::new(black_box(sample));
        let mut tokens = Vec::new();
        while !token_list.is_empty() {
          if let Ok(Some(token)) = token_list.consume_next_token() {
            tokens.push(token);
          } else {
            break;
          }
        }
        black_box(tokens)
      })
    });
  }

  group.finish();
}

/// The walk `consume_tokens_*` times, counted rather than collected. Used by
/// the check only, never inside a timed closure.
fn consume_all(sample: &str) -> usize {
  let mut token_list = TokenList::new(sample);
  let mut count = 0;

  while !token_list.is_empty() {
    match token_list.consume_next_token() {
      Ok(Some(_)) => count += 1,
      _ => break,
    }
  }

  count
}

fn complex_parsing_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("ComplexParsing");

  bench_answer(
    &mut group,
    "nested_transformations",
    Ok("12".to_string()),
    || {
      let parser = TokenParser::always(black_box(1))
        .map(|x| x * 2, Some("double"))
        .map(|x| x + 10, Some("add_ten"))
        .map(|x| x.to_string(), Some("to_string"))
        .where_fn(|s| s.len() > 1, Some("length_check"));
      parser.parse("test")
    },
  );

  bench_answer(
    &mut group,
    "surrounded_by_parsing",
    Ok("content".to_string()),
    || {
      let content = TokenParser::always("content".to_string());
      let prefix = TokenParser::always("(".to_string());
      let suffix = TokenParser::always(")".to_string());
      let parser = content.surrounded_by(prefix, Some(suffix));
      parser.parse("(content)")
    },
  );

  bench_answer(
    &mut group,
    "parser_composition",
    Ok(("42:test".to_string(), true)),
    || {
      // Build a complex parser from simple parts
      let number_parser = TokenParser::always(black_box(42));
      let string_parser = TokenParser::always("test".to_string());
      let bool_parser = TokenParser::always(true);

      let combined = number_parser
        .flat_map(
          move |n| string_parser.map(move |s| format!("{}:{}", n, s), None),
          None,
        )
        .flat_map(
          move |s| bool_parser.map(move |b| (s.clone(), b), None),
          None,
        );

      combined.parse("input")
    },
  );

  group.finish();
}

criterion_group!(
  token_parser_benches,
  basic_parser_benchmarks,
  combinator_benchmarks,
  sequence_benchmarks,
  repetition_benchmarks,
  token_list_benchmarks,
  complex_parsing_benchmarks
);

criterion_main!(token_parser_benches);
