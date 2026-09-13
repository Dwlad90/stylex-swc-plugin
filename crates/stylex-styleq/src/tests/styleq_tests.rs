//! What the cache answers when a lock is lost, and one merge run from inside
//! the crate.
//!
//! Every case about what a merge *answers* lives in `tests/styleq_test.rs`,
//! which reads the crate the way a consumer does. What is here is what only
//! this side can reach: the private cache handles, and the recovery a poisoned
//! lock takes.

use super::*;
use std::panic::{AssertUnwindSafe, catch_unwind};

use crate::capturing_logger::logged_at;

/// Calling the `_assert_cache_send_sync` helper covers its body in tests
/// while still serving its compile-time purpose of asserting `Send`+`Sync`
/// for `CacheEntry`/`CacheKey` (relevant when StyleX is invoked from
/// parallel processors like Rayon/Tokio).
#[test]
fn cache_types_are_send_and_sync() {
  super::_assert_cache_send_sync();
}

/// One merge, run from inside the crate.
///
/// Every case about a merge lives in `tests/styleq_test.rs`, and this does
/// not duplicate one -- it is here so the walk is *instantiated* in this
/// binary. The cases beside it build a `Styleq` and never call it, so the
/// generic methods were emitted here as an uninstantiated shell whose
/// counters are all zero, and the coverage gate read that shell as a region
/// no test runs.
#[test]
fn a_merge_runs_from_the_crate_s_own_binary() {
  let styleq = create_styleq::<crate::StyleValue>(StyleqOptions::default());
  let mut style = StyleMap::new();
  style.insert(COMPILED_KEY.to_string(), crate::StyleValue::Bool(true));
  style.insert("color".to_string(), crate::StyleValue::string("color-red"));

  let mut dynamic = StyleMap::new();
  dynamic.insert("width".to_string(), crate::StyleValue::string("10px"));

  let result = styleq.styleq(&[
    StyleqInput::Style(style),
    StyleqInput::Style(dynamic.clone()),
  ]);

  assert_eq!(result.class_name, "color-red");
  assert_eq!(result.inline_style, Some(dynamic));
}

#[test]
fn cache_lock_recovers_from_poisoned_rwlock() {
  let styleq = create_styleq::<crate::StyleValue>(StyleqOptions::default());

  let result = catch_unwind(AssertUnwindSafe(|| {
    let _guard = styleq.cache.entries.write();
    panic!("poison cache rwlock");
  }));

  assert!(result.is_err());
  assert!(styleq.cache_read(&styleq.cache).is_empty());
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
    let _guard = styleq.cache.entries.write();
    panic!("poison cache rwlock for write recovery");
  }));
  assert!(result.is_err(), "writer panic should propagate");

  // Sanity: the lock is now poisoned for both read and write.
  assert!(styleq.cache.entries.read().is_err());
  assert!(styleq.cache.entries.write().is_err());

  // `insert_cache_entry` goes through `cache_write` → must transparently
  // recover from the poisoned lock and complete the insert.
  let key = CacheKey::Hash(0xDEAD_BEEF);
  let entry = CacheEntry {
    class_name: Arc::from(""),
    defined_properties: Arc::from(Vec::<Arc<str>>::new()),
    debug_string: Arc::from(""),
    next: OnceLock::new(),
  };
  styleq.insert_cache_entry(&styleq.cache, key, entry);

  let cache = styleq.cache_read(&styleq.cache);
  assert!(
    cache.contains_key(&key),
    "recovered cache must accept new entries after poisoning"
  );
}

/// The `poison_warned` latch must flip exactly once: the first poisoned
/// recovery is logged at `error!` (actionable), every later one falls back
/// to `debug!` so a single panicked writer can't flood the log under load.
///
/// The messages are read back, because the latch is only observable in what
/// was written: the flag alone cannot tell a second `error!` from the
/// `debug!` that has to replace it.
#[test]
fn poison_warning_latches_after_first_recovery() {
  let styleq = create_styleq::<crate::StyleValue>(StyleqOptions::default());

  assert!(
    !styleq.poison_warned.load(Ordering::Relaxed),
    "freshly-built Styleq must not have its poison flag set"
  );

  let _ = catch_unwind(AssertUnwindSafe(|| {
    let _guard = styleq.cache.entries.write();
    panic!("poison cache rwlock for latch test");
  }));

  let messages = logged_at(log::Level::Debug, || {
    // First recovery: latches the flag and emits `error!`.
    drop(styleq.cache_read(&styleq.cache));
    assert!(
      styleq.poison_warned.load(Ordering::Relaxed),
      "first recovery must latch the poison-warned flag"
    );

    // Second recovery: same path, no re-latch (still true). This call is
    // what would previously have produced a second `error!` log line; now
    // it's demoted to `debug!` and the flag stays unchanged.
    drop(styleq.cache_read(&styleq.cache));
    assert!(
      styleq.poison_warned.load(Ordering::Relaxed),
      "subsequent recoveries must keep the flag set without flipping it back"
    );
  });

  assert_eq!(
    messages,
    vec![
      "styleq: cache RwLock was poisoned (read); continuing with inner cache.".to_string(),
      "styleq: cache RwLock still poisoned (read); recovered transparently.".to_string(),
    ],
    "the first recovery is the loud one and every later one is quiet"
  );
}

/// The write path names itself, so a reader of the log can tell which guard
/// was recovered. The kind is an argument of the message, which is built only
/// while a logger admits the level.
#[test]
fn a_recovered_write_guard_names_itself() {
  let styleq = create_styleq::<crate::StyleValue>(StyleqOptions::default());

  let _ = catch_unwind(AssertUnwindSafe(|| {
    let _guard = styleq.cache.entries.write();
    panic!("poison cache rwlock for the write path");
  }));

  let messages = logged_at(log::Level::Error, || {
    drop(styleq.cache_write(&styleq.cache))
  });

  assert_eq!(
    messages,
    vec!["styleq: cache RwLock was poisoned (write); continuing with inner cache.".to_string()]
  );
}
