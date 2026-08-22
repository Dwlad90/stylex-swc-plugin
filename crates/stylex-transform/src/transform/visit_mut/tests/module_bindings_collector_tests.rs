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
  resolved_module_in(
    code,
    Syntax::Es(EsSyntax {
      jsx: true,
      ..Default::default()
    }),
  )
}

/// The same, over TypeScript syntax — for the binding forms only TypeScript
/// spells, which the ES parser cannot read at all.
fn resolved_ts_module(code: &str) -> Module {
  resolved_module_in(code, Syntax::Typescript(Default::default()))
}

fn resolved_module_in(code: &str, syntax: Syntax) -> Module {
  let source_map = SourceMap::default();
  let source_file = source_map.new_source_file(
    Arc::new(FileName::Custom("module_bindings_fixture.tsx".to_string())),
    code.to_string(),
  );

  let lexer = Lexer::new(
    syntax,
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

/// Names of the bindings the collector recorded as written under any kind,
/// ignoring syntax contexts (asserted separately where shadowing is what's
/// under test). Most shapes here are about *whether* a write is seen at all,
/// which the union answers; `splits_…` below pins which set each lands in.
fn written_names(code: &str) -> FxHashSet<String> {
  written_ids(code)
    .iter()
    .map(|(sym, _)| sym.to_string())
    .collect()
}

fn written_ids(code: &str) -> FxHashSet<Id> {
  collect(code, |collector| {
    collector
      .binding_reassignments
      .union(&collector.binding_mutations)
      .chain(collector.binding_deep_mutations.iter())
      .cloned()
      .collect()
  })
}

fn reassigned_names(code: &str) -> FxHashSet<String> {
  names(collect(code, |collector| {
    collector.binding_reassignments.clone()
  }))
}

fn mutated_names(code: &str) -> FxHashSet<String> {
  names(collect(code, |collector| {
    collector.binding_mutations.clone()
  }))
}

/// Names recorded as written further down a chain than upstream's `isMutated`
/// looks — one hop is a mutation, two or more is this.
fn deeply_mutated_names(code: &str) -> FxHashSet<String> {
  names(collect(code, |collector| {
    collector.binding_deep_mutations.clone()
  }))
}

fn names(ids: FxHashSet<Id>) -> FxHashSet<String> {
  ids.iter().map(|(sym, _)| sym.to_string()).collect()
}

/// Runs the collector over `code` and reads whichever set the caller asks for,
/// so the three readers above differ only in that choice.
fn collect(code: &str, read: impl Fn(&ModuleBindingsCollector) -> FxHashSet<Id>) -> FxHashSet<Id> {
  collect_parsed_by(resolved_module, code, read)
}

/// The same over TypeScript syntax, for the forms the ES parser cannot read.
fn collect_ts(
  code: &str,
  read: impl Fn(&ModuleBindingsCollector) -> FxHashSet<Id>,
) -> FxHashSet<Id> {
  collect_parsed_by(resolved_ts_module, code, read)
}

fn collect_parsed_by(
  parse: fn(&str) -> Module,
  code: &str,
  read: impl Fn(&ModuleBindingsCollector) -> FxHashSet<Id>,
) -> FxHashSet<Id> {
  GLOBALS.set(&Globals::default(), || {
    let module = parse(code);
    let mut collector = ModuleBindingsCollector::writes_only();
    module.visit_with(&mut collector);

    read(&collector)
  })
}

#[track_caller]
fn assert_written(code: &str, expected: &[&str]) {
  assert_names(written_names(code), expected, code);
}

#[track_caller]
fn assert_reassigned(code: &str, expected: &[&str]) {
  assert_names(reassigned_names(code), expected, code);
}

#[track_caller]
fn assert_mutated(code: &str, expected: &[&str]) {
  assert_names(mutated_names(code), expected, code);
}

#[track_caller]
fn assert_deeply_mutated(code: &str, expected: &[&str]) {
  assert_names(deeply_mutated_names(code), expected, code);
}

#[track_caller]
fn assert_names(actual: FxHashSet<String>, expected: &[&str], code: &str) {
  let expected: FxHashSet<String> = expected.iter().map(|name| name.to_string()).collect();

  assert_eq!(actual, expected, "for source: {}", code);
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

// ==================== which set a write lands in ====================

/// A bare name given a new value is a reassignment — the reference
/// implementation's constant violation — and nothing about the value it used to
/// reference has changed, so it is not also a mutation. Every shape that writes
/// a name directly answers this way.
#[test]
fn splits_a_rebound_name_into_the_reassignment_set() {
  assert_reassigned("let a = 1; a = 2;", &["a"]);
  assert_reassigned("let a = 1; a += 2;", &["a"]);
  assert_reassigned("let a = 1; a++;", &["a"]);
  assert_reassigned("let a = 1; ((a)) = 2;", &["a"]);
  assert_reassigned("let a, b; [a, b] = [1, 2];", &["a", "b"]);
  assert_reassigned("let a, rest; ({ a, ...rest } = {});", &["a", "rest"]);
  assert_reassigned("let key; const o = {}; for (key in o) {}", &["key"]);

  assert_mutated("let a = 1; a = 2;", &[]);
  assert_mutated("let a = 1; a++;", &[]);
  assert_mutated("let a, b; [a, b] = [1, 2];", &[]);
}

/// Reaching the binding through a member keeps the binding and changes what it
/// points at, which is the reference implementation's `isMutated`. So `o.n++`
/// mutates where `n++` reassigns, out of the one walk that serves both.
#[test]
fn splits_a_write_through_a_member_into_the_mutation_set() {
  assert_mutated("const o = { n: 1 }; o.n = 2;", &["o"]);
  assert_mutated("const o = { n: 1 }; o.n++;", &["o"]);
  assert_mutated("const o = { n: 1 }; (o.n)++;", &["o"]);
  assert_mutated("const o = { a: {} }; delete o.a;", &["o"]);
  assert_mutated("const o = {}; ({ x: o.x } = { x: 1 });", &["o"]);
  assert_mutated("const o = { a: [] }; o[0] = 1;", &["o"]);

  assert_reassigned("const o = { n: 1 }; o.n = 2;", &[]);
  assert_reassigned("const o = { n: 1 }; o.n++;", &[]);
  assert_reassigned("const o = {}; ({ x: o.x } = { x: 1 });", &[]);
}

/// A second member hop takes the write past what `isMutated` looks at: it asks
/// that the reference's own parent be the member the write lands on, and
/// `o.a.b = 2` puts another member in between. Recorded apart so the chain can
/// keep refusing it — which upstream does not — without changing the refusal a
/// binding of a kind that folds nothing already had.
#[test]
fn splits_a_write_below_the_first_member_into_the_deep_mutation_set() {
  for (source, root) in [
    ("const o = { a: { b: 1 } }; o.a.b = 2;", "o"),
    ("const o = { a: {} }; delete o.a.b;", "o"),
    ("const o = { a: [] }; o.a[0] = 1;", "o"),
    ("const state = { items: [] }; state.items.push(1);", "state"),
    ("const o = { a: {} }; Object.assign(o.a, {});", "o"),
    ("const o = { a: { b: 1 } }; o.a.b++;", "o"),
  ] {
    assert_deeply_mutated(source, &[root]);
    assert_mutated(source, &[]);
    assert_reassigned(source, &[]);
  }
}

/// And one hop stays one hop however many wrappers sit around it, so a
/// parenthesised or `as`-cast single write is not mistaken for a deep one.
#[test]
fn wrappers_around_a_single_hop_do_not_deepen_it() {
  assert_mutated("const o = { n: 1 }; ((o).n) = 2;", &["o"]);
  assert_deeply_mutated("const o = { n: 1 }; ((o).n) = 2;", &[]);
}

/// A mutating method mutates its receiver, and `Object.assign` mutates its
/// first argument even when that argument is spelled as a bare name — the one
/// shape where a write reaching an identifier directly is still a mutation
/// rather than a reassignment.
#[test]
fn splits_a_mutated_receiver_and_an_assign_target_into_the_mutation_set() {
  assert_mutated("const items = []; items.push(1);", &["items"]);
  assert_mutated("const items = []; items?.push(1);", &["items"]);
  assert_mutated("const o = {}; Object.assign(o, { a: 1 });", &["o"]);
  assert_mutated("const o = {}; Object.assign((o), { a: 1 });", &["o"]);

  // A receiver reached through a member of its own is one hop further down,
  // which `splits_a_write_below_the_first_member_into_the_deep_mutation_set`
  // pins: the method mutates `state.items`, and `state` only holds it.
  assert_mutated("const state = { items: [] }; state.items.push(1);", &[]);
  assert_mutated("const o = { a: {} }; Object.assign(o.a, {});", &[]);

  assert_reassigned("const items = []; items.push(1);", &[]);
  assert_reassigned("const o = {}; Object.assign(o, { a: 1 });", &[]);
}

/// One name can reach both sets, and each records it independently: the
/// evaluator refuses on the first probe either way, so the value of keeping
/// them apart is that each step answers for itself.
#[test]
fn records_a_name_both_rebound_and_mutated_in_both_sets() {
  let code = "let o = { n: 1 }; o.n = 2; o = { n: 3 };";

  assert_reassigned(code, &["o"]);
  assert_mutated(code, &["o"]);
}

#[test]
fn keeps_shadowed_bindings_distinct() {
  assert_only_the_shadowing_binding_is_written(
    r#"
      const tokens = { color: "red" };

      export function Component() {
        let tokens = { color: "blue" };
        tokens = { color: "green" };
        return tokens;
      }
    "#,
    |collector| &collector.binding_reassignments,
  );
}

/// The same invariant for the other set. Splitting the writes in two doubled the
/// places a shadowed binding could be confused with the one it shadows, so both
/// sets have to answer, not just the one that happened to be asserted.
#[test]
fn keeps_shadowed_bindings_distinct_for_mutations_too() {
  assert_only_the_shadowing_binding_is_written(
    r#"
      const tokens = { color: "red" };

      export function Component() {
        const tokens = { color: "blue" };
        tokens.color = "green";
        return tokens;
      }
    "#,
    |collector| &collector.binding_mutations,
  );
}

/// Collects `code`, reads the set `pick` names, and asserts the only `tokens`
/// recorded there is the inner one — under an `Id` distinct from the module-level
/// binding's.
#[track_caller]
fn assert_only_the_shadowing_binding_is_written(
  code: &str,
  pick: impl Fn(&ModuleBindingsCollector) -> &FxHashSet<Id>,
) {
  GLOBALS.set(&Globals::default(), || {
    let module = resolved_module(code);
    let outer_id = first_top_level_binding(&module);

    let mut collector = ModuleBindingsCollector::writes_only();
    module.visit_with(&mut collector);

    let recorded = pick(&collector);
    let written: Vec<&Id> = recorded.iter().filter(|(sym, _)| sym == "tokens").collect();

    assert_eq!(
      written.len(),
      1,
      "only the inner `tokens` is written: {:?}",
      recorded
    );
    assert_ne!(
      *written[0], outer_id,
      "the write to the shadowing `tokens` must not answer for the outer one"
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

// ==================== the bindings the module declares ====================
//
// The `Id`-keyed set the evaluator's globals step reads. Unlike the two write
// sets, it is filled in both of the collector's modes, so every case here runs
// under `writes_only` — the mode that collects *less* — to pin that.

fn bound_names(code: &str) -> FxHashSet<String> {
  names(collect(code, |collector| {
    collector.declared_bindings.clone()
  }))
}

#[track_caller]
fn assert_binds(code: &str, expected: &[&str]) {
  assert_names(bound_names(code), expected, code);
}

/// The same over TypeScript syntax.
#[track_caller]
fn assert_ts_binds(code: &str, expected: &[&str]) {
  let recorded = names(collect_ts(code, |collector| {
    collector.declared_bindings.clone()
  }));

  assert_names(recorded, expected, code);
}

/// Every binding form JavaScript spells, in the one place a reader would look
/// for the list. A name missing here is a global the evaluator would fold where
/// the module had taken the name over.
///
/// TypeScript's own three -- `enum`, `namespace` and `import x = require()` --
/// are deliberately not among them, and are pinned as absent by
/// `records_nothing_for_typescript_only_binding_forms` below.
#[test]
fn records_every_javascript_binding_form() {
  assert_binds("const a = 1; let b = 2; var c = 3;", &["a", "b", "c"]);
  assert_binds("function f(p, q) {}", &["f", "p", "q"]);
  assert_binds("class K {}", &["K"]);
  assert_binds("const f = function named() {};", &["f", "named"]);
  assert_binds("const K = class Named {};", &["K", "Named"]);
  assert_binds("const g = (p) => p;", &["g", "p"]);
  assert_binds("try {} catch (e) {}", &["e"]);
  assert_binds("const [a, [b], ...rest] = xs;", &["a", "b", "rest"]);
  assert_binds(
    "const { a, b: c, d = 1, ...rest } = o;",
    &["a", "c", "d", "rest"],
  );
  assert_binds("for (const k in o) {}", &["k"]);
  assert_binds("for (const v of xs) {}", &["v"]);
  assert_binds(
    "function f({ a }, [b], c = 1, ...d) {}",
    &["f", "a", "b", "c", "d"],
  );
  assert_binds("import x, { y, z as w } from 'm';", &["x", "y", "w"]);
  assert_binds("import * as ns from 'm';", &["ns"]);
}

/// A reference is not a binding. Nothing here declares anything, so nothing is
/// recorded — the set would otherwise answer `true` for every name the module
/// mentions and the globals step would refuse every global.
#[test]
fn records_nothing_for_a_module_that_declares_nothing() {
  assert_binds("f(NaN, Infinity, undefined);", &[]);
  assert_binds("o.NaN = 1;", &[]);
  assert_binds("export default 1;", &[]);
  assert_binds("", &[]);
}

/// TypeScript's three binding forms are not collected, because they do not
/// reach this walk: `typescript_strip` runs ahead of the StyleX pass and lowers
/// each one to a `var` or a `const`, which `visit_binding_ident` then records
/// like any other. Pinned as absent rather than left unstated, so the day the
/// strip moves the gap reads as a failing test instead of a global folding
/// where the module had bound the name.
///
/// The one place the gap is real is this crate's own test transform, which runs
/// the resolver but not the strip (`transform::mod`'s `_typescript_factory` is
/// unused). Every input in that suite is JavaScript, so nothing there reaches
/// it.
#[test]
fn records_nothing_for_typescript_only_binding_forms() {
  assert_ts_binds("enum NaN { a }", &[]);
  assert_ts_binds("import NaN = require('m');", &[]);
  // The `namespace` name itself is not recorded; the `const` inside it is, by
  // the same `visit_binding_ident` that records every other declarator.
  assert_ts_binds("namespace NaN { export const a = 1; }", &["a"]);
}

/// The three names the globals step asks about are ordinary bindings to the
/// collector, in every position one can be written.
#[test]
fn records_a_binding_that_takes_a_global_name_over() {
  assert_binds("const NaN = 1;", &["NaN"]);
  assert_binds("let Infinity;", &["Infinity"]);
  assert_binds("const f = (undefined) => undefined;", &["f", "undefined"]);
  assert_binds("try {} catch (NaN) {}", &["NaN"]);
  assert_binds("import { x as NaN } from 'm';", &["NaN"]);
}

/// Two bindings of one name are two entries, because the set is keyed by `Id`.
/// This is what keeps a reference to the global apart from a reference to the
/// parameter that took its name: the resolver gives them different contexts and
/// the reference carries the one it resolved to.
#[test]
fn keeps_two_bindings_of_one_name_apart() {
  GLOBALS.set(&Globals::default(), || {
    let code = "const NaN = 1; function f(NaN) { return NaN; }";
    let module = resolved_module(code);
    let outer_id = first_top_level_binding(&module);

    let mut collector = ModuleBindingsCollector::writes_only();
    module.visit_with(&mut collector);

    let recorded: Vec<&Id> = collector
      .declared_bindings
      .iter()
      .filter(|(sym, _)| sym == "NaN")
      .collect();

    assert_eq!(recorded.len(), 2, "two bindings named NaN: {:?}", recorded);
    assert!(
      recorded.contains(&&outer_id),
      "the module-level binding is one of them"
    );
  });
}

/// The set survives the collector's cheaper mode, which is the mode most
/// modules are scanned in — the `sx` prop is off by default, and the evaluator
/// runs either way.
#[test]
fn collects_bindings_in_both_modes() {
  GLOBALS.set(&Globals::default(), || {
    let module = resolved_module("import { x as NaN } from 'm'; const f = (Infinity) => 1;");

    for mut collector in [
      ModuleBindingsCollector::for_sx(),
      ModuleBindingsCollector::writes_only(),
    ] {
      module.visit_with(&mut collector);

      assert_names(
        names(collector.declared_bindings.clone()),
        &["NaN", "f", "Infinity"],
        "both modes record the module's bindings",
      );
    }
  });
}
