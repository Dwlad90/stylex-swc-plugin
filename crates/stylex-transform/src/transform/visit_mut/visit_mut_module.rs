use swc_core::{
  common::{BytePos, Span, comments::Comments},
  ecma::{
    ast::{
      ArrayPat, ArrowExpr, AssignExpr, AssignTarget, AssignTargetPat, BindingIdent, BlockStmt,
      CallExpr, Callee, CatchClause, ClassDecl, ClassExpr, Expr, FnDecl, FnExpr, ForHead,
      ForInStmt, ForOfStmt, Function, Id, Ident, ImportDecl, MemberExpr, MemberProp, Module,
      ModuleItem, ObjectPat, ObjectPatProp, OptChainBase, Pat, Program, Script, SimpleAssignTarget,
      UnaryExpr, UnaryOp, UpdateExpr, VarDecl, VarDeclKind,
    },
    visit::{Visit, VisitMutWith, VisitWith},
  },
};

use crate::{
  StyleXTransform,
  shared::{
    structures::state_manager::{
      build_decl_use_graph, compute_live_set, flush_pending_insertions, mark_style_vars_to_keep,
    },
    utils::{ast::convertors::convert_atom_to_string, common::fill_top_level_expressions},
  },
};
use rustc_hash::{FxHashMap, FxHashSet};
use stylex_constants::constants::common::{MUTATING_ARRAY_METHODS, MUTATING_OBJECT_METHODS};
use stylex_enums::core::TransformationCycle;

/// Span covering the whole source, used for the module-level scope frame so
/// top-level bindings enclose every `sx` site. The scope stack is seeded with
/// this frame and never emptied, so the scope lookups always resolve to a real
/// frame and fall back to this span only in theory.
const MODULE_SCOPE_SPAN: Span = Span {
  lo: BytePos(0),
  hi: BytePos(u32::MAX),
};

/// The kind of lexical scope a frame on the [`ModuleBindingsCollector`] stack
/// represents. `var` declarations hoist to the nearest `Function` scope;
/// everything else (`let`/`const`/`class`, function declarations in module
/// code, params, catch bindings) belongs to the innermost scope, `Block` or
/// `Function`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ScopeKind {
  Function,
  Block,
}

/// A lexical scope on the collector's stack, identified by the source span it
/// covers. Shadowing is decided by span containment: a binding shadows an
/// `sx` site when the binding's scope span encloses that site's span.
#[derive(Clone, Copy)]
struct ScopeFrame {
  span: Span,
  kind: ScopeKind,
}

/// One-time read-only pre-scan of the module performed at the start of the
/// `Discover` cycle. Captures the sources of every import declaration
/// (including type-only ones) in body order, the names of every bound
/// identifier in the module, and — for non-import bindings — the source span
/// of the scope each one occupies. It also records writes to bindings and
/// mutations of their referenced values for static evaluation. SWC visitors
/// have no parent pointers or scope chain, so this pre-scan supplies the
/// binding information used by both runtime-binding resolution and evaluation.
struct ModuleBindingsCollector {
  collect_sx_bindings: bool,
  import_sources: Vec<String>,
  bound_names: FxHashSet<String>,
  /// For each name bound by a non-import declaration, the spans of the scopes
  /// in which it is bound. A name shadows an `sx` site iff one of its scope
  /// spans encloses that site (see [`StateManager::is_locally_rebound_at`]).
  local_rebinding_scopes: FxHashMap<String, Vec<Span>>,
  /// Bindings rebound or mutated anywhere in the module. See
  /// [`StateManager::binding_writes`].
  binding_writes: FxHashSet<Id>,
  /// Stack of enclosing lexical scopes, outermost (module) first.
  scope_stack: Vec<ScopeFrame>,
  /// `VarDeclKind` of the `VarDecl` currently being visited, if any — needed
  /// to route `var` bindings to the nearest function scope (hoisting) while
  /// `let`/`const` stay in the innermost scope.
  current_var_kind: Option<VarDeclKind>,
}

impl ModuleBindingsCollector {
  fn new(collect_sx_bindings: bool) -> Self {
    Self {
      collect_sx_bindings,
      import_sources: Vec::new(),
      bound_names: FxHashSet::default(),
      local_rebinding_scopes: FxHashMap::default(),
      binding_writes: FxHashSet::default(),
      // Seed a module-level function scope spanning the whole source so
      // top-level bindings enclose every `sx` site.
      scope_stack: vec![ScopeFrame {
        span: MODULE_SCOPE_SPAN,
        kind: ScopeKind::Function,
      }],
      current_var_kind: None,
    }
  }

  /// Span of the innermost enclosing function scope (where `var`/function
  /// declarations hoist to). Falls back to the module scope, always present.
  fn nearest_function_scope(&self) -> Span {
    self
      .scope_stack
      .iter()
      .rev()
      .find(|frame| frame.kind == ScopeKind::Function)
      .map(|frame| frame.span)
      .unwrap_or(MODULE_SCOPE_SPAN)
  }

  /// Span of the innermost enclosing scope (where `let`/`const`, params, etc.
  /// belong). Always present — the module scope sits at the stack bottom.
  fn innermost_scope(&self) -> Span {
    self
      .scope_stack
      .last()
      .map(|frame| frame.span)
      .unwrap_or(MODULE_SCOPE_SPAN)
  }

  /// Record a binding produced by a non-import declaration in both
  /// `bound_names` (every binding) and `local_rebinding_scopes` (non-import
  /// only), scoping it to the function scope when `hoisted` (`var`
  /// declarations) or the innermost scope otherwise.
  fn add_local_binding(&mut self, name: &str, hoisted: bool) {
    if !self.collect_sx_bindings {
      return;
    }

    let name = name.to_string();
    let scope = if hoisted {
      self.nearest_function_scope()
    } else {
      self.innermost_scope()
    };
    self.bound_names.insert(name.clone());
    self
      .local_rebinding_scopes
      .entry(name)
      .or_default()
      .push(scope);
  }

  /// Record a write to `ident`'s binding — a rebinding or an in-place
  /// mutation of the value it references. Both make the declaration
  /// initializer unsafe to inline at a use site.
  fn add_binding_write(&mut self, ident: &Ident) {
    self.binding_writes.insert(ident.to_id());
  }

  /// Record a write reached through a member expression (`obj.x = 1`,
  /// `obj.a.b.push(…)`, `delete obj.x`). Only the *root* object of the chain
  /// is a binding, so walk past intermediate members, parens and optional
  /// links to reach it — stopping at anything that is not a plain identifier
  /// (a call result or literal owns no binding to invalidate).
  fn add_member_root_write(&mut self, member_expression: &MemberExpr) {
    let mut object = member_expression.obj.as_ref();

    loop {
      match object {
        Expr::Ident(ident) => {
          self.add_binding_write(ident);
          return;
        },
        Expr::Member(inner) => object = inner.obj.as_ref(),
        Expr::Paren(paren) => object = paren.expr.as_ref(),
        Expr::OptChain(opt_chain) => match opt_chain.base.as_ref() {
          OptChainBase::Member(inner) => object = inner.obj.as_ref(),
          OptChainBase::Call(_) => return,
        },
        Expr::TsNonNull(non_null) => object = non_null.expr.as_ref(),
        Expr::TsAs(ts_as) => object = ts_as.expr.as_ref(),
        Expr::TsSatisfies(satisfies) => object = satisfies.expr.as_ref(),
        _ => return,
      }
    }
  }

  /// Record every binding written by an assignment or `for-in`/`for-of`
  /// pattern. Identifier targets rebind (`[a, b] = …`); `Pat::Expr` targets
  /// are member writes (`({ x: obj.x } = …)`) and invalidate the member's
  /// root object instead.
  fn add_pattern_writes(&mut self, pattern: &Pat) {
    match pattern {
      Pat::Ident(binding_ident) => self.add_binding_write(&binding_ident.id),
      Pat::Array(array_pattern) => self.add_array_pattern_writes(array_pattern),
      Pat::Object(object_pattern) => self.add_object_pattern_writes(object_pattern),
      Pat::Rest(rest_pattern) => self.add_pattern_writes(&rest_pattern.arg),
      Pat::Assign(assign_pattern) => self.add_pattern_writes(&assign_pattern.left),
      Pat::Expr(expression) => {
        if let Expr::Member(member_expression) = expression.as_ref() {
          self.add_member_root_write(member_expression);
        }
      },
      Pat::Invalid(_) => {},
    }
  }

  /// `[a, obj.x, ...rest] = …`, shared by `Pat` and `AssignTargetPat`.
  fn add_array_pattern_writes(&mut self, array_pattern: &ArrayPat) {
    for element in array_pattern.elems.iter().flatten() {
      self.add_pattern_writes(element);
    }
  }

  /// `{ a, b: obj.x, c = 1, ...rest } = …`, shared by `Pat` and
  /// `AssignTargetPat`.
  fn add_object_pattern_writes(&mut self, object_pattern: &ObjectPat) {
    for property in &object_pattern.props {
      match property {
        ObjectPatProp::KeyValue(key_value) => self.add_pattern_writes(&key_value.value),
        ObjectPatProp::Assign(assign) => self.add_binding_write(&assign.key.id),
        ObjectPatProp::Rest(rest) => self.add_pattern_writes(&rest.arg),
      }
    }
  }

  /// Record the write performed by an assignment target, unwrapping the
  /// parenthesised and TypeScript wrappers that can sit between the target
  /// and the identifier or member it resolves to.
  fn add_simple_target_write(&mut self, target: &SimpleAssignTarget) {
    match target {
      SimpleAssignTarget::Ident(ident) => self.add_binding_write(&ident.id),
      SimpleAssignTarget::Member(member_expression) => {
        self.add_member_root_write(member_expression)
      },
      SimpleAssignTarget::Paren(paren) => self.add_expr_target_write(&paren.expr),
      SimpleAssignTarget::OptChain(opt_chain) => {
        if let OptChainBase::Member(member_expression) = opt_chain.base.as_ref() {
          self.add_member_root_write(member_expression);
        }
      },
      SimpleAssignTarget::TsAs(ts_as) => self.add_expr_target_write(&ts_as.expr),
      SimpleAssignTarget::TsSatisfies(satisfies) => self.add_expr_target_write(&satisfies.expr),
      SimpleAssignTarget::TsNonNull(non_null) => self.add_expr_target_write(&non_null.expr),
      SimpleAssignTarget::TsTypeAssertion(assertion) => self.add_expr_target_write(&assertion.expr),
      SimpleAssignTarget::TsInstantiation(instantiation) => {
        self.add_expr_target_write(&instantiation.expr)
      },
      // `super.x = 1` and invalid targets bind no module-level name.
      SimpleAssignTarget::SuperProp(_) | SimpleAssignTarget::Invalid(_) => {},
    }
  }

  /// Record the write performed by an expression used as an assignment target
  /// (the payload of the TypeScript assignment-target wrappers).
  fn add_expr_target_write(&mut self, expression: &Expr) {
    match expression {
      Expr::Ident(ident) => self.add_binding_write(ident),
      Expr::Member(member_expression) => self.add_member_root_write(member_expression),
      _ => {},
    }
  }
}

impl Visit for ModuleBindingsCollector {
  fn visit_import_decl(&mut self, import_decl: &ImportDecl) {
    if !self.collect_sx_bindings {
      return;
    }

    self
      .import_sources
      .push(convert_atom_to_string(&import_decl.src.value));
    for specifier in &import_decl.specifiers {
      let local = match specifier {
        swc_core::ecma::ast::ImportSpecifier::Named(named) => &named.local,
        swc_core::ecma::ast::ImportSpecifier::Default(default) => &default.local,
        swc_core::ecma::ast::ImportSpecifier::Namespace(namespace) => &namespace.local,
      };
      // Import locals are bindings, but never count as a re-binding that
      // would shadow another import.
      self.bound_names.insert(local.sym.to_string());
    }
  }

  fn visit_function(&mut self, function: &Function) {
    self.scope_stack.push(ScopeFrame {
      span: function.span,
      kind: ScopeKind::Function,
    });
    function.visit_children_with(self);
    self.scope_stack.pop();
  }

  fn visit_arrow_expr(&mut self, arrow_expr: &ArrowExpr) {
    self.scope_stack.push(ScopeFrame {
      span: arrow_expr.span,
      kind: ScopeKind::Function,
    });
    arrow_expr.visit_children_with(self);
    self.scope_stack.pop();
  }

  fn visit_fn_expr(&mut self, fn_expr: &FnExpr) {
    self.scope_stack.push(ScopeFrame {
      span: fn_expr.function.span,
      kind: ScopeKind::Function,
    });

    if let Some(ident) = &fn_expr.ident {
      // A named function expression's name is bound only inside the function
      // body, where it can shadow an imported `stylex` namespace.
      self.add_local_binding(ident.sym.as_ref(), false);
    }

    fn_expr.function.visit_children_with(self);
    self.scope_stack.pop();
  }

  fn visit_class_expr(&mut self, class_expr: &ClassExpr) {
    self.scope_stack.push(ScopeFrame {
      span: class_expr.class.span,
      kind: ScopeKind::Block,
    });

    if let Some(ident) = &class_expr.ident {
      // A named class expression's name is visible inside the class body.
      self.add_local_binding(ident.sym.as_ref(), false);
    }

    class_expr.class.visit_children_with(self);
    self.scope_stack.pop();
  }

  fn visit_block_stmt(&mut self, block_stmt: &BlockStmt) {
    self.scope_stack.push(ScopeFrame {
      span: block_stmt.span,
      kind: ScopeKind::Block,
    });
    block_stmt.visit_children_with(self);
    self.scope_stack.pop();
  }

  fn visit_catch_clause(&mut self, catch_clause: &CatchClause) {
    self.scope_stack.push(ScopeFrame {
      span: catch_clause.span,
      kind: ScopeKind::Block,
    });
    catch_clause.visit_children_with(self);
    self.scope_stack.pop();
  }

  fn visit_var_decl(&mut self, var_decl: &VarDecl) {
    let previous = self.current_var_kind.replace(var_decl.kind);
    var_decl.visit_children_with(self);
    self.current_var_kind = previous;
  }

  fn visit_binding_ident(&mut self, binding_ident: &BindingIdent) {
    let hoisted = self.current_var_kind == Some(VarDeclKind::Var);
    self.add_local_binding(binding_ident.id.sym.as_ref(), hoisted);
    binding_ident.visit_children_with(self);
  }

  fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
    // Function declarations in modules are block-scoped; top-level ones still
    // land in the module scope because it is the innermost frame there.
    self.add_local_binding(fn_decl.ident.sym.as_ref(), false);
    fn_decl.visit_children_with(self);
  }

  fn visit_class_decl(&mut self, class_decl: &ClassDecl) {
    // Class declarations are block-scoped, not hoisted.
    self.add_local_binding(class_decl.ident.sym.as_ref(), false);
    class_decl.visit_children_with(self);
  }

  fn visit_assign_expr(&mut self, assign_expression: &AssignExpr) {
    match &assign_expression.left {
      AssignTarget::Simple(target) => self.add_simple_target_write(target),
      AssignTarget::Pat(pattern) => match pattern {
        AssignTargetPat::Array(array_pattern) => self.add_array_pattern_writes(array_pattern),
        AssignTargetPat::Object(object_pattern) => self.add_object_pattern_writes(object_pattern),
        AssignTargetPat::Invalid(_) => {},
      },
    }

    assign_expression.visit_children_with(self);
  }

  fn visit_update_expr(&mut self, update_expression: &UpdateExpr) {
    self.add_expr_target_write(update_expression.arg.as_ref());

    update_expression.visit_children_with(self);
  }

  fn visit_unary_expr(&mut self, unary_expression: &UnaryExpr) {
    if unary_expression.op == UnaryOp::Delete
      && let Expr::Member(member_expression) = unary_expression.arg.as_ref()
    {
      self.add_member_root_write(member_expression);
    }

    unary_expression.visit_children_with(self);
  }

  fn visit_call_expr(&mut self, call_expression: &CallExpr) {
    if let Callee::Expr(callee) = &call_expression.callee
      && let Expr::Member(member_expression) = callee.as_ref()
      && let MemberProp::Ident(property) = &member_expression.prop
    {
      // `arr.push(…)`, `arr.sort()`, … mutate the receiver in place. The
      // receiver may itself be a member chain (`state.items.push(…)`), in
      // which case the binding to invalidate is the chain's root object.
      if MUTATING_ARRAY_METHODS.contains(property.sym.as_ref()) {
        self.add_member_root_write(member_expression);
      }

      // `Object.assign(target, …)` and friends mutate their first argument.
      // Matching on the `Object` name is a deliberate over-approximation: a
      // shadowed `Object` only costs a deopt, never a wrong inline.
      if MUTATING_OBJECT_METHODS.contains(property.sym.as_ref())
        && member_expression
          .obj
          .as_ident()
          .is_some_and(|object| object.sym == "Object")
        && let Some(first_argument) = call_expression.args.first()
        && first_argument.spread.is_none()
      {
        self.add_expr_target_write(first_argument.expr.as_ref());
      }
    }

    call_expression.visit_children_with(self);
  }

  fn visit_for_in_stmt(&mut self, for_in_statement: &ForInStmt) {
    if let ForHead::Pat(pattern) = &for_in_statement.left {
      self.add_pattern_writes(pattern);
    }

    for_in_statement.visit_children_with(self);
  }

  fn visit_for_of_stmt(&mut self, for_of_statement: &ForOfStmt) {
    if let ForHead::Pat(pattern) = &for_of_statement.left {
      self.add_pattern_writes(pattern);
    }

    for_of_statement.visit_children_with(self);
  }
}

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn visit_mut_program_impl(&mut self, program: &mut Program) {
    match program {
      Program::Module(module) => self.visit_mut_module_impl(module),
      Program::Script(script) => {
        // A `Script` body holds plain statements, not `ModuleItem`s, so it
        // cannot carry the injected `import * as stylex` declaration that the
        // `sx` runtime binding may need. Promote it to a `Module`, run the
        // standard pipeline, then restore the `Script` form when no
        // module-level declaration was added — preserving non-module output
        // verbatim for inputs that need no injection.
        let mut module = Module {
          span: script.span,
          body: std::mem::take(&mut script.body)
            .into_iter()
            .map(ModuleItem::Stmt)
            .collect(),
          shebang: script.shebang.take(),
        };

        self.visit_mut_module_impl(&mut module);

        if module
          .body
          .iter()
          .any(|item| matches!(item, ModuleItem::ModuleDecl(_)))
        {
          *program = Program::Module(module);
        } else {
          // The `any(ModuleDecl)` check above is false here, so every item is
          // a `Stmt`; `ModuleItem::stmt` extracts each one.
          *script = Script {
            span: module.span,
            shebang: module.shebang,
            body: module
              .body
              .into_iter()
              .filter_map(ModuleItem::stmt)
              .collect(),
          };
        }
      },
    }
  }

  pub(crate) fn visit_mut_module_impl(&mut self, module: &mut Module) {
    if cfg!(debug_assertions) || !self.state.options.use_real_file_for_source {
      self.state.set_seen_module_source_code(module, None);
    }

    self.discover_module(module);

    if !self.state.has_import_paths() && self.state.atom_imports.is_empty() {
      return;
    }

    // Binding writes are only read by the evaluator, which runs from here on,
    // so modules that never reach this point pay no pre-scan. When the `sx`
    // feature is on, `discover_module` already scanned (its output is needed
    // mid-walk) and this is a no-op.
    self.collect_binding_writes(module);

    self.transform_producers(module);
    self.transform_atoms(module);
    self.transform_consumers(module);
    self.finalize_module(module);
  }

  /// Run the discovery pass.
  ///
  /// Walks the module once under the `Discover` cycle, populating import
  /// state, transforming compiled-JSX `sx` attributes, counting variable /
  /// member-expression references, and pre-filling top-level declarations —
  /// all the work the legacy `Initializing` + `StateFilling` two-pass split
  /// used to do separately. Whenever stylex was imported, also captures the
  /// top-level expressions consumed by later phases.
  pub(crate) fn discover_module(&mut self, module: &mut Module) {
    self.state.cycle = TransformationCycle::Discover;

    // The `sx` runtime-binding injection runs mid-walk in this same cycle and
    // consults the pre-scan, so with `sx` enabled the scan has to happen up
    // front. It collects binding writes in the same pass; without `sx` the
    // scan is deferred to `collect_binding_writes`, which only runs for
    // modules that actually reach evaluation.
    if self.state.options.sx_prop_name.is_some() {
      let mut collector = ModuleBindingsCollector::new(true);
      module.visit_with(&mut collector);

      self.state.binding_writes = collector.binding_writes;
      self.state.existing_import_sources = collector.import_sources;
      self.state.bound_names = collector.bound_names;
      self.state.local_rebinding_scopes = collector.local_rebinding_scopes;
    }

    module.visit_mut_children_with(self);

    if self.state.has_import_paths() {
      fill_top_level_expressions(module, &mut self.state);
    }
  }

  /// Record every binding the module rebinds or mutates, so the evaluator
  /// never inlines a declaration initializer that no longer holds at the use
  /// site. No-op when `discover_module` already collected them for `sx`.
  pub(crate) fn collect_binding_writes(&mut self, module: &Module) {
    if self.state.options.sx_prop_name.is_some() {
      return;
    }

    let mut collector = ModuleBindingsCollector::new(false);
    module.visit_with(&mut collector);

    self.state.binding_writes = collector.binding_writes;
  }

  /// Run the producer transformation pass.
  ///
  /// Transforms `stylex.create` / `defineVars` / `keyframes` / etc. — the calls
  /// that *produce* style namespaces consumed by later phases.
  pub(crate) fn transform_producers(&mut self, module: &mut Module) {
    self.state.cycle = TransformationCycle::TransformProducers;
    module.visit_mut_children_with(self);
  }

  /// Run the consumer transformation pass plus pending-insertion flush.
  ///
  /// Transforms `stylex.props` / `stylex.attrs` (which consume the style
  /// namespaces produced by the prior phase). After the consumer walk
  /// completes, drains the pending-insertion buffer with a single
  /// linear merge into the module body. Runtime helpers and per-decl
  /// metadata are gated on `options.runtime_injection.is_some()`,
  /// matching the legacy gate; hoisted dynamic-style consts always
  /// emit.
  pub(crate) fn transform_consumers(&mut self, module: &mut Module) {
    self.state.cycle = TransformationCycle::TransformConsumers;
    module.visit_mut_children_with(self);

    let runtime_injection = self.state.options.runtime_injection.is_some();
    flush_pending_insertions(&mut self.state, &mut module.body, runtime_injection);
  }

  /// Run the cleanup phase: materialize deferred JSX-spread
  /// replacements, build the decl-reference graph, compute the live
  /// set, then sweep unused declarations in a single forward pass.
  ///
  /// The mark step (`mark_style_vars_to_keep`) walks the module once
  /// and populates `state.style_vars_to_keep` plus materializes any
  /// deferred JSX-spread replacements. The graph is then captured at
  /// G-PostHoc against the post-mark AST, fixing the live set used by
  /// the sweep. The sweep itself runs under `TransformationCycle::Finalize`
  /// in original module-body order — the live-set is computed up
  /// front, so no body reversal is needed to handle transitive removal.
  pub(crate) fn finalize_module(&mut self, module: &mut Module) {
    // The mark phase materializes any deferred JSX-spread replacements
    // accumulated during the consumer walk; running it first ensures the
    // subsequent graph capture sees the final JSX shape (no leftover
    // `stylex.props(styles.X)` SpreadElement nodes).
    mark_style_vars_to_keep(module, &mut self.state);

    // Capture the decl-reference graph after producer + consumer
    // transforms — and the JSX-spread materialization above — have run.
    // At this point inlined references (e.g. `styles.container` in JSX,
    // replaced with literal class strings) are gone from the AST, so
    // the graph reflects only the references that survive into the
    // emitted module.
    build_decl_use_graph(module, &mut self.state);

    // Compute the live-set up front so the sweep filter has a single
    // immutable membership check per declarator. Computation depends on
    // `state.roots` and `state.decl_uses`, both finalized by the
    // mark-phase pass above.
    self.state.live_set = compute_live_set(&self.state);

    self.state.cycle = TransformationCycle::Finalize;

    module.visit_mut_children_with(self);
  }
}

#[cfg(test)]
#[path = "tests/module_bindings_collector_tests.rs"]
mod tests;
