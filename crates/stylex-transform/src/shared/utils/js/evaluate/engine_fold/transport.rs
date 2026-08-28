//! What crosses into the engine, and what it costs to carry.
//!
//! A name the guard resolved becomes a parameter of the printed arrow and its
//! value an argument to it, so a value travels *beside* the printed source
//! rather than inside it. This is that carriage: the shapes a value crosses as,
//! the two counts that bound them, and the last step into the engine's own
//! values, taken once there is an engine to build them in.
//!
//! Everything that can refuse refuses while the value is being carried, before
//! anything is printed or built — so the conversion with the engine in hand only
//! builds.

use boa_engine::{
  Context, JsObject, JsString, JsValue, NativeFunction, native_function::NativeFunctionPointer,
  object::builtins::JsArray,
};
use swc_core::{
  atoms::{Atom, Wtf8Atom},
  common::DUMMY_SP,
  ecma::ast::{
    ArrayLit, AssignPat, Expr, ExprOrSpread, KeyValueProp, Lit, ObjectLit, Pat, Prop, PropName,
    PropOrSpread,
  },
};

use stylex_ast::ast::convertors::{atom_utf16_length, is_js_undefined};
use stylex_ast::ast::factories::{create_binding_ident, create_ident};
use stylex_constants::constants::evaluation_errors::{
  bound_value_has_too_many_entries, bound_value_too_large,
};
use stylex_utils::number::to_js_string;

use super::engine::read;
use super::{Ceilings, Decline, Depth};
use crate::shared::enums::data_structures::evaluate_result_value::EvaluateResultValue;

/// The one property name that is not a property when it is written as one.
///
/// `{ __proto__: x }` sets the object's prototype, so the object the source
/// describes carries no own property called `__proto__`. This evaluator's own
/// object form keeps it as an ordinary key, which is why the two directions of
/// this bridge have to agree on it explicitly: an expression written out reaches
/// the engine as text and the language drops it, and a value the guard resolved
/// is built property by property and would keep it.
const PROTOTYPE_KEY: &str = "__proto__";

/// The names the guard resolved and the values it resolved them to, ready to
/// cross the bridge inward.
///
/// The value travels *beside* the printed source rather than inside it: the
/// expression is printed as an arrow taking these names as parameters, and the
/// values are passed as arguments. Substituting a literal into the text instead
/// would reprint and reparse the whole value at every use site, and a value with
/// no literal spelling could not cross at all — where an argument keeps the
/// printed text the size of the expression however large the value is.
///
/// The author's own name is the parameter name. Nothing has to be rewritten for
/// that, and a callback parameter of the same name shadows it in the printed
/// arrow exactly as it does in the module the author wrote.
///
/// Chosen over registering the names on the engine because the engine is one
/// leaked instance per thread, shared by every file that thread compiles: a name
/// left behind or shadowed there would be a cross-file correctness bug.
pub(super) struct Transport {
  params: Vec<Atom>,
  values: Vec<Crossing>,
  totals: Totals,
  ceilings: Ceilings,
  /// Whether anything that crossed was a theme reference, read as the string it
  /// resolves to. See [`admit_value`](super::guard::Walk::admit_value) for the
  /// read that makes such a crossing unsafe, and why the two are asked together
  /// rather than separately.
  pub(super) read_a_theme_reference: bool,
}

impl Transport {
  /// An empty transport, holding the ceilings every value it carries is counted
  /// against.
  pub(super) fn new(ceilings: Ceilings) -> Self {
    Self {
      params: Vec::new(),
      values: Vec::new(),
      totals: Totals::default(),
      ceilings,
      read_a_theme_reference: false,
    }
  }

  /// Records what `name` resolves to, or does nothing where the name is already
  /// carried.
  ///
  /// One parameter per name however often the expression reads it, because a
  /// repeated parameter is a syntax error in the arrow this is printed into. The
  /// second reading resolves to the same value — the evaluator memoises it — so
  /// dropping it loses nothing.
  ///
  /// The value is converted here rather than when the engine is in hand, so a
  /// value past a bound refuses before anything is printed or built. What comes
  /// out is [`Carried`]: the shapes the bridge carries, measured, and not yet
  /// the engine's own values.
  pub(super) fn bind(
    &mut self,
    name: &Atom,
    value: &EvaluateResultValue,
    depth: Depth,
  ) -> Result<(), Decline> {
    // Asked before the conversion rather than left to `carry`, so a name read
    // twice is not converted twice.
    if self.holds(name) {
      return Ok(());
    }

    let carried = Inward {
      name,
      totals: &mut self.totals,
      ceilings: self.ceilings,
      read_a_theme_reference: &mut self.read_a_theme_reference,
    }
    .value(value, depth.restart())?;

    self.carry(name, Crossing::Value(carried));

    Ok(())
  }

  /// Records something the bridge already holds in its own shapes, under the
  /// name the printed source reads it through.
  ///
  /// Nothing here can refuse, which is the whole difference from [`bind`]: what
  /// it takes was not copied out of the module, so there is no text and no entry
  /// count to compare a ceiling to. A [StyleX function the engine may
  /// call](super::super::engine_stylex_functions) and a resolved arrow are what
  /// arrive this way.
  ///
  /// [`bind`]: Transport::bind
  pub(super) fn carry(&mut self, name: &Atom, crossing: Crossing) {
    if self.holds(name) {
      return;
    }

    self.params.push(name.clone());
    self.values.push(crossing);
  }

  /// The names as the parameters of the printed arrow.
  ///
  /// A value's name is a bare parameter and its value arrives as an argument. A
  /// function's is a parameter with the expression it was declared from as its
  /// default, because a function has no argument form to arrive as — so the
  /// printed source carries it and the call passes nothing.
  ///
  /// A default is evaluated where the parameter stands, so the order the names
  /// were carried in is the order they have to be printed in: a name a default
  /// reads is carried before the name whose default reads it, which is what
  /// walking a declaration before recording it buys.
  pub(super) fn parameters(&self) -> Vec<Pat> {
    self
      .params
      .iter()
      .zip(&self.values)
      .map(|(name, crossing)| {
        let bound = Pat::Ident(create_binding_ident(create_ident(name)));

        match crossing {
          Crossing::Value(_) => bound,
          Crossing::Source(source) => Pat::Assign(AssignPat {
            span: DUMMY_SP,
            left: Box::new(bound),
            right: source.clone(),
          }),
        }
      })
      .collect()
  }

  /// Whether a value is already travelling under `name`.
  ///
  /// A scan rather than a set, because an expression carries a handful of names
  /// and hashing them would cost more than comparing them.
  pub(super) fn holds(&self, name: &Atom) -> bool {
    self.params.contains(name)
  }

  /// The carried values as the arguments the printed arrow is called with.
  ///
  /// Built with the engine in hand, because an array and an object are engine
  /// objects and there is no way to make one without it. Everything that could
  /// refuse was answered when the value was carried, so this step only builds.
  pub(super) fn arguments(
    &self,
    engine: &mut Context,
    method: &Atom,
  ) -> Result<Vec<JsValue>, Decline> {
    let mut arguments = Vec::with_capacity(self.values.len());

    for value in &self.values {
      arguments.push(match value {
        Crossing::Value(carried) => to_js(carried, engine, method)?,
        // A function travels in the printed source as the parameter's default,
        // so passing nothing for it is what makes the default the value.
        Crossing::Source(_) => JsValue::undefined(),
      });
    }

    Ok(arguments)
  }
}

/// What travels under one parameter name of the printed arrow.
///
/// Two arms because a value and a function cross by different routes, and the
/// routes are not interchangeable. A value is copied into the engine and passed
/// as an argument, which keeps the printed text the size of the expression
/// however large the value is. A function has no such form — there is no engine
/// value a resolved arrow could be built into before the engine exists, and
/// nothing an argument could carry — so what crosses is the source it was
/// declared from, printed back where the parameter stands.
///
/// Printing it as a *default* rather than substituting it at each reading is
/// what keeps shadowing the language's answer instead of this walk's: a callback
/// parameter of the same name shadows the default exactly as it shadowed the
/// module binding, and a substitution would have had to work that out for
/// itself.
pub(super) enum Crossing {
  Value(Carried),
  Source(Box<Expr>),
}

/// A resolved value on its way into the engine, in the shapes the bridge
/// carries.
///
/// A tree of the bridge's own rather than the evaluator's value or the engine's.
/// The evaluator's can be walked with no engine in hand, which is what lets
/// every bound be answered before one is built; the engine's cannot exist until
/// it is. So a value crosses in two steps, and this is what stands between them.
///
/// Refused in both directions, as stated rules rather than omissions: function
/// configurations, callbacks, the environment object, an unresolved theme
/// reference, and the AST-keyed map variants. A theme reference therefore
/// crosses only as the `var(--…)` string it already resolved to, because
/// resolving it is what mutates compiler state and that happens before the
/// bridge.
///
/// A *function* is the one thing here that is not copied out of the module. It
/// is one of this compiler's own, handed over for the engine to call rather than
/// to read, and only where nothing an author writes could reach it as a value —
/// see [`engine_stylex_functions`](super::super::engine_stylex_functions).
pub(super) enum Carried {
  Str(Wtf8Atom),
  Num(f64),
  Bool(bool),
  Null,
  Undefined,
  List(Vec<Carried>),
  Object(Vec<(Wtf8Atom, Carried)>),
  Function(NativeFunctionPointer),
}

/// How much one fold has already promised to copy into the engine.
///
/// Counted across every name a fold carries rather than per name, because what
/// is about to be copied is all of them: eight names each one unit under the
/// limit is eight times the limit, and a per-value check would wave every one of
/// them through.
///
/// Two counts, because a value costs in two ways that do not stand in for each
/// other. A thousand empty arrays hold no text at all and are still a thousand
/// values to build; a single string is one entry and can be a megabyte.
#[derive(Default)]
struct Totals {
  /// UTF-16 code units of every string and key carried — the unit the engine's
  /// own strings are measured in, and the one that bounds a folded string on the
  /// way out.
  units: u64,
  /// Array elements and object properties carried.
  entries: u64,
}

/// What the inward conversion carries: the name whose value is crossing, so a
/// bound can say which binding to shorten, and the totals it counts against.
struct Inward<'a> {
  name: &'a Atom,
  totals: &'a mut Totals,
  ceilings: Ceilings,
  /// Set where a theme reference crossed, so the walk can refuse the one shape
  /// that would read a property off the string it crossed as.
  read_a_theme_reference: &'a mut bool,
}

impl Inward<'_> {
  /// One value the evaluator answered as the bridge's own, or the reason it does
  /// not cross.
  ///
  /// A shape the bridge does not carry is not a refusal. The dispatch below the
  /// fold owns those values and answers for them, and a rule fired here would
  /// stop it ever being asked.
  fn value(&mut self, value: &EvaluateResultValue, depth: Depth) -> Result<Carried, Decline> {
    match value {
      EvaluateResultValue::Expr(expr) => self.expr(expr, depth),
      // The evaluator answers an array either as a list of its own or as the
      // array literal it was written as. Both are one array here, which is the
      // whole of why the two dispatch arms that carried the array methods could
      // disagree about which names they knew: they were answering for the same
      // value in two shapes.
      EvaluateResultValue::Vec(items) => {
        let inner = depth.descend()?;

        self.count(items.len())?;

        let mut list = Vec::with_capacity(items.len());

        for item in items {
          list.push(self.value(item, inner)?);
        }

        Ok(Carried::List(list))
      },
      // A `defineVars` group reaching the bridge as a value rather than through
      // a property: its own `toString` answers the variable-group hash, which is
      // read off the reference itself and mutates nothing — so this is the one
      // of this compiler's values that has a JavaScript form to cross as.
      //
      // Crossing as that string is only right where nothing reads a property
      // off it, because a string has none of the group's members. The walk
      // refuses that shape rather than the bridge, which sees one value at a
      // time and cannot know what the expression around it does — see
      // [`admit_value`].
      EvaluateResultValue::ThemeRef(theme) => {
        *self.read_a_theme_reference = true;

        Ok(Carried::Str(
          self.text(&Wtf8Atom::from(theme.to_string_value().as_str()))?,
        ))
      },
      _ => Err(Decline::NotACandidate),
    }
  }

  /// One evaluated expression as the bridge's own value.
  ///
  /// One level of the budget per node, leaves included, exactly as the guard's
  /// own walk spends it — the two walk the same shapes and would otherwise
  /// disagree about how deep the same value is.
  fn expr(&mut self, expr: &Expr, depth: Depth) -> Result<Carried, Decline> {
    let inner = depth.descend()?;

    match expr {
      Expr::Lit(Lit::Str(text)) => Ok(Carried::Str(self.text(&text.value)?)),
      Expr::Lit(Lit::Num(number)) => Ok(Carried::Num(number.value)),
      Expr::Lit(Lit::Bool(truth)) => Ok(Carried::Bool(truth.value)),
      Expr::Lit(Lit::Null(_)) => Ok(Carried::Null),
      // The one value the grammar has no literal for, so an author spells it as
      // a name and the evaluator hands it back as one. A binding of that name
      // never arrives here — the evaluator refuses a shadowed `undefined`
      // ahead of every reader of this shape — and `void 0` is the same value
      // under a different spelling, which the evaluator has already answered as
      // this identifier.
      Expr::Ident(ident) if is_js_undefined(ident) => Ok(Carried::Undefined),
      Expr::Array(ArrayLit { elems, .. }) => {
        self.count(elems.len())?;

        let mut list = Vec::with_capacity(elems.len());

        for elem in elems {
          // A hole is `undefined` and a spread was refused where it was
          // written, so neither reaches a fold: the reference compiler refuses
          // a method call on both, and folding a hole as anything at all would
          // write a value the source does not describe.
          let Some(ExprOrSpread { spread: None, expr }) = elem else {
            return Err(Decline::NotACandidate);
          };

          list.push(self.expr(expr, inner)?);
        }

        Ok(Carried::List(list))
      },
      Expr::Object(ObjectLit { props, .. }) => {
        self.count(props.len())?;

        let mut entries = Vec::with_capacity(props.len());

        for prop in props {
          let PropOrSpread::Prop(prop) = prop else {
            return Err(Decline::NotACandidate);
          };

          let Prop::KeyValue(KeyValueProp { key, value }) = prop.as_ref() else {
            return Err(Decline::NotACandidate);
          };

          let key = self.key(key)?;

          // `__proto__` written as a plain key sets the prototype rather than a
          // member, so the object the source describes has no own property of
          // that name. The evaluator keeps it as one, so it is dropped here —
          // where an expression written out reaches the engine as text and the
          // language drops it for us. Both paths then answer alike, and alike is
          // what the reference compiler answers.
          if key == PROTOTYPE_KEY {
            continue;
          }

          entries.push((key, self.expr(value, inner)?));
        }

        Ok(Carried::Object(entries))
      },
      _ => Err(Decline::NotACandidate),
    }
  }

  /// A property name as the string the language reads it as.
  fn key(&mut self, key: &PropName) -> Result<Wtf8Atom, Decline> {
    let name = match key {
      PropName::Ident(name) => Wtf8Atom::from(&*name.sym),
      PropName::Str(name) => name.value.clone(),
      // A numeric key names the property its own string form spells, read by
      // the conversion every other number-to-string in this compiler uses
      // rather than by a spelling of its own.
      PropName::Num(number) => Wtf8Atom::from(to_js_string(number.value).as_str()),
      // A computed key was evaluated before it reached a value, and a BigInt is
      // not a value this bridge carries in any position.
      _ => return Err(Decline::NotACandidate),
    };

    self.text(&name)
  }

  /// One string, counted against what this fold may copy in.
  fn text(&mut self, text: &Wtf8Atom) -> Result<Wtf8Atom, Decline> {
    // Saturating because the sum exists to be refused on, and a wrapped one
    // would admit.
    self.totals.units = self
      .totals
      .units
      .saturating_add(atom_utf16_length(text) as u64);

    if self.totals.units > self.ceilings.characters {
      return Err(Decline::rule(bound_value_too_large(
        self.name,
        self.ceilings.characters,
      )));
    }

    Ok(text.clone())
  }

  /// Some elements or properties, counted against the same budget.
  ///
  /// Counted before they are walked, so a list past the bound refuses without
  /// first converting every entry in it.
  fn count(&mut self, entries: usize) -> Result<(), Decline> {
    self.totals.entries = self.totals.entries.saturating_add(entries as u64);

    if self.totals.entries > self.ceilings.entries {
      return Err(Decline::rule(bound_value_has_too_many_entries(
        self.name,
        self.ceilings.entries,
      )));
    }

    Ok(())
  }
}

/// One carried value as the engine's own, under the name of the method whose
/// fold it is about to be an argument to.
///
/// Needs no bound of its own: what it walks was bounded when it was carried, in
/// text, in entries and in nesting, and nothing has been added to it since.
fn to_js(carried: &Carried, engine: &mut Context, method: &Atom) -> Result<JsValue, Decline> {
  let value = match carried {
    Carried::Str(text) => JsValue::from(carry_string(text)),
    Carried::Num(number) => JsValue::from(*number),
    Carried::Bool(truth) => JsValue::from(*truth),
    Carried::Null => JsValue::null(),
    Carried::Undefined => JsValue::undefined(),
    Carried::List(items) => {
      let mut values = Vec::with_capacity(items.len());

      for item in items {
        values.push(to_js(item, engine, method)?);
      }

      JsValue::from(JsArray::from_iter(values, engine))
    },
    Carried::Object(entries) => {
      let object = JsObject::with_object_proto(engine.intrinsics());

      for (key, value) in entries {
        let value = to_js(value, engine, method)?;

        // A fresh ordinary object takes a data property without complaint, so
        // the throw is unreachable — and answered rather than asserted, because
        // this runs inside an evaluation whose whole contract is that it may
        // fail.
        read(method, || {
          object.create_data_property_or_throw(carry_string(key), value, engine)
        })?;
      }

      JsValue::from(object)
    },
    // Built per fold rather than kept, because a function object belongs to the
    // realm that made it and costs one allocation — where keeping one would tie
    // a value's lifetime to the engine's, which is the arrangement the memo
    // beside it already had to be written around.
    Carried::Function(call) => {
      JsValue::from(NativeFunction::from_fn_ptr(*call).to_js_function(engine.realm()))
    },
  };

  Ok(value)
}

/// One resolved string as the engine's own string type.
///
/// Two readings of the same atom, because a JavaScript string literal can hold
/// an unpaired surrogate and no Rust `str` can. The engine's strings are UTF-16,
/// so the ill-formed reading carries such a value across *exactly* — where the
/// outward direction has to substitute the replacement character, since a
/// `Lit::Str` is what it has to land in. A valid value takes the direct reading
/// rather than being taken apart into code units it would only be rebuilt from.
fn carry_string(value: &Wtf8Atom) -> JsString {
  match value.as_str() {
    Some(text) => JsString::from(text),
    None => JsString::from(&value.to_ill_formed_utf16().collect::<Vec<u16>>()[..]),
  }
}
