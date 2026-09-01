//! What a chain of `+` costs as the chain grows, at a fixed amount of text.
//!
//! A concatenation is grown through a buffer that measures every append against
//! the character ceiling. The left operand of a link is everything the links
//! below it joined, so a buffer that read its own length again at every link
//! spent the whole of the accumulated text once per remaining link: the square
//! of a chain's text rather than its length. The count now travels with the
//! text and the link above adopts the buffer below it, so each operand is read
//! exactly once.
//!
//! That is what these points pin, and why every one of them joins the *same*
//! nine hundred thousand characters through a different number of links: what
//! varies between them is only how many times the accumulated text could be
//! read again. Re-reading the left side, the line climbs with the link count
//! while the text stays put -- 0.73, 2.33, 4.55, 9.17 ms across the four.
//! Adopting it, the same four are 0.72, 0.99, 1.33, 2.00 ms.
//!
//! The line does still climb, and is meant to. What is left is the memo's own
//! cost -- the key is a hash of the whole remaining subtree and is taken again
//! at every link -- plus the one copy per link that boxes a folded chain back
//! into the tree for the memo to hold. Both are
//! `stylex-evaluator/docs/adr/0005-the-memo-key-is-a-whole-subtree-hash.md` rather than
//! anything this file's subject can remove, and the neighbouring
//! `evaluate_depth_bench` is where that curve is pinned on its own.
//!
//! The default depth ceiling bounds a chain at 32 links, and the widest point
//! here is 20 -- so unlike the depth benchmark beside it, this measures lengths
//! a project can actually write.
//!
//! Every fold runs inside `GLOBALS.set`, because the fold can reach the
//! code-frame path and that path calls `Mark::new()`. Why that is not optional
//! is in `guidelines/PERFORMANCE.md` under "Writing a bench".

use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use stylex_evaluator::evaluate::evaluate;
use stylex_state::state_writers::fill_state_declarations;
use stylex_state::{
  evaluate_result_value::EvaluateResultValue, functions::FunctionMap, state_manager::StateManager,
};
use swc_core::{
  common::{FileName, GLOBALS, Globals, SourceMap, input::StringInput, sync::Lrc},
  ecma::{
    ast::{Decl, EsVersion, Expr, Lit, Module, ModuleItem, Stmt},
    parser::{EsSyntax, Parser, Syntax, lexer::Lexer},
  },
};

/// How many links each point joins, and how long each operand is. The product
/// is held at nine hundred thousand across all four, so what varies between
/// them is only the number of times the accumulated text could be re-read.
const CHAINS: [(usize, usize); 4] = [(2, 450_000), (5, 180_000), (10, 90_000), (20, 45_000)];

/// A module binding `A` to `width` characters and then joining `links` copies
/// of it with `+`.
fn chain_module(links: usize, width: usize) -> String {
  format!(
    "const A = '{}';\n{};",
    "x".repeat(width),
    vec!["A"; links].join(" + ")
  )
}

fn parse(source: &str) -> Module {
  let cm: Lrc<SourceMap> = Default::default();
  let fm = cm.new_source_file(FileName::Anon.into(), source.to_owned());
  let lexer = Lexer::new(
    Syntax::Es(EsSyntax::default()),
    EsVersion::EsNext,
    StringInput::from(&*fm),
    None,
  );
  let mut parser = Parser::new_from(lexer);

  match parser.parse_module() {
    Ok(module) => module,
    Err(error) => panic!("Failed to parse a chain module: {:#?}", error),
  }
}

/// The declaration the chain resolves through, and the chain to fold.
struct Chain {
  module: Module,
  expr: Expr,
}

fn chain(links: usize, width: usize) -> Chain {
  let module = parse(&chain_module(links, width));
  let expr = chain_expr(&module);

  Chain { module, expr }
}

/// The last expression statement of `module` -- the chain, as opposed to the
/// declaration it reads.
fn chain_expr(module: &Module) -> Expr {
  let expr = module
    .body
    .iter()
    .filter_map(|item| match item {
      ModuleItem::Stmt(Stmt::Expr(expr_stmt)) => Some(expr_stmt.expr.as_ref().clone()),
      _ => None,
    })
    .next_back();

  match expr {
    Some(expr) => expr,
    None => panic!("every source built here ends in an expression statement"),
  }
}

/// A state manager that resolves `A` and runs under the shipped defaults, since
/// every chain here is inside both of them.
fn state(module: &Module) -> StateManager {
  let mut state = StateManager::default();

  for item in &module.body {
    if let ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) = item {
      for declarator in &var_decl.decls {
        fill_state_declarations(&mut state, declarator);
      }
    }
  }

  state
}

/// Panics unless the chain folded to the whole of the text it joins.
///
/// The length is the only answer a complete fold produces, which makes this the
/// check that the benchmark is timing a fold rather than a refusal -- a chain
/// that passed either ceiling would deopt at the same speed at every point and
/// report the flat line the fix is supposed to produce.
fn assert_folds_whole(chain: &Chain, links: usize, width: usize) {
  let mut state = state(&chain.module);
  let result = evaluate(&chain.expr, &mut state, &FunctionMap::default());

  let folded = match (result.confident, result.value.as_ref()) {
    (true, Some(EvaluateResultValue::Expr(Expr::Lit(Lit::Str(text))))) => match text.value.as_str()
    {
      Some(text) => text.len(),
      None => panic!("a {links}-link chain folded to a string with no readable text"),
    },
    (confident, value) => panic!(
      "a {links}-link chain folded to confident={confident}, value={value:?}, \
       which is not a string -- the benchmark below would be timing that instead \
       of a fold"
    ),
  };

  assert_eq!(
    folded,
    links * width,
    "a {links}-link chain of {width} characters folded to {folded} characters"
  );
}

/// The same text joined through more and more links.
fn chain_benchmarks(c: &mut Criterion) {
  let globals = Globals::default();

  GLOBALS.set(&globals, || {
    let mut group = c.benchmark_group("ConcatenationChain");
    let functions = FunctionMap::default();

    for (links, width) in CHAINS {
      let chain = chain(links, width);

      assert_folds_whole(&chain, links, width);

      // Batched rather than plain `iter`, so building the state manager is not
      // measured. A fold cannot reuse one: `seen` memoizes what it folded, so a
      // second iteration against the same state would hit the memo and time
      // nothing.
      group.bench_function(format!("links/{links}"), |b| {
        b.iter_batched(
          || state(&chain.module),
          |mut state| {
            black_box(evaluate(
              black_box(&chain.expr),
              black_box(&mut state),
              black_box(&functions),
            ))
          },
          BatchSize::SmallInput,
        )
      });
    }

    group.finish();
  });
}

criterion_group!(benches, chain_benchmarks);
criterion_main!(benches);
