use std::hash::Hash;

use rustc_hash::FxHashMap;

/// Where the entries holding a given thing live, bucketed by a key that narrows
/// to them.
///
/// Five of the state manager's questions have the same shape: "which recorded
/// entry holds *this*?", asked once per `stylex.*` call or per identifier
/// reference the transform meets. Answered by walking the whole collection and
/// comparing whole subtrees with `eq_ignore_span`, each of those runs once per
/// question against every entry -- quadratic in the size of the module, and on a
/// 1,500-component JSX module the largest single cost in the transform.
///
/// `K` is what narrows: a structural hash where the question is about an
/// expression, a source position where it is about where something was written,
/// a name where it is about what a declarator binds. `H` is how an entry is
/// addressed in the collection beside it -- a position for a `Vec`, a name for a
/// map. The index holds no entry itself, so it cannot disagree with one about
/// its contents, only about where it is, which is what the confirmation below
/// settles.
///
/// **The key narrows and equality still decides.** Callers confirm the
/// candidates this hands back, which is the shape `adr/0005` calls "narrow a
/// bucket by hash and then confirm" and what keeps every answer the one the walk
/// gave: a hit alone would make these consumers ones for which the key *is* the
/// equality test, and would refuse a match the walk made whenever
/// `EQ_IGNORE_SPAN_IGNORE_CTXT` is in scope, since a structural key hashes an
/// identifier's `SyntaxContext` and `eq_ignore_span` there does not.
#[derive(Clone, Debug)]
pub(crate) struct CandidateIndex<K, H> {
  buckets: FxHashMap<K, Vec<H>>,
}

impl<K, H> Default for CandidateIndex<K, H> {
  fn default() -> Self {
    Self {
      buckets: FxHashMap::default(),
    }
  }
}

impl<K: Eq + Hash, H: PartialEq> CandidateIndex<K, H> {
  /// Records that the entry at `handle` holds what `key` narrows to.
  ///
  /// Recording one that is already there is a no-op rather than a second bucket
  /// entry, so a collection filled twice -- which the discovery cycle does --
  /// indexes each entry once.
  pub(crate) fn record(&mut self, key: K, handle: H) {
    let bucket = self.buckets.entry(key).or_default();

    if !bucket.contains(&handle) {
      bucket.push(handle);
    }
  }

  /// Drops the record that the entry at `handle` holds what `key` narrows to,
  /// forgetting the bucket once nothing is left in it.
  pub(crate) fn forget(&mut self, key: &K, handle: &H) {
    let Some(bucket) = self.buckets.get_mut(key) else {
      return;
    };

    bucket.retain(|candidate| candidate != handle);

    if bucket.is_empty() {
      self.buckets.remove(key);
    }
  }

  /// Moves the entry at `handle` from the key it was recorded under to the one
  /// it now belongs to, either of which may be absent.
  ///
  /// The one way an entry's key changes, and the reason it is a method rather
  /// than a pair of calls at four sites: the order matters. Forgetting first
  /// means an entry replaced by something under the *same* key keeps its record
  /// instead of losing it to the forget that would otherwise follow.
  pub(crate) fn move_entry(&mut self, replaced: Option<K>, recorded: Option<K>, handle: H) {
    if let Some(replaced) = replaced {
      self.forget(&replaced, &handle);
    }

    if let Some(recorded) = recorded {
      self.record(recorded, handle);
    }
  }

  /// The entries that may hold what `key_of` describes -- the candidates the
  /// caller confirms.
  ///
  /// The key is computed only when there is something to look it up in, because
  /// computing a structural one walks the whole expression. A module that
  /// records nothing of the kind this index holds -- every style in one
  /// top-level array is the common shape -- then answers for free, where hashing
  /// a style object first would cost more than the scan this replaces did.
  pub(crate) fn candidates(&self, key_of: impl FnOnce() -> K) -> &[H] {
    if self.buckets.is_empty() {
      return &[];
    }

    self.buckets.get(&key_of()).map_or(&[], Vec::as_slice)
  }
}
