//! What the runtime-injection walk costs over a large top-level array.
//!
//! `flush_pending_insertions` places each queued `_inject2(...)` statement in
//! front of the statement that holds the styles it belongs to. To find that
//! statement it reads every expression the statement holds and hashes each
//! object and each name it meets, and the shapes differ by a lot in how many
//! candidates that is:
//!
//! - A declarator bound to one compiled object is hashed once, and the walk
//!   stops there. That is `export const styles = stylex.create({ ... })`.
//!   One hash is not the same as a cheap one: past 128 properties that hash
//!   takes the deep-clone arm, which the table in the ADR below shows costing
//!   more than reading the same styles written as an array.
//! - A declarator bound to an array is looked inside, because a `create` call
//!   written in an array declares its rules through the statement rather than
//!   through the object. Every element is hashed, and an element that is not
//!   the styles is read further down.
//!
//! The second shape is the one this file is about, and before it nothing
//! measured that shape: the benchmark corpus runs its two large-array fixtures
//! with runtime injection off, which drops every `BeforeDecl` item and skips
//! the walk entirely. The groups below put the paths side by side at the same
//! style count, and run the array shape at the corpus size as well, so the
//! price is read rather than reasoned about. What they said, and why no fixture in the corpus prices this shape,
//! is
//! `crates/stylex-state/docs/adr/0002-the-injection-walk-over-a-large-array-is-measured-and-kept.md`.
//!
//! No `GLOBALS` scope is needed here. The subject builds no `Mark`: it reads
//! spans off parsed nodes and hashes them without them.

// The allocator the published addon links, so a measurement that allocation
// binds matches what a consumer gets. Rust links a dev-dependency only where a
// target names it, so this line is what makes the choice real.
use swc_malloc as _;

use std::hint::black_box;

use criterion::{
  BatchSize, BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
use stylex_state::state_manager::{InsertionSlot, StateManager, flush_pending_insertions};
use stylex_utils::hash::{stable_hash_unspanned, stable_hash_wide};
use swc_core::{
  common::{DUMMY_SP, FileName, SourceMap, input::StringInput, sync::Lrc},
  ecma::{
    ast::{EsVersion, Expr, ExprStmt, Lit, Module, ModuleDecl, ModuleItem, Stmt, VarDeclarator},
    parser::{EsSyntax, Parser, Syntax, lexer::Lexer},
    utils::drop_span,
  },
};

/// The `create` call counts the curve is read from. Each step is four times the
/// one before, which is enough to tell a linear growth from a flat one.
///
/// The first is also the last count at which the object control stays on the
/// fast hash arm; see [`object_fixture`].
const CREATE_CALLS: [usize; 3] = [128, 512, 2048];

/// The `create` calls in `lotsOfStyles.js`, the largest module in the
/// benchmark corpus.
///
/// The array shape is measured at this size as well as along the curve, so the
/// figure for the module that pays the most is read off a clock rather than
/// extrapolated from a smaller one. Only the array shape: the object control
/// at this size prices the deep-clone hash arm over half a million nodes, which
/// answers no question this file asks.
const CORPUS_CALLS: usize = 21_733;

/// Style namespaces per compiled object, and class-name declarations per
/// namespace. A compiled `create` result is one object of namespaces, each of
/// which is an object of class names plus the `$$css` marker, so these two
/// numbers decide how deep and how wide the descent goes.
///
/// `lotsOfStyles.js` averages 2.92 namespaces per call and 3.01 declarations
/// per namespace, so these are the corpus shape rounded to whole numbers.
const NAMESPACES: usize = 3;
const DECLARATIONS: usize = 3;

/// How deep the authored object in the data leg nests, and how many names each
/// of its levels holds.
///
/// The width is over the 128 properties `stable_hash_unspanned` gives up at,
/// so every level of the object takes the deep-clone arm. That is the arm that
/// decides this leg: a hash of one level copies the whole subtree under it, so
/// reading the object level by level copies the bytes of every level again.
/// The two together are far more than a configuration table an author writes,
/// which is the point -- the leg is the worst case of reading a statement to
/// the bottom, not the common one.
const DATA_DEPTH: usize = 8;
const DATA_WIDTH: usize = 200;

/// The source of one compiled `create` result: what the transform leaves where
/// the author wrote `stylex.create({ ... })`.
fn compiled_object(index: usize) -> String {
  let namespaces = (0..NAMESPACES)
    .map(|namespace| {
      let declarations = (0..DECLARATIONS)
        .map(|declaration| format!("p{declaration}: \"x{index}_{namespace}_{declaration}\""))
        .collect::<Vec<_>>()
        .join(", ");

      format!("ns{namespace}: {{ {declarations}, $$css: true }}")
    })
    .collect::<Vec<_>>()
    .join(", ");

  format!("{{ {namespaces} }}")
}

/// A module whose one style declaration is an array of `calls` compiled
/// objects -- the shape the walk descends into.
fn array_module_source(calls: usize) -> String {
  let elements = (0..calls)
    .map(compiled_object)
    .collect::<Vec<_>>()
    .join(",\n");

  format!("'use strict';\nimport 'side-effect';\nexport const styles = [\n{elements}\n];\n")
}

/// A module holding the same styles under one object initializer -- the shape
/// the walk hashes once and leaves alone.
///
/// The declaration is the whole object, so a producer keys its items to that
/// one hash however many groups it holds. That is why this leg queues one
/// item where the array leg queues `calls` of them: the difference in what has
/// to be placed is part of what the two shapes cost.
fn object_module_source(calls: usize) -> String {
  let groups = (0..calls)
    .map(|index| format!("g{index}: {}", compiled_object(index)))
    .collect::<Vec<_>>()
    .join(",\n");

  format!("'use strict';\nimport 'side-effect';\nexport const styles = {{\n{groups}\n}};\n")
}

/// A module whose styles are an array, behind a top-level object of authored
/// data the styles have nothing to do with -- a configuration table, a message
/// catalogue, a theme map.
///
/// This is the shape that decides what reading the whole statement costs. The
/// walk hashes an object it did not match and then reads inside it, and a hash
/// reads the whole subtree under it, so an authored object `d` levels deep is
/// hashed `d` times at its own root. The walk before it hashed such an
/// initializer once and never looked inside, which is what makes this the one
/// leg where the widening is a cost rather than a saving.
///
/// `depth` levels of nesting, each holding one object and two names, so the
/// node count grows with the depth rather than with the width.
fn data_module_source(calls: usize, depth: usize, width: usize) -> String {
  let names = (0..width)
    .map(|name| format!("n{name}: \"name\""))
    .collect::<Vec<_>>()
    .join(", ");
  let mut data = format!("{{ {names} }}");

  for level in 0..depth {
    data = format!("{{ k{level}: {data}, {names} }}");
  }

  let elements = (0..calls)
    .map(compiled_object)
    .collect::<Vec<_>>()
    .join(",\n");

  format!(
    "'use strict';\nimport 'side-effect';\nconst config = {data};\nexport const styles = [\n{elements}\n];\n"
  )
}

fn parse(source: &str) -> Module {
  let source_map: Lrc<SourceMap> = Default::default();
  let file = source_map.new_source_file(FileName::Anon.into(), source.to_owned());
  let lexer = Lexer::new(
    Syntax::Es(EsSyntax::default()),
    EsVersion::EsNext,
    StringInput::from(&*file),
    None,
  );
  let mut parser = Parser::new_from(lexer);

  match parser.parse_module() {
    Ok(module) => module,
    Err(error) => panic!("the benchmark source failed to parse: {error:?}"),
  }
}

/// A parsed module body and the hashes a producer would key its items to.
struct Fixture {
  body: Vec<ModuleItem>,
  keys: Vec<u128>,
}

/// The initializer of the one exported declaration the sources above write.
fn exported_init(module: &Module) -> &Expr {
  let init = module
    .body
    .iter()
    .find_map(|item| match item {
      ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export)) => export.decl.as_var(),
      _ => None,
    })
    .and_then(|var_decl| var_decl.decls.first())
    .and_then(|declarator: &VarDeclarator| declarator.init.as_deref());

  match init {
    Some(init) => init,
    None => panic!("every source built here exports one initialized declaration"),
  }
}

/// The array fixture: one key per element, the way the transform keys the
/// items of each `create` call written inside an array.
fn array_fixture(calls: usize) -> Fixture {
  let module = parse(&array_module_source(calls));
  let keys = match exported_init(&module) {
    Expr::Array(array) => array
      .elems
      .iter()
      .flatten()
      .map(|element| stable_hash_unspanned(&element.expr))
      .collect(),
    _ => panic!("the array source did not parse to an array initializer"),
  };

  Fixture {
    body: module.body,
    keys,
  }
}

/// The data fixture: the array fixture's keys, over a body that also holds the
/// authored object. The keys are read off the array, so the data is what the
/// walk reads for nothing.
fn data_fixture(calls: usize, depth: usize, width: usize) -> Fixture {
  let module = parse(&data_module_source(calls, depth, width));
  let keys = match exported_init(&module) {
    Expr::Array(array) => array
      .elems
      .iter()
      .flatten()
      .map(|element| stable_hash_unspanned(&element.expr))
      .collect(),
    _ => panic!("the data source did not parse to an array initializer"),
  };

  Fixture {
    body: module.body,
    keys,
  }
}

/// The object fixture: the one key the whole initializer has.
///
/// The arm that key was hashed on is checked here, because it decides whether
/// this leg is still a clean control. `stable_hash_unspanned` gives up on an
/// object of more than 128 properties and falls back to a span-stripped deep
/// clone of the subtree, which is a different price from the in-place walk the
/// array leg pays at every level. Above the boundary the pair therefore reads
/// the two arms rather than the descent, and the numbers say so only if this
/// says which arm ran.
fn object_fixture(calls: usize) -> Fixture {
  let module = parse(&object_module_source(calls));
  let init = exported_init(&module);
  let key = stable_hash_unspanned(init);
  let took_fallback = key == stable_hash_wide(&drop_span(init.clone()));

  assert_eq!(
    took_fallback,
    calls > 128,
    "an object of {calls} groups {} the deep-clone hash arm. Either this count \
     moved or the collection boundary in `stylex_utils::hash` did, and until one \
     of them is put back this control does not price what its name says",
    if took_fallback {
      "took"
    } else {
      "did not take"
    }
  );

  Fixture {
    body: module.body,
    keys: vec![key],
  }
}

/// A statement standing in for an `_inject2(...)` call, marked so the check
/// below can find it again.
fn injected_item(index: usize) -> ModuleItem {
  ModuleItem::Stmt(Stmt::Expr(ExprStmt {
    span: DUMMY_SP,
    expr: Box::new(Expr::Lit(Lit::Str(format!("inject{index}").into()))),
  }))
}

/// A state manager holding one queued item per key.
fn queued_state(fixture: &Fixture) -> StateManager {
  let mut state = StateManager::default();

  for (index, key) in fixture.keys.iter().enumerate() {
    state.queue_insertion(InsertionSlot::BeforeDecl(*key), injected_item(index));
  }

  state
}

/// Whether `item` is one of the statements [`injected_item`] builds.
fn is_injected(item: &ModuleItem) -> bool {
  match item {
    ModuleItem::Stmt(Stmt::Expr(expr_stmt)) => match expr_stmt.expr.as_lit() {
      Some(Lit::Str(value)) => value
        .value
        .as_str()
        .is_some_and(|text| text.starts_with("inject")),
      _ => false,
    },
    _ => false,
  }
}

/// Whether `item` is the exported style declaration. Both sources above write
/// exactly one, and nothing else in them is an export.
fn is_style_decl(item: &ModuleItem) -> bool {
  matches!(item, ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(_)))
}

/// Panics unless the flush placed what `runtime_injection` says it should.
///
/// An unmatched key is dropped without a word, so a walk that stopped
/// descending would leave a shorter body and report as a win. With injection
/// off the opposite is the risk: every queued item is dropped on purpose, and a
/// leg that placed one would be timing the walk under the name of the skip.
fn assert_placement(fixture: &Fixture, label: &str, runtime_injection: bool) {
  let mut state = queued_state(fixture);
  let mut body = fixture.body.clone();

  flush_pending_insertions(&mut state, &mut body, runtime_injection);

  let expected = if runtime_injection {
    fixture.keys.len()
  } else {
    0
  };
  let placed = body.iter().filter(|item| is_injected(item)).count();

  assert_eq!(
    placed, expected,
    "{label}: {placed} items were placed where {expected} were due, so this leg \
     does not time what its name says"
  );

  if runtime_injection {
    let declaration = body.iter().position(is_style_decl);
    let last_injected = body.iter().rposition(is_injected);

    assert!(
      matches!((declaration, last_injected), (Some(decl), Some(last)) if last < decl),
      "{label}: the injected items did not all land in front of the style \
       declaration"
    );
  }
}

/// Times one flush over `fixture`, after checking it placed what it should.
fn measure(
  group: &mut BenchmarkGroup<'_, WallTime>,
  fixture: &Fixture,
  label: &str,
  runtime_injection: bool,
) {
  assert_placement(fixture, label, runtime_injection);

  // Batched because the flush consumes the queue and rewrites the body, so no
  // iteration can reuse what the one before it left. The routine returns both,
  // which keeps tearing down a body of thousands of nodes out of the measured
  // window.
  group.bench_function(label, |b| {
    b.iter_batched(
      || (queued_state(fixture), fixture.body.clone()),
      |(mut state, mut body)| {
        flush_pending_insertions(
          black_box(&mut state),
          black_box(&mut body),
          black_box(runtime_injection),
        );
        (state, body)
      },
      BatchSize::SmallInput,
    )
  });
}

/// Both groups: each shape with runtime injection on, then the array shape
/// again with it off.
///
/// The array fixtures are parsed once and read by both groups. `InjectionWalk`
/// is the descent; `InjectionWalkDisabled` is what the corpus measures today,
/// and the gap between the two is what no release gate watches.
///
/// The disabled group is not a flat floor. It drops every queued item in the
/// drain loop before the walk starts, so it grows with the number of calls like
/// the descent does -- it is the cost of throwing the queue away, not the cost
/// of the walk, and it is the part of the subject the corpus already pays.
fn injection_walk_benchmarks(c: &mut Criterion) {
  let arrays: Vec<(usize, Fixture)> = CREATE_CALLS
    .into_iter()
    .chain([CORPUS_CALLS])
    .map(|calls| (calls, array_fixture(calls)))
    .collect();

  {
    let mut group = c.benchmark_group("InjectionWalk");

    for (calls, array) in &arrays {
      measure(&mut group, array, &format!("array/{calls}"), true);

      if *calls != CORPUS_CALLS {
        measure(
          &mut group,
          &object_fixture(*calls),
          &format!("object/{calls}"),
          true,
        );
        measure(
          &mut group,
          &data_fixture(*calls, DATA_DEPTH, DATA_WIDTH),
          &format!("array-with-data/{calls}"),
          true,
        );
      }
    }

    group.finish();
  }

  let mut group = c.benchmark_group("InjectionWalkDisabled");

  for (calls, array) in &arrays {
    measure(&mut group, array, &format!("array/{calls}"), false);
  }

  group.finish();
}

criterion_group!(injection_walk_benches, injection_walk_benchmarks);
criterion_main!(injection_walk_benches);
