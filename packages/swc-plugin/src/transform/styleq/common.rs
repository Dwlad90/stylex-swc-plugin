use std::collections::BTreeMap;

use indexmap::IndexMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::shared::constants::common::COMPILED_KEY;
use crate::shared::enums::FlatCompiledStylesValue;
use crate::shared::{
  structures::types::FlatCompiledStyles,
  utils::stylex::parse_nullable_style::{ResolvedArg, StyleObject},
};

pub(crate) struct StyleQResult {
  pub(crate) class_name: String,
  pub(crate) inline_style: Option<FlatCompiledStyles>,
}

fn get_hash<T>(obj: T) -> u64
where
  T: Hash,
{
  let mut hasher = DefaultHasher::new();
  obj.hash(&mut hasher);
  hasher.finish()
}

pub(crate) fn styleq(arguments: &Vec<ResolvedArg>) -> StyleQResult {
  let mut class_name = "".to_string();

  if arguments.is_empty() {
    // Early return if there are no arguments
    return StyleQResult {
      class_name,
      inline_style: Option::None,
    };
  }

  let mut defined_properties: Vec<String> = vec![]; // The className and inline style to build up

  let inline_style: Option<FlatCompiledStyles> = None;

  let mut next_cache: Option<IndexMap<u64, (String, Vec<String>)>> = Option::Some(IndexMap::new()); // This way of creating an array from arguments is fastest

  let mut styles = vec![];

  for arg in arguments {
    styles.push(arg);
  }

  while !styles.is_empty() {
    let possible_style = match styles.pop() {
      Some(possible_style) => match possible_style {
        ResolvedArg::StyleObject(_, _, _) => possible_style,
        ResolvedArg::ConditionalStyle(_, value, _, _, _) => {
          if value.is_some() {
            possible_style
          } else {
            continue;
          }
        }
      },
      None => continue,
    };

    match possible_style {
      ResolvedArg::StyleObject(style, _, _) => match style {
        StyleObject::Style(style) => {
          let Some(a) = style.get(COMPILED_KEY) else {
            panic!("Style object does not contain a compiled key")
          };

          if let FlatCompiledStylesValue::Bool(_) = a.as_ref() {
            let btree_map: BTreeMap<_, _> = style.clone().into_iter().collect();

            let style_hash = get_hash(btree_map);

            // Build up the class names defined by this object
            let mut class_name_chunk = "".to_string(); // Check the cache to see if we've already done this work

            if next_cache
              .clone()
              .and_then(|cache| cache.get(&style_hash).cloned())
              .is_some()
            {
              todo!("Cache entry found");
            } else {
              // The properties defined by this object
              let mut defined_properties_chunk: Vec<String> = vec![];

              for (prop, value) in style.iter() {
                if prop.eq(COMPILED_KEY) {
                  continue;
                }

                match value.as_ref() {
                  FlatCompiledStylesValue::IncludedStyle(_) => {
                    eprintln!(
                      "styleq: {} typeof IncludedStyle is not \"string\" or \"null\".",
                      prop
                    )
                  }
                  FlatCompiledStylesValue::Bool(_) => {
                    eprintln!(
                      "styleq: {} typeof {:?} is not \"string\" or \"null\".",
                      prop, "Bool"
                    )
                  }
                  _ => {}
                }

                // Only add to chunks if this property hasn't already been seen
                if !defined_properties.contains(prop) {
                  defined_properties.push(prop.clone());

                  if next_cache.is_some() {
                    defined_properties_chunk.push(prop.clone())
                  }

                  if let FlatCompiledStylesValue::String(value) = *value.clone() {
                    class_name_chunk = if class_name_chunk.is_empty() {
                      value.to_string()
                    } else {
                      format!("{} {}", class_name_chunk, value)
                    };
                  }
                }
              }

              if let Some(next_cache) = next_cache.as_mut() {
                next_cache.insert(
                  style_hash,
                  (class_name_chunk.clone(), defined_properties_chunk.clone()),
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
            todo!("DYNAMIC: Process inline style object")
          }
        }
        StyleObject::Nullable => panic!("Nullable style object is not allowed in styleq"),
        StyleObject::Other => panic!("Other style object is not allowed in styleq"),
      },
      ResolvedArg::ConditionalStyle(_, _, _, _, _) => todo!("ConditionalStyle"),
    };
  }

  StyleQResult {
    class_name,
    inline_style,
  }
}
