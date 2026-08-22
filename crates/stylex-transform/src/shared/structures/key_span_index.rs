use std::{cmp::Reverse, rc::Rc};

use rustc_hash::{FxHashMap, FxHashSet};
use swc_core::{
  atoms::Atom,
  common::{BytePos, DUMMY_SP, Span, Spanned},
  ecma::{
    ast::{CallExpr, Expr, Module, ObjectLit},
    visit::{Visit, VisitWith, noop_visit_type},
  },
};

use crate::shared::utils::ast::helpers::{
  collect_object_lit_keys, namespace_name_from_prop_key, prop_as_key_value,
};

/// Every authored position a style namespace key could be resolved to, in the
/// parsed source of one module, collected in a single walk.
///
/// The debug path asks for the authored position of *every* namespace of every
/// `stylex.create` call in the file, and the question is always the same shape:
/// which object literal in the source spells this key. Answering it by walking
/// the module per namespace is `O(namespaces x file size)`, which is quadratic
/// in the size of a file that is one long list of styles -- and that is what a
/// `dev` build of one spends most of its time doing. One walk builds this
/// instead, and each lookup becomes a hash hit followed by a comparison of the
/// handful of candidates that actually spell the key.
///
/// Built from the memoized parsed source and discarded with it, so a candidate
/// span always belongs to the module the caller is resolving against.
#[derive(Clone, Debug, Default)]
pub(crate) struct KeySpanIndex {
  by_key: FxHashMap<Atom, Vec<IndexedCandidate>>,
}

/// One object literal that spells a given key: where the key is written, and
/// what stands beside it for disambiguation.
#[derive(Clone, Debug)]
struct IndexedCandidate {
  /// The span of the key itself -- the answer a lookup returns.
  span: Span,
  /// The keys of the namespace's own value object, empty when the value is not
  /// an object literal.
  namespace_value_keys: Vec<Atom>,
  /// Every namespace key of the containing object argument, shared between that
  /// object's candidates because they all rank against the same siblings.
  sibling_keys: Rc<Vec<Atom>>,
  /// Where the candidate's call is written, for the distance tie-break.
  candidate_lo: BytePos,
}

impl KeySpanIndex {
  pub(crate) fn build(module: &Module) -> Self {
    let mut index = Self::default();

    module.visit_with(&mut index);

    index
  }

  /// The authored span the `query` names, or [`DUMMY_SP`] when no candidate
  /// spells its key or when two candidates are equally good.
  ///
  /// Two namespaces in one file can spell the same key, so a match on the key
  /// alone is not an answer. Candidates are ranked by how much of the compiled
  /// call they reproduce -- see [`CandidateRank`] -- and a tie is refused rather
  /// than guessed, because a wrong `file:line` is worse than none.
  pub(crate) fn resolve(&self, query: &NamespaceKeyQuery) -> Span {
    // Interned rather than borrowed: an `Atom` key cannot be looked up by
    // `&str`, and interning one name is nothing against the walk this replaces.
    let candidates = match self.by_key.get(&Atom::from(query.namespace_key)) {
      Some(candidates) => candidates,
      None => return DUMMY_SP,
    };

    let mut best: Option<(CandidateRank, Span)> = None;
    let mut ambiguous = false;

    for candidate in candidates {
      let rank = candidate.rank(query);

      match best.as_ref() {
        None => {
          best = Some((rank, candidate.span));
          ambiguous = false;
        },
        Some((best_rank, _)) if rank > *best_rank => {
          best = Some((rank, candidate.span));
          ambiguous = false;
        },
        Some((best_rank, _)) if rank == *best_rank => {
          ambiguous = true;
        },
        Some(_) => {},
      }
    }

    if ambiguous {
      return DUMMY_SP;
    }

    best.map_or(DUMMY_SP, |(_, span)| span)
  }

  /// Records every namespace key of one call's object argument.
  ///
  /// The candidate a key resolves to is its *last* occurrence in the object,
  /// which is the property a runtime object literal would keep, and one
  /// candidate per (object, key) pair rather than per property -- so a key
  /// written twice in one object is not read as two objects disagreeing about
  /// where it lives.
  ///
  /// Last occurrence for the value keys too, and that is a deliberate change
  /// from the walk this replaces: that one moved its answer to the later
  /// property but kept the earlier property's value keys when the later value
  /// was not an object literal, so `{ root: { color: 'red' }, root: someVar }`
  /// ranked as though `root` still spelled `color`. The surviving property is
  /// the one the compiled call was built from, so it is the one whose value can
  /// be compared against it.
  fn index_object(&mut self, call: &CallExpr, object: &ObjectLit) {
    let sibling_keys = Rc::new(collect_object_lit_keys(object).collect::<Vec<_>>());

    if sibling_keys.is_empty() {
      return;
    }

    let candidate_lo = object_lo(object).unwrap_or(call.span.lo);
    let mut in_this_object: FxHashMap<Atom, IndexedCandidate> = FxHashMap::default();

    for prop in &object.props {
      if let Some(key_value) = prop_as_key_value(prop)
        && let Some(name) = namespace_name_from_prop_key(&key_value.key)
      {
        in_this_object.insert(
          name,
          IndexedCandidate {
            span: key_value.key.span(),
            namespace_value_keys: object_lit_keys(&key_value.value).collect(),
            sibling_keys: Rc::clone(&sibling_keys),
            candidate_lo,
          },
        );
      }
    }

    // Iterated out of a hash map, and each candidate list is still in a
    // deterministic order: `in_this_object` is keyed by name, so one object
    // contributes at most one candidate per name and the shuffled order is the
    // order *different* names are appended to *different* lists in. Within one
    // name the order is the visit order, which is the source order.
    //
    // What that buys is that `resolve` may rank candidates without a tiebreak
    // on position. It does not rely on it -- a tie there sets `ambiguous` and a
    // strict improvement clears it, in any order -- so keep it that way: a
    // "first best wins" shortcut would make the answer depend on this order,
    // and this order is only stable per name, not across the map.
    for (name, candidate) in in_this_object {
      self.by_key.entry(name).or_default().push(candidate);
    }
  }
}

impl Visit for KeySpanIndex {
  noop_visit_type!();

  fn visit_call_expr(&mut self, call: &CallExpr) {
    if let Some(object) = first_object_arg(call) {
      self.index_object(call, object);
    }

    call.visit_children_with(self);
  }
}

/// How well a candidate matches the namespace being placed, higher being
/// better.
///
/// The ordering is derived, so the field order *is* the precedence: how much of
/// the namespace's own value the candidate reproduces, then how many of the
/// call's other namespace keys it spells, then how close it is written to the
/// compiled call. The distance is `Reverse`d so a nearer candidate wins, which
/// also means no measured distance outranks every measured one -- a call with no
/// position of its own cannot be placed, so it cannot be placed badly either.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct CandidateRank {
  pub(crate) namespace_value_overlap: usize,
  pub(crate) sibling_overlap: usize,
  pub(crate) distance_from_target: Reverse<Option<u32>>,
}

impl IndexedCandidate {
  fn rank(&self, query: &NamespaceKeyQuery) -> CandidateRank {
    CandidateRank {
      namespace_value_overlap: overlap(&self.namespace_value_keys, &query.namespace_value_keys),
      sibling_overlap: overlap(&self.sibling_keys, &query.sibling_keys),
      distance_from_target: Reverse(
        query
          .target_lo
          .map(|target_lo| self.candidate_lo.0.abs_diff(target_lo.0)),
      ),
    }
  }
}

/// How many of `authored` the compiled call also spells. A key written twice is
/// counted twice, because it really is two properties.
fn overlap(authored: &[Atom], compiled: &FxHashSet<Atom>) -> usize {
  authored
    .iter()
    .filter(|name| compiled.contains(*name))
    .count()
}

/// One namespace of one compiled `stylex.create` call, described the way the
/// index ranks candidates against: the key to find, what else the call spells,
/// and where the call itself sits.
///
/// Read from the *compiled* call, which is why none of it can be taken for
/// granted: shorthand expansion has already rewritten the values, and a
/// synthesized call carries no position at all.
#[derive(Clone, Debug)]
pub(crate) struct NamespaceKeyQuery<'a> {
  pub(crate) namespace_key: &'a str,
  /// The namespace keys of the call's object argument, this one included.
  pub(crate) sibling_keys: Rc<FxHashSet<Atom>>,
  /// The keys of this namespace's own value object.
  pub(crate) namespace_value_keys: FxHashSet<Atom>,
  /// Where the call's object argument starts, for the proximity tie-break.
  pub(crate) target_lo: Option<BytePos>,
}

/// The half of a lookup that belongs to the *call* rather than to one of its
/// namespaces.
///
/// Every namespace of one `stylex.create` shares its sibling keys and its
/// proximity anchor, and the debug path asks for a span once per namespace — so
/// building them per namespace made each call quadratic in its own namespace
/// count, on top of the walk the index already replaced. Built once per call and
/// borrowed by each lookup instead.
pub(crate) struct CallKeys {
  /// The namespace keys of the call's object argument.
  pub(crate) sibling_keys: Rc<FxHashSet<Atom>>,
  /// Where the call's object argument starts, for the proximity tie-break.
  pub(crate) target_lo: Option<BytePos>,
}

impl CallKeys {
  pub(crate) fn from_call(call_expr: &CallExpr) -> Self {
    let object_arg = first_object_arg(call_expr);

    Self {
      sibling_keys: Rc::new(
        object_arg
          .map(|object| collect_object_lit_keys(object).collect())
          .unwrap_or_default(),
      ),
      target_lo: object_arg
        .and_then(object_lo)
        .or_else(|| (!call_expr.span.is_dummy()).then_some(call_expr.span.lo)),
    }
  }

  /// The sibling keys in a stable order, for a cache key that must not depend on
  /// a hash set's iteration order.
  ///
  /// Sorted once per call, beside the set it sorts, because the digest it feeds
  /// is the same digest for every namespace of that call.
  pub(crate) fn sorted_sibling_keys(&self) -> Vec<&Atom> {
    let mut sorted: Vec<&Atom> = self.sibling_keys.iter().collect();
    sorted.sort();
    sorted
  }
}

impl<'a> NamespaceKeyQuery<'a> {
  /// A lookup for one namespace of a call whose shared half is already built.
  pub(crate) fn for_namespace(
    call_keys: &'a CallKeys,
    call_expr: &CallExpr,
    namespace_key: &'a str,
  ) -> Self {
    Self {
      namespace_key,
      sibling_keys: Rc::clone(&call_keys.sibling_keys),
      // The one genuinely per-namespace signal: the keys of *this* namespace's
      // own value object.
      namespace_value_keys: first_object_arg(call_expr)
        .map(|object| namespace_value_keys(object, namespace_key))
        .unwrap_or_default(),
      target_lo: call_keys.target_lo,
    }
  }
}

/// The call's first argument, when it is an object literal -- the only shape
/// either side of this index reads, because that is what `stylex.create` takes.
fn first_object_arg(call_expr: &CallExpr) -> Option<&ObjectLit> {
  match call_expr.args.first().map(|arg| arg.expr.as_ref()) {
    Some(Expr::Object(object)) => Some(object),
    _ => None,
  }
}

/// Where `object` is written, or `None` when nothing wrote it: a synthesized
/// node carries `DUMMY_SP`, and byte zero would sort before every authored
/// position rather than mean "unknown".
fn object_lo(object: &ObjectLit) -> Option<BytePos> {
  (!object.span.is_dummy()).then_some(object.span.lo)
}

/// The keys of `namespace_key`'s own value object in `object`, empty when the
/// object does not spell the key or does not bind it to an object literal.
///
/// The first such property wins, unlike the index side's last -- this reads the
/// compiled call, where the namespace was built from a map and cannot repeat.
fn namespace_value_keys(object: &ObjectLit, namespace_key: &str) -> FxHashSet<Atom> {
  object
    .props
    .iter()
    .find_map(|prop| {
      let key_value = prop_as_key_value(prop)?;

      let names_the_namespace = namespace_name_from_prop_key(&key_value.key)
        .is_some_and(|name| name.as_ref() == namespace_key);

      match key_value.value.as_ref() {
        Expr::Object(value) if names_the_namespace => {
          Some(collect_object_lit_keys(value).collect())
        },
        _ => None,
      }
    })
    .unwrap_or_default()
}

/// The literal keys of `value` when it is an object literal, and none when it is
/// anything else -- a namespace bound to a reference or a call contributes no
/// keys to compare.
fn object_lit_keys(value: &Expr) -> impl Iterator<Item = Atom> + '_ {
  let object = match value {
    Expr::Object(object) => Some(object),
    _ => None,
  };

  object.into_iter().flat_map(collect_object_lit_keys)
}
