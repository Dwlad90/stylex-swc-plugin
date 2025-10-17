use swc_core::{
  common::DUMMY_SP,
  css::{
    ast::{
      ComponentValue, Declaration, DeclarationName, Dimension, Function, Ident, Length,
      ListOfComponentValues, Number, Stylesheet,
    },
    visit::{Fold, FoldWith},
  },
};

use crate::shared::{constants::common::ROOT_FONT_SIZE, utils::common::dashify};

struct CssFolder {
  enable_font_size_px_to_rem: bool,
  parent_key: Option<String>,
  is_function_arg: bool,
}

impl CssFolder {
  fn convert_font_size_to_rem_normalizer<'a>(
    &'a mut self,
    declaration: &'a mut Declaration,
  ) -> &'a mut Declaration {
    if let DeclarationName::Ident(ident) = &declaration.name
      && (ident.value == "fontSize" || self.parent_key.as_deref() == Some("fontSize"))
    {
      self.parent_key = Some("fontSize".into());
      declaration.value = declaration.value.clone().fold_children_with(self);
      self.parent_key = None;
    }

    declaration
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
    let declaration = kebab_case_normalizer(&mut declaration);

    if self.enable_font_size_px_to_rem {
      self.convert_font_size_to_rem_normalizer(declaration);
    }

    declaration.clone().fold_children_with(self)
  }

  fn fold_dimension(&mut self, mut dimension: Dimension) -> Dimension {
    let dimension = timing_normalizer(&mut dimension);
    let dimension = zero_dimension_normalizer(dimension, self.is_function_arg);

    dimension.clone().fold_children_with(self)
  }

  fn fold_length(&mut self, mut length: Length) -> Length {
    if self.parent_key == Some("fontSize".into())
      && length.unit.value.eq("px")
      && length.value.value != 0.0
    {
      length = Length {
        value: Number {
          value: length.value.value / ROOT_FONT_SIZE as f64,
          raw: None,
          span: length.span,
        },
        unit: Ident {
          value: "rem".into(),
          raw: None,
          span: length.span,
        },
        span: DUMMY_SP,
      };
    };

    length
  }

  fn fold_function(&mut self, func: Function) -> Function {
    self.is_function_arg = true;

    let mut fnc = func;

    // NOTE: only last css fucntion value should be folded
    if let Some(last) = fnc.value.last_mut() {
      *last = last.clone().fold_children_with(self);
    }

    self.is_function_arg = false;

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
        raw: None,
        span: DUMMY_SP,
      };

      time.unit = Ident {
        span: DUMMY_SP,
        value: "s".into(),
        raw: None,
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

  declaration.value.iter_mut().for_each(|value| {
    if let ComponentValue::Ident(ident) = value
      && !ident.value.starts_with("--")
    {
      ident.value = dashify(ident.value.as_str()).into();
    }
  });

  declaration
}

pub(crate) fn base_normalizer(ast: Stylesheet, enable_font_size_px_to_rem: bool) -> Stylesheet {
  let mut folder = CssFolder {
    enable_font_size_px_to_rem,
    parent_key: None,
    is_function_arg: false,
  };
  ast.fold_with(&mut folder)
}

fn zero_dimension_normalizer(dimension: &mut Dimension, is_function_arg: bool) -> &mut Dimension {
  if is_function_arg {
    return dimension;
  }

  match dimension {
    Dimension::Length(length) => {
      if length.value.value != 0.0 {
        return dimension;
      }

      length.value = get_zero_demansion_value();
      length.unit = get_zero_demansion_unit(&length.unit);

      dimension
    }
    Dimension::Angle(angle) => {
      if angle.value.value != 0.0 {
        return dimension;
      }

      angle.value = get_zero_demansion_value();

      angle.unit = Ident {
        span: DUMMY_SP,
        value: "deg".into(),
        raw: None,
      };

      dimension
    }
    Dimension::Time(time) => {
      if time.value.value != 0.0 {
        return dimension;
      }

      time.value = get_zero_demansion_value();

      time.unit = Ident {
        span: DUMMY_SP,
        value: "s".into(),
        raw: None,
      };

      dimension
    }
    Dimension::Frequency(frequency) => {
      if frequency.value.value != 0.0 {
        return dimension;
      }

      frequency.value = get_zero_demansion_value();
      frequency.unit = get_zero_demansion_unit(&frequency.unit);

      dimension
    }
    Dimension::Resolution(resolution) => {
      if resolution.value.value != 0.0 {
        return dimension;
      }

      resolution.value = get_zero_demansion_value();
      resolution.unit = get_zero_demansion_unit(&resolution.unit);

      dimension
    }
    Dimension::Flex(flex) => {
      if flex.value.value != 0.0 {
        return dimension;
      }

      flex.value = get_zero_demansion_value();
      flex.unit = get_zero_demansion_unit(&flex.unit);

      dimension
    }
    Dimension::UnknownDimension(unknown) => {
      if unknown.value.value != 0.0 {
        return dimension;
      }

      unknown.value = get_zero_demansion_value();
      unknown.unit = get_zero_demansion_unit(&unknown.unit);

      dimension
    }
  }
}

fn get_zero_demansion_value() -> Number {
  Number {
    value: 0.0,
    raw: None,
    span: DUMMY_SP,
  }
}

fn get_zero_demansion_unit(unit: &Ident) -> Ident {
  if unit.value.eq("fr") {
    return Ident {
      value: "fr".into(),
      raw: None,
      span: DUMMY_SP,
    };
  }

  if unit.value.eq("%") {
    return Ident {
      value: "%".into(),
      raw: None,
      span: DUMMY_SP,
    };
  }

  Ident {
    value: "".into(),
    raw: None,
    span: DUMMY_SP,
  }
}
