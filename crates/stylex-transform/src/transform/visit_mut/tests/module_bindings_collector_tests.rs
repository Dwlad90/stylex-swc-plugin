use std::sync::Arc;

use rustc_hash::FxHashSet;
use swc_core::{
  common::{FileName, GLOBALS, Globals, Mark, SourceMap, input::StringInput},
  ecma::{
    ast::{Decl, Id, Module, ModuleItem, Pass, Program, Stmt},
    parser::{EsSyntax, Parser, Syntax, lexer::Lexer},
    transforms::base::resolver,
    visit::VisitWith,
  },
};

use super::ModuleBindingsCollector;

/// Parses `code` and runs SWC's resolver over it, the same way the compiler
/// does before the StyleX pass, so `Id`s carry real syntax contexts and
/// shadowed bindings stay distinguishable.
fn resolved_module(code: &str) -> Module {
  let source_map = SourceMap::default();
  let source_file = source_map.new_source_file(
    Arc::new(FileName::Custom("module_bindings_fixture.tsx".to_string())),
    code.to_string(),
  );

  let lexer = Lexer::new(
    Syntax::Es(EsSyntax {
      jsx: true,
      ..Default::default()
    }),
    Default::default(),
    StringInput::from(&*source_file),
    None,
  );

  let module = match Parser::new_from(lexer).parse_module() {
    Ok(module) => module,
    Err(error) => panic!("failed to parse fixture: {:?}", error),
  };

  let mut program = Program::Module(module);
  resolver(Mark::new(), Mark::new(), true).process(&mut program);

  match program {
    Program::Module(module) => module,
    Program::Script(_) => unreachable!("a parsed module never becomes a script"),
  }
}

/// Names of the bindings the collector recorded as written, ignoring syntax
/// contexts (asserted separately where shadowing is what's under test).
fn written_names(code: &str) -> FxHashSet<String> {
  written_ids(code)
    .iter()
    .map(|(sym, _)| sym.to_string())
    .collect()
}

fn written_ids(code: &str) -> FxHashSet<Id> {
  GLOBALS.set(&Globals::default(), || {
    let module = resolved_module(code);
    let mut collector = ModuleBindingsCollector::writes_only();
    module.visit_with(&mut collector);

    collector.binding_writes
  })
}

fn assert_written(code: &str, expected: &[&str]) {
  let written = written_names(code);
  let expected: FxHashSet<String> = expected.iter().map(|name| name.to_string()).collect();

  assert_eq!(written, expected, "for source: {}", code);
}

#[test]
fn records_plain_and_compound_assignments() {
  assert_written("let a = 1; a = 2;", &["a"]);
  assert_written("let a = 1; a += 2;", &["a"]);
  assert_written("let a = 1; (a) = 2;", &["a"]);
}

#[test]
fn records_update_expressions() {
  assert_written("let a = 1; a++;", &["a"]);
  assert_written("let a = 1; --a;", &["a"]);
  assert_written("const o = { n: 1 }; o.n++;", &["o"]);
}

#[test]
fn records_destructuring_assignment_targets() {
  assert_written("let a, b; [a, b] = [1, 2];", &["a", "b"]);
  assert_written("let a, rest; ({ a, ...rest } = {});", &["a", "rest"]);
  assert_written("let a; ({ a = 1 } = {});", &["a"]);
  // A member target inside a destructuring assignment mutates the object it
  // is reached through, it does not rebind a name of its own.
  assert_written("const o = {}; ({ x: o.x } = { x: 1 });", &["o"]);
}

#[test]
fn records_loop_head_rebindings() {
  assert_written("let key; const o = {}; for (key in o) {}", &["key"]);
  assert_written("let item; for (item of []) {}", &["item"]);
  // A `for` head that declares its own binding is not a write.
  assert_written("for (const item of []) { item; }", &[]);
}

#[test]
fn records_member_writes_through_the_chain_root() {
  assert_written("const o = { a: { b: 1 } }; o.a.b = 2;", &["o"]);
  assert_written("const o = { a: {} }; delete o.a.b;", &["o"]);
  assert_written("const o = { a: [] }; o.a[0] = 1;", &["o"]);
  // Nothing to invalidate when the chain root owns no binding.
  assert_written("getConfig().a.b = 1;", &[]);
}

#[test]
fn records_mutating_array_methods_on_nested_receivers() {
  assert_written("const items = []; items.push(1);", &["items"]);
  assert_written(
    "const state = { items: [] }; state.items.push(1);",
    &["state"],
  );
  assert_written("const items = []; items.at(0);", &[]);
}

#[test]
fn records_mutating_object_methods_on_their_target() {
  assert_written("const o = {}; Object.assign(o, { a: 1 });", &["o"]);
  assert_written("const o = { a: {} }; Object.assign(o.a, {});", &["o"]);
  assert_written(
    "const o = {}; Object.defineProperty(o, 'a', { value: 1 });",
    &["o"],
  );
  // A spread first argument names no single binding to invalidate.
  assert_written("const o = {}; Object.assign(...[o], {});", &[]);
  // Non-mutating `Object` statics leave their argument alone.
  assert_written("const o = {}; Object.keys(o);", &[]);
}

/// Parenthesised and nested-wrapper targets reach the same binding as their
/// bare form, so every write shape must look through them. Each case here
/// silently escaped detection while the expression-target match was shallow.
#[test]
fn looks_through_parenthesised_targets() {
  assert_written("let a = 1; (a)++;", &["a"]);
  assert_written("let a = 1; ((a)) = 2;", &["a"]);
  assert_written("const o = { n: 1 }; (o.n)++;", &["o"]);
  assert_written("const o = { n: 1 }; ((o).n) = 2;", &["o"]);
  assert_written("const o = {}; Object.assign((o), { a: 1 });", &["o"]);
  assert_written("const o = { a: {} }; delete ((o).a);", &["o"]);
}

/// `arr?.push(1)` parses as an optional call rather than a `CallExpr`, but
/// mutates the receiver just the same whenever it is non-nullish.
#[test]
fn records_mutating_methods_reached_through_optional_calls() {
  assert_written("const items = []; items?.push(1);", &["items"]);
  assert_written(
    "const state = { items: [] }; state.items?.push(1);",
    &["state"],
  );
  assert_written(
    "const state = { items: [] }; state?.items?.push(1);",
    &["state"],
  );
  // Non-mutating optional calls leave the receiver alone.
  assert_written("const items = []; items?.at(0);", &[]);
}

/// A string-literal computed property names its method as unambiguously as
/// dot access does.
#[test]
fn records_mutating_methods_named_by_a_string_literal() {
  assert_written("const items = []; items['push'](1);", &["items"]);
  assert_written("const o = {}; Object['assign'](o, { a: 1 });", &["o"]);
  // A genuinely dynamic property name is unknowable, so nothing is recorded
  // rather than deopting every computed call in the module.
  assert_written("const items = []; const m = 'push'; items[m](1);", &[]);
}

#[test]
fn keeps_shadowed_bindings_distinct() {
  GLOBALS.set(&Globals::default(), || {
    let module = resolved_module(
      r#"
        const tokens = { color: "red" };

        export function Component() {
          let tokens = { color: "blue" };
          tokens = { color: "green" };
          return tokens;
        }
      "#,
    );

    let outer_id = first_top_level_binding(&module);

    let mut collector = ModuleBindingsCollector::writes_only();
    module.visit_with(&mut collector);

    let written: Vec<&Id> = collector
      .binding_writes
      .iter()
      .filter(|(sym, _)| sym == "tokens")
      .collect();

    assert_eq!(
      written.len(),
      1,
      "only the inner `tokens` is written: {:?}",
      collector.binding_writes
    );
    assert_ne!(
      *written[0], outer_id,
      "the write to the shadowing `tokens` must not invalidate the outer one"
    );
  });
}

/// `Id` of the first module-level `var`/`let`/`const` binding, used to assert
/// that a write to a shadowing binding is recorded under a different `Id`.
fn first_top_level_binding(module: &Module) -> Id {
  module
    .body
    .iter()
    .find_map(|item| match item {
      ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) => var_decl
        .decls
        .first()
        .and_then(|declarator| declarator.name.as_ident())
        .map(|binding_ident| binding_ident.id.to_id()),
      _ => None,
    })
    .expect("fixture declares a top-level binding")
}
