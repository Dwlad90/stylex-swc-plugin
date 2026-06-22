//! Inline atomic-style transform for `@stylexjs/atoms`.
//!
//! Detects and compiles inline atomic-style expressions:
//!
//! - `css.display.flex` → `{ $$css: true, display: "x..." }` (static)
//! - `css.color(value)` → a hoisted dynamic-style function call (dynamic)
//!
//! The detection and AST rewriting live here; the heavy style-compilation
//! utilities (the `compile` object plus the needed `StateManager` access)
//! are injected through the [`Compile`] trait so this crate does not depend on
//! `stylex-transform`.

use rustc_hash::FxHashMap;
use stylex_constants::constants::common::COMPILED_KEY;
use swc_core::{
  common::{DUMMY_SP, SyntaxContext},
  ecma::{
    ast::{
      ArrayLit, ArrowExpr, BinExpr, BinaryOp, BindingIdent, BlockStmtOrExpr, Bool, CallExpr,
      Callee, ComputedPropName, CondExpr, Expr, ExprOrSpread, Id, Ident, IdentName, KeyValueProp,
      Lit, MemberExpr, MemberProp, Null, Number, ObjectLit, Pat, Prop, PropName, PropOrSpread, Str,
    },
    visit::{VisitMut, VisitMutWith},
  },
};

/// The npm package name that marks an import as an atoms source.
pub const ATOMS_SOURCE: &str = "@stylexjs/atoms";

/// A flat compiled-style value, representing the subset of compiled values that
/// atoms produce (string class names, the `$$css` marker, or `null`).
#[derive(Debug, Clone, PartialEq)]
pub enum AtomFlatValue {
  String(String),
  Bool(bool),
  Null,
}

/// A single injected CSS rule produced while compiling an atom.
#[derive(Debug, Clone, PartialEq)]
pub struct InjectedAtomStyle {
  pub class_name: String,
  pub priority: f64,
  pub ltr: String,
  pub rtl: Option<String>,
}

/// The result of compiling a single `{ [property]: value }` atom through the
/// injected [`Compile::style_x_create_set`].
#[derive(Debug, Clone)]
pub struct AtomCompileResult {
  /// The compiled `__inline__` namespace as an object-literal expression,
  /// e.g. `{ display: "x78zum5", $$css: true }`.
  pub compiled_ast: Expr,
  /// A flat key/value view of the same namespace, used to locate the property
  /// hash + class name when building a dynamic-style function.
  pub compiled_flat: Vec<(String, AtomFlatValue)>,
  /// The injected CSS rules for this atom.
  pub injected: Vec<InjectedAtomStyle>,
}

/// A detected static style, e.g. `{ property: "display", value: "flex" }`.
#[derive(Debug, Clone, PartialEq)]
pub struct StaticStyle {
  pub property: String,
  pub value: String,
}

/// A detected dynamic style, e.g. `css.color(value)` →
/// `{ property: "color", value: <value expr> }`.
#[derive(Debug, Clone)]
pub struct DynamicStyle {
  pub property: String,
  pub value: Box<Expr>,
}

/// Compilation utilities injected by the consumer (the JS `compile` object plus
/// the `StateManager` access the atoms transform needs).
pub trait Compile {
  /// Map of local identifier → imported name. Namespace/default imports map to
  /// `"*"`. Populated while reading import declarations.
  fn atom_imports(&self) -> &FxHashMap<Id, String>;

  /// Compile a single `{ [property]: value }` style, returning the compiled
  /// object AST, a flat view of it, and the injected CSS rules. Equivalent to
  /// `compile.styleXCreateSet` (plus dev class names) applied to
  /// `{ __inline__: { [property]: value } }`.
  ///
  /// Returns `None` when the style is not statically evaluable, so the caller
  /// leaves the original expression untouched for runtime instead of panicking.
  fn style_x_create_set(&mut self, property: &str, value: &str) -> Option<AtomCompileResult>;

  /// Register injected CSS rules so they are emitted at runtime / collected as
  /// metadata. Equivalent to `state.registerStyles`.
  fn register_styles(&mut self, injected: &[InjectedAtomStyle]);

  /// Hoist an expression to module scope and return an identifier referencing
  /// it. Equivalent to `compile.hoistExpression`.
  fn hoist_expression(&mut self, expr: Expr) -> Expr;
}

/// Extracts the key/value name from a member property.
///
/// - non-computed identifier → the identifier name
/// - computed string literal → the string value
/// - computed numeric literal → the number rendered as a string
pub fn get_prop_key(prop: &MemberProp) -> Option<String> {
  match prop {
    MemberProp::Ident(ident) => Some(ident.sym.to_string()),
    MemberProp::Computed(computed) => match computed.expr.as_ref() {
      Expr::Lit(Lit::Str(s)) => s.value.as_str().map(|value| value.to_string()),
      Expr::Lit(Lit::Num(n)) => Some(number_to_string(n)),
      _ => None,
    },
    MemberProp::PrivateName(_) => None,
  }
}

/// Strips a single leading underscore from CSS values. This allows using
/// underscore-prefixed identifiers for CSS values that conflict with JS
/// reserved words or start with a digit (e.g., `css.display._flex` or
/// `css.zIndex._1`).
pub fn normalize_value(value: &str) -> String {
  match value.strip_prefix('_') {
    Some(stripped) => stripped.to_string(),
    None => value.to_string(),
  }
}

/// Whether `ident` refers to an atoms import.
///
/// Atoms imports are keyed by full SWC `Id` (`Atom` plus `SyntaxContext`) so a
/// shadowed local with the same symbol text is not mistaken for the import after
/// the resolver pass has assigned distinct contexts.
pub fn is_utility_styles_identifier(ident: &Ident, atom_imports: &FxHashMap<Id, String>) -> bool {
  atom_imports.contains_key(&ident.to_id())
}

/// Detects a static atom from a member expression:
/// `css.display.flex`, `css.width['calc(...)']`, or the default-import
/// `css.flex` form.
pub fn get_static_style_from_path(
  member: &MemberExpr,
  atom_imports: &FxHashMap<Id, String>,
) -> Option<StaticStyle> {
  let value_key = get_prop_key(&member.prop)?;

  match member.obj.as_ref() {
    Expr::Member(parent) => {
      let prop_name = get_prop_key(&parent.prop)?;
      if let Expr::Ident(base) = parent.obj.as_ref()
        && is_utility_styles_identifier(base, atom_imports)
      {
        return Some(StaticStyle {
          property: prop_name,
          value: normalize_value(&value_key),
        });
      }
      None
    },
    Expr::Ident(base) => match atom_imports.get(&base.to_id()) {
      Some(imported_name) => {
        let property = if imported_name == "*" {
          value_key.clone()
        } else {
          imported_name.clone()
        };
        Some(StaticStyle {
          property,
          value: normalize_value(&value_key),
        })
      },
      None => None,
    },
    _ => None,
  }
}

/// Detects a dynamic atom from a call expression: `css.color(value)` or
/// `css.a.b(value)` (single argument).
pub fn get_dynamic_style_from_path(
  call: &CallExpr,
  atom_imports: &FxHashMap<Id, String>,
) -> Option<DynamicStyle> {
  let callee = match &call.callee {
    Callee::Expr(expr) => expr.as_ref(),
    _ => return None,
  };

  let callee_member = match callee {
    Expr::Member(member) => member,
    _ => return None,
  };

  let value_key = get_prop_key(&callee_member.prop)?;

  if call.args.len() != 1 {
    return None;
  }

  let arg = &call.args[0];
  if arg.spread.is_some() {
    return None;
  }
  let value = arg.expr.clone();

  match callee_member.obj.as_ref() {
    Expr::Ident(base) if is_utility_styles_identifier(base, atom_imports) => Some(DynamicStyle {
      property: value_key,
      value,
    }),
    Expr::Member(parent) => {
      let prop_name = get_prop_key(&parent.prop)?;
      if let Expr::Ident(base) = parent.obj.as_ref()
        && is_utility_styles_identifier(base, atom_imports)
      {
        return Some(DynamicStyle {
          property: prop_name,
          value,
        });
      }
      None
    },
    _ => None,
  }
}

/// Compile a static atom directly to a compiled style object.
/// `css.display.flex` → `{ $$css: true, display: "x78zum5" }`.
///
/// Returns `None` when the member is not an atom or the style is not statically
/// evaluable; the caller then leaves the original expression in place.
pub fn compile_static_style<T: Compile>(compiler: &mut T, member: &MemberExpr) -> Option<Expr> {
  let style = get_static_style_from_path(member, compiler.atom_imports())?;

  let result = compiler.style_x_create_set(&style.property, &style.value)?;

  compiler.register_styles(&result.injected);

  Some(result.compiled_ast)
}

/// Compile a dynamic atom to a hoisted function pattern.
/// `css.color(value)` → `_temp.color(value)`.
///
/// Returns `None` when the call is not a dynamic atom or the style is not
/// statically evaluable; the caller then leaves the original call in place.
pub fn compile_dynamic_style<T: Compile>(compiler: &mut T, call: &CallExpr) -> Option<Expr> {
  let style = get_dynamic_style_from_path(call, compiler.atom_imports())?;
  let property = style.property;

  let var_name = sanitize_custom_property(&property);
  let css_value = format!("var({var_name})");

  let mut result = compiler.style_x_create_set(&property, &css_value)?;

  result.injected.push(InjectedAtomStyle {
    class_name: var_name.clone(),
    priority: 0.0,
    ltr: format!("@property {var_name} {{ syntax: \"*\"; inherits: false;}}"),
    rtl: None,
  });

  compiler.register_styles(&result.injected);

  // compiled_flat has: [($$css, true), (prop-hash, "xAbc123"), maybe (devClass, devClass)].
  // We need the prop-hash key (not $$css, not the dev class name where key === value).
  let (prop_key, class_name) = result.compiled_flat.iter().find_map(|(key, value)| {
    if key == COMPILED_KEY {
      return None;
    }
    match value {
      AtomFlatValue::String(class_name) if class_name != key => {
        Some((key.clone(), class_name.clone()))
      },
      _ => None,
    }
  })?;

  let param = ident("_v");
  let param_expr = || Expr::Ident(param.clone());
  let null_check = || {
    Expr::Bin(BinExpr {
      span: DUMMY_SP,
      op: BinaryOp::NotEq,
      left: Box::new(param_expr()),
      right: Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))),
    })
  };

  let compiled_obj = object_expr(vec![
    string_key_value(
      &prop_key,
      Expr::Cond(CondExpr {
        span: DUMMY_SP,
        test: Box::new(null_check()),
        cons: Box::new(string_expr(&class_name)),
        alt: Box::new(param_expr()),
      }),
    ),
    string_key_value(
      COMPILED_KEY,
      Expr::Lit(Lit::Bool(Bool {
        span: DUMMY_SP,
        value: true,
      })),
    ),
  ]);

  let inline_vars_obj = object_expr(vec![string_key_value(
    &var_name,
    Expr::Cond(CondExpr {
      span: DUMMY_SP,
      test: Box::new(null_check()),
      cons: Box::new(param_expr()),
      alt: Box::new(Expr::Ident(ident("undefined"))),
    }),
  )]);

  let fn_body = Expr::Array(ArrayLit {
    span: DUMMY_SP,
    elems: vec![
      Some(expr_or_spread(compiled_obj)),
      Some(expr_or_spread(inline_vars_obj)),
    ],
  });

  let arrow_fn = Expr::Arrow(ArrowExpr {
    span: DUMMY_SP,
    ctxt: SyntaxContext::empty(),
    params: vec![Pat::Ident(BindingIdent {
      id: param,
      type_ann: None,
    })],
    body: Box::new(BlockStmtOrExpr::Expr(Box::new(fn_body))),
    is_async: false,
    is_generator: false,
    type_params: None,
    return_type: None,
  });

  let styles_obj = object_expr(vec![style_key_value(&property, arrow_fn)]);

  let hoisted_id = compiler.hoist_expression(styles_obj);

  Some(Expr::Call(CallExpr {
    span: DUMMY_SP,
    ctxt: SyntaxContext::empty(),
    callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
      span: DUMMY_SP,
      obj: Box::new(hoisted_id),
      prop: member_prop_from_key(&property),
    }))),
    args: vec![expr_or_spread(*style.value)],
    type_args: None,
  }))
}

/// A `VisitMut` pass that transforms utility style expressions into compiled
/// style objects. Member expressions become static compiled objects and call
/// expressions become hoisted dynamic-style calls.
pub struct UtilityStylesVisitor<'a, T: Compile> {
  compiler: &'a mut T,
}

/// Creates a [`UtilityStylesVisitor`] borrowing the injected compiler.
pub fn create_utility_styles_visitor<T: Compile>(compiler: &mut T) -> UtilityStylesVisitor<'_, T> {
  UtilityStylesVisitor { compiler }
}

impl<T: Compile> VisitMut for UtilityStylesVisitor<'_, T> {
  fn visit_mut_expr(&mut self, expr: &mut Expr) {
    match expr {
      Expr::Call(call) => {
        // Compile atoms nested in the arguments first; the callee is
        // intentionally skipped for the atom-call check so an atom member used
        // as a call callee is not compiled as a static style.
        // `compile_dynamic_style` then clones the already-compiled argument into
        // the hoisted call.
        call.args.visit_mut_with(self);

        if let Some(replacement) = compile_dynamic_style(self.compiler, call) {
          *expr = replacement;
          return;
        }

        // Not a dynamic atom call — still descend into the callee so atoms
        // nested there (e.g. a computed callee) are compiled. Args were already
        // visited above.
        call.callee.visit_mut_with(self);
        return;
      },
      Expr::Member(member) => {
        if let Some(replacement) = compile_static_style(self.compiler, member) {
          *expr = replacement;
          return;
        }
      },
      _ => {},
    }

    expr.visit_mut_children_with(self);
  }
}

/// Renders a numeric member key as a string from its parsed value, never the
/// raw source token: `css[0x10]` keys on `"16"`, matching `String(prop.value)`.
///
/// Mirrors JS `String(Number)` for the edge values too: `NaN`/`Infinity`
/// render as in JS, and only safe-integer-range whole numbers take the integer
/// path (larger magnitudes keep the float rendering to avoid a saturating cast).
fn number_to_string(n: &Number) -> String {
  let value = n.value;

  if value.is_nan() {
    return "NaN".to_string();
  }
  if value.is_infinite() {
    return if value > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
  }

  // 2^53 — beyond this an f64 can no longer represent consecutive integers, and
  // `as i64` would start saturating/rounding, so fall back to float rendering.
  const MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_992.0;

  if value.fract() == 0.0 && value.abs() < MAX_SAFE_INTEGER {
    format!("{}", value as i64)
  } else {
    format!("{value}")
  }
}

/// Renders a property name safe for use inside a CSS custom-property name.
///
/// Property names from namespace/default imports come from user-controlled
/// computed member keys (`css['weird; }'](v)`), so any character outside
/// `[A-Za-z0-9_-]` is replaced with `-` before being interpolated into the
/// `var(...)` reference and the `@property` at-rule, preventing malformed or
/// injected CSS.
fn sanitize_custom_property(property: &str) -> String {
  let cleaned: String = property
    .chars()
    .map(|c| {
      if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
        c
      } else {
        '-'
      }
    })
    .collect();

  format!("--x-{cleaned}")
}

fn ident(name: &str) -> Ident {
  Ident {
    span: DUMMY_SP,
    ctxt: SyntaxContext::empty(),
    sym: name.into(),
    optional: false,
  }
}

fn string_expr(value: &str) -> Expr {
  Expr::Lit(Lit::Str(Str {
    span: DUMMY_SP,
    value: value.into(),
    raw: None,
  }))
}

fn string_member_prop(value: &str) -> MemberProp {
  MemberProp::Computed(ComputedPropName {
    span: DUMMY_SP,
    expr: Box::new(string_expr(value)),
  })
}

fn member_prop_from_key(key: &str) -> MemberProp {
  if is_ascii_identifier_name(key) {
    MemberProp::Ident(IdentName {
      span: DUMMY_SP,
      sym: key.into(),
    })
  } else {
    string_member_prop(key)
  }
}

fn object_expr(props: Vec<PropOrSpread>) -> Expr {
  Expr::Object(ObjectLit {
    span: DUMMY_SP,
    props,
  })
}

fn expr_or_spread(expr: Expr) -> ExprOrSpread {
  ExprOrSpread {
    spread: None,
    expr: Box::new(expr),
  }
}

fn string_key_value(key: &str, value: Expr) -> PropOrSpread {
  PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
    key: PropName::Str(Str {
      span: DUMMY_SP,
      value: key.into(),
      raw: None,
    }),
    value: Box::new(value),
  })))
}

fn style_key_value(key: &str, value: Expr) -> PropOrSpread {
  if is_ascii_identifier_name(key) {
    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: key.into(),
      }),
      value: Box::new(value),
    })))
  } else {
    string_key_value(key, value)
  }
}

fn is_ascii_identifier_name(value: &str) -> bool {
  let mut chars = value.chars();
  match chars.next() {
    Some(first) if first.is_ascii_alphabetic() || first == '_' || first == '$' => {},
    _ => return false,
  }

  chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
}
