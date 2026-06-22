use swc_core::{
  common::{BytePos, Span, comments::Comments},
  ecma::{
    ast::{
      ArrowExpr, BindingIdent, BlockStmt, CatchClause, ClassDecl, ClassExpr, FnDecl, FnExpr,
      Function, ImportDecl, Module, ModuleItem, Program, Script, VarDecl, VarDeclKind,
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
/// of the scope each one occupies. SWC visitors have no parent pointers or
/// scope chain, so this pre-scan supplies the import-source and binding
/// information `get_stylex_runtime_binding` needs; the recorded scope spans
/// let `is_locally_rebound_at` perform its position-aware shadow check.
struct ModuleBindingsCollector {
  import_sources: Vec<String>,
  bound_names: FxHashSet<String>,
  /// For each name bound by a non-import declaration, the spans of the scopes
  /// in which it is bound. A name shadows an `sx` site iff one of its scope
  /// spans encloses that site (see [`StateManager::is_locally_rebound_at`]).
  local_rebinding_scopes: FxHashMap<String, Vec<Span>>,
  /// Stack of enclosing lexical scopes, outermost (module) first.
  scope_stack: Vec<ScopeFrame>,
  /// `VarDeclKind` of the `VarDecl` currently being visited, if any — needed
  /// to route `var` bindings to the nearest function scope (hoisting) while
  /// `let`/`const` stay in the innermost scope.
  current_var_kind: Option<VarDeclKind>,
}

impl ModuleBindingsCollector {
  fn new() -> Self {
    Self {
      import_sources: Vec::new(),
      bound_names: FxHashSet::default(),
      local_rebinding_scopes: FxHashMap::default(),
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
  fn add_local_binding(&mut self, name: String, hoisted: bool) {
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
}

impl Visit for ModuleBindingsCollector {
  fn visit_import_decl(&mut self, import_decl: &ImportDecl) {
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
      self.add_local_binding(ident.sym.to_string(), false);
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
      self.add_local_binding(ident.sym.to_string(), false);
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
    self.add_local_binding(binding_ident.id.sym.to_string(), hoisted);
    binding_ident.visit_children_with(self);
  }

  fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
    // Function declarations in modules are block-scoped; top-level ones still
    // land in the module scope because it is the innermost frame there.
    self.add_local_binding(fn_decl.ident.sym.to_string(), false);
    fn_decl.visit_children_with(self);
  }

  fn visit_class_decl(&mut self, class_decl: &ClassDecl) {
    // Class declarations are block-scoped, not hoisted.
    self.add_local_binding(class_decl.ident.sym.to_string(), false);
    class_decl.visit_children_with(self);
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

    if !self.state.has_import_paths() {
      return;
    }

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

    // Pre-scan the whole module once so the `sx` runtime-binding injection
    // (which runs mid-walk, in this same cycle) can consult existing import
    // sources and bound names without parent-pointer / scope APIs. The scan's
    // output is consumed solely by `get_stylex_runtime_binding` on the `sx`
    // path, so skip the extra full-module traversal entirely when the `sx`
    // feature is disabled.
    if self.state.options.sx_prop_name.is_some() {
      let mut collector = ModuleBindingsCollector::new();
      module.visit_with(&mut collector);
      self.state.existing_import_sources = collector.import_sources;
      self.state.bound_names = collector.bound_names;
      self.state.local_rebinding_scopes = collector.local_rebinding_scopes;
    }

    module.visit_mut_children_with(self);

    if self.state.has_import_paths() {
      fill_top_level_expressions(module, &mut self.state);
    }
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
