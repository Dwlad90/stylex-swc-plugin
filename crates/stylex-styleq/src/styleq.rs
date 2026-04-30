use std::{
  hash::{Hash, Hasher},
  sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use log::error;
use rustc_hash::{FxHashMap, FxHasher};

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
}

pub fn create_styleq<V: StyleqValue>(options: StyleqOptions<V>) -> Styleq<V> {
  Styleq {
    options,
    cache: RwLock::new(FxHashMap::default()),
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
    let mut defined_properties = Vec::new();
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
    defined_properties: &mut Vec<String>,
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
      defined_properties.extend(cache_entry.defined_properties.iter().map(|s| s.to_string()));
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
          if !defined_properties.contains(prop) {
            defined_properties.push(prop.clone());

            if use_cache {
              defined_properties_chunk.push(Arc::from(prop.as_str()));
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
    defined_properties: &mut Vec<String>,
    inline_style: &mut Option<StyleMap<V>>,
    use_cache: &mut bool,
  ) {
    let mut sub_style: Option<StyleMap<V>> = None;

    for (prop, value) in style {
      if !defined_properties.contains(prop) {
        if !value.is_null() {
          sub_style
            .get_or_insert_with(StyleMap::new)
            .insert(prop.clone(), value.clone());
        }

        defined_properties.push(prop.clone());
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
      .unwrap_or_else(|poisoned| recover_poisoned_cache_read(poisoned))
  }

  fn cache_write(&self) -> RwLockWriteGuard<'_, FxHashMap<CacheKey, Arc<CacheEntry>>> {
    self
      .cache
      .write()
      .unwrap_or_else(|poisoned| recover_poisoned_cache_write(poisoned))
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

fn recover_poisoned_cache_read<'a>(
  poisoned: std::sync::PoisonError<RwLockReadGuard<'a, FxHashMap<CacheKey, Arc<CacheEntry>>>>,
) -> RwLockReadGuard<'a, FxHashMap<CacheKey, Arc<CacheEntry>>> {
  error!("styleq: cache RwLock was poisoned (read); continuing with inner cache.");
  poisoned.into_inner()
}

fn recover_poisoned_cache_write<'a>(
  poisoned: std::sync::PoisonError<RwLockWriteGuard<'a, FxHashMap<CacheKey, Arc<CacheEntry>>>>,
) -> RwLockWriteGuard<'a, FxHashMap<CacheKey, Arc<CacheEntry>>> {
  error!("styleq: cache RwLock was poisoned (write); continuing with inner cache.");
  poisoned.into_inner()
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::panic::{AssertUnwindSafe, catch_unwind};

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
}
