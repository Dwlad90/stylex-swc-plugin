use std::collections::HashMap;

use crate::shared::structures::{
    application_order::ApplicationOrder,
    order::Order,
    order_pair::OrderPair,
    pair::Pair,
    pre_rule::{PreRuleValue, PreRules},
    stylex_options::StyleResolution,
    stylex_state_options::StyleXStateOptions,
};

fn logical_to_physical(input: &str) -> &str {
    match input {
        "start" => "left",
        "end" => "right",
        _ => input,
    }
}

fn property_to_ltr(pair: (&str, &str)) -> Pair {
    if pair.0 == "background-position" {
        let words: Vec<&str> = pair.1.split_whitespace().collect();
        let new_val = words
            .iter()
            .map(|&word| match word {
                "start" => "left",
                "end" => "right",
                _ => word,
            })
            .collect::<Vec<&str>>()
            .join(" ");
        return Pair {
            key: pair.0.to_string(),
            value: new_val,
        };
    };

    let result = match pair.0 {
        "margin-start" => ("margin-left", pair.1),
        "margin-end" => ("margin-right", pair.1),
        "padding-start" => ("padding-left", pair.1),
        "padding-end" => ("padding-right", pair.1),
        "border-start" => ("border-left", pair.1),
        "border-end" => ("border-right", pair.1),
        "border-start-width" => ("border-left-width", pair.1),
        "border-end-width" => ("border-right-width", pair.1),
        "border-start-color" => ("border-left-color", pair.1),
        "border-end-color" => ("border-right-color", pair.1),
        "border-start-style" => ("border-left-style", pair.1),
        "border-end-style" => ("border-right-style", pair.1),
        "border-top-start-radius" => ("border-top-left-radius", pair.1),
        "border-bottom-start-radius" => ("border-bottom-left-radius", pair.1),
        "border-top-end-radius" => ("border-top-right-radius", pair.1),
        "border-bottom-end-radius" => ("border-bottom-right-radius", pair.1),
        "text-align" => (pair.0, logical_to_physical(pair.1)),
        "float" => (pair.0, logical_to_physical(pair.1)),
        "clear" => (pair.0, logical_to_physical(pair.1)),
        "start" => ("left", pair.1),
        "end" => ("right", pair.1),

        _ => pair,
    };

    Pair {
        key: result.0.to_string(),
        value: result.1.to_string(),
    }
}

pub(crate) fn generate_ltr(pair: Pair) -> Pair {
    property_to_ltr((pair.key.as_str(), pair.value.as_str()))
}

fn flip_sign(value: String) -> String {
    if value == "0" {
        value.to_string()
    } else {
        if value.chars().next().unwrap() == '-' {
            value[1..].to_string()
        } else {
            format!("-{}", value)
        }
    }
}

fn flip_shadow(value: &str) -> Option<String> {
    eprint!("logic of flip_shadow is not implemented yet");
    let defs: Vec<&str> = value.split(',').collect();
    let mut built_defs = Vec::new();

    for def in defs {
        let mut parts = def
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let index = if parts[0].parse::<f32>().is_ok() {
            0
        } else {
            1
        };
        if index < parts.len() {
            let flipped = flip_sign(parts[index].clone());
            parts[index] = flipped;
        }
        built_defs.push(parts.join(" "));
    }

    let rtl = built_defs.join(", ");
    if rtl != value {
        Some(rtl)
    } else {
        None
    }
}

// fn flip_shadow(val: &str) -> Option<String> {
//   // Implement your flip_shadow function here
//   None
// }

fn shadows_flip(key: &str, val: &str) -> Option<Pair> {
    match key {
        "box-shadow" | "text-shadow" => {
            let rtl_val = flip_shadow(val);
            match rtl_val {
                Some(rtl_val) => Some(Pair {
                    key: key.to_string(),
                    value: rtl_val,
                }),
                None => None,
            }
        }
        _ => None,
    }
}

fn property_to_rtl(key: &str, val: &str) -> Option<Pair> {
    let logical_to_physical = [("start", "right"), ("end", "left")]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>();

    match key {
        "margin-start" => Some(Pair {
            key: "margin-right".to_string(),
            value: val.to_string(),
        }),
        "margin-end" => Some(Pair {
            key: "margin-left".to_string(),
            value: val.to_string(),
        }),
        "padding-start" => Some(Pair {
            key: "padding-right".to_string(),
            value: val.to_string(),
        }),
        "padding-end" => Some(Pair {
            key: "padding-left".to_string(),
            value: val.to_string(),
        }),
        "border-start" => Some(Pair {
            key: "border-right".to_string(),
            value: val.to_string(),
        }),
        "border-end" => Some(Pair {
            key: "border-left".to_string(),
            value: val.to_string(),
        }),
        // ... add the rest of your properties here ...
        "text-align" | "float" | "clear" => match logical_to_physical.get(val) {
            Some(&physical_val) => Some(Pair {
                key: key.to_string(),
                value: physical_val.to_string(),
            }),
            None => None,
        },
        "start" => Some(Pair {
            key: "right".to_string(),
            value: val.to_string(),
        }),
        "end" => Some(Pair {
            key: "left".to_string(),
            value: val.to_string(),
        }),
        "background-position" => {
            let words: Vec<&str> = val.split_whitespace().collect();
            if words.contains(&"start") || words.contains(&"end") {
                let new_val: String = words
                    .iter()
                    .map(|&word| match word {
                        "start" => "right",
                        "end" => "left",
                        _ => word,
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                Some(Pair {
                    key: key.to_string(),
                    value: new_val,
                })
            } else {
                None
            }
        }
        _ => shadows_flip(key, val),
    }
}

// fn generate_rtl(pair: Pair) -> Option<Pair> {
//   let property_to_rtl: HashMap<String, fn(&str) -> Option<String>> = [
//       ("margin-start".to_string(), flip_value),
//       // ... other properties ...
//   ].iter().cloned().collect();

//   if let Some(func) = property_to_rtl.get(&pair.key) {
//       if let Some(value) = func(&pair.value) {
//           Some(Pair { key: pair.key, value })
//       } else {
//           None
//       }
//   } else {
//       None
//   }
// }

fn flip_value(value: &PreRules) -> Option<PreRules> {
    // Implement your logic here to flip the value
    // For now, I'm just returning the same value
    Some(value.clone())
}

pub(crate) fn generate_rtl(pair: Pair) -> Option<Pair> {
    let result = property_to_rtl(pair.key.as_str(), pair.value.as_str());

    if let Some(pair) = result {
        Option::Some(pair)
    } else {
        Option::None
    }
}

pub(crate) fn flat_map_expanded_shorthands(
    obj_entry: (String, PreRuleValue),
    options: &StyleXStateOptions,
) -> Vec<OrderPair> {
    let (key, raw_value) = obj_entry.clone();

    let value = match raw_value {
        PreRuleValue::String(value) => Some(value),
        PreRuleValue::Vec(_) => {
            panic!("Cannot use fallbacks for shorthands. Use the expansion instead.")
        }
        PreRuleValue::Expr(_) => {
            panic!("Cannot use expressions for shorthands. Use the expansion instead.")
        }
        PreRuleValue::Null => None,
    };

    let key = if key.starts_with("var(") && key.ends_with(")") {
        key[4..key.len() - 1].to_string()
    } else {
        key
    };

    let expansion_fn = match &options.style_resolution {
        StyleResolution::ApplicationOrder => ApplicationOrder::get_expansion_fn(key.clone()),
        _ => todo!(),
    };

    if let Some(expansion_fn) = expansion_fn {
        return (expansion_fn)(value.clone());
    }

    let order_pair = OrderPair(key, value);

    let vec_order_pair: Vec<OrderPair> = vec![order_pair];

    vec_order_pair
}
