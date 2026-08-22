//! Resolving a reference to the binding it names.
//!
//! One ordered question with one answer, asked once per identifier the
//! evaluator's dispatch could not fold from the injected function map. The
//! steps run in the reference implementation's order and each cites the
//! `evaluate-path.js` 0.19.0 line range it mirrors, so the two can be read side
//! by side; `docs/adr/0003-one-ordered-chain-resolves-a-reference.md` records
//! why the order is the reference implementation's rather than this compiler's.

use super::*;

/// Whether a reference reads a binding the program does not hold yet, because
/// the declarator naming it ends after the reference begins.
///
/// Mirrors the position comparison in `evaluate-path.js`'s
/// `isReferencedIdentifier` branch (`path.node.start < binding.path.node.end`,
/// 0.19.0 line 664), which is the reference implementation's whole answer to the
/// question. Declarations here are collected module-wide with no notion of
/// position, so without this the initializer is reachable from above its own
/// declaration and folds into CSS for a value that has not been assigned.
///
/// A dummy span on either side answers `false` rather than being compared. A
/// synthesized node carries no authored position, so it sits at byte zero,
/// before every authored declarator's end, and would be refused for having no
/// position rather than for being early. `expand_shorthand_prop` is the producer
/// that reaches here — the injected function mappers `get_var_decl_by_ident`
/// also folds in do not, because `nodes::identifier::evaluate` answers for every
/// name in `functions.identifiers` before this chain is entered.
///
/// Only a `VarDeclarator` is ever asked. A hoisted `function` or `class`
/// declaration holds its value from the top of the scope, so a reference above
/// one is not early; those reach `check_ident_declaration` instead and are
/// refused there, as they are upstream.
fn reads_before_its_declaration(reference: &Ident, declarator: &VarDeclarator) -> bool {
  !reference.span.is_dummy()
    && !declarator.span.is_dummy()
    && reference.span.lo < declarator.span.hi
}

/// Resolve `reference` to the binding it names, and fold that binding to a
/// value — or refuse.
///
/// `path` is the expression as the caller received it and `normalized_path` the
/// same expression with its parenthesis and TypeScript wrappers peeled; the two
/// differ only in which one a diagnostic points at, and each step keeps the one
/// it reported against before this chain had a home of its own.
///
/// The steps, in `evaluate-path.js` 0.19.0 order:
///
/// 1. an import specifier resolves to a theme reference (599-650) — a *named*
///    one; the other two specifier kinds are answered inside this step
/// 2. a default-import specifier (652-654)
/// 3. a reassigned binding is not a constant (656-658)
/// 4. a binding mutated in place is not a constant (660-662)
/// 5. a reference above its own declarator is early (664-666)
/// 6. a binding carrying a folded value (668-669) — absent, see below
/// 7. `undefined` / `Infinity` / `NaN` (670-683)
/// 8. the declarator's initializer, else the class / function refusals
///    (685-690)
pub(super) fn resolve_reference(
  ident: &Ident,
  path: &Expr,
  normalized_path: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  // ── 1. an import specifier resolves to a theme reference (599-650) ────────
  //
  // Steps 1 and 2 read one lookup, so they are nested rather than sequential:
  // both ask what kind of specifier binds this reference, and upstream's step 1
  // guard (`!bindingPath.isImportDefaultSpecifier() &&
  // !bindingPath.isImportNamespaceSpecifier() && bindingPath.isImportSpecifier()`,
  // 0.19.0 lines 600-605) is the same question steps 2 and this one's own
  // namespace arm answer. The order is upstream's either way — neither of the
  // other two specifier kinds resolves a theme reference here.
  if let Some((import_path, specifier)) = get_import_by_ident(ident, traversal_state) {
    // Which single export of the imported file this reference names, or `None`
    // where the specifier names no single export and the step therefore does
    // not apply.
    let imported: Option<ModuleExportName> = match specifier {
      // ── 2. a default-import specifier (652-654) ─────────────────────────
      //
      // A theme file is read through its named exports, so there is no theme
      // reference a default binding could answer with. Measured on both
      // compilers as `modules-1266-default-theme-import`: upstream refuses, and
      // resolving one here emitted a variable the theme file does not define.
      //
      // Deliberately outside the `disable_imports` guard below, as upstream is:
      // it gates only the resolution on `state.functions.disableImports` and
      // refuses a default import either way.
      //
      // Upstream does not return from its refusal — the reference falls through
      // the rest of the chain and lands on `UNDEFINED_CONST`, which deopts a
      // second time. The first refusal wins on both sides, so the fall-through
      // is unobservable; returning here says so.
      ImportSpecifier::Default(_) => return deopt(path, state, IMPORT_FILE_EVAL_ERROR),

      ImportSpecifier::Named(named) => Some(
        named
          .imported
          .clone()
          .unwrap_or_else(|| ModuleExportName::Ident(named.local.clone())),
      ),

      // A namespace specifier binds the module's whole export object, so it
      // names no export for a theme reference to be built from — which is why
      // upstream's step 1 excludes it: the step reads `importSpecifierNode.
      // imported`, a field an `ImportNamespaceSpecifier` does not carry. It is
      // a guard on the step's input rather than a verdict on the import kind,
      // and unlike a default specifier it is given no refusal of its own; the
      // reference falls through to `UNDEFINED_CONST` at the tail of the chain.
      //
      // Resolving one here instead — by synthesizing the reference's own local
      // alias as the export name — is what this arm gives up, and giving it up
      // is a decision rather than a mirror. ADR 0003 argues it from the
      // measurement, and `modules-1266-a-namespace-theme-import` is where that
      // measurement is executed rather than described.
      ImportSpecifier::Namespace(_) => None,
    };

    if let Some(imported) = imported
      && !state.functions.disable_imports
    {
      let abs_path = traversal_state.import_path_resolver(
        convert_atom_to_str_ref(&import_path.src.value),
        &mut FxHashMap::default(),
      );

      let imported_name = match imported {
        ModuleExportName::Ident(ident) => ident.sym.to_string(),
        ModuleExportName::Str(strng) => convert_atom_to_string(&strng.value),
      };

      let return_value = match abs_path {
        ImportPathResolution::Resolved { path: value } => {
          evaluate_theme_ref(&value, imported_name, traversal_state)
        },
        ImportPathResolution::Unresolved => {
          return deopt(path, state, IMPORT_PATH_RESOLUTION_ERROR);
        },
      };

      if state.confident {
        let import_path_src = convert_atom_to_string(&import_path.src.value);

        if !state.added_imports.contains(&import_path_src)
          && traversal_state.get_treeshake_compensation()
        {
          let prepend_import_module_item = add_import_expression(&import_path_src);
          // Theme side-effect imports go under ThemeImports — the slot
          // whose flush position matches the legacy
          // `prepend_import_module_items` placement (between the
          // runtime helpers and the existing import block,
          // regardless of producer queue order). Dedup is by stable
          // hash on the StateManager so it survives across
          // evaluations.
          traversal_state.queue_theme_import_if_absent(prepend_import_module_item);

          state.added_imports.insert(import_path_src);
        }

        return Some(EvaluateResultValue::ThemeRef(return_value));
      }

      // Upstream gives `IMPORT_FILE_EVAL_ERROR` a second time here, where a
      // resolution came back *unconfident* (0.19.0 line 6360). Deliberately
      // absent, and unreachable rather than skipped: upstream reaches that
      // branch only out of `evaluateImportedFile`, which parses the imported
      // module and folds the named export out of it. This compiler has no such
      // arm — `ImportPathResolution` resolves to a path or to nothing — and
      // `evaluate_theme_ref` is a constructor over `&StateManager` that cannot
      // clear confidence. So no resolution reached from here can leave the
      // evaluation unconfident, and there is no input for the branch to answer.
      //
      // It becomes reachable the day this compiler evaluates an imported file in
      // its own right, which is the same capability the globals step's
      // cross-file gap waits on.
    }
  }

  // A binding whose value can differ from its declaration initializer at this
  // use site — rebound or mutated — makes inlining that initializer bake in a
  // stale value. Both steps below are a hash probe guarded by a declaration
  // lookup, and both are asked before the cloning lookup further down, which
  // deep-clones the `VarDeclarator` it finds and throws the clone away on the
  // refusal path.
  //
  // Each step probes its own set and each is spelled with the probe first, so
  // the guard's scan of the declaration list runs only for a name some write
  // was actually recorded against — and step 4 costs nothing once step 3 has
  // refused.
  //
  // The guard is narrower than upstream's, and deliberately unchanged here:
  // upstream asks whether a *binding* exists, where this asks whether a
  // `VarDeclarator` does. A `function` or `class` binding that is reassigned
  // therefore reaches step 8 and is refused for its declaration kind instead of
  // for the write. Both refuse; only the text differs, and closing that is a
  // change to a message rather than a move.
  //
  // Existence is confirmed with the borrowing `get_var_decl_from`, which —
  // unlike `get_var_decl_by_ident` — does not also match injected function
  // mappers; those are regenerated per evaluation and can never hold a stale
  // value.

  // ── 3. a reassigned binding is not a constant (656-658) ───────────────────
  if traversal_state.has_binding_reassignment(ident)
    && get_var_decl_from(traversal_state, ident).is_some()
  {
    return deopt(path, state, NON_CONSTANT);
  }

  // ── 4. a binding mutated in place is not a constant (660-662) ─────────────
  if traversal_state.has_binding_mutation(ident)
    && get_var_decl_from(traversal_state, ident).is_some()
  {
    return deopt(path, state, NON_CONSTANT);
  }

  let declarator = get_var_decl_by_ident(ident, traversal_state, &state.functions);

  // ── 5. a reference above its own declarator is early (664-666) ────────────
  //
  // Asked of the declarator already looked up, so the answer costs a
  // comparison rather than a second scan of the declaration list.
  if let Some(declarator) = declarator.as_ref()
    && reads_before_its_declaration(ident, declarator)
  {
    return deopt(path, state, USED_BEFORE_DECLARATION);
  }

  // ── 6. a binding carrying a folded value (668-669) ────────────────────────
  //
  // Deliberately absent, and it is a comment rather than dead code because
  // there is nothing to write: upstream reads `binding.hasValue`, which Babel
  // sets only through `setValue` / `clearValue`, and the reference plugin never
  // calls either. The field is always false there, so this step never fires
  // there either, and every input falls through to the two below exactly as it
  // does here.

  // ── 7. `undefined` / `Infinity` / `NaN` (670-683) ─────────────────────────
  //
  // This step used to be asked before the import step, and moving it behind one
  // is the single outcome the reorder is not inert on: `NaN` is a legal binding
  // name, so `import { zIndex as NaN }` is the one shape where an import
  // specifier and one of these globals name the same binding, and a syntax
  // context cannot keep those two apart. The import now answers it, as it does
  // upstream — measured on both compilers as
  // `modules-1266-import-aliased-to-a-global-name`, which the reorder turns from
  // a divergence into agreement.
  //
  // The three names are ordinary bindings to the language, so the question is
  // which of the two this reference is — the global, or something in scope that
  // took the name over. The binding decides, and nothing below it does: the
  // step answers for all three names either way, and a reference that names a
  // binding never reaches the initializer read below.
  //
  // A binding that took one of these names over carries no value the evaluator
  // holds — the reference implementation reads `binding.hasValue`, which is step
  // 6, deliberately absent above and always false there — so there is nothing
  // to fold and the step refuses. That refusal is what turns a dynamic style's
  // parameter named `NaN` into an inline style: the value falls through to the
  // inline-style path, which is where the parameter comes from.
  //
  // The binding question is asked of the module being compiled, where upstream
  // asks the scope chain of whatever file the reference sits in. The two part
  // company on a reference read out of an *imported* file, whose bindings carry
  // a syntax context this module's pre-scan never saw: the name misses and the
  // global stands. It fails in the safe direction — a fold that should have
  // been refused, never a refusal of something that should fold — and closing
  // it means evaluating an imported file in its own right, which this compiler
  // does not do at all yet.
  //
  // What the global answers *with* is the value, not the name. `NaN` and
  // `Infinity` are numbers the grammar has no literal for, and a consumer that
  // reads the expression's shape rather than coercing it -- style-value
  // validation is the one that does -- sees an identifier and refuses. Handing
  // back the name made `height: [NaN, '2px']` refuse an array the reference
  // implementation accepts, while `height: [0/0, '2px']`, the same value
  // reached by arithmetic, folded and agreed. `undefined` has no other
  // spelling and answers itself.
  if let Some(value) = global_spelled_as_an_identifier_as_a_value(ident) {
    return if traversal_state.declares_binding(ident) {
      deopt(path, state, UNINITIALIZED_CONST)
    } else {
      Some(EvaluateResultValue::Expr(value))
    };
  }

  // ── 8. the declarator's initializer, else the class / function refusals ───
  //      (685-690)
  if let Some(init) = declarator.and_then(|mut declarator| declarator.init.take()) {
    return evaluate_cached(&init, state, traversal_state, fns);
  }

  check_ident_declaration(
    ident,
    &[
      (
        DeclarationType::Class,
        traversal_state.class_name_declarations(),
      ),
      (
        DeclarationType::Function,
        traversal_state.function_name_declarations(),
      ),
    ],
    state,
    normalized_path,
  )
}

#[cfg(test)]
#[path = "tests/resolution_order.rs"]
mod resolution_order;

#[cfg(test)]
#[path = "tests/used_before_declaration.rs"]
mod used_before_declaration;
