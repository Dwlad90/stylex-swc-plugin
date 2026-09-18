//! What queueing the runtime-injection statements costs.
//!
//! `register_styles` files the rules a producer compiled and queues one
//! `_inject2(...)` statement per rule against each key the placement walk can
//! find. `crates/stylex-state/benches/injection_walk_bench.rs` measures the
//! walk that reads those keys; this measures the other half, which no reading
//! priced before.
//!
//! The shape that decides the cost is the number of rules one call declares,
//! because the queue is deduped per statement and per rule. `lotsOfStyles.js`
//! -- the largest module in the benchmark corpus -- compiles 21,733 calls into
//! 153,437 rules, so about seven rules per call, and a module of wide
//! namespaces writes many more.
//!
//! No `GLOBALS` scope is needed here. The subject builds no `Mark`: it hashes
//! spanless nodes and writes module items with `DUMMY_SP`.

// The allocator the published addon links, so a measurement that allocation
// binds matches what a consumer gets.
use swc_malloc as _;

use std::hint::black_box;
use std::rc::Rc;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use indexmap::IndexMap;
use stylex_ast::ast::factories::{create_ident, create_object_expression};
use stylex_state::state_manager::{StateManager, flush_pending_insertions};
use stylex_state::types::InjectableStylesMap;
use stylex_structures::named_import_source::RuntimeInjectionState;
use stylex_structures::stylex_state_options::StyleXStateOptions;
use stylex_types::enums::data_structures::injectable_style::InjectableStyleKind;
use stylex_types::structures::injectable_style::InjectableStyle;
use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{
    BindingIdent, CallExpr, Callee, Decl, Expr, ModuleItem, Pat, Stmt, VarDecl, VarDeclKind,
    VarDeclarator,
  },
};

/// Rules per call, four times apart, so a linear cost reads differently from a
/// square one. The first is the corpus average rounded down.
const RULES_PER_CALL: [usize; 3] = [8, 32, 128];

/// How many calls one measurement registers. Enough that the per-call work is
/// what the reading is made of, and small enough that the batch stays short.
const CALLS: usize = 64;

fn call_expr() -> CallExpr {
  CallExpr {
    span: DUMMY_SP,
    callee: Callee::Expr(Box::new(Expr::Ident(create_ident("create")))),
    args: vec![],
    type_args: None,
    ctxt: Default::default(),
  }
}

/// One compiled object, standing for what a `create` call leaves behind. Its
/// hash is the key every rule of that call is queued under, so each call gets
/// an object of its own.
fn compiled_object(index: usize) -> Expr {
  create_object_expression(vec![
    stylex_ast::ast::factories::create_ident_key_value_prop(
      "ns",
      Expr::Lit(stylex_ast::ast::factories::create_string_lit(&format!(
        "x{index}"
      ))),
    ),
  ])
}

/// `rules` distinct rules, the way one call's styles reach the state.
fn styles(index: usize, rules: usize) -> InjectableStylesMap {
  let mut map: InjectableStylesMap = IndexMap::new();

  for rule in 0..rules {
    map.insert(
      format!("x{index}_{rule}").into(),
      Rc::new(InjectableStyleKind::Regular(InjectableStyle {
        ltr: format!(".x{index}_{rule}{{color:red}}"),
        rtl: None,
        priority: Some(3000.0),
      })),
    );
  }

  map
}

fn state() -> StateManager {
  StateManager::for_test(
    None,
    StyleXStateOptions::default()
      .with_runtime_injection(Some(RuntimeInjectionState::Boolean(true))),
  )
}

/// Registers every call of one measurement. The state is answered back, which
/// keeps dropping it out of the measured window.
fn register_all(mut state: StateManager, calls: &[(Expr, InjectableStylesMap)]) -> StateManager {
  let call = call_expr();

  for (ast, style) in calls {
    state.register_styles(&call, style, ast, None);
  }

  state
}

/// How many injecting statements a flush places for `calls`, read outside the
/// measured window.
///
/// A queue that dropped a rule would read as a win, and the count is the only
/// thing that says it did not. The body it flushes over holds each compiled
/// object under a declaration, which is the shape the walk finds.
fn placed_count(calls: &[(Expr, InjectableStylesMap)]) -> usize {
  let mut state = register_all(state(), calls);
  let mut body: Vec<ModuleItem> = calls
    .iter()
    .map(|(ast, _)| {
      ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
        span: DUMMY_SP,
        kind: VarDeclKind::Const,
        declare: false,
        decls: vec![VarDeclarator {
          span: DUMMY_SP,
          name: Pat::Ident(BindingIdent {
            id: create_ident("styles"),
            type_ann: None,
          }),
          init: Some(Box::new(ast.clone())),
          definite: false,
        }],
        ctxt: Default::default(),
      }))))
    })
    .collect();

  flush_pending_insertions(&mut state, &mut body, true);

  body
    .iter()
    .filter(
      |item| matches!(item, ModuleItem::Stmt(Stmt::Expr(statement)) if statement.expr.is_call()),
    )
    .count()
}

fn injection_queue_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("InjectionQueue");

  for rules in RULES_PER_CALL {
    let calls: Vec<(Expr, InjectableStylesMap)> = (0..CALLS)
      .map(|index| (compiled_object(index), styles(index, rules)))
      .collect();

    let expected = CALLS * rules;
    let placed = placed_count(&calls);

    assert_eq!(
      placed, expected,
      "the leg for {rules} rules per call queued {placed} statements where \
       {expected} were due, so it does not time what its name says"
    );

    group.bench_function(format!("rules/{rules}"), |b| {
      b.iter_batched(
        state,
        |state| register_all(black_box(state), black_box(&calls)),
        BatchSize::SmallInput,
      )
    });
  }

  group.finish();
}

criterion_group!(injection_queue_benches, injection_queue_benchmarks);
criterion_main!(injection_queue_benches);
