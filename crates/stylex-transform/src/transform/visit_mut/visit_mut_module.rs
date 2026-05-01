use swc_core::{
  common::comments::Comments,
  ecma::{ast::Module, visit::VisitMutWith},
};

use crate::{
  StyleXTransform,
  shared::utils::common::fill_top_level_expressions,
  transform::visit_mut::visit_mut_module_items::inject_runtime_styles,
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

  /// Run the producer transformation pass (`TransformEnter`).
  ///
  /// Transforms `stylex.create` / `defineVars` / `keyframes` / etc. — the calls
  /// that *produce* style namespaces consumed by later phases.
  pub(crate) fn transform_producers(&mut self, module: &mut Module) {
    self.state.cycle = TransformationCycle::TransformEnter;
    module.visit_mut_children_with(self);
  }

  /// Run the consumer transformation pass plus runtime style injection.
  ///
  /// Transforms `stylex.props` / `stylex.attrs` (which consume the style
  /// namespaces produced by the prior phase) under `TransformExit`, with the
  /// `evaluate_preserve_bindings` flag held for the duration of the walk so
  /// the evaluator does not decrement binding counts on call arguments. Then,
  /// if runtime injection is enabled, runs the `InjectStyles` pass.
  pub(crate) fn transform_consumers(&mut self, module: &mut Module) {
    self.state.cycle = TransformationCycle::TransformExit;
    self.state.evaluate_preserve_bindings = true;
    module.visit_mut_children_with(self);
    self.state.evaluate_preserve_bindings = false;

    if self.state.options.runtime_injection.is_some() {
      inject_runtime_styles(&self.state, &mut module.body);
    }
  }

  /// Run the cleanup phases (`PreCleaning` + `Cleaning`).
  ///
  /// First marks which style namespaces / object properties survive (member
  /// accesses left after consumer transformation). Then runs the sweep with
  /// the module body reversed so removing later declarations does not
  /// invalidate the counts of earlier ones; the body is restored to original
  /// order afterwards.
  pub(crate) fn finalize_module(&mut self, module: &mut Module) {
    self.state.cycle = TransformationCycle::PreCleaning;
    module.visit_mut_children_with(self);

    self.state.cycle = TransformationCycle::Cleaning;

    // NOTE: Reversing the module body to clean the module items in the correct
    // order, so removing unused variable declarations will more efficient
    // After cleaning the module items, the module body will be reversed back to
    // its original order
    module.body.reverse();

    module.visit_mut_children_with(self);

    module.body.reverse();
  }
}
