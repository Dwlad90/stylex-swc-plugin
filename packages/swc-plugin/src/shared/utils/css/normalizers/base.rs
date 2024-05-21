use swc_core::{
  common::DUMMY_SP,
  css::{
    ast::{
      ComponentValue, Declaration, DeclarationName, Dimension, Function, Ident,
      Length, ListOfComponentValues, Number, Stylesheet,
    },
    visit::{Fold, FoldWith},
  },
};

use crate::shared::{constants::common::ROOT_FONT_SIZE, utils::common::dashify};

struct CssFolder {
  use_rem_for_font_size: bool,
  parent_key: Option<String>,
}

impl CssFolder {
  fn convert_font_size_to_rem_normalizer<'a>(
    &'a mut self,
    declaration: &'a mut Declaration,
  ) -> Declaration {
    if let DeclarationName::Ident(ident) = &declaration.name {
      if ident.value.eq("fontSize") || self.parent_key.as_deref() == Some("fontSize") {
        self.parent_key = Some("fontSize".into());
        declaration.value = declaration.value.clone().fold_children_with(self);
        self.parent_key = None;
      }
    }

    declaration.clone()
  }
}

impl Fold for CssFolder {
  fn fold_list_of_component_values(
    &mut self,
    list: ListOfComponentValues,
  ) -> ListOfComponentValues {
    list.fold_children_with(self)
  }

  fn fold_declaration(&mut self, mut declaration: Declaration) -> Declaration {
    let mut declaration = kebab_case_normalizer(&mut declaration).clone();

    if self.use_rem_for_font_size {
      declaration = self.convert_font_size_to_rem_normalizer(&mut declaration);
    }

    declaration.fold_children_with(self)
  }

  fn fold_dimension(&mut self, mut dimension: Dimension) -> Dimension {
    let dimension = timing_normalizer(&mut dimension);

    dimension.clone().fold_children_with(self)
  }

  fn fold_length(&mut self, mut length: Length) -> Length {
    if self.parent_key == Option::Some("fontSize".into())
      && length.unit.value.eq("px")
      && length.value.value != 0.0
    {
      length = Length {
        value: Number {
          value: length.value.value / ROOT_FONT_SIZE as f64,
          raw: Option::None,
          span: length.span,
        },
        unit: Ident {
          value: "rem".into(),
          raw: Option::None,
          span: length.span,
        },
        span: DUMMY_SP,
      };
    };

    length
  }

  fn fold_function(&mut self, func: Function) -> Function {
    let mut fnc = func;

    // NOTE: only last css fucntion value should be folded
    if let Some(last) = fnc.value.last_mut() {
      *last = last.clone().fold_with(self);
    }

    fnc
  }
}

fn timing_normalizer(dimension: &mut Dimension) -> &mut Dimension {
  match dimension {
    Dimension::Time(time) => {
      if !time.unit.eq("ms") || time.value.value < 10.0 {
        return dimension;
      }

      time.value = Number {
        value: time.value.value / 1000.0,
        raw: Option::None,
        span: DUMMY_SP,
      };

      time.unit = Ident {
        span: DUMMY_SP,
        value: "s".into(),
        raw: Option::None,
      };

      dimension
    }
    _ => dimension,
  }
}

fn kebab_case_normalizer(declaration: &mut Declaration) -> &mut Declaration {
  match &declaration.name {
    DeclarationName::Ident(ident) => {
      if !ident.value.eq("transitionProperty") && !ident.value.eq("willChange") {
        return declaration;
      }
    }
    DeclarationName::DashedIdent(_) => return declaration,
  }

  declaration.value = declaration
    .value
    .clone()
    .into_iter()
    .map(|value| match value {
      ComponentValue::Ident(ident) => {
        let ident = Ident {
          value: dashify(ident.value.as_str()).into(),
          raw: Option::None,
          span: ident.span,
        };

        ComponentValue::Ident(Box::new(ident))
      }
      _ => value,
    })
    .collect();

  declaration
}

pub(crate) fn base_normalizer(ast: Stylesheet, use_rem_for_font_size: bool) -> Stylesheet {
  let mut folder = CssFolder {
    use_rem_for_font_size,
    parent_key: Option::None,
  };
  ast.fold_with(&mut folder)
}


