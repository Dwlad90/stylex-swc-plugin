use swc_core::{
  common::comments::Comments,
  ecma::{ast::Module, visit::VisitMutWith},
};

use crate::{
  StyleXTransform,
  shared::{
    structures::state_manager::{
      build_decl_use_graph, compute_live_set, flush_pending_insertions, mark_style_vars_to_keep,
    },
    utils::common::fill_top_level_expressions,
  },
};
use stylex_enums::core::TransformationCycle;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
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
