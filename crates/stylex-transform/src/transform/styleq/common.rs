use std::collections::BTreeMap;
use std::rc::Rc;

use indexmap::IndexMap;
use log::error;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use stylex_macros::{stylex_panic, stylex_unimplemented, stylex_unreachable};

use crate::shared::enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue;
use crate::shared::structures::types::FlatCompiledStyles;
use crate::shared::utils::core::parse_nullable_style::{ResolvedArg, StyleObject};
use stylex_constants::constants::common::COMPILED_KEY;

pub(crate) struct StyleQResult {
  pub(crate) class_name: String,
  pub(crate) inline_style: Option<FlatCompiledStyles>,
  pub(crate) data_style_src: Option<String>,
}

fn get_hash<T>(obj: T) -> u64
where
  T: Hash,
{
  let mut hasher = DefaultHasher::new();
  obj.hash(&mut hasher);
  hasher.finish()
}

pub(crate) fn styleq(arguments: &[ResolvedArg]) -> StyleQResult {
  let mut class_name = String::default();
  let mut debug_string = String::default();

  if arguments.is_empty() {
    return StyleQResult {
      class_name,
      inline_style: None,
      data_style_src: None,
    };
  }

  let mut defined_properties: Vec<String> = vec![];
  let mut inline_style: Option<FlatCompiledStyles> = None;
  let mut next_cache: Option<IndexMap<u64, (String, Vec<String>, String)>> = Some(IndexMap::new());
  let mut styles = arguments.iter().collect::<Vec<_>>();

  // Iterate over styles from last to first (pop from end)
  while let Some(possible_style) = styles.pop() {
    let possible_style = match possible_style {
      ResolvedArg::StyleObject(_, _, _) => possible_style,
      ResolvedArg::ConditionalStyle(_, value, _, _, _) => {
        if value.is_some() {
          possible_style
        } else {
          continue;
        }
      },
    };

    match possible_style {
      ResolvedArg::StyleObject(style, _, _) => match style {
        StyleObject::Style(style) => {
          if let Some(compiled_key) = style.get(COMPILED_KEY) {
            // ----- COMPILED: Process compiled style object (has $$css) -----
            if let FlatCompiledStylesValue::Bool(_) | FlatCompiledStylesValue::String(_) =
              compiled_key.as_ref()
            {
              let mut class_name_chunk = String::default();

              // Try cache read first
              let cache_hit = if let Some(ref next_cache) = next_cache {
                let btree_map: BTreeMap<_, _> = style.iter().collect();
                let style_hash = get_hash(btree_map);

                next_cache.get(&style_hash).map(|entry| {
                  (
                    style_hash,
                    entry.0.clone(),
                    entry.1.clone(),
                    entry.2.clone(),
                  )
                })
              } else {
                None
              };

              if let Some((_hash, cached_class_name, cached_properties, cached_debug_string)) =
                cache_hit
              {
                // Cache hit
                class_name_chunk = cached_class_name;
                defined_properties.extend(cached_properties);
                debug_string = cached_debug_string;
              } else {
                // Compute from scratch
                let mut defined_properties_chunk: Vec<String> = vec![];

                for (prop, value) in style.iter() {
                  if prop.eq(COMPILED_KEY) {
                    let compiled_key_value = &style[prop];

                    let mut compiled_key_value_is_true = false;

                    if let FlatCompiledStylesValue::Bool(value) = compiled_key_value.as_ref()
                      && *value
                    {
                      compiled_key_value_is_true = true;
                    }

                    if !compiled_key_value_is_true {
                      let compiled_key_string_value = match compiled_key_value.as_ref() {
                        FlatCompiledStylesValue::String(strng) => strng.clone(),
                        #[cfg(not(tarpaulin_include))]
                        other => {
                          let other_debug_info = format!("{:?}", other);
                          let variant_name =
                            other_debug_info.split("::").last().unwrap_or("unknown");

                          stylex_unimplemented!(
                            "String conversion not implemented for FlatCompiledStylesValue::{}",
                            variant_name
                          )
                        },
                      };

                      debug_string = if !debug_string.is_empty() {
                        format!("{}; {}", compiled_key_string_value, debug_string)
                      } else {
                        compiled_key_string_value
                      };
                    }

                    continue;
                  }

                  // Each property value should be a string or null
                  match value.as_ref() {
                    FlatCompiledStylesValue::String(_) | FlatCompiledStylesValue::Null => {
                      if !defined_properties.contains(prop) {
                        defined_properties.push(prop.clone());
                        if next_cache.is_some() {
                          defined_properties_chunk.push(prop.clone());
                        }

                        if let FlatCompiledStylesValue::String(value) = value.as_ref() {
                          class_name_chunk = if class_name_chunk.is_empty() {
                            value.to_string()
                          } else {
                            format!("{} {}", class_name_chunk, value)
                          };
                        }
                        // Null: property is defined (won't be overridden) but
                        // no class name is added — matches JS `null` handling.
                      }
                    },
                    _ => {
                      error!(
                        "styleq: {} typeof {:?} is not \"string\" or \"null\".",
                        prop, value
                      );
                    },
                  }
                }

                // Cache write (only when cache is active)
                if let Some(ref mut cache) = next_cache {
                  let btree_map: BTreeMap<_, _> = style.iter().collect();
                  let style_hash = get_hash(btree_map);
                  cache.insert(
                    style_hash,
                    (
                      class_name_chunk.clone(),
                      defined_properties_chunk,
                      debug_string.clone(),
                    ),
                  );
                }
              }

              if !class_name_chunk.is_empty() {
                class_name = if class_name.is_empty() {
                  class_name_chunk
                } else if !class_name.contains(class_name_chunk.as_str()) {
                  format!("{} {}", class_name_chunk, class_name)
                } else {
                  class_name
                };
              }
            } else {
              #[cfg(not(tarpaulin_include))]
              {
                stylex_panic!(
                  "styleq: {:#?} typeof {:?} is not \"string\" or \"null\".",
                  compiled_key,
                  "Bool"
                )
              }
            }
          } else {
            // ----- DYNAMIC: Process inline style object (no $$css) -----
            let mut sub_style: Option<FlatCompiledStyles> = None;

            for (prop, value) in style.iter() {
              if !defined_properties.contains(prop) {
                match value.as_ref() {
                  FlatCompiledStylesValue::Null => {
                    // null values mark the property as defined but don't
                    // contribute to the inline style output.
                    defined_properties.push(prop.clone());
                  },
                  _ => {
                    if sub_style.is_none() {
                      sub_style = Some(IndexMap::new());
                    }
                    if let Some(ref mut sub) = sub_style {
                      sub.insert(prop.clone(), Rc::new(value.as_ref().clone()));
                    }
                    defined_properties.push(prop.clone());
                  },
                }
              }
            }

            if let Some(sub) = sub_style {
              // Merge: sub_style first, then existing inline_style (earlier
              // properties win, matching JS `Object.assign(subStyle, inlineStyle)`)
              inline_style = if let Some(existing) = inline_style {
                let mut merged = sub;
                for (k, v) in existing {
                  merged.entry(k).or_insert(v);
                }
                Some(merged)
              } else {
                Some(sub)
              };
            }

            // Cache is unnecessary overhead when inline styles are present
            next_cache = None;
          }
        },
        StyleObject::Nullable => {},
        #[cfg(not(tarpaulin_include))]
        StyleObject::Other => {
          stylex_panic!("Only compiled StyleX style objects are allowed in styleq().")
        },
        #[cfg(not(tarpaulin_include))]
        StyleObject::Unreachable => {
          stylex_unreachable!(
            "Encountered an unexpected style object variant in styleq processing."
          )
        },
      },
      #[cfg(not(tarpaulin_include))]
      _ => stylex_unreachable!("Unexpected ResolvedArg variant in styleq loop"),
    };
  }

  StyleQResult {
    class_name,
    inline_style,
    data_style_src: Some(debug_string),
  }
}
