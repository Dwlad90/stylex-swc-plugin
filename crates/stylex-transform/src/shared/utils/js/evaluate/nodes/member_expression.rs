use super::super::*;
use stylex_ast::ast::convertors::{
  atom_utf16_length, convert_member_prop_to_string, normalize_expr,
};
use stylex_constants::constants::evaluation_errors::unreadable_index;
use swc_core::ecma::ast::MemberExpr;

/// The one property a string or an array answers by counting.
const LENGTH: &str = "length";

/// What a member lookup asks of a string or an array.
///
/// One classification for all three receiver kinds — a string, an array literal
/// a fold produced, and an evaluated array — because they have to agree. Three
/// copies of the same property test drifted into three diagnostics for one
/// author mistake, which is what this replaces.
enum ArrayLikeLookup {
  /// `length`, which is counted rather than looked up.
  Length,
  /// An index, carrying the slot it names. Whether one can be *read* depends on
  /// the receiver, so the arms decide: both array receivers read one, and a
  /// string does not -- a string index is a single UTF-16 code unit, which can
  /// be an unpaired surrogate no Rust string holds.
  ///
  /// A key that names no slot is `Missing` and not an out-of-range `Index`, so
  /// this variant means "a slot was asked for" everywhere it is matched. Which
  /// keys those are is `index_slot`'s question and is settled before an arm
  /// sees one -- deciding it a second time inside an arm is how the receivers
  /// could come to disagree about `list["00"]`.
  ///
  /// An index past the end answers `undefined`, the language's own reading and
  /// the one a key an object does not carry already gets.
  Index(usize),
  /// A property the receiver does not carry, carrying its key. `undefined` in
  /// the language, which is the answer the object arm below already gives for a
  /// key an object does not hold, and what lets `token.missing ?? fallback`
  /// fold.
  Missing(String),
  /// A computed key with no name the evaluator could read.
  Unreadable,
}

/// Reads what a member lookup is asking for, from the evaluated property.
///
/// A key of nothing but digits is an index however it was written, because
/// `list[0]` and `list["0"]` name the same element in the language. Nothing
/// else is: `"1.5"`, `"-1"` and `"NaN"` are property names that no array
/// carries, so they answer `undefined` exactly as they do upstream. Testing
/// with `parse::<f64>()` instead would call all three indices — it accepts
/// `"NaN"` and `"inf"` — and refuse a fold the reference implementation makes.
fn classify_lookup(property: Option<&EvaluateResultValue>) -> ArrayLikeLookup {
  match property.and_then(|prop| prop.as_string_key()) {
    None => ArrayLikeLookup::Unreadable,
    Some(key) if key == LENGTH => ArrayLikeLookup::Length,
    Some(key) => match index_slot(&key) {
      Some(slot) => ArrayLikeLookup::Index(slot),
      None => ArrayLikeLookup::Missing(key),
    },
  }
}

/// The slot an index key names, or `None` where the key names no slot.
///
/// A key of nothing but digits is not automatically a slot: the language reads
/// an index only where the key is the canonical spelling of the number, so
/// `list["0"]` is the first element and `list["00"]` is a property no array
/// carries. A key that overflows `usize` names no slot this evaluator could
/// hold either, and reads `undefined` for the same reason an out-of-range one
/// does.
///
/// Written as a digit test rather than as `parse::<f64>()`, which accepts
/// `"NaN"` and `"inf"` and would call all three indices -- refusing a fold the
/// reference implementation makes. The leading-zero test is the canonical
/// spelling, without the allocation that formatting the parsed number back
/// would cost on every index read.
fn index_slot(key: &str) -> Option<usize> {
  let is_canonical_digits = !key.is_empty()
    && key.bytes().all(|byte| byte.is_ascii_digit())
    && (key.len() == 1 || !key.starts_with('0'));

  is_canonical_digits
    .then(|| key.parse::<usize>().ok())
    .flatten()
}

/// What an array answers for a slot it was asked for.
///
/// One function, because "past the end is `undefined`" is a rule about the
/// language and not about either receiver -- an array literal a fold produced
/// and an array the evaluator holds as its own value must give the same answer,
/// and two copies of the bounds check agree only by inspection. How an element
/// is read back is what does differ between them, so each supplies that.
fn index_answer<T>(
  elements: &[T],
  slot: usize,
  read: impl FnOnce(&T) -> Option<EvaluateResultValue>,
) -> Option<EvaluateResultValue> {
  match elements.get(slot) {
    Some(element) => read(element),
    None => Some(js_undefined()),
  }
}

/// The value a folded function map's object form carries under `key`.
///
/// Reads the object `function_fold_to_object` just built rather than an object
/// an author wrote, which is why it is not the general object reader above: no
/// spread, getter, setter or computed key can appear in it, so there is no
/// shape here that has to refuse.
fn fold_entry_value(object: &ObjectLit, key: &str) -> Option<Expr> {
  object.props.iter().find_map(|prop| match prop {
    PropOrSpread::Prop(prop) => match prop.as_ref() {
      Prop::KeyValue(key_value) if convert_key_value_to_str(key_value) == key => {
        Some(*key_value.value.clone())
      },
      _ => None,
    },
    PropOrSpread::Spread(_) => None,
  })
}

/// A member read off a value whose only form is the object a fold stands for.
///
/// The reference implementation's `identifiers` entry *is* a JavaScript object,
/// so a member read off it resolves a key the entry carries and reads
/// `undefined` off one it does not -- and `undefined` is what the position that
/// wanted a value then refuses, with the sentence that names the input. This
/// evaluator holds the entry as a configuration with no expression form, and
/// used to report that it could not name the property, which is a sentence
/// about this compiler rather than about what was written.
fn read_fold_member(
  value: &EvaluateResultValue,
  property: Option<&EvaluateResultValue>,
  path: &Expr,
  state: &mut EvaluationState,
) -> Option<EvaluateResultValue> {
  let Some(object) = function_fold_to_object(value) else {
    deopt_unsupported!(path, state, UNEXPECTED_MEMBER_LOOKUP);
  };

  let Some(key) = property.and_then(|prop| prop.as_string_key()) else {
    deopt_unsupported!(path, state, UNEXPECTED_MEMBER_LOOKUP);
  };

  Some(match fold_entry_value(&object, &key) {
    Some(expr) => EvaluateResultValue::Expr(expr),
    None => js_undefined(),
  })
}

/// Refuses a lookup the receiver cannot answer, naming the index where the
/// evaluator could read one.
///
/// A string is the only receiver that still refuses an index -- both array
/// receivers read one -- so what this keeps single is the *wording*: `"ab"[0]`
/// and `"ab".length` name what was asked for rather than describing the member
/// expression. The unnameable case is the refusal the reference implementation
/// gives at this point, `errMsgs.UNEXPECTED_MEMBER_LOOKUP`.
///
/// Reads what was asked for off the classification rather than re-deriving it
/// from the property: deciding what a lookup asks for is `classify_lookup`'s
/// job, and asking twice is how the two could come to disagree.
fn refuse_lookup(
  path: &Expr,
  state: &mut EvaluationState,
  lookup: &ArrayLikeLookup,
) -> Option<EvaluateResultValue> {
  match lookup {
    ArrayLikeLookup::Length => deopt(path, state, &unreadable_index(LENGTH)),
    ArrayLikeLookup::Index(slot) => deopt(path, state, &unreadable_index(&slot.to_string())),
    ArrayLikeLookup::Missing(key) => deopt(path, state, &unreadable_index(key)),
    ArrayLikeLookup::Unreadable => deopt(path, state, UNEXPECTED_MEMBER_LOOKUP),
  }
}

/// The number of slots an array literal writes, or `None` where the literal
/// cannot be counted from its own elements.
///
/// A hole is what makes the count come from the AST rather than from the
/// evaluated elements: it occupies a slot and has no value, so `[, 1]` writes
/// two slots and does not evaluate at all — `array_expression` refuses it rather
/// than answering one element short of what was written.
///
/// The spread arm is a guard rather than a live path. A spread is one element
/// standing for however many the spread value holds, so neither count is the
/// language's answer — but evaluating the array refuses every spread first, so
/// no receiver carrying one reaches here. Kept because the cost is a bounds
/// check and the alternative, if that order ever changes, is a number that is
/// confidently wrong.
fn written_slot_count(elems: &[Option<ExprOrSpread>]) -> Option<usize> {
  match elems.iter().flatten().any(|elem| elem.spread.is_some()) {
    true => None,
    false => Some(elems.len()),
  }
}

/// The slot count of an evaluated array, read from the receiver's own literal
/// where it has one.
///
/// Two sources, because the evaluated elements need not be the slots: a fold's
/// own array output can carry a hole where the source wrote none. A literal
/// receiver is therefore counted as written, and a receiver reached any other
/// way — a binding, a fold — is counted by its elements.
///
/// A receiver written with a hole never arrives here: the array refuses to fold,
/// so `holey_receiver_length` answers it from the source ahead of the receiver
/// being evaluated at all. That is the only reading of a hole, and both this
/// function and that one take it — a slot the source wrote counts, whether or
/// not anything filled it.
///
/// The receiver is unwrapped before it is asked, because a parenthesis is not a
/// different receiver: `([1, 2]).length` reaches the evaluated count otherwise,
/// which is the same number here and need not stay so.
fn written_slot_count_of(obj: &Expr, items: &[EvaluateResultValue]) -> Option<usize> {
  match normalize_expr(obj).as_array() {
    Some(ArrayLit { elems, .. }) => written_slot_count(elems),
    None => Some(items.len()),
  }
}

/// Whether a member lookup names `length` in the source, without evaluating
/// anything.
///
/// The name is read off the AST rather than off an evaluated property, because
/// the one caller runs before the receiver is evaluated and evaluating the key
/// there would report the key's refusal in place of the receiver's. Read through
/// the same function every other AST-level key read uses, so the spellings the
/// language has for one property -- `.length`, `["length"]`, and a template that
/// folds to it -- cannot come to be a shorter set here than there. Four private
/// copies of a property test is how `classify_lookup` above came to exist.
fn is_written_length(prop: &MemberProp) -> bool {
  convert_member_prop_to_string(prop).as_deref() == Some(LENGTH)
}

/// The elements of a receiver written as an array literal carrying a hole, or
/// `None` for every other receiver.
///
/// A hole makes the array itself unfoldable -- it is a slot with no value, so
/// `array_expression` refuses the array rather than answering one short of what
/// was written. What survives the refusal is the slot count, because counting
/// never needed the values: `[, 1].length` is two in the language, and the
/// source is the only place left that says so.
///
/// Narrow on purpose: an array with no hole folds, so its count is answered by
/// the arms below from its evaluated form, and only a receiver those arms can no
/// longer reach is read here.
fn holey_receiver_elems(obj: &Expr) -> Option<&[Option<ExprOrSpread>]> {
  let ArrayLit { elems, .. } = normalize_expr(obj).as_array()?;

  elems
    .iter()
    .any(Option::is_none)
    .then_some(elems.as_slice())
}

pub(in super::super) fn evaluate(
  member: &MemberExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  let path = Expr::Member(member.clone());
  let path = &path;
  let parent_is_call_expr = traversal_state.is_member_call_callee(member);

  let evaluated_value = if parent_is_call_expr {
    None
  } else {
    // A hole is answered from the source, ahead of a receiver that will refuse
    // for it. A spread still refuses, through the same `written_slot_count` the
    // arms below use -- one written element standing for however many the spread
    // holds is not a count either reading can give.
    if let Some(elems) = holey_receiver_elems(&member.obj)
      && is_written_length(&member.prop)
    {
      return match written_slot_count(elems) {
        Some(count) => Some(EvaluateResultValue::Expr(create_number_expr(count as f64))),
        None => deopt(path, state, SPREAD_ELEMENT),
      };
    }

    // ThemeRef fast-path. Only run for member chains whose base is a plain
    // identifier — the only shape that can resolve to a `ThemeRef` (either a
    // local `ThemeRefMapper` registered in `fns.identifiers` or a cross-file
    // `*.stylex.js` import). Skipping computed / call / object bases avoids
    // paying for a speculative `evaluate_cached` that can never produce a
    // ThemeRef and may early-deopt via `state.confident` for unrelated deep
    // member accesses.
    if let Some((base_path, parts)) = get_full_member_path(member)
      && is_theme_ref_base(&base_path)
    {
      let base_object = evaluate_cached(&base_path, state, traversal_state, fns);

      if !state.confident {
        return None;
      }

      if let Some(EvaluateResultValue::ThemeRef(mut theme_ref)) = base_object {
        let value = theme_ref.get(&parts.join("."), traversal_state);

        let Some(css_var) = value.as_css_var() else {
          deopt_unsupported!(path, state, EXPECTED_CSS_VAR);
        };

        return Some(EvaluateResultValue::Expr(create_string_expr(css_var)));
      }
    }

    evaluate_cached(&member.obj, state, traversal_state, fns)
  };
  match evaluated_value {
    Some(object) => {
      if !state.confident {
        return None;
      }

      let prop_path = &member.prop;

      let property = match prop_path {
        MemberProp::Ident(ident) => Some(EvaluateResultValue::Expr(Expr::from(ident.clone()))),
        MemberProp::Computed(ComputedPropName { expr, .. }) => {
          let result = evaluate_cached(expr, state, traversal_state, fns);

          if !state.confident {
            return None;
          }

          result
        },
        MemberProp::PrivateName(_) => {
          return deopt(path, state, UNEXPECTED_MEMBER_LOOKUP);
        },
      };

      match object {
        EvaluateResultValue::Expr(expr) => match &expr {
          Expr::Array(ArrayLit { elems, .. }) => {
            let Some(eval_res) = property else {
              deopt_unsupported!(path, state, PROPERTY_NOT_FOUND);
            };

            let lookup = classify_lookup(Some(&eval_res));

            match &lookup {
              // The count of slots the language reports — a hole occupies one.
              // Read off the array this value holds, which is the same reading
              // the `Vec` arm below takes from the receiver's AST and for the
              // same reason: an evaluated array has already dropped its holes.
              ArrayLikeLookup::Length => {
                return match written_slot_count(elems) {
                  Some(count) => Some(EvaluateResultValue::Expr(create_number_expr(count as f64))),
                  None => deopt(path, state, SPREAD_ELEMENT),
                };
              },
              // A property an array does not carry is `undefined`, the answer
              // the language gives and the one the object arm below gives for
              // the matching case.
              ArrayLikeLookup::Missing(_) => return Some(js_undefined()),
              ArrayLikeLookup::Unreadable => {
                deopt_unsupported!(path, state, UNEXPECTED_MEMBER_LOOKUP);
              },
              // Read below, which is the arm that folds one.
              ArrayLikeLookup::Index(_) => {},
            }

            let ArrayLikeLookup::Index(slot) = lookup else {
              return refuse_lookup(path, state, &lookup);
            };

            // A spread stands for however many elements its value holds, so no
            // slot after it is the one the source names. Refused rather than
            // counted, for the reason `written_slot_count` gives — though
            // evaluating an array refuses every spread first, so no receiver
            // carrying one arrives here.
            if written_slot_count(elems).is_none() {
              deopt_unsupported!(path, state, SPREAD_ELEMENT);
            }

            index_answer(elems, slot, |element| {
              // An array hole is `undefined` in the language, but this
              // evaluator does not hold one: `array_expression` refuses a
              // written hole ahead of this, and only a fold's own output can
              // carry one. Refused rather than answered, because a hole
              // reaching here would mean a fold produced a slot it could not
              // fill, and answering `undefined` would hide that.
              let Some(element) = element.as_ref() else {
                deopt_unsupported!(path, state, MEMBER_NOT_RESOLVED);
              };

              Some(EvaluateResultValue::Expr(*element.expr.clone()))
            })
          },
          Expr::Object(ObjectLit { props, .. }) => {
            let Some(eval_res) = property else {
              deopt_unsupported!(path, state, PROPERTY_NOT_FOUND);
            };

            let ident = match eval_res {
              EvaluateResultValue::Expr(ident) => ident,
              EvaluateResultValue::ThemeRef(theme) => {
                // NOTE: it's a very edge case, but it's possible to have a theme ref as a key
                // in an object, when theme import key is same as other variable name.
                // One of the reasons is code minification or obfuscation,
                // when theme import key is renamed to a shorter name.
                // Also it may be a result of a bug in the code.

                warn!(
                  "A theme import key is being used as an object key. This might be caused by code minification or an internal error.\r\nFor additional details, please recompile using debug mode."
                );

                debug!("Evaluating member access on object:");
                debug!("Object expression: {:?}", expr);
                debug!("Theme reference: {:?}", theme);
                debug!("Original property: {:?}", prop_path);

                return deopt(path, state, THEME_IMPORT_KEY_AS_OBJECT_KEY);
              },
              _ => {
                debug!("Property not found for expression: {:?}", expr);
                debug!("Evaluation result: {:?}", eval_res);
                debug!("Original property: {:?}", prop_path);

                deopt_unsupported!(path, state, PROPERTY_NOT_FOUND);
              },
            };

            let normalized_ident = normalize_expr(&ident);

            let ident_string_name = match normalized_ident {
              Expr::Ident(ident) => ident.sym.to_string(),
              // A regex or a BigInt key has no string form the evaluator
              // reads, and a key that is still an expression never resolved.
              Expr::Lit(lit) => match convert_lit_to_string(lit) {
                Some(key) => key,
                None => deopt_unsupported!(path, state, UNEXPECTED_MEMBER_LOOKUP),
              },
              _ => deopt_unsupported!(path, state, UNEXPECTED_MEMBER_LOOKUP),
            };

            // Written as a loop rather than a `find`, because a property the
            // evaluator cannot read has to refuse the whole lookup and a
            // predicate has no way to say so — the closure would have to
            // abort, which is the failure this split exists to remove.
            let mut found = None;

            for prop in props {
              let PropOrSpread::Prop(prop) = prop else {
                // A spread leaves the object's own keys unknown, so a key that
                // is not among the literal ones cannot be called absent.
                deopt_unsupported!(path, state, SPREAD_HIDES_OBJECT_KEYS);
              };

              let mut prop = prop.clone();

              expand_shorthand_prop(&mut prop);

              // A getter, a setter or a method carries no value to read.
              let Prop::KeyValue(key_value) = prop.as_ref() else {
                deopt_unsupported!(path, state, OBJECT_METHOD);
              };

              if ident_string_name == convert_key_value_to_str(key_value) {
                found = Some(key_value.value.clone());
                break;
              }
            }

            // A key the object does not carry reads as `undefined`, which is a
            // value the evaluator is confident about rather than one it failed
            // to resolve. Returning it is what lets `token.missing ?? fallback`
            // fold, where a deopt here would send the whole declaration to the
            // runtime.
            let Some(value) = found else {
              return Some(js_undefined());
            };

            Some(EvaluateResultValue::Expr(*value))
          },
          Expr::Member(member_expr) => evaluate_cached(
            &Expr::Member(member_expr.clone()),
            state,
            traversal_state,
            fns,
          ),
          // A string answers its length, in the UTF-16 code units the
          // language counts — `"\u{1F600}a".length` is 3, not 2.
          //
          // This arm used to read every literal receiver by re-evaluating it
          // and dropping the property, so `"abc".length` folded to `"abc"` and
          // shipped `content: "abc"`. A wrong value is worse than a refusal:
          // nothing errors, and the stylesheet is simply not what the source
          // says.
          //
          // An index is the one lookup here that refuses rather than answering.
          // The reference implementation folds `"\u{1F600}"[0]` to a lone
          // surrogate, which no Rust string can hold, so answering it would be
          // the same class of quietly-wrong value this arm stopped producing.
          Expr::Lit(Lit::Str(strng)) => match classify_lookup(property.as_ref()) {
            ArrayLikeLookup::Length => Some(EvaluateResultValue::Expr(create_number_expr(
              atom_utf16_length(&strng.value) as f64,
            ))),
            ArrayLikeLookup::Missing(_) => Some(js_undefined()),
            lookup @ (ArrayLikeLookup::Index(_) | ArrayLikeLookup::Unreadable) => {
              refuse_lookup(path, state, &lookup)
            },
          },
          // Reading a property off `undefined` throws in the language, and
          // `undefined` is what a property the receiver does not carry now
          // answers — so `"abc".foo.length` has to refuse rather than answer
          // `undefined` a second time. Checked here rather than in the arms
          // that produce it, because this is the arm that would swallow it.
          Expr::Ident(nested_ident) if is_js_undefined(nested_ident) => {
            deopt(path, state, UNEXPECTED_MEMBER_LOOKUP)
          },
          Expr::Ident(nested_ident) => evaluate_cached(
            &Expr::Ident(nested_ident.clone()),
            state,
            traversal_state,
            fns,
          ),
          // A member access on a call, an arrow, a class — expression kinds
          // this evaluator reads no properties from.
          _ => deopt_unsupported!(
            path,
            state,
            &unsupported_expression(get_expr_node_kind(&expr))
          ),
        },
        EvaluateResultValue::FunctionConfigMap(fc_map) => {
          // The name the member expression asks for, read once and through the
          // same reading the fold below takes. Three readers ask for this key,
          // and a spelling only one of them recognised would answer a different
          // value from the others: `stylex["when"]` has to resolve the entry
          // `stylex.when` does, not the object the fold stands for.
          let name = property.as_ref().and_then(|prop| prop.as_string_key());

          // The entry the map carries, in the map's own form. `stylex.when` as
          // a callee is read through this, so a hit must not be materialized
          // into an object the call step cannot call.
          //
          // Each variant answers as the value it stands for, which is the same
          // reading `nodes/identifier.rs` gives the same entry reached without a
          // member step. The two have to agree: `stylex.env` and a bare `env`
          // from a named import are one object in the language.
          if let Some(name) = &name
            && let Some(entry) = fc_map.get(&Atom::from(name.as_str()))
          {
            match entry {
              FunctionConfigType::Regular(config) => {
                return Some(EvaluateResultValue::FunctionConfig(config.clone()));
              },
              FunctionConfigType::EnvObject(env_map) => {
                // The name is a key of the namespace however the compiler is
                // configured, so an unset option is reported as the option
                // being unset rather than as a property nobody can find --
                // which would send an author looking in their source.
                if env_map.is_empty() {
                  deopt_unsupported!(
                    path,
                    state,
                    "The stylex.env object is not configured. Check that the 'env' option is set in your StyleX configuration."
                  );
                }

                return Some(EvaluateResultValue::EnvObject(env_map.clone()));
              },
              FunctionConfigType::Map(nested) => {
                return Some(EvaluateResultValue::FunctionConfigMap(nested.clone()));
              },
              // `defaultMarker`'s shape, which reaches the namespace through
              // `member_expressions` rather than through this map and so has no
              // entry here today. Spelled out rather than left to a catch-all
              // for the reason `nodes/identifier.rs` spells its own out: a
              // shape that starts arriving here should be decided, not silently
              // answered as whichever neighbour compiled.
              FunctionConfigType::IndexMap(_) => {
                deopt_unsupported!(path, state, &unsupported_expression("IndexMap"));
              },
            }
          }

          // A key the map has no entry for is `undefined`, read off the object
          // the fold stands for, rather than a report that this compiler could
          // not name the property.
          read_fold_member(
            &EvaluateResultValue::FunctionConfigMap(fc_map),
            property.as_ref(),
            path,
            state,
          )
        },
        // One entry of the same map, reached through a named import rather
        // than through the namespace. `keyframes.fn` is the one key the entry
        // carries and every other name is `undefined` -- both of which the
        // position that wanted a value refuses, which is what the reference
        // implementation does with the object it holds there.
        EvaluateResultValue::FunctionConfig(_) => {
          read_fold_member(&object, property.as_ref(), path, state)
        },
        // An array literal evaluates to this variant rather than to an
        // `ArrayLit`, so it is where `["a", "b"].length` is answered. Only
        // `length`: an index refuses, which is what it did before this arm
        // existed, and folding one is the separate scope the array-literal arm
        // above notes.
        //
        // The count comes from the written slots where the receiver is a
        // literal, and from the evaluated elements otherwise. A receiver written
        // with a hole reaches neither reading: the array refuses to fold, and
        // `holey_receiver_length` has already answered its count from the
        // source — including through a binding, where the refusal travels with
        // the value and no short count is answered.
        EvaluateResultValue::Vec(items) => match classify_lookup(property.as_ref()) {
          ArrayLikeLookup::Length => match written_slot_count_of(&member.obj, &items) {
            Some(count) => Some(EvaluateResultValue::Expr(create_number_expr(count as f64))),
            None => deopt(path, state, SPREAD_ELEMENT),
          },
          // An index reads the element it names, and answers `undefined` past
          // the end. The slots are counted from the receiver first, so an
          // index is read only where the count is the language's — the same
          // guard `length` above takes, and for the same reason.
          ArrayLikeLookup::Index(slot) => match written_slot_count_of(&member.obj, &items) {
            None => deopt(path, state, SPREAD_ELEMENT),
            Some(_) => index_answer(&items, slot, |item| Some(item.clone())),
          },
          ArrayLikeLookup::Missing(_) => Some(js_undefined()),
          lookup @ ArrayLikeLookup::Unreadable => refuse_lookup(path, state, &lookup),
        },
        EvaluateResultValue::ThemeRef(mut theme_ref) => {
          let key = match property {
            Some(EvaluateResultValue::Expr(Expr::Ident(Ident { sym, .. }))) => sym.to_string(),
            Some(EvaluateResultValue::Expr(Expr::Lit(lit))) => match convert_lit_to_string(&lit) {
              Some(key) => key,
              None => deopt_unsupported!(path, state, UNEXPECTED_MEMBER_LOOKUP),
            },
            _ => deopt_unsupported!(path, state, MEMBER_NOT_RESOLVED),
          };

          let value = theme_ref.get(&key, traversal_state);

          let Some(css_var) = value.as_css_var() else {
            deopt_unsupported!(path, state, EXPECTED_CSS_VAR);
          };

          Some(EvaluateResultValue::Expr(create_string_expr(css_var)))
        },
        EvaluateResultValue::EnvObject(env_map) => {
          let Some(key) = property.as_ref().and_then(|prop| prop.as_string_key()) else {
            deopt_unsupported!(path, state, UNEXPECTED_MEMBER_LOOKUP);
          };

          let Some(entry) = env_map.get(&key) else {
            deopt_unsupported!(
              path,
              state,
              format!(
                "The property '{}' was not found in the stylex.env configuration.",
                key
              )
              .as_str()
            );
          };

          match resolve_env_entry_to_result(entry, &env_map) {
            Some(result) => Some(result),
            None => deopt_unsupported!(path, state, ILLEGAL_PROP_VALUE),
          }
        },
        // An evaluated value the member path reads no properties from: a
        // callback, an entries map, a raw function configuration.
        _ => deopt_unsupported!(path, state, UNEXPECTED_MEMBER_LOOKUP),
      }
    },
    _ => None,
  }
}

fn get_full_member_path(member: &MemberExpr) -> Option<(Expr, Vec<String>)> {
  let mut parts = Vec::new();
  let mut current = member;

  loop {
    parts.insert(0, convert_member_prop_to_string(&current.prop)?);

    match current.obj.as_ref() {
      Expr::Member(member) => {
        current = member;
      },
      base => {
        if parts.len() < 2 {
          return None;
        }

        return Some((base.clone(), parts));
      },
    }
  }
}

/// Returns `true` when `base` is a plain identifier — the only shape that can
/// resolve to a `ThemeRef` in our evaluator (either via `fns.identifiers` for
/// in-file `defineVars` exports, or via cross-file `*.stylex.js` imports
/// handled in `evaluate::mod`). Any other expression kind (`Member`, `Call`,
/// `Object`, `Array`, …) is guaranteed not to produce a `ThemeRef`, so we
/// skip the fast-path eval to avoid the speculative work the Copilot review
/// flagged.
fn is_theme_ref_base(base: &Expr) -> bool {
  matches!(base, Expr::Ident(_))
}
