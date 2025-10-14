use std::collections::BTreeMap;

use indexmap::IndexMap;
use log::warn;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::shared::{
  constants::common::COMPILED_KEY,
  enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue,
};
use crate::shared::{
  structures::types::FlatCompiledStyles,
  utils::core::parse_nullable_style::{ResolvedArg, StyleObject},
};

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
    // Early return if there are no arguments
    return StyleQResult {
      class_name,
      inline_style: None,
      data_style_src: None,
    };
  }

  let mut defined_properties = vec![]; // The className and inline style to build up

  let inline_style = None;
  let mut next_cache: Option<IndexMap<u64, (String, Vec<String>, String)>> = Some(IndexMap::new());
  let mut styles = arguments.iter().collect::<Vec<_>>();

  while let Some(possible_style) = styles.pop() {
    let possible_style = match possible_style {
      ResolvedArg::StyleObject(_, _, _) => possible_style,
      ResolvedArg::ConditionalStyle(_, value, _, _, _) => {
        if value.is_some() {
          possible_style
        } else {
          continue;
        }
      }
    };

    match possible_style {
      ResolvedArg::StyleObject(style, _, _) => match style {
        StyleObject::Style(style) => {
          let Some(a) = style.get(COMPILED_KEY) else {
            panic!("Style object does not contain a compiled key")
          };

          if let FlatCompiledStylesValue::Bool(_) | FlatCompiledStylesValue::String(_) = a.as_ref()
          {
            let btree_map: BTreeMap<_, _> = style.iter().collect();

            let style_hash = get_hash(btree_map);
            let mut class_name_chunk = String::default();

            // Build up the class names defined by this object
            if let Some(next_cache) = next_cache.as_mut() {
              if let Some((cached_class_name, cached_properties, cached_debug_string)) =
                next_cache.get(&style_hash)
              {
                class_name_chunk = cached_class_name.clone();
                defined_properties.extend(cached_properties.iter().cloned());
                debug_string = cached_debug_string.clone();
              } else {
                // The properties defined by this object
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
                        other => {
                          let other_debug_info = format!("{:?}", other);
                          let variant_name =
                            other_debug_info.split("::").last().unwrap_or("unknown");

                          unimplemented!(
                            "String conversion not implemented for FlatCompiledStylesValue::{}",
                            variant_name
                          )
                        }
                      };

                      debug_string = if !debug_string.is_empty() {
                        format!("{}; {}", compiled_key_string_value, debug_string)
                      } else {
                        compiled_key_string_value
                      };
                    }

                    continue;
                  }

                  if let FlatCompiledStylesValue::Bool(_) = value.as_ref() {
                    warn!(
                      "styleq: {} typeof {:?} is not \"string\" or \"null\".",
                      prop, "Bool"
                    )
                  }

                  if !defined_properties.contains(prop) {
                    defined_properties.push(prop.clone());
                    defined_properties_chunk.push(prop.clone());

                    if let FlatCompiledStylesValue::String(value) = (**value).clone() {
                      class_name_chunk = if class_name_chunk.is_empty() {
                        value.to_string()
                      } else {
                        format!("{} {}", class_name_chunk, value)
                      };
                    }
                  }
                }

                next_cache.insert(
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
                class_name_chunk.clone()
              } else if !class_name.contains(class_name_chunk.as_str()) {
                format!("{} {}", class_name_chunk, class_name)
              } else {
                class_name
              };
            }
          } else {
            unimplemented!("DYNAMIC: Process inline style object")
          }
        }
        StyleObject::Nullable => panic!("Nullable style object is not allowed in styleq"),
        StyleObject::Other => panic!("Other style object is not allowed in styleq"),
        StyleObject::Unreachable => unreachable!("StyleObject::Unreachable in styleq"),
      },
      _ => unreachable!(),
    };
  }

  StyleQResult {
    class_name,
    inline_style,
    data_style_src: Some(debug_string),
  }
}
