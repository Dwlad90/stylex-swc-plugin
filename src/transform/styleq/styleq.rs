use std::collections::BTreeMap;

use indexmap::IndexMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::shared::{
    structures::flat_compiled_styles::{FlatCompiledStyles, FlatCompiledStylesValue},
    utils::stylex::parse_nallable_style::{ResolvedArg, StyleObject},
};

pub(crate) struct StyleQResult {
    pub(crate) class_name: String,
    pub(crate) _inline_style: Option<FlatCompiledStyles>,
}

// pub(crate) static CACHE: phf::Map<&'static IndexMap<String, FlatCompiledStylesValue>, &'static str> = phf_map! {};

static COMPILED_KEY: &str = "$$css";

fn get_hash<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

// lazy_static! {
//     static ref CACHE: HashMap<IndexMap<String, FlatCompiledStylesValue>, String> = HashMap::new();
// }
pub(crate) fn styleq(arguments: &Vec<ResolvedArg>) -> StyleQResult {
    // let mut next_cache: HashMap<String, (String, Vec<String>, HashMap<String, String>)> = HashMap::new();
    // let mut weak_map: HashMap<String, String> = HashMap::new();

    // let style = "style".to_string();
    // let class_name_chunk = "classNameChunk".to_string();
    // let defined_properties_chunk: Vec<String> = vec!["property1".to_string(), "property2".to_string()];

    // next_cache.insert(style.clone(), (class_name_chunk, defined_properties_chunk, weak_map.clone()));

    // // If you want to use weak_map later, you can retrieve it from next_cache
    // let weak_map_retrieved = next_cache.get(&style).unwrap().2.clone();

    // Keep track of property commits to the className
    dbg!(&arguments);

    let mut class_name = "".to_string();

    if arguments.is_empty() {
        // Early return if there are no arguments
        return StyleQResult {
            class_name,
            _inline_style: Option::None,
        };
    }

    let mut defined_properties: Vec<String> = vec![]; // The className and inline style to build up

    let inline_style: Option<FlatCompiledStyles> = None;

    let mut next_cache: Option<IndexMap<u64, (String, Vec<String>)>> =
        Option::Some(IndexMap::new()); // This way of creating an array from arguments is fastest

    let mut styles = vec![];

    for arg in arguments {
        styles.push(arg);
    }

    // let mut styles: Vec<Vec<ResolvedArg>> = arguments.clone();

    while styles.len() > 0 {
        let possible_style = match styles.pop() {
            Some(possible_style) => match possible_style {
                ResolvedArg::StyleObject(_, _) => possible_style,
                ResolvedArg::ConditionalStyle(_, value, _, _) => {
                    if value.is_some() {
                        possible_style
                    } else {
                        continue;
                    }
                }
            },
            None => continue,
        };

        dbg!(&possible_style, &styles);

        // let style = possible_style.clone();

        match possible_style {
            ResolvedArg::StyleObject(style, _) => match style {
                StyleObject::Style(style) => {
                    if let Some(FlatCompiledStylesValue::Bool(_)) = style.get(COMPILED_KEY) {
                        let btree_map: BTreeMap<_, _> = style.clone().into_iter().collect();

                        let style_hash = get_hash(btree_map);

                        // Build up the class names defined by this object
                        let mut class_name_chunk = "".to_string(); // Check the cache to see if we've already done this work

                        if let Some(cache_entry) = next_cache
                            .clone()
                            .and_then(|cache| cache.get(&style_hash).cloned())
                        {
                            todo!("Cache entry found");
                            class_name_chunk = cache_entry.0;

                            defined_properties.extend(cache_entry.1);

                            // class_name_chunk = cache_entry.clone();
                        } else {
                            // The properties defined by this object
                            let mut defined_properties_chunk: Vec<String> = vec![];

                            for (prop, value) in style.iter() {
                                if prop.eq(COMPILED_KEY) {
                                    continue;
                                }

                                match value {
                                    FlatCompiledStylesValue::IncludedStyle(_) => {
                                        eprintln!(
                                            "styleq: {} typeof {} is not \"string\" or \"null\".",
                                            prop, "IncludedStyle"
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

                                    if let Some(_) = next_cache {
                                        defined_properties_chunk.push(prop.clone())
                                    }

                                    if let FlatCompiledStylesValue::String(value) = value {
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
            ResolvedArg::ConditionalStyle(_, _, _, _) => todo!("ConditionalStyle"),
        };
    }

    StyleQResult {
        class_name,
        _inline_style: inline_style,
    }
}
