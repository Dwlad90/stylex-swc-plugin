/*!
Token Parser Benchmarks for StyleX CSS Parser.

These benchmarks test the performance of our token parser combinators
and compare with simple parsing approaches.

Provides benchmarks for token parsing functionality and performance.
*/

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stylex_css_parser::token_parser::TokenParser;
use stylex_css_parser::token_types::TokenList;

fn basic_parser_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("BasicParsers");

  group.bench_function("always_parser", |b| {
    b.iter(|| {
      let parser = TokenParser::always(black_box(42));
      black_box(parser.parse("anything"))
    })
  });

  group.bench_function("never_parser", |b| {
    b.iter(|| {
      let parser = TokenParser::<i32>::never();
      black_box(parser.parse("anything"))
    })
  });

  group.bench_function("optional_parser", |b| {
    b.iter(|| {
      let parser = TokenParser::always(42).optional();
      black_box(parser.parse("test"))
    })
  });

  group.finish();
}

fn combinator_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("Combinators");

  group.bench_function("map_transformation", |b| {
    b.iter(|| {
      let parser = TokenParser::always(black_box(5)).map(|x| x * 2, Some("double"));
      black_box(parser.parse("test"))
    })
  });

  group.bench_function("flat_map_chaining", |b| {
    b.iter(|| {
      let parser =
        TokenParser::always(black_box(5)).flat_map(|x| TokenParser::always(x * 3), Some("triple"));
      black_box(parser.parse("test"))
    })
  });

  group.bench_function("where_clause_filtering", |b| {
    b.iter(|| {
      let parser = TokenParser::always(black_box(10)).where_fn(|&x| x > 5, Some("greater_than_5"));
      black_box(parser.parse("test"))
    })
  });

  group.bench_function("or_combination", |b| {
    b.iter(|| {
      let parser1 = TokenParser::always(1);
      let parser2 = TokenParser::always(2);
      let combined = parser1.or(parser2);
      black_box(combined.parse("test"))
    })
  });

  group.finish();
}

fn sequence_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("Sequences");

  group.bench_function("simple_sequence", |b| {
    b.iter(|| {
      let parser = TokenParser::<String>::sequence(vec![
        TokenParser::always("a".to_string()),
        TokenParser::always("b".to_string()),
        TokenParser::always("c".to_string()),
      ]);
      black_box(parser.parse("test"))
    })
  });

  group.bench_function("long_sequence", |b| {
    b.iter(|| {
      let parsers: Vec<_> = (0..10)
        .map(|i| TokenParser::always(format!("item_{}", i)))
        .collect();
      let parser = TokenParser::<String>::sequence(parsers);
      black_box(parser.parse("test"))
    })
  });

  group.bench_function("one_of_small", |b| {
    b.iter(|| {
      let parser = TokenParser::one_of(vec![
        TokenParser::always("first".to_string()),
        TokenParser::always("second".to_string()),
        TokenParser::always("third".to_string()),
      ]);
      black_box(parser.parse("test"))
    })
  });

  group.bench_function("one_of_large", |b| {
    b.iter(|| {
      let parsers: Vec<_> = (0..20)
        .map(|i| TokenParser::always(format!("option_{}", i)))
        .collect();
      let parser = TokenParser::one_of(parsers);
      black_box(parser.parse("test"))
    })
  });

  group.finish();
}

fn repetition_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("Repetition");

  group.bench_function("zero_or_more_empty", |b| {
    b.iter(|| {
      let parser = TokenParser::zero_or_more(TokenParser::<String>::never());
      black_box(parser.parse(""))
    })
  });

  group.bench_function("one_or_more_failure", |b| {
    b.iter(|| {
      let parser = TokenParser::one_or_more(TokenParser::<String>::never());
      black_box(parser.parse("test"))
    })
  });

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

  for (i, sample) in css_samples.iter().enumerate() {
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

fn complex_parsing_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("ComplexParsing");

  group.bench_function("nested_transformations", |b| {
    b.iter(|| {
      let parser = TokenParser::always(black_box(1))
        .map(|x| x * 2, Some("double"))
        .map(|x| x + 10, Some("add_ten"))
        .map(|x| x.to_string(), Some("to_string"))
        .where_fn(|s| s.len() > 1, Some("length_check"));
      black_box(parser.parse("test"))
    })
  });

  group.bench_function("surrounded_by_parsing", |b| {
    b.iter(|| {
      let content = TokenParser::always("content".to_string());
      let prefix = TokenParser::always("(".to_string());
      let suffix = TokenParser::always(")".to_string());
      let parser = content.surrounded_by(prefix, Some(suffix));
      black_box(parser.parse("(content)"))
    })
  });

  group.bench_function("parser_composition", |b| {
    b.iter(|| {
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

      black_box(combined.parse("input"))
    })
  });

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
