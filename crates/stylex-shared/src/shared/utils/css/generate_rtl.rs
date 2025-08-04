use crate::shared::{
  constants::{
    cursor_flip::CURSOR_FLIP,
    logical_to_rtl::{INLINE_TO_RTL, LOGICAL_TO_RTL},
  },
  regex::LENGTH_UNIT_TESTER_REGEX,
  structures::{
    pair::Pair, pre_rule::PreRules, stylex_options::StyleResolution,
    stylex_state_options::StyleXStateOptions,
  },
};

fn logical_to_physical_rtl(input: &str) -> Option<&str> {
  match input {
    "start" => Some("right"),
    "end" => Some("left"),
    "inline-start" => Some("right"),
    "inline-end" => Some("left"),
    _ => None,
  }
}

fn property_to_rtl(pair: &Pair, options: &StyleXStateOptions) -> Option<Pair> {
  if let Some(&ltr_key) = LOGICAL_TO_RTL.get(pair.key.as_str()) {
    return Some(Pair::new(ltr_key.to_string(), pair.value.clone()));
  }

  match pair.key.as_str() {
    "float" | "clear" => logical_to_physical_rtl(pair.value.as_str())
      .map(|value| Pair::new(pair.key.clone(), value.to_string())),
    "background-position" => {
      let new_val = pair
        .value
        .split_whitespace()
        .map(|word| match word {
          "start" | "inset-inline-start" => "right",
          "end" | "inset-inline-end" => "left",
          _ => word,
        })
        .collect::<Vec<_>>()
        .join(" ");
      Some(Pair::new(pair.key.clone(), new_val))
    }
    "cursor" => {
      if !options.enable_legacy_value_flipping {
        return None;
      }

      CURSOR_FLIP
        .get(pair.value.as_str())
        .map(|val| Pair::new(pair.key.clone(), val.to_string()))
    }
    _ => shadows_flip(pair.key.as_str(), pair.value.as_str(), options),
  }
}

pub(crate) fn generate_rtl(pair: &Pair, options: &StyleXStateOptions) -> Option<Pair> {
  let enable_logical_styles_polyfill = options.enable_logical_styles_polyfill;
  let style_resolution = &options.style_resolution;
  let key = pair.key.as_str();

  if style_resolution == &StyleResolution::LegacyExpandShorthands {
    if !enable_logical_styles_polyfill {
      if let Some(value) = legacy_values_polyfill(pair, key) {
        return Some(value);
      }

      return None;
    }

    if let Some(inline_to_rtl_value) = INLINE_TO_RTL.get(key) {
      return Some(Pair::new(
        inline_to_rtl_value.to_string(),
        pair.value.clone(),
      ));
    }
  }

  if let Some(value) = legacy_values_polyfill(pair, key) {
    return Some(value);
  }

  property_to_rtl(pair, options)
}

fn shadows_flip(key: &str, val: &str, options: &StyleXStateOptions) -> Option<Pair> {
  match key {
    "box-shadow" | "text-shadow" => {
      if !options.enable_legacy_value_flipping {
        return None;
      }

      let rtl_val = flip_shadow(val);
      rtl_val.map(|rtl_val| Pair::new(key.to_string(), rtl_val.to_string()))
    }
    _ => None,
  }
}

fn flip_shadow(value: &str) -> Option<String> {
  let defs: Vec<&str> = value.split(',').collect();
  let mut built_defs = Vec::new();

  for def in defs {
    let mut parts = def
      .split_whitespace()
      .map(|x| x.to_string())
      .collect::<Vec<String>>();

    // NOTE: temporary solution, need to implement unit parser
    let index = if is_unit(parts[0].as_str()) { 0 } else { 1 };

    if index < parts.len() {
      let flipped = flip_sign(&parts[index]);
      parts[index] = flipped;
    }
    built_defs.push(parts.join(" "));
  }

  let rtl = built_defs.join(",");
  if rtl != value { Some(rtl) } else { None }
}

fn is_unit(input: &str) -> bool {
  LENGTH_UNIT_TESTER_REGEX.is_match(input)
}

fn _flip_value(value: &PreRules) -> Option<PreRules> {
  // Implement your logic here to flip the value
  // For now, I'm just returning the same value
  Some(value.clone())
}

fn flip_sign(value: &str) -> String {
  if value == "0" {
    value.to_string()
  } else if value.starts_with('-') {
    value.strip_prefix('-').unwrap().to_string()
  } else {
    format!("-{}", value)
  }
}

fn legacy_values_polyfill(pair: &Pair, key: &str) -> Option<Pair> {
  if key == "float" || key == "clear" {
    let new_val = logical_to_physical_rtl(pair.value.as_str()).unwrap_or(pair.value.as_str());
    return Some(Pair::new(key.to_string(), new_val.to_string()));
  }
  None
}
