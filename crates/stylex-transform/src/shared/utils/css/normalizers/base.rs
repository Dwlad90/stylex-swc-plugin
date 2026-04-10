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

use crate::shared::utils::common::dashify;
use stylex_constants::constants::common::ROOT_FONT_SIZE;

struct CssFolder {
  enable_font_size_px_to_rem: bool,
  parent_key: Option<&'static str>,
  is_function_arg: bool,
  current_property: Option<String>,
}

impl CssFolder {
  fn convert_font_size_to_rem_normalizer(&mut self, declaration: &mut Declaration) {
    if let DeclarationName::Ident(ident) = &declaration.name
      && (ident.value == "fontSize" || self.parent_key == Some("fontSize"))
    {
      self.parent_key = Some("fontSize");
      declaration.value = declaration.value.clone().fold_children_with(self);
      self.parent_key = None;
    }
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
    kebab_case_normalizer(&mut declaration);

    if self.enable_font_size_px_to_rem {
      self.convert_font_size_to_rem_normalizer(&mut declaration);
    }

    declaration.fold_children_with(self)
  }

  fn fold_dimension(&mut self, mut dimension: Dimension) -> Dimension {
    timing_normalizer(&mut dimension);
    zero_dimension_normalizer(
      &mut dimension,
      self.is_function_arg,
      self.current_property.as_deref(),
    );

    dimension.fold_children_with(self)
  }

  fn fold_length(&mut self, mut length: Length) -> Length {
    if self.parent_key == Some("fontSize")
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

    // NOTE: only last css function value should be folded
    if let Some(last) = fnc.value.last_mut() {
      *last = last.clone().fold_children_with(self);
    }

    self.is_function_arg = false;

    fnc
  }
}

fn timing_normalizer(dimension: &mut Dimension) {
  if let Dimension::Time(time) = dimension {
    if !time.unit.eq("ms") || time.value.value < 10.0 {
      return;
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
  }
}

fn kebab_case_normalizer(declaration: &mut Declaration) {
  let should_normalize = match &declaration.name {
    DeclarationName::Ident(ident) => {
      ident.value == "transitionProperty" || ident.value == "willChange"
    },
    DeclarationName::DashedIdent(_) => false,
  };

  if !should_normalize {
    return;
  }

  for value in &mut declaration.value {
    if let ComponentValue::Ident(ident) = value
      && !ident.value.starts_with("--")
    {
      ident.value = dashify(ident.value.as_str()).into();
    }
  }
}

pub(crate) fn base_normalizer(
  ast: Stylesheet,
  enable_font_size_px_to_rem: bool,
  current_property: Option<&str>,
) -> Stylesheet {
  let mut folder = CssFolder {
    enable_font_size_px_to_rem,
    parent_key: None,
    is_function_arg: false,
    current_property: current_property.map(|p| p.to_string()),
  };
  ast.fold_with(&mut folder)
}

/// Shared zero value for all dimension types.
fn zero_value() -> Number {
  Number {
    value: 0.0,
    raw: None,
    span: DUMMY_SP,
  }
}

/// Returns the appropriate unit for a zero-value dimension.
/// Most units are stripped (empty string); `fr` and `%` are preserved.
fn zero_unit(unit: &Ident) -> Ident {
  let value = match unit.value.as_ref() {
    "fr" => "fr",
    "%" => "%",
    _ => "",
  };

  Ident {
    value: value.into(),
    raw: None,
    span: DUMMY_SP,
  }
}

fn zero_dimension_normalizer(
  dimension: &mut Dimension,
  is_function_arg: bool,
  current_property: Option<&str>,
) {
  // Skip normalization for CSS custom properties
  if let Some(prop) = current_property
    && prop.starts_with("--")
  {
    return;
  }

  if is_function_arg {
    return;
  }

  // Macro to eliminate repetition across dimension variants.
  // Each variant has a `.value` (Number) and `.unit` (Ident) field.
  macro_rules! normalize_zero {
    ($dim:expr) => {{
      if $dim.value.value != 0.0 {
        return;
      }
      $dim.value = zero_value();
      $dim.unit = zero_unit(&$dim.unit);
    }};
  }

  match dimension {
    Dimension::Length(d) => normalize_zero!(d),
    Dimension::Frequency(d) => normalize_zero!(d),
    Dimension::Resolution(d) => normalize_zero!(d),
    Dimension::Flex(d) => normalize_zero!(d),
    Dimension::UnknownDimension(d) => normalize_zero!(d),
    Dimension::Angle(angle) => {
      if angle.value.value != 0.0 {
        return;
      }
      angle.value = zero_value();
      angle.unit = Ident {
        span: DUMMY_SP,
        value: "deg".into(),
        raw: None,
      };
    },
    Dimension::Time(time) => {
      if time.value.value != 0.0 {
        return;
      }
      time.value = zero_value();
      time.unit = Ident {
        span: DUMMY_SP,
        value: "s".into(),
        raw: None,
      };
    },
  }
}
