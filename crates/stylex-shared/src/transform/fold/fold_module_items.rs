use swc_core::ecma::ast::{ExportDecl, ImportDecl, Str};
use swc_core::{
  common::{DUMMY_SP, comments::Comments},
  ecma::{
    ast::{Decl, Expr, Ident, Lit, ModuleDecl, ModuleItem, Pat, Stmt, VarDeclarator},
    visit::FoldWith,
  },
};

use crate::{
  StyleXTransform,
  shared::{
    enums::core::TransformationCycle,
    regex::STYLEX_CONSTS_IMPORT_REGEX,
    utils::{
      ast::{convertors::atom_to_string, factories::binding_ident_factory},
      common::{fill_state_declarations, stable_hash},
    },
  },
};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn fold_module_items(&mut self, module_items: Vec<ModuleItem>) -> Vec<ModuleItem> {
    match self.state.cycle {
      TransformationCycle::Skip => module_items,
      TransformationCycle::Initializing => {
        let mut transformed_module_items = module_items.fold_children_with(self);

        if self.state.import_paths.is_empty() {
          self.state.cycle = TransformationCycle::Skip;

          return transformed_module_items;
        }

        if self.state.options.inject_stylex_side_effects {
          let mut side_effect_imports = Vec::new();

          for module_item in &transformed_module_items {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = module_item {
              let source_path = atom_to_string(&import_decl.src.value);
              if STYLEX_CONSTS_IMPORT_REGEX
                .is_match(&source_path)
                .unwrap_or(false)
              {
                side_effect_imports.push(ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                  span: DUMMY_SP,
                  specifiers: vec![],
                  src: Box::new(Str {
                    span: DUMMY_SP,
                    value: import_decl.src.value.clone(),
                    raw: None,
                  }),
                  type_only: false,
                  with: None,
                  phase: Default::default(),
                })));
              }
            }
          }

          transformed_module_items.extend(side_effect_imports);
        }

        transformed_module_items
      }
      TransformationCycle::StateFilling => {
        module_items.iter().for_each(|module_item| {
          if let ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) = module_item {
            var_decl.decls.iter().for_each(|decl| {
              if let Pat::Ident(_) = &decl.name {
                fill_state_declarations(&mut self.state, decl);
              }
            });
          }
        });

        module_items.fold_children_with(self)
      }
      TransformationCycle::TransformEnter
      | TransformationCycle::PreCleaning
      | TransformationCycle::Recounting => module_items.fold_children_with(self),
      TransformationCycle::TransformExit => {
        if self.state.hoisted_module_items.is_empty() {
          return module_items.fold_children_with(self);
        }

        let import_end = module_items
          .iter()
          .enumerate()
          .skip(1)
          .find(|(_, item)| !matches!(item, ModuleItem::ModuleDecl(ModuleDecl::Import(_))))
          .map(|(idx, _)| idx)
          .unwrap_or(0);

        let mut module_items = module_items;

        let total_capacity = module_items.len() + self.state.hoisted_module_items.len();

        let mut result_module_items = Vec::with_capacity(total_capacity);

        result_module_items.extend(module_items.drain(..import_end));

        result_module_items.extend(self.state.hoisted_module_items.iter().cloned());

        result_module_items.extend(module_items);

        result_module_items.fold_children_with(self)
      }
      TransformationCycle::InjectStyles => {
        let mut result_module_items: Vec<ModuleItem> =
          self.state.prepend_include_module_items.clone();

        result_module_items.extend(self.state.prepend_import_module_items.clone());

        let mut items_to_skip: usize = 0;

        if let Some(first) = module_items
          .first()
          .and_then(|first| first.as_stmt())
          .and_then(|stmp| stmp.as_expr())
          && let Some(Lit::Str(_)) = first.expr.as_lit()
        {
          result_module_items.insert(0, module_items.first().unwrap().clone());
          items_to_skip = 1;
        }

        for module_item in module_items.iter().skip(items_to_skip) {
          if let Some(decls) = match &module_item {
            ModuleItem::ModuleDecl(decl) => match decl {
              ModuleDecl::ExportDecl(export_decl) => export_decl.decl.as_var().map(|var_decl| {
                var_decl
                  .decls
                  .iter() // Use iter() to avoid cloning the entire collection
                  .filter(|decl| {
                    decl
                      .init
                      .as_ref()
                      .is_some_and(|init| init.is_object() || init.is_lit())
                  })
                  .cloned() // Clone only the filtered elements
                  .collect::<Vec<VarDeclarator>>()
              }),
              ModuleDecl::ExportDefaultExpr(export_default_expr) => {
                export_default_expr.expr.as_object().map(|obj| {
                  vec![VarDeclarator {
                    definite: true,
                    span: DUMMY_SP,
                    name: Pat::Ident(binding_ident_factory(Ident::from("default"))),
                    init: Some(Box::new(Expr::from(obj.clone()))),
                  }]
                })
              }
              _ => None,
            },
            ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) => Some(
              var_decl
                .decls
                .iter()
                .filter(|decl| {
                  decl
                    .init
                    .as_ref()
                    .is_some_and(|init| init.is_object() || init.is_lit())
                })
                .cloned()
                .collect::<Vec<VarDeclarator>>(),
            ),
            _ => None,
          } {
            for decl in decls {
              let key = decl.init.clone().unwrap();

              let key_hash = stable_hash(key.as_ref());

              if let Some(metadata_items) = self.state.styles_to_inject.get(&key_hash) {
                for module_item in metadata_items.iter() {
                  result_module_items.push(module_item.clone());
                }
              }
            }
          }
          result_module_items.push(module_item.clone());
        }

        result_module_items
      }
      TransformationCycle::Cleaning => {
        // We need it twice for a clear dead code after declaration transforms
        let mut module_items = module_items.fold_children_with(self);

        // We remove `Stmt::Empty` from the statement list.
        // This is optional, but it's required if you don't want extra `;` in output.
        module_items.retain(|module_item| {
          !matches!(module_item, ModuleItem::Stmt(Stmt::Empty(..)))
            && !matches!(module_item, ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl { decl: Decl::Var(var), .. })) if var.decls.is_empty())
        });

        module_items
      }
    }
  }
}
