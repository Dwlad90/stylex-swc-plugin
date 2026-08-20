//! The order the resolution chain asks its questions in.
//!
//! Each step is reachable on its own, but the *order* is only observable where
//! two steps could both answer one reference — and those pairs are exactly what
//! no source text can spell, because the resolver hands a shadowing binding a
//! syntax context of its own and at most one of {import specifier, declarator}
//! then matches. So the module state here is assembled directly, with an import
//! and a declarator sharing one context, which is the only way to see which
//! question is asked first.
//!
//! The import source resolves to nothing under a state manager with no
//! filename, so a reference that reaches the import step reports
//! `IMPORT_PATH_RESOLUTION_ERROR`. That refusal is used throughout as the marker
//! for "step 1 answered", never as a claim about path resolution.

use super::*;

use swc_core::atoms::Atom;
use swc_core::common::{BytePos, DUMMY_SP, GLOBALS, Globals, Span, SyntaxContext};
use swc_core::ecma::ast::{
  BindingIdent, ImportDecl, ImportNamedSpecifier, ImportPhase, ImportSpecifier, Str,
};

use stylex_constants::constants::evaluation_errors::{
  IMPORT_PATH_RESOLUTION_ERROR, NON_CONSTANT, UNDEFINED_CONST, UNINITIALIZED_CONST,
  USED_BEFORE_DECLARATION, unsupported_expression,
};
use stylex_structures::stylex_options::StyleXOptions;

/// The three names the globals step asks about. Every case that is about the
/// step runs over all three rather than picking one, because the step answers
/// for them together and a regression that reached only one of them would
/// otherwise pass.
const FOLDED_GLOBALS: [&str; 3] = ["undefined", "Infinity", "NaN"];

/// The context every binding here is declared in. `SyntaxContext::empty()` is
/// what makes an import specifier and a declarator of the same name resolve to
/// the same binding — the collision the order is visible through.
const MODULE_CONTEXT: SyntaxContext = SyntaxContext::empty();

/// Any other context. What the resolver hands a shadowing binding, and the
/// reason at most one step of the chain can answer a real reference.
///
/// `from_u32` rather than `apply_mark`, which would want a `Mark` allocated for
/// what is only "some context other than that one".
fn shadowing_context() -> SyntaxContext {
  SyntaxContext::from_u32(1)
}

fn ident_at(name: &str, span: Span) -> Ident {
  ident_in(name, span, MODULE_CONTEXT)
}

fn ident_in(name: &str, span: Span, ctxt: SyntaxContext) -> Ident {
  Ident {
    span,
    sym: name.into(),
    optional: false,
    ctxt,
  }
}

fn span_at(lo: u32, hi: u32) -> Span {
  Span {
    lo: BytePos(lo),
    hi: BytePos(hi),
  }
}

/// The span every declarator below occupies, and a reference span safely after
/// it — so a case that is not about position never trips the early-reference
/// step.
const DECLARATOR_SPAN: Span = Span {
  lo: BytePos(10),
  hi: BytePos(20),
};
const LATER_REFERENCE_SPAN: Span = Span {
  lo: BytePos(30),
  hi: BytePos(31),
};

fn declarator_at(name: &str, span: Span, init: Option<Expr>) -> VarDeclarator {
  VarDeclarator {
    span,
    name: Pat::Ident(BindingIdent {
      id: ident_at(name, span),
      type_ann: None,
    }),
    init: init.map(Box::new),
    definite: false,
  }
}

/// `import { <name> } from './tokens.stylex.js'`, the shape the chain's first
/// step matches.
fn theme_import_of(name: &str) -> ImportDecl {
  ImportDecl {
    span: DUMMY_SP,
    specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
      span: DUMMY_SP,
      local: ident_at(name, DUMMY_SP),
      imported: None,
      is_type_only: false,
    })],
    src: Box::new(Str {
      span: DUMMY_SP,
      value: "./tokens.stylex.js".into(),
      raw: None,
    }),
    type_only: false,
    with: None,
    phase: ImportPhase::Evaluation,
  }
}

/// What the module holds about the one name under test — which of the chain's
/// inputs are true of it. Written as a builder so each case names only the steps
/// it is about, and the ones it leaves out read as deliberately absent.
///
/// Named for the state rather than for the AST `Module` it stands in for: this
/// crate has a `resolved_module()` elsewhere that returns the real thing.
#[derive(Default)]
struct ModuleState {
  imported: bool,
  /// `Some(init)` where the module declares the name under test, `init` being
  /// the declarator's initializer if it has one. The name is filled in at
  /// evaluation, so a case can declare `NaN` as readily as `c`.
  declarator: Option<Option<Expr>>,
  reassigned: bool,
  mutated: bool,
  class_declaration: bool,
  function_declaration: bool,
  /// Declarators of *other* names, pushed ahead of the one under test, so a
  /// case can say how far into the declaration list the match sits.
  padding: usize,
  disable_imports: bool,
  reference_context: Option<SyntaxContext>,
  /// Where a binding that leaves no declarator behind sits, when the case has
  /// one — a function parameter or a catch binding.
  parameter: Option<ParameterScope>,
}

/// Which scope a parameter-shaped binding occupies, relative to the reference
/// reading it. The two are what the globals step has to tell apart, and a
/// single field rather than two flags because no case is both.
#[derive(PartialEq, Eq)]
enum ParameterScope {
  /// The context the reference itself reads from — a reference to a parameter
  /// from inside its own function, which is what a dynamic style's body is.
  Own,
  /// A context the reference never reads from — the same name bound in some
  /// sibling scope, which must leave the reference resolving to the global.
  Unrelated,
}

impl ModuleState {
  fn imported(mut self) -> Self {
    self.imported = true;
    self
  }

  fn declared_with(mut self, init: Expr) -> Self {
    self.declarator = Some(Some(init));
    self
  }

  fn declared_without_initializer(mut self) -> Self {
    self.declarator = Some(None);
    self
  }

  fn reassigned(mut self) -> Self {
    self.reassigned = true;
    self
  }

  fn mutated(mut self) -> Self {
    self.mutated = true;
    self
  }

  fn declares_a_class(mut self) -> Self {
    self.class_declaration = true;
    self
  }

  fn declares_a_function(mut self) -> Self {
    self.function_declaration = true;
    self
  }

  fn behind(mut self, unrelated_declarations: usize) -> Self {
    self.padding = unrelated_declarations;
    self
  }

  fn with_imports_disabled(mut self) -> Self {
    self.disable_imports = true;
    self
  }

  /// Binds the name with nothing to read, in the reference's own scope: a
  /// function parameter or a catch binding. The chain's only question about one
  /// is whether it exists, which is the question a dynamic style's parameter
  /// named `NaN` turns on.
  fn bound_as_a_parameter(mut self) -> Self {
    self.parameter = Some(ParameterScope::Own);
    self
  }

  /// Binds the name in a scope the reference does not read from, which is not
  /// the binding the reference names however alike the two are spelled.
  fn bound_in_an_unrelated_scope(mut self) -> Self {
    self.parameter = Some(ParameterScope::Unrelated);
    self
  }

  /// Reads the reference from a context of its own, the way the resolver marks
  /// a binding that shadows the module-level one.
  fn read_from_a_shadowing_scope(mut self) -> Self {
    self.reference_context = Some(shadowing_context());
    self
  }

  /// Evaluates a bare reference to `name` at `reference_span` against this
  /// module.
  fn evaluate(self, name: &str, reference_span: Span) -> Box<EvaluateResult> {
    GLOBALS.set(&Globals::new(), || {
      let mut traversal_state = StateManager::new(StyleXOptions::default());
      let reference = ident_in(
        name,
        reference_span,
        self.reference_context.unwrap_or(MODULE_CONTEXT),
      );

      for index in 0..self.padding {
        traversal_state.declarations.push(declarator_at(
          &format!("unrelated{}", index),
          DECLARATOR_SPAN,
          Some(create_string_expr("blue")),
        ));
      }

      // Every binding site below also registers the binding itself, because the
      // module pre-scan records both: a declarator is a declaration *and* a
      // binding, and the chain's globals step asks only the second question.
      let module_binding = (Atom::from(name), MODULE_CONTEXT);

      if self.imported {
        traversal_state.top_imports.push(theme_import_of(name));
        traversal_state
          .declared_bindings
          .insert(module_binding.clone());
      }

      if let Some(init) = self.declarator {
        traversal_state
          .declarations
          .push(declarator_at(name, DECLARATOR_SPAN, init));
        traversal_state
          .declared_bindings
          .insert(module_binding.clone());
      }

      match self.parameter {
        Some(ParameterScope::Own) => {
          traversal_state.declared_bindings.insert(reference.to_id());
        },
        Some(ParameterScope::Unrelated) => {
          traversal_state
            .declared_bindings
            .insert((Atom::from(name), shadowing_context()));
        },
        None => {},
      }

      if self.reassigned {
        traversal_state
          .binding_reassignments
          .insert(reference.to_id());
      }

      if self.mutated {
        traversal_state.binding_mutations.insert(reference.to_id());
      }

      if self.class_declaration {
        traversal_state.add_class_name_declaration(reference.clone());
        traversal_state
          .declared_bindings
          .insert(module_binding.clone());
      }

      if self.function_declaration {
        traversal_state.add_function_name_declaration(reference.clone());
        traversal_state.declared_bindings.insert(module_binding);
      }

      let functions = FunctionMap {
        disable_imports: self.disable_imports,
        ..FunctionMap::default()
      };

      evaluate(&Expr::Ident(reference), &mut traversal_state, &functions)
    })
  }

  /// The common case: a reference to `c`, positioned after every declarator.
  fn evaluate_a_later_reference(self) -> Box<EvaluateResult> {
    self.evaluate("c", LATER_REFERENCE_SPAN)
  }
}

#[track_caller]
fn assert_refused_with(result: &EvaluateResult, reason: &str) {
  assert!(
    !result.confident,
    "expected a refusal, got the value {:?}",
    result.value
  );

  assert_eq!(result.reason.as_deref(), Some(reason));
}

/// The global stood as itself — the answer step 7 gives a name nothing bound.
#[track_caller]
fn assert_folded_to_the_global(result: &EvaluateResult, name: &str) {
  assert!(
    result.confident,
    "expected `{}` to fold, got the refusal {:?}",
    name, result.reason
  );

  match &result.value {
    Some(EvaluateResultValue::Expr(Expr::Ident(ident))) => assert_eq!(ident.sym, *name),
    other => panic!("expected the global `{}`, got {:?}", name, other),
  }
}

#[track_caller]
fn assert_folded_to_the_string(result: &EvaluateResult, expected: &str) {
  assert!(
    result.confident,
    "expected a fold, got the refusal {:?}",
    result.reason
  );

  match &result.value {
    Some(EvaluateResultValue::Expr(Expr::Lit(Lit::Str(strng)))) => {
      assert_eq!(strng.value, expected)
    },
    other => panic!("expected the string {:?}, got {:?}", expected, other),
  }
}

// ==================== step 1 — the import specifier ====================

/// The step exists, and the refusal below is what "it answered" looks like for
/// every ordering case after this one.
#[test]
fn an_imported_name_resolves_through_the_import_step() {
  let result = ModuleState::default()
    .imported()
    .evaluate_a_later_reference();

  assert_refused_with(&result, IMPORT_PATH_RESOLUTION_ERROR);
}

/// The move this chain performed: the import step used to run *after* the
/// declarator read, so a name that was both would have folded to the
/// initializer. Inert on any real module — the resolver keeps the two apart —
/// and asserted anyway, because "inert" is a claim about the order and this is
/// where it is visible.
#[test]
fn the_import_step_is_asked_before_the_declarator_read() {
  let result = ModuleState::default()
    .imported()
    .declared_with(create_string_expr("red"))
    .evaluate_a_later_reference();

  assert_refused_with(&result, IMPORT_PATH_RESOLUTION_ERROR);
}

/// And before both write probes, which used to run first of all. Same
/// inertness, same reason for asserting it.
#[test]
fn the_import_step_is_asked_before_the_write_probes() {
  let result = ModuleState::default()
    .imported()
    .declared_with(create_string_expr("red"))
    .reassigned()
    .mutated()
    .evaluate_a_later_reference();

  assert_refused_with(&result, IMPORT_PATH_RESOLUTION_ERROR);
}

// ==================== steps 3 and 4 — the write probes ====================

#[test]
fn a_reassigned_binding_is_not_a_constant() {
  let result = ModuleState::default()
    .declared_with(create_string_expr("red"))
    .reassigned()
    .evaluate_a_later_reference();

  assert_refused_with(&result, NON_CONSTANT);
}

/// The second probe answers on its own, which is the whole point of splitting
/// the one write set in two: a mutated-but-never-rebound binding reaches step 4
/// with step 3 silent.
#[test]
fn a_binding_mutated_in_place_is_not_a_constant() {
  let result = ModuleState::default()
    .declared_with(create_string_expr("red"))
    .mutated()
    .evaluate_a_later_reference();

  assert_refused_with(&result, NON_CONSTANT);
}

/// Both probes are guarded on the name having a declaration at all, so a write
/// recorded against a name this module declares nowhere falls past them — and
/// reaches the chain's terminal refusal on its own merits rather than reporting
/// the write. An import, a global or an injected function is written nowhere
/// this compiler can see, and a stray recorded write must not answer for them.
#[test]
fn a_recorded_write_against_no_declaration_is_not_a_constant_violation() {
  let result = ModuleState::default()
    .reassigned()
    .mutated()
    .evaluate_a_later_reference();

  assert_refused_with(&result, UNDEFINED_CONST);
}

/// The write probes sit above the early-reference comparison, so a binding that
/// is both reports the write. Position is the narrower fault: a reference below
/// the declaration would still be refused, where an early one might only be
/// early.
#[test]
fn a_write_probe_answers_before_the_early_reference_comparison() {
  let result = ModuleState::default()
    .declared_with(create_string_expr("red"))
    .reassigned()
    .evaluate("c", span_at(1, 2));

  assert_refused_with(&result, NON_CONSTANT);
}

// ==================== step 5 — the early reference ====================

/// Covered exhaustively beside this file in `used_before_declaration.rs`,
/// including the spans that are not positions. Here only to fix its rung on the
/// chain: it answers after the write probes and before the initializer read.
#[test]
fn an_early_reference_answers_before_the_initializer_read() {
  let result = ModuleState::default()
    .declared_with(create_string_expr("red"))
    .evaluate("c", span_at(1, 2));

  assert_refused_with(&result, USED_BEFORE_DECLARATION);
}

// ==================== step 7 — the three globals ====================

/// With no declaration of the name anywhere, the global is what the reference
/// names, and it stands as itself.
#[test]
fn an_undeclared_global_folds_to_itself() {
  for name in FOLDED_GLOBALS {
    let result = ModuleState::default().evaluate(name, LATER_REFERENCE_SPAN);

    assert_folded_to_the_global(&result, name);
  }
}

/// A binding of one of those names takes the global over, and the step refuses
/// rather than folding — there is no value to fold, because the step upstream
/// reads one from is the absent step 6. The declaration's initializer is *not*
/// read: a reference that names a binding never reaches step 8.
#[test]
fn a_declared_global_is_refused_rather_than_read() {
  for name in FOLDED_GLOBALS {
    assert_refused_with(
      &ModuleState::default()
        .declared_with(create_string_expr("red"))
        .evaluate(name, LATER_REFERENCE_SPAN),
      UNINITIALIZED_CONST,
    );
  }
}

/// And with nothing to read either way, which is the shape the message names.
#[test]
fn a_global_declared_without_an_initializer_is_refused() {
  let result = ModuleState::default()
    .declared_without_initializer()
    .evaluate("NaN", LATER_REFERENCE_SPAN);

  assert_refused_with(&result, UNINITIALIZED_CONST);
}

/// The case the step exists for: a binding that leaves no declarator behind —
/// a dynamic style's parameter. Nothing else in the chain can see one, so the
/// binding is the whole of what makes this a refusal, and the refusal is what
/// sends the value down the inline-style path the parameter comes from.
#[test]
fn a_global_taken_over_by_a_parameter_is_refused() {
  for name in FOLDED_GLOBALS {
    assert_refused_with(
      &ModuleState::default()
        .bound_as_a_parameter()
        .evaluate(name, LATER_REFERENCE_SPAN),
      UNINITIALIZED_CONST,
    );
  }
}

/// A parameter read from its own scope, which is what the resolver hands a
/// dynamic style's body — the reference and the binding share a context that is
/// nobody else's.
#[test]
fn a_parameter_in_a_scope_of_its_own_still_takes_the_global_over() {
  let result = ModuleState::default()
    .bound_as_a_parameter()
    .read_from_a_shadowing_scope()
    .evaluate("NaN", LATER_REFERENCE_SPAN);

  assert_refused_with(&result, UNINITIALIZED_CONST);
}

/// And the half that keeps the step honest: a binding of the same *name* in
/// some unrelated scope is not the binding this reference names. The question is
/// asked of the `Id`, so the module-level global stands, exactly as it does
/// upstream where the scope chain is walked from the reference.
#[test]
fn a_global_beside_an_unrelated_binding_of_its_name_still_folds_to_itself() {
  for name in FOLDED_GLOBALS {
    let result = ModuleState::default()
      .bound_in_an_unrelated_scope()
      .evaluate(name, LATER_REFERENCE_SPAN);

    assert_folded_to_the_global(&result, name);
  }
}

/// The step is about these three names and no others. A binding of any other
/// name is an ordinary binding and falls through to the steps below.
#[test]
fn a_binding_of_any_other_name_is_untouched_by_the_globals_step() {
  assert_folded_to_the_string(
    &ModuleState::default()
      .declared_with(create_string_expr("red"))
      .evaluate_a_later_reference(),
    "red",
  );

  // Near-misses, so the comparison is against the whole name rather than a
  // prefix, a case fold, or a trimmed edge.
  for name in [
    "nan",
    "NaNs",
    "NAN",
    "Infinit",
    "Infinity2",
    "undefined_",
    " NaN",
  ] {
    assert_refused_with(
      &ModuleState::default()
        .bound_as_a_parameter()
        .evaluate(name, LATER_REFERENCE_SPAN),
      UNDEFINED_CONST,
    );
  }
}

/// The steps above the globals step still answer first for these names, so the
/// refusal a reader sees is the narrowest true one rather than always this
/// step's. The import step is covered separately by
/// `an_import_aliased_to_a_global_name_resolves_as_the_import`.
#[test]
fn the_steps_above_the_globals_step_answer_for_these_names_too() {
  assert_refused_with(
    &ModuleState::default()
      .declared_with(create_string_expr("red"))
      .reassigned()
      .evaluate("NaN", LATER_REFERENCE_SPAN),
    NON_CONSTANT,
  );

  assert_refused_with(
    &ModuleState::default()
      .declared_with(create_string_expr("red"))
      .mutated()
      .evaluate("Infinity", LATER_REFERENCE_SPAN),
    NON_CONSTANT,
  );

  assert_refused_with(
    &ModuleState::default()
      .declared_with(create_string_expr("red"))
      .evaluate("undefined", span_at(1, 2)),
    USED_BEFORE_DECLARATION,
  );
}

// ==================== step 8 — the declaration, then the refusals ====================

#[test]
fn a_declarator_folds_to_its_initializer() {
  let result = ModuleState::default()
    .declared_with(create_string_expr("red"))
    .evaluate_a_later_reference();

  assert_folded_to_the_string(&result, "red");
}

/// The chain's terminal step. A hoisted `class` or `function` holds its value
/// from the top of its scope, so neither is ever early and neither has an
/// initializer to read; both land here, with the texts the reference
/// implementation emits.
#[test]
fn a_class_or_function_declaration_is_refused_as_unsupported() {
  assert_refused_with(
    &ModuleState::default()
      .declares_a_class()
      .evaluate_a_later_reference(),
    &unsupported_expression("ClassDeclaration"),
  );

  assert_refused_with(
    &ModuleState::default()
      .declares_a_function()
      .evaluate_a_later_reference(),
    &unsupported_expression("FunctionDeclaration"),
  );
}

/// A name the module binds nowhere at all — no import, no declaration, not one
/// of the three globals.
#[test]
fn an_unbound_name_is_refused_as_undefined() {
  let result = ModuleState::default().evaluate_a_later_reference();

  assert_refused_with(&result, UNDEFINED_CONST);
}

// ==================== the inputs at their edges ====================

/// The invariant every "the reorder is inert" claim above rests on: a reference
/// from a scope of its own matches neither the import specifier nor the
/// declarator, however the two are arranged. This is what a dynamic style's
/// parameter shadowing an imported theme looks like from the chain's side, and
/// resolving it to the binding it shadows is issue #1266.
#[test]
fn a_reference_from_a_shadowing_scope_matches_no_module_binding() {
  let result = ModuleState::default()
    .imported()
    .declared_with(create_string_expr("red"))
    .reassigned()
    .mutated()
    .read_from_a_shadowing_scope()
    .evaluate_a_later_reference();

  assert_refused_with(&result, UNDEFINED_CONST);
}

/// A binding name is any JavaScript identifier, and the chain compares symbols
/// rather than bytes it has vetted — so a name outside ASCII resolves on the
/// same comparison as one inside it. A unicode *escape* is not a case here: the
/// parser resolves `\u{7a}Index` to the symbol `zIndex` before any of this
/// state exists, so the chain never has two spellings to reconcile. The corpus
/// carries that one, where a parser runs.
#[test]
fn a_non_ascii_binding_name_resolves_like_any_other() {
  for name in ["zÍndex", "цвет", "переменная_цвета", "\u{5f20}"] {
    let result = ModuleState::default()
      .declared_with(create_string_expr("red"))
      .evaluate(name, LATER_REFERENCE_SPAN);

    assert_folded_to_the_string(&result, "red");
  }
}

/// The position comparison runs on the parser's `BytePos`, so the far end of
/// that space is a boundary worth holding: a reference at the last addressable
/// byte is after every declarator, and the comparison must reach that
/// conclusion by comparing rather than by overflowing.
#[test]
fn the_position_comparison_holds_at_the_far_end_of_the_address_space() {
  let result = ModuleState::default()
    .declared_with(create_string_expr("red"))
    .evaluate("c", span_at(u32::MAX - 1, u32::MAX));

  assert_folded_to_the_string(&result, "red");
}

/// A zero-width reference span is a position all the same — `lo == hi` says
/// where, not how much — so it is compared rather than exempted. Only
/// `DUMMY_SP` is exempt, and `DUMMY_SP` is zero-width *at zero*.
#[test]
fn a_zero_width_reference_span_is_still_a_position() {
  assert_refused_with(
    &ModuleState::default()
      .declared_with(create_string_expr("red"))
      .evaluate("c", span_at(15, 15)),
    USED_BEFORE_DECLARATION,
  );

  assert_folded_to_the_string(
    &ModuleState::default()
      .declared_with(create_string_expr("red"))
      .evaluate("c", span_at(25, 25)),
    "red",
  );
}

/// Both lookups the chain performs scan the module's declaration list, so a
/// module with a great many declarations is the shape that would expose a
/// lookup accidentally bounded, sampled, or quadratic. Ten thousand entries with
/// the match last: the answer is the same one a two-entry module gives.
#[test]
fn a_match_at_the_end_of_a_long_declaration_list_still_resolves() {
  let result = ModuleState::default()
    .behind(10_000)
    .declared_with(create_string_expr("red"))
    .evaluate_a_later_reference();

  assert_folded_to_the_string(&result, "red");
}

/// And the same list with the name written, so the write probes' guard — which
/// runs the scan a second time only when a write was recorded — is exercised at
/// the same length.
#[test]
fn a_written_match_at_the_end_of_a_long_declaration_list_still_refuses() {
  let result = ModuleState::default()
    .behind(10_000)
    .declared_with(create_string_expr("red"))
    .mutated()
    .evaluate_a_later_reference();

  assert_refused_with(&result, NON_CONSTANT);
}

/// `disable_imports` is set where a fold must not reach outside the module —
/// the runtime function map is evaluated that way. The import step is skipped
/// entirely rather than refused, so an imported name falls through the whole
/// chain to the terminal refusal.
#[test]
fn a_disabled_import_step_falls_through_the_rest_of_the_chain() {
  let result = ModuleState::default()
    .imported()
    .with_imports_disabled()
    .evaluate_a_later_reference();

  assert_refused_with(&result, UNDEFINED_CONST);
}

/// With the import step skipped, the steps behind it answer for themselves —
/// which is the other half of "skipped, not refused".
#[test]
fn a_disabled_import_step_leaves_the_steps_behind_it_answering() {
  assert_folded_to_the_string(
    &ModuleState::default()
      .imported()
      .with_imports_disabled()
      .declared_with(create_string_expr("red"))
      .evaluate_a_later_reference(),
    "red",
  );

  assert_refused_with(
    &ModuleState::default()
      .imported()
      .with_imports_disabled()
      .declared_with(create_string_expr("red"))
      .reassigned()
      .evaluate_a_later_reference(),
    NON_CONSTANT,
  );
}

/// The one pair the reorder is not inert on, and the reason it is not: `NaN` is
/// an ordinary binding name, so an import can be aliased to it and no syntax
/// context keeps the two apart. The import answers, where the global used to.
///
/// Recorded here because it is the whole observable extent of the reorder;
/// `modules-1266-import-aliased-to-a-global-name` in the parity corpus measures
/// the same shape against the reference implementation, which agrees.
#[test]
fn an_import_aliased_to_a_global_name_resolves_as_the_import() {
  for name in FOLDED_GLOBALS {
    let result = ModuleState::default()
      .imported()
      .evaluate(name, LATER_REFERENCE_SPAN);

    assert_refused_with(&result, IMPORT_PATH_RESOLUTION_ERROR);
  }
}

/// The write probes' guard asks for a `VarDeclarator`, where upstream asks
/// whether a binding exists at all — so a hoisted `function` or `class` that is
/// reassigned falls past both probes and is refused for its declaration kind
/// instead. Both compilers refuse; the texts differ. Pinned so the narrowing is
/// a measured difference rather than a thing to rediscover.
#[test]
fn a_reassigned_function_declaration_is_refused_for_its_kind_not_the_write() {
  assert_refused_with(
    &ModuleState::default()
      .declares_a_function()
      .reassigned()
      .evaluate_a_later_reference(),
    &unsupported_expression("FunctionDeclaration"),
  );

  assert_refused_with(
    &ModuleState::default()
      .declares_a_class()
      .mutated()
      .evaluate_a_later_reference(),
    &unsupported_expression("ClassDeclaration"),
  );
}
