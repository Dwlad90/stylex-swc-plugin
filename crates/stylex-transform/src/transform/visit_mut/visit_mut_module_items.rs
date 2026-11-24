use stylex_ast::ast::convertors::convert_atom_to_string;
use stylex_state::state_writers::fill_state_declarations;
use swc_core::{
  common::{DUMMY_SP, comments::Comments},
  ecma::{
    ast::{
      ArrowExpr, Class, ClassExpr, Decl, DefaultDecl, ExportDecl, ExportDefaultDecl, Function, Id,
      ImportDecl, ModuleDecl, ModuleItem, Pat, Stmt, Str, TsModuleName, VarDecl, VarDeclKind,
    },
    utils::find_pat_ids,
    visit::{Visit, VisitMutWith, VisitWith},
  },
};

use crate::StyleXTransform;
use stylex_enums::core::TransformationCycle;
use stylex_regex::regex::STYLEX_CONSTS_IMPORT_REGEX;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  /// Transforms each item, and puts the declarations hoisted out of an item in
  /// front of it.
  ///
  /// A hoisted declaration can read a program-level binding, and most bindings
  /// have no value before their statement runs (see [`bindings_declared_by`]).
  /// So the declaration goes where the reference puts it: in front of the
  /// statement that held the expression. The other queued items are merged once, after
  /// the consumer walk, by `flush_pending_insertions`.
  ///
  /// The front of a statement comes before the bindings that it and the later
  /// statements declare, so the walk tells the state which of them are not
  /// declared yet. An object that reads one of them is not hoisted.
  ///
  /// Only the consumer merge asks about these bindings, so the producer walk
  /// does not collect them.
  fn visit_mut_placing_hoists(&mut self, module_items: &mut Vec<ModuleItem>) {
    if self.state.places_statement_hoists() {
      module_items.visit_mut_children_with(self);
      return;
    }

    // The names of one item, read once. A transform changes what an item
    // computes, not the names it declares, so they hold after the visit too.
    let declared: Vec<Vec<Id>> = match self.state.cycle {
      TransformationCycle::TransformConsumers => {
        module_items.iter().map(bindings_declared_by).collect()
      },
      _ => Vec::new(),
    };

    self
      .state
      .start_statement_hoists(declared.iter().flatten().cloned().collect());

    let items = std::mem::take(module_items);
    let mut declared = declared.into_iter();
    module_items.reserve(items.len());

    for mut item in items {
      item.visit_mut_with(self);
      module_items.extend(self.state.take_statement_hoists());

      if let Some(ids) = declared.next() {
        self.state.note_program_bindings_declared(ids);
      }

      module_items.push(item);
    }

    self.state.stop_statement_hoists();
  }

  pub(crate) fn visit_mut_module_items_impl(&mut self, module_items: &mut Vec<ModuleItem>) {
    match self.state.cycle {
      TransformationCycle::Discover => {
        // Pre-fill `state.declarations` for top-level ident var decls so any
        // subsequent lookup during the descent (or in later phases) can find
        // them. The visit_mut_var_declarator hook will re-fill them
        // per-decl, but `fill_state_declarations` is idempotent.
        module_items.iter().for_each(|module_item| {
          if let ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) = module_item {
            var_decl.decls.iter().for_each(|decl| {
              if let Pat::Ident(_) = &decl.name {
                fill_state_declarations(&mut self.state, decl);
              }
            });
          }
        });

        // Single descent: discovers stylex imports, transforms compiled-JSX
        // sx attributes, and accumulates ident / member-expr counts.
        module_items.visit_mut_children_with(self);

        if !self.state.has_import_paths() {
          return;
        }

        if self.state.options.inject_stylex_side_effects {
          let side_effect_imports: Vec<_> = module_items
            .iter()
            .filter_map(|module_item| {
              if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = module_item {
                let source_path = convert_atom_to_string(&import_decl.src.value);
                STYLEX_CONSTS_IMPORT_REGEX
                  .is_match(&source_path)
                  .unwrap_or(false)
                  .then(|| {
                    ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
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
                    }))
                  })
              } else {
                None
              }
            })
            .collect();

          module_items.extend(side_effect_imports);
        }
      },
      TransformationCycle::TransformProducers | TransformationCycle::TransformConsumers => {
        self.visit_mut_placing_hoists(module_items);
      },
      TransformationCycle::Finalize => {
        // We need it twice for a clear dead code after declaration transforms
        module_items.visit_mut_children_with(self);

        // We remove `Stmt::Empty` from the statement list.
        // This is optional, but it's required if you don't want extra `;` in output.
        module_items.retain(|module_item| {
          !matches!(module_item, ModuleItem::Stmt(Stmt::Empty(..)))
            && !matches!(module_item, ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl { decl: Decl::Var(var), .. })) if var.decls.is_empty())
        });
      },
    }
  }
}

/// The program-level bindings that `item` declares and that have no value
/// before it runs.
///
/// That is a `let`, a `const`, a class, a `using`, and each TypeScript
/// declaration that compiles to one of them. It is also every `var`, at any
/// depth outside a function, because a `var` in a block or a loop head is a
/// program-level binding too.
///
/// A function declaration and an import hold their value before the first
/// statement runs, so a hoisted declaration can read them anywhere. A
/// `declare` form is ambient: it names a value from outside the module, so it
/// is not here either.
fn bindings_declared_by(item: &ModuleItem) -> Vec<Id> {
  let mut vars = VarBindings::default();
  item.visit_with(&mut vars);

  let mut ids = vars.ids;
  ids.extend(lexical_bindings_declared_by(item));
  ids
}

/// The bindings that `item` declares at program level, other than a `var`.
fn lexical_bindings_declared_by(item: &ModuleItem) -> Vec<Id> {
  let decl = match item {
    ModuleItem::Stmt(Stmt::Decl(decl))
    | ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl { decl, .. })) => decl,
    ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(ExportDefaultDecl {
      decl: DefaultDecl::Class(ClassExpr {
        ident: Some(ident), ..
      }),
      ..
    })) => return vec![ident.to_id()],
    _ => return Vec::new(),
  };

  match decl {
    Decl::Class(class) if !class.declare => vec![class.ident.to_id()],
    Decl::Var(var) if var.kind != VarDeclKind::Var && !var.declare => find_pat_ids(&var.decls),
    Decl::Using(using) => find_pat_ids(&using.decls),
    Decl::TsEnum(ts_enum) if !ts_enum.declare => vec![ts_enum.id.to_id()],
    Decl::TsModule(ts_module) => match &ts_module.id {
      TsModuleName::Ident(ident) if !ts_module.declare => vec![ident.to_id()],
      _ => Vec::new(),
    },
    _ => Vec::new(),
  }
}

/// Collects the names of every `var` that a statement declares. A function,
/// an arrow and a class hold their own scope, so the search does not go into
/// them.
#[derive(Default)]
struct VarBindings {
  ids: Vec<Id>,
}

impl Visit for VarBindings {
  fn visit_var_decl(&mut self, var: &VarDecl) {
    if var.kind == VarDeclKind::Var && !var.declare {
      self.ids.extend(find_pat_ids::<_, Id>(&var.decls));
    }
  }

  fn visit_function(&mut self, _: &Function) {}

  fn visit_arrow_expr(&mut self, _: &ArrowExpr) {}

  fn visit_class(&mut self, _: &Class) {}
}
