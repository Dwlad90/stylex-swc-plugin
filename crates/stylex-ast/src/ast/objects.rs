//! Operations on the own properties of an object literal.
//!
//! An object literal is a list of properties, and three questions get asked of
//! that list often enough to be answered once: what order does the language
//! enumerate it in, which properties does a later one replace, and what does
//! spreading one object into another produce. Every answer turns on which key a
//! property declares, so that reading is stated once here too.

use rustc_hash::{FxHashMap, FxHashSet};
use stylex_utils::number::to_js_string;
use swc_core::atoms::Atom;
use swc_core::ecma::ast::{Prop, PropName, PropOrSpread};

use super::convertors::convert_wtf8_to_atom;

/// The name a property declares, or `None` where it declares no nameable one.
///
/// `None` is a spread, a getter, a setter, a method, or a computed key -- each
/// of which either has no key at all or has one this cannot read without
/// evaluating it. Stated once because three readers depend on the answer, and
/// three spellings of "which key is this" would let them disagree about whether
/// two properties collide.
fn prop_key(prop: &PropOrSpread) -> Option<Atom> {
  match prop {
    PropOrSpread::Prop(prop) => match prop.as_ref() {
      Prop::Shorthand(ident) => Some(ident.sym.clone()),
      Prop::KeyValue(key_val) => match &key_val.key {
        PropName::Ident(ident) => Some(ident.sym.clone()),
        PropName::Str(strng) => Some(convert_wtf8_to_atom(&strng.value)),
        // A numeric key is a string key spelled as a number: `{ 42: x }` and
        // `{ '42': x }` name one property in the language, so they have to read
        // as one name here. Rendered through `to_js_string` so the spelling is
        // the language's -- `1e21`, not `1000000000000000000000`.
        PropName::Num(number) => Some(Atom::from(to_js_string(number.value))),
        // `{ 1n: x }` names the property `"1"`, as every non-computed key that
        // is not already a string does.
        PropName::BigInt(big_int) => Some(Atom::from(big_int.value.to_string())),
        _ => None,
      },
      _ => None,
    },
    _ => None,
  }
}

/// Whether a key is an array index, in the sense JavaScript uses to decide
/// enumeration order: the canonical decimal spelling of an integer below
/// 2^32 - 1.
///
/// Canonical is what makes `"0"` one and `"00"`, `"+0"` and `"01"` not — those
/// round-trip to a different string, so the language treats them as ordinary
/// string keys and enumerates them in insertion order.
fn array_index_of(key: &str) -> Option<u32> {
  // Only digits, so a signed spelling such as `+0` stays a string key -- Rust
  // reads the sign that JavaScript does not. This also settles every ordinary
  // property name at its first byte, before any parse runs.
  if key.is_empty() || !key.bytes().all(|byte| byte.is_ascii_digit()) {
    return None;
  }

  if key.len() > 1 && key.starts_with('0') {
    return None;
  }

  let index = key.parse::<u32>().ok()?;

  match index == u32::MAX {
    true => None,
    false => Some(index),
  }
}

/// Reorders an object's own properties the way JavaScript enumerates them:
/// every array-index key first in ascending numeric order, then every other key
/// in insertion order.
///
/// Not a detail of the object literal's spelling. The order properties come out
/// in is the order their declarations reach the stylesheet, so it decides which
/// of two rules at equal specificity wins -- and `{ color: 'red', ...['a'] }`
/// was emitting `color` before `0` where the language, and so upstream, puts
/// `0` first.
///
/// Stable within each group, so the insertion order of the string keys is
/// preserved exactly.
pub fn order_own_keys(props: Vec<PropOrSpread>) -> Vec<PropOrSpread> {
  let mut indexed: Vec<(u32, PropOrSpread)> = Vec::new();
  let mut named: Vec<PropOrSpread> = Vec::with_capacity(props.len());

  for prop in props {
    match prop_key(&prop).as_deref().and_then(array_index_of) {
      Some(index) => indexed.push((index, prop)),
      None => named.push(prop),
    }
  }

  if indexed.is_empty() {
    return named;
  }

  indexed.sort_by_key(|(index, _)| *index);

  let mut ordered = Vec::with_capacity(indexed.len() + named.len());
  ordered.extend(indexed.into_iter().map(|(_, prop)| prop));
  ordered.extend(named);

  ordered
}

/// Keeps the last property declared under each name, in the position that name
/// last took, and drops every property that declares no readable name.
pub fn remove_duplicates(props: Vec<PropOrSpread>) -> Vec<PropOrSpread> {
  let mut set = FxHashSet::default();
  let mut result = Vec::with_capacity(props.len());

  for prop in props.into_iter().rev() {
    let Some(key) = prop_key(&prop) else {
      continue;
    };

    if set.insert(key) {
      result.push(prop);
    }
  }

  result.reverse();

  result
}

/// The properties of `{ ...old_props, ...new_props }`.
///
/// This is `Object.assign`, which is what the reference implementation calls at
/// the spread and so is what "merge" has to mean here: shallow. A repeated key
/// takes the later value and keeps the position the key first took, so
/// `{ ...{ a: 1, b: 2 }, ...{ a: 3 } }` is `{ a: 3, b: 2 }` -- `a` is third in
/// no ordering.
///
/// Not a deep merge, despite what this used to be called: `{ ...{ a: { x: 1 } },
/// ...{ a: { y: 2 } } }` is `{ a: { y: 2 } }` in the language, and the nested
/// `x` is gone rather than merged in. Nesting is what a style object is mostly
/// made of -- a pseudo, an at-rule -- so a deep merge here quietly kept
/// declarations the source had replaced.
///
/// It also used to reverse. The result was assembled by pushing the old
/// properties onto the new and reversing the whole thing, which put the groups
/// in the right order and every property inside them in the wrong one, so
/// `{ ...{ color: 'red', opacity: 1 } }` emitted its two rules back to front.
/// One property is the common case and hid it.
pub fn assign_props(
  old_props: Vec<PropOrSpread>,
  new_props: Vec<PropOrSpread>,
) -> Vec<PropOrSpread> {
  let mut merged: Vec<PropOrSpread> = Vec::with_capacity(old_props.len() + new_props.len());
  // The position each key already took, so a repeated one is found by lookup
  // rather than by re-reading every property placed so far -- which was
  // quadratic, and cloned an `Atom` per comparison, on the path every object
  // spread goes through.
  let mut positions: FxHashMap<Atom, usize> = FxHashMap::default();

  for prop in old_props.into_iter().chain(new_props) {
    let Some(key) = prop_key(&prop) else {
      // A spread, a getter, a computed key: nothing here can say which key it
      // would land on, so it keeps its place and collides with nothing.
      merged.push(prop);
      continue;
    };

    match positions.get(&key) {
      Some(&position) => merged[position] = prop,
      None => {
        positions.insert(key, merged.len());
        merged.push(prop);
      },
    }
  }

  merged
}
