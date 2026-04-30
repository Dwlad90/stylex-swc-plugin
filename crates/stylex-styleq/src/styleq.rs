use std::{
  hash::{Hash, Hasher},
  sync::{
    Arc, RwLock, RwLockReadGuard, RwLockWriteGuard,
    atomic::{AtomicBool, Ordering},
  },
};

use log::{debug, error};
use rustc_hash::{FxHashMap, FxHashSet, FxHasher};

use crate::{
  COMPILED_KEY, StyleMap, StyleqArgument, StyleqInput, StyleqOptions, StyleqResult, StyleqValue,
};

// JS-parity: styleq/src/styleq.js — `compiledStyleCache` (a Map keyed by
// either the source array reference or a structural hash). Order is never
// observed downstream, so an unordered FxHashMap is appropriate. The entry
// itself is wrapped in `Arc` so cache hits are a refcount bump rather than
// a deep clone of three owned strings + a `Vec<Arc<str>>`.
struct CacheEntry {
  class_name: Arc<str>,
  defined_properties: Arc<[Arc<str>]>,
  debug_string: Arc<str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum CacheKey {
  Identity(usize),
  Hash(u64),
}

pub struct Styleq<V: StyleqValue> {
  options: StyleqOptions<V>,
  cache: RwLock<FxHashMap<CacheKey, Arc<CacheEntry>>>,
  /// Latched after the first poisoned-lock recovery so subsequent recoveries
  /// log at `debug!` level instead of flooding `error!` once per call. The
  /// first occurrence is still surfaced as an error (the actionable signal).
  poison_warned: AtomicBool,
}

// Compile-time guarantee that the cache layer is safe to share across threads
// (relevant for parallel SWC processing of multiple files via Rayon/Tokio).
//
// Implemented as a `#[cfg(test)]`-only function so the `Send`/`Sync` trait
// bounds are still type-checked during normal compilation **and** the body is
// only emitted in test builds where it can be reached, keeping coverage at
// 100% without needing `#[coverage(off)]` (which is unstable on consts).
#[cfg(test)]
#[allow(dead_code)]
fn _assert_cache_send_sync() {
  fn assert_send<T: Send>() {}
  fn assert_sync<T: Sync>() {}
  assert_send::<CacheEntry>();
  assert_sync::<CacheEntry>();
  assert_send::<CacheKey>();
  assert_sync::<CacheKey>();
}

pub fn create_styleq<V: StyleqValue>(options: StyleqOptions<V>) -> Styleq<V> {
  Styleq {
    options,
    cache: RwLock::new(FxHashMap::default()),
    poison_warned: AtomicBool::new(false),
  }
}

pub fn styleq<V: StyleqValue>(styles: &[StyleqInput<V>]) -> StyleqResult<V> {
  create_styleq(StyleqOptions::default()).styleq(styles)
}

impl<V: StyleqValue> Styleq<V> {
  pub fn styleq<A>(&self, arguments: &[A]) -> StyleqResult<V>
  where
    A: StyleqArgument<V>,
  {
    // Membership-only set (`Arc<str>` keys are cheap to clone-on-insert and
    // make the property-already-defined check O(1) instead of the previous
    // O(n) `Vec::contains`. Property iteration order is never observed
    // downstream — only "have we seen this prop?" matters here.
    let mut defined_properties: FxHashSet<Arc<str>> = FxHashSet::default();
    let mut class_name = String::new();
    let mut inline_style: Option<StyleMap<V>> = None;
    let mut debug_string = String::new();
    let mut use_cache = !self.options.disable_cache;
    let mut styles = arguments.iter().collect::<Vec<_>>();

    while let Some(possible_style) = styles.pop() {
      if possible_style.should_skip() {
        continue;
      }

      if let Some(nested_styles) = possible_style.as_nested() {
        styles.extend(nested_styles);
        continue;
      }

      let Some(style) = possible_style.as_style() else {
        continue;
      };
      let cache_key = possible_style.cache_key();

      let transformed_style;
      let style = match &self.options.transform {
        Some(transform) => {
          transformed_style = transform(style.clone());
          &transformed_style
        },
        None => style,
      };

      if style.contains_key(COMPILED_KEY) {
        self.process_compiled_style(
          style,
          &mut defined_properties,
          &mut class_name,
          &mut debug_string,
          cache_key,
          use_cache,
        );
      } else if self.options.disable_mix {
        let mut next_inline_style = style.clone();

        if let Some(existing_inline_style) = inline_style.take() {
          for (prop, value) in existing_inline_style {
            next_inline_style.insert(prop, value);
          }
        }

        inline_style = Some(next_inline_style);
      } else {
        self.process_inline_style(
          style,
          &mut defined_properties,
          &mut inline_style,
          &mut use_cache,
        );
      }
    }

    StyleqResult {
      class_name,
      inline_style,
      data_style_src: debug_string,
    }
  }

  fn process_compiled_style(
    &self,
    style: &StyleMap<V>,
    defined_properties: &mut FxHashSet<Arc<str>>,
    class_name: &mut String,
    debug_string: &mut String,
    cache_key: Option<usize>,
    use_cache: bool,
  ) {
    let mut class_name_chunk = String::new();
    let cache_key = match cache_key {
      Some(cache_key) if self.options.transform.is_none() => CacheKey::Identity(cache_key),
      _ => CacheKey::Hash(hash_style(style)),
    };

    if use_cache && let Some(cache_entry) = self.get_cache_entry(&cache_key) {
      class_name_chunk.push_str(&cache_entry.class_name);
      debug_string.clear();
      debug_string.push_str(&cache_entry.debug_string);
      // `Arc<str>` clone is a refcount bump — no per-element heap allocation
      // on the cache-hit fast path (was a `String::clone` per property).
      defined_properties.extend(cache_entry.defined_properties.iter().cloned());
    } else {
      let mut defined_properties_chunk: Vec<Arc<str>> = Vec::new();

      for (prop, value) in style {
        if prop == COMPILED_KEY {
          if !value.is_true_bool() {
            if let Some(compiled_key_value) = value.as_class_name() {
              if debug_string.is_empty() {
                debug_string.push_str(compiled_key_value);
              } else {
                debug_string.insert_str(0, "; ");
                debug_string.insert_str(0, compiled_key_value);
              }
            } else {
              error!(
                "styleq: {} typeof {:?} is not \"string\" or \"true\".",
                COMPILED_KEY, value
              );
            }
          }

          continue;
        }

        if value.as_class_name().is_some() || value.is_null() {
          // Allocate the `Arc<str>` once and share between the membership
          // set and the cache chunk (when caching). Avoids a duplicate
          // `String`+`Arc` allocation for the same property name.
          let prop_arc: Arc<str> = Arc::from(prop.as_str());
          if defined_properties.insert(prop_arc.clone()) {
            if use_cache {
              defined_properties_chunk.push(prop_arc);
            }

            if let Some(value) = value.as_class_name() {
              if !class_name_chunk.is_empty() {
                class_name_chunk.push(' ');
              }

              class_name_chunk.push_str(value);
            }
          }
        } else {
          error!(
            "styleq: {} typeof {:?} is not \"string\" or \"null\".",
            prop, value
          );
        }
      }

      if use_cache {
        self.insert_cache_entry(
          cache_key,
          CacheEntry {
            class_name: Arc::from(class_name_chunk.as_str()),
            defined_properties: Arc::from(defined_properties_chunk.into_boxed_slice()),
            debug_string: Arc::from(debug_string.as_str()),
          },
        );
      }
    }

    if !class_name_chunk.is_empty() {
      if class_name.is_empty() {
        class_name.push_str(&class_name_chunk);
      } else if !self.options.dedupe_class_name_chunks
        || !class_name.contains(class_name_chunk.as_str())
      {
        class_name.insert(0, ' ');
        class_name.insert_str(0, &class_name_chunk);
      }
    }
  }

  fn process_inline_style(
    &self,
    style: &StyleMap<V>,
    defined_properties: &mut FxHashSet<Arc<str>>,
    inline_style: &mut Option<StyleMap<V>>,
    use_cache: &mut bool,
  ) {
    let mut sub_style: Option<StyleMap<V>> = None;

    for (prop, value) in style {
      // O(1) borrow-based lookup; only allocate an `Arc<str>` if the
      // property is genuinely new to the set.
      if !defined_properties.contains(prop.as_str()) {
        if !value.is_null() {
          sub_style
            .get_or_insert_with(StyleMap::new)
            .insert(prop.clone(), value.clone());
        }

        defined_properties.insert(Arc::from(prop.as_str()));
        *use_cache = false;
      }
    }

    if let Some(mut sub_style) = sub_style {
      if let Some(existing_inline_style) = inline_style.take() {
        for (prop, value) in existing_inline_style {
          sub_style.entry(prop).or_insert(value);
        }
      }

      *inline_style = Some(sub_style);
    }
  }

  fn cache_read(&self) -> RwLockReadGuard<'_, FxHashMap<CacheKey, Arc<CacheEntry>>> {
    self
      .cache
      .read()
      .unwrap_or_else(|poisoned| self.recover_poisoned_read(poisoned))
  }

  fn cache_write(&self) -> RwLockWriteGuard<'_, FxHashMap<CacheKey, Arc<CacheEntry>>> {
    self
      .cache
      .write()
      .unwrap_or_else(|poisoned| self.recover_poisoned_write(poisoned))
  }

  /// Logs the poisoning event at `error!` exactly once per `Styleq`
  /// instance; subsequent recoveries are demoted to `debug!` so a single
  /// panicked writer can't flood the log under sustained load.
  fn report_poisoned(&self, kind: &str) {
    if !self.poison_warned.swap(true, Ordering::Relaxed) {
      error!("styleq: cache RwLock was poisoned ({kind}); continuing with inner cache.");
    } else {
      debug!("styleq: cache RwLock still poisoned ({kind}); recovered transparently.");
    }
  }

  fn recover_poisoned_read<'a>(
    &self,
    poisoned: std::sync::PoisonError<RwLockReadGuard<'a, FxHashMap<CacheKey, Arc<CacheEntry>>>>,
  ) -> RwLockReadGuard<'a, FxHashMap<CacheKey, Arc<CacheEntry>>> {
    self.report_poisoned("read");
    poisoned.into_inner()
  }

  fn recover_poisoned_write<'a>(
    &self,
    poisoned: std::sync::PoisonError<RwLockWriteGuard<'a, FxHashMap<CacheKey, Arc<CacheEntry>>>>,
  ) -> RwLockWriteGuard<'a, FxHashMap<CacheKey, Arc<CacheEntry>>> {
    self.report_poisoned("write");
    poisoned.into_inner()
  }

  fn get_cache_entry(&self, cache_key: &CacheKey) -> Option<Arc<CacheEntry>> {
    self.cache_read().get(cache_key).map(Arc::clone)
  }

  fn insert_cache_entry(&self, cache_key: CacheKey, cache_entry: CacheEntry) {
    self.cache_write().insert(cache_key, Arc::new(cache_entry));
  }
}

// JS-parity: styleq/src/styleq.js#L100 (structural hash branch). Switched
// from `DefaultHasher` (SipHash-1-3) to `FxHasher` — keys are short and the
// cache is process-local, so DOS resistance is unnecessary.
fn hash_style<V: StyleqValue>(style: &StyleMap<V>) -> u64 {
  let mut hasher = FxHasher::default();

  for (prop, value) in style {
    prop.hash(&mut hasher);
    value.hash(&mut hasher);
  }

  hasher.finish()
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::panic::{AssertUnwindSafe, catch_unwind};

  /// Calling the `_assert_cache_send_sync` helper covers its body in tests
  /// while still serving its compile-time purpose of asserting `Send`+`Sync`
  /// for `CacheEntry`/`CacheKey` (relevant when StyleX is invoked from
  /// parallel processors like Rayon/Tokio).
  #[test]
  fn cache_types_are_send_and_sync() {
    super::_assert_cache_send_sync();
  }

  #[test]
  fn cache_lock_recovers_from_poisoned_rwlock() {
    let styleq = create_styleq::<crate::StyleValue>(StyleqOptions::default());

    let result = catch_unwind(AssertUnwindSafe(|| {
      let _guard = styleq.cache.write();
      panic!("poison cache rwlock");
    }));

    assert!(result.is_err());
    assert!(styleq.cache_read().is_empty());
  }

  /// After the cache RwLock has been poisoned by a panicking writer, the
  /// **write** path (`cache_write` + `recover_poisoned_write`) must also
  /// recover and let subsequent inserts succeed. Without this test, the
  /// poisoned-write recovery branch (`unwrap_or_else(...)` in `cache_write`)
  /// is never exercised, leaving a coverage hole exactly in the recovery
  /// code path most likely to silently break.
  #[test]
  fn cache_write_recovers_from_poisoned_rwlock() {
    let styleq = create_styleq::<crate::StyleValue>(StyleqOptions::default());

    let result = catch_unwind(AssertUnwindSafe(|| {
      let _guard = styleq.cache.write();
      panic!("poison cache rwlock for write recovery");
    }));
    assert!(result.is_err(), "writer panic should propagate");

    // Sanity: the lock is now poisoned for both read and write.
    assert!(styleq.cache.read().is_err());
    assert!(styleq.cache.write().is_err());

    // `insert_cache_entry` goes through `cache_write` → must transparently
    // recover from the poisoned lock and complete the insert.
    let key = CacheKey::Hash(0xDEAD_BEEF);
    let entry = CacheEntry {
      class_name: Arc::from(""),
      defined_properties: Arc::from(Vec::<Arc<str>>::new()),
      debug_string: Arc::from(""),
    };
    styleq.insert_cache_entry(key, entry);

    let cache = styleq.cache_read();
    assert!(
      cache.contains_key(&key),
      "recovered cache must accept new entries after poisoning"
    );
  }

  /// The `poison_warned` latch must flip exactly once: the first poisoned
  /// recovery is logged at `error!` (actionable), every later one falls back
  /// to `debug!` so a single panicked writer can't flood the log under load.
  #[test]
  fn poison_warning_latches_after_first_recovery() {
    let styleq = create_styleq::<crate::StyleValue>(StyleqOptions::default());

    assert!(
      !styleq.poison_warned.load(Ordering::Relaxed),
      "freshly-built Styleq must not have its poison flag set"
    );

    let _ = catch_unwind(AssertUnwindSafe(|| {
      let _guard = styleq.cache.write();
      panic!("poison cache rwlock for latch test");
    }));

    // First recovery: latches the flag and emits `error!`.
    drop(styleq.cache_read());
    assert!(
      styleq.poison_warned.load(Ordering::Relaxed),
      "first recovery must latch the poison-warned flag"
    );

    // Second recovery: same path, no re-latch (still true). This call is
    // what would previously have produced a second `error!` log line; now
    // it's demoted to `debug!` and the flag stays unchanged.
    drop(styleq.cache_read());
    assert!(
      styleq.poison_warned.load(Ordering::Relaxed),
      "subsequent recoveries must keep the flag set without flipping it back"
    );
  }
}
