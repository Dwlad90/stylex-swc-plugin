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
      read_array(compiled_styles, arr);

      if compiled_styles.is_empty() {
        return Some(StyleObject::Other);
      }
      // Taken rather than copied. The caller drops the map as soon as this
      // answers, so a copy of every name it holds was made only to be thrown
      // away on the next line.
      return Some(StyleObject::Style(std::mem::take(compiled_styles)));
    },
    EvaluateResultValue::Expr(expr) => {
      if expr.is_object() {
        parse_nullable_object(compiled_styles, expr);
        return Some(StyleObject::Style(std::mem::take(compiled_styles)));
      }
    },
    EvaluateResultValue::ThemeRef(_) => {
      return Some(StyleObject::Other);
    },
    _ => {
      stylex_unimplemented!("Encountered a style argument the compiler cannot read.");
    },
  }
  None
}

/// Reads every style one array of evaluated arguments holds into
/// `compiled_styles`.
///
/// Split from the answer above so the map is taken once, where the reading is
/// finished. It also reads a nested array where it stands, rather than
/// rebuilding it as an evaluation result first, which copied every element of
/// it.
fn read_array(
  compiled_styles: &mut IndexMap<String, Rc<FlatCompiledStylesValue>>,
  arr: &[EvaluateResultValue],
) {
  for item in arr.iter() {
    match item {
      EvaluateResultValue::Expr(expr) => parse_nullable_object(compiled_styles, expr),
      EvaluateResultValue::Vec(arr) => read_array(compiled_styles, arr),
      EvaluateResultValue::Null => {},
      _ => {
        stylex_unimplemented!("Encountered an element of a style list the compiler cannot read.");
      },
    };
  }
}

fn parse_nullable_object(
  compiled_styles: &mut IndexMap<String, Rc<FlatCompiledStylesValue>>,
  expr: &Expr,
) {
  match expr {
    Expr::Object(object) => {
      for (key, value) in declarations_of(object) {
        compiled_styles.insert(key, value);
      }
    },
    _ => {
      stylex_unimplemented!("Encountered a style argument that is not an object.");
    },
  }
}

/// The one name an object literal writes that declares no property of its own.
///
/// Writing it sets what the object inherits from, whichever of the three ways
/// it was spelled, so the object holds no such name afterwards.
const PROTOTYPE_KEY: &str = "__proto__";

/// Every declaration one style argument makes: the names it writes itself, and
/// after them the names it inherits.
///
/// The merge walks what an object inherits from, so a prototype declares its
/// own names too -- after the object's, and shadowed by them. A prototype that
/// was given one of its own is read the same way again, which is why the walk
/// is a loop rather than one step.
///
/// The names of one object go into a map of their own before they reach the
/// caller's, because the two levels are combined by different rules: a name
/// written twice in one object keeps the value of its last writing, and a name
/// an object both writes and inherits keeps the one it wrote.
fn declarations_of(object: &ObjectLit) -> IndexMap<String, Rc<FlatCompiledStylesValue>> {
  let mut declarations: IndexMap<String, Rc<FlatCompiledStylesValue>> =
    IndexMap::with_capacity(object.props.len());
  let mut prototype = read_declarations(&mut declarations, object);

  while let Some(object) = prototype {
    let mut inherited: IndexMap<String, Rc<FlatCompiledStylesValue>> =
      IndexMap::with_capacity(object.props.len());

    prototype = read_declarations(&mut inherited, object);

    for (key, value) in inherited {
      declarations.entry(key).or_insert(value);
    }
  }

  declarations
}

/// Reads the names one object literal writes into `compiled_styles`, and
/// answers the object it was told to inherit from.
///
/// A name written twice keeps the place of its first writing and the value of
/// its last, which is how the language reads such an object. Only an object
/// can be inherited from, so a prototype the author gave anything else sets
/// nothing and declares nothing.
fn read_declarations<'a>(
  compiled_styles: &mut IndexMap<String, Rc<FlatCompiledStylesValue>>,
  ObjectLit { props, .. }: &'a ObjectLit,
) -> Option<&'a ObjectLit> {
  let mut prototype = None;

  for prop in props.iter() {
    if let Some(key_value) = prop.as_prop().and_then(|p| p.as_key_value()) {
      let key = convert_key_value_to_str(key_value);

      if key == PROTOTYPE_KEY {
        prototype = key_value.value.as_object();
        continue;
      }

      compiled_styles.insert(key, Rc::new(parse_nullable_value(key_value.value.as_ref())));
    }
  }

  prototype
}

/// What one declaration of an inline style holds.
///
/// An inline style is the object the author wrote, so a value keeps the kind
/// they gave it: `opacity: 0.5` is a number, `color: true` is a boolean,
/// `color: undefined` is a declaration with no value, and a pseudo-class such
/// as `:hover` holds an object of declarations of its own.
/// Every one of them is read back where the call is written, so a kind that is
/// dropped here is a declaration the runtime never applies.
fn parse_nullable_value(expr: &Expr) -> FlatCompiledStylesValue {
  match expr {
    Expr::Lit(lit) => parse_nullable_literal(lit),
    // A declaration with no value is kept rather than dropped here, because
    // the merge is what decides it declares nothing: it skips the property
    // whole and leaves it for a later style to set.
    Expr::Ident(ident) if is_js_undefined(ident) => FlatCompiledStylesValue::Undefined,
    // A nested object is written back with the names it holds itself. Nothing
    // walks what it inherits from, because nothing merges it -- the runtime is
    // handed the object, and a prototype is not part of what is written.
    Expr::Object(object) => {
      let mut nested: IndexMap<String, Rc<FlatCompiledStylesValue>> =
        IndexMap::with_capacity(object.props.len());

      read_declarations(&mut nested, object);

      FlatCompiledStylesValue::Object(nested)
    },
    Expr::Array(array) => FlatCompiledStylesValue::List(
      array
        .elems
        .iter()
        .map(|element| match element {
          Some(element) if element.spread.is_none() => Rc::new(parse_nullable_value(&element.expr)),
          // A hole and a spread are the two slots that hold no value of their
          // own. Neither survives being read, so the call is left to the
          // runtime rather than written short of an element.
          _ => stylex_unimplemented!("Encountered a list element with no value of its own."),
        })
        .collect(),
    ),
    _ => {
      stylex_unimplemented!("Encountered a style value the compiler cannot read.");
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
      stylex_panic!("Encountered a literal a style value cannot hold.");
    },
  }
}

#[cfg(test)]
#[path = "tests/parse_nullable_style_tests.rs"]
mod tests;
