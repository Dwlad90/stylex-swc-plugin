use std::borrow::Cow;
use std::rc::Rc;

use indexmap::IndexMap;
use stylex_ast::ast::convertors::{
  convert_key_value_to_str, convert_lit_to_string, convert_str_lit_to_string, is_js_undefined,
  normalize_expr,
};
use stylex_macros::{stylex_panic, stylex_unimplemented};
use swc_core::ecma::ast::{Expr, Lit, MemberProp, ObjectLit};

use stylex_evaluator::evaluate::evaluate;
use stylex_state::{
  evaluate_result_value::EvaluateResultValue,
  flat_compiled_styles_value::FlatCompiledStylesValue,
  functions::FunctionMap,
  state_manager::StateManager,
  types::{FlatCompiledStyles, StylesObjectMap},
};

/// What one argument of a `stylex.props`-family call was read as.
///
/// Exactly the three answers the reader gives: the compiled style it names, the
/// absence the author wrote, and anything the compiler cannot read, which the
/// runtime is left to apply.
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum StyleObject {
  Style(FlatCompiledStyles),
  Nullable,
  Other,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ResolvedArg {
  StyleObject(StyleObject),
  ConditionalStyle(Expr, Option<StyleObject>, Option<StyleObject>),
}

impl ResolvedArg {
  /// Creates a `ResolvedArg::StyleObject`.
  ///
  /// # Arguments
  /// * `style_obj` - The resolved style object
  ///
  /// # Example
  /// ```ignore
  /// let arg = ResolvedArg::style_object(StyleObject::Style(...));
  /// ```
  #[inline]
  pub(crate) fn style_object(style_obj: StyleObject) -> Self {
    ResolvedArg::StyleObject(style_obj)
  }

  /// Creates a `ResolvedArg::ConditionalStyle`.
  ///
  /// # Arguments
  /// * `test` - The test expression
  /// * `primary` - Primary style object
  /// * `fallback` - Fallback style object
  ///
  /// # Example
  /// ```ignore
  /// let arg = ResolvedArg::conditional(test, Some(primary), Some(fallback));
  /// ```
  #[inline]
  pub(crate) fn conditional(
    test: Expr,
    primary: Option<StyleObject>,
    fallback: Option<StyleObject>,
  ) -> Self {
    ResolvedArg::ConditionalStyle(test, primary, fallback)
  }
}

pub(crate) fn parse_nullable_style(
  path: &Expr,
  state: &mut StateManager,
  evaluate_path_fn_config: &FunctionMap,
) -> StyleObject {
  // A parenthesis is not a different argument, so the style shape is read
  // through it. Read bare, `stylex.props((styles.root))` reached no arm below
  // and the whole merge was handed back to the runtime.
  let path = normalize_expr(path);

  // Call expressions (dynamic atom `_temp.color(c)`, dynamic create style
  // `styles.opacity(1)`, etc.) always bail out to runtime so the props merge
  // keeps the conditional class / inline-var semantics intact.
  if path.is_call() {
    return StyleObject::Other;
  }

  let result = match path {
    Expr::Lit(lit) => {
      if let Lit::Null(_) = lit {
        StyleObject::Nullable
      } else {
        StyleObject::Other
      }
    },
    Expr::Ident(ident) => {
      if is_js_undefined(ident) {
        StyleObject::Nullable
      } else {
        StyleObject::Other
      }
    },
    Expr::Member(member) => {
      // The namespaces come back with the name, rather than being looked up
      // again once the name is admitted: the reader that admits the name reads
      // the same map, so a second look-up asked a question already answered.
      // Both names are read against maps that take a string slice, so only the
      // computed key needs an owned string of its own.
      let mut namespaces: Option<&StylesObjectMap> = None;
      let mut obj_name: Option<&str> = None;
      let mut prop_name: Option<Cow<'_, str>> = None;

      if let Some(obj_ident) = normalize_expr(&member.obj).as_ident()
        && let Some(style_var_namespaces) = state.style_var_namespaces(obj_ident)
      {
        match &member.prop {
          MemberProp::Ident(prop_ident) => {
            namespaces = Some(style_var_namespaces);
            obj_name = Some(obj_ident.sym.as_str());
            prop_name = Some(Cow::Borrowed(prop_ident.sym.as_str()));
          },
          MemberProp::Computed(computed) => {
            if let Some(lit) = normalize_expr(&computed.expr).as_lit() {
              namespaces = Some(style_var_namespaces);
              obj_name = Some(obj_ident.sym.as_str());
              prop_name = convert_lit_to_string(lit).map(Cow::Owned);
            }
          },
          MemberProp::PrivateName(_) => {},
        }
      }

      if let Some(namespaces) = namespaces
        && let Some(obj_name) = obj_name
        && let Some(prop_name) = prop_name
      {
        // Dynamic style functions (e.g. `styles.opacity` where `opacity` is a
        // dynamic style) must bail out so runtime props handling stays intact.
        if state
          .dynamic_style_namespaces
          .get(obj_name)
          .is_some_and(|namespaces| namespaces.contains(prop_name.as_ref()))
        {
          return StyleObject::Other;
        }

        if let Some(style_value) = namespaces.get(prop_name.as_ref()) {
          return StyleObject::Style((**style_value).clone());
        }
      }

      StyleObject::Other
    },
    _ => StyleObject::Other,
  };

  if result == StyleObject::Other {
    let parsed_obj = evaluate(path, state, evaluate_path_fn_config);

    if parsed_obj.confident
      && let Some(result) = parsed_obj.value.as_ref()
    {
      let mut compiled_styles: IndexMap<String, Rc<FlatCompiledStylesValue>> = IndexMap::new();

      if let Some(value) = parse_compiled_styles(&mut compiled_styles, result) {
        return value;
      }
    }
  }

  result
}

fn parse_compiled_styles(
  compiled_styles: &mut IndexMap<String, Rc<FlatCompiledStylesValue>>,
  result: &EvaluateResultValue,
) -> Option<StyleObject> {
  match result {
    EvaluateResultValue::Vec(arr) => {
      for item in arr.iter() {
        match item {
          EvaluateResultValue::Expr(expr) => parse_nullable_object(compiled_styles, expr),
          EvaluateResultValue::Vec(arr) => {
            parse_compiled_styles(compiled_styles, &EvaluateResultValue::Vec(arr.clone()));
          },
          EvaluateResultValue::Null => {},
          _ => {
            stylex_unimplemented!(
              "Encountered an unsupported evaluation result while parsing a nullable style array."
            );
          },
        };
      }
      if compiled_styles.is_empty() {
        return Some(StyleObject::Other);
      }
      return Some(StyleObject::Style(compiled_styles.clone()));
    },
    EvaluateResultValue::Expr(expr) => {
      if expr.is_object() {
        parse_nullable_object(compiled_styles, expr);
        return Some(StyleObject::Style(compiled_styles.clone()));
      }
    },
    EvaluateResultValue::ThemeRef(_) => {
      return Some(StyleObject::Other);
    },
    _ => {
      stylex_unimplemented!(
        "Encountered an unsupported evaluation result while parsing a nullable style."
      );
    },
  }
  None
}

fn parse_nullable_object(
  compiled_styles: &mut IndexMap<String, Rc<FlatCompiledStylesValue>>,
  expr: &Expr,
) {
  match expr {
    Expr::Object(object) => read_declarations(compiled_styles, object),
    _ => {
      stylex_unimplemented!(
        "Encountered an unsupported expression type while parsing a nullable style array."
      );
    },
  }
}

/// Reads every declaration one object literal writes into `compiled_styles`.
///
/// A name written twice keeps the place of its first writing and the value of
/// its last, which is how the language reads such an object.
fn read_declarations(
  compiled_styles: &mut IndexMap<String, Rc<FlatCompiledStylesValue>>,
  ObjectLit { props, .. }: &ObjectLit,
) {
  for prop in props.iter() {
    if let Some(key_value) = prop.as_prop().and_then(|p| p.as_key_value()) {
      let key = convert_key_value_to_str(key_value);

      compiled_styles.insert(key, Rc::new(parse_nullable_value(key_value.value.as_ref())));
    }
  }
}

/// What one declaration of an inline style holds.
///
/// An inline style is the object the author wrote, so a value keeps the kind
/// they gave it: `opacity: 0.5` is a number, `color: true` is a boolean, and a
/// pseudo-class such as `:hover` holds an object of declarations of its own.
/// Every one of them is read back where the call is written, so a kind that is
/// dropped here is a declaration the runtime never applies.
fn parse_nullable_value(expr: &Expr) -> FlatCompiledStylesValue {
  match expr {
    Expr::Lit(lit) => parse_nullable_literal(lit),
    Expr::Object(object) => {
      let mut nested: IndexMap<String, Rc<FlatCompiledStylesValue>> =
        IndexMap::with_capacity(object.props.len());

      read_declarations(&mut nested, object);

      FlatCompiledStylesValue::Object(nested)
    },
    _ => {
      stylex_unimplemented!(
        "Encountered an unsupported expression type while parsing a nullable style array."
      );
    },
  }
}

fn parse_nullable_literal(lit: &Lit) -> FlatCompiledStylesValue {
  match lit {
    // Read as the text it is rather than through the converter that asks what
    // kind of literal it is: the arm has already answered that.
    Lit::Str(text) => FlatCompiledStylesValue::String(convert_str_lit_to_string(text)),
    Lit::Num(number) => FlatCompiledStylesValue::Number(number.value),
    Lit::Bool(bool_lit) => FlatCompiledStylesValue::Bool(bool_lit.value),
    Lit::Null(_) => FlatCompiledStylesValue::Null,
    _ => {
      stylex_panic!("Unhandled literal type in nullable style parsing array");
    },
  }
}

#[cfg(test)]
#[path = "tests/parse_nullable_style_tests.rs"]
mod tests;
