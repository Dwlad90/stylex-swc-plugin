use std::{
  collections::hash_map::DefaultHasher,
  hash::{Hash, Hasher},
  sync::{Mutex, MutexGuard},
};

use indexmap::IndexMap;
use log::error;

use crate::{
  COMPILED_KEY, StyleMap, StyleqArgument, StyleqInput, StyleqOptions, StyleqResult, StyleqValue,
};

#[derive(Clone)]
struct CacheEntry {
  class_name: String,
  defined_properties: Vec<String>,
  debug_string: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum CacheKey {
  Identity(usize),
  Hash(u64),
}

pub struct Styleq<V: StyleqValue> {
  options: StyleqOptions<V>,
  cache: Mutex<IndexMap<CacheKey, CacheEntry>>,
}

pub fn create_styleq<V: StyleqValue>(options: StyleqOptions<V>) -> Styleq<V> {
  Styleq {
    options,
    cache: Mutex::new(IndexMap::new()),
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
      class_name_chunk = cache_entry.class_name;
      *debug_string = cache_entry.debug_string;
      defined_properties.extend(cache_entry.defined_properties);
    } else {
      let mut defined_properties_chunk = Vec::new();

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
              defined_properties_chunk.push(prop.clone());
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
            class_name: class_name_chunk.clone(),
            defined_properties: defined_properties_chunk,
            debug_string: debug_string.clone(),
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
            .get_or_insert_with(IndexMap::new)
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

  fn cache_lock(&self) -> MutexGuard<'_, IndexMap<CacheKey, CacheEntry>> {
    match self.cache.lock() {
      Ok(cache) => cache,
      Err(poisoned) => {
        error!("styleq: cache mutex was poisoned; continuing with inner cache.");
        poisoned.into_inner()
      },
    }
  }

  fn get_cache_entry(&self, cache_key: &CacheKey) -> Option<CacheEntry> {
    self.cache_lock().get(cache_key).cloned()
  }

  fn insert_cache_entry(&self, cache_key: CacheKey, cache_entry: CacheEntry) {
    self.cache_lock().insert(cache_key, cache_entry);
  }
}

fn hash_style<V: StyleqValue>(style: &StyleMap<V>) -> u64 {
  let mut hasher = DefaultHasher::new();

  for (prop, value) in style {
    prop.hash(&mut hasher);
    value.hash(&mut hasher);
  }

  hasher.finish()
}
