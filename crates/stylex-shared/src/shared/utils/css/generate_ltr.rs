use crate::shared::{
  constants::logical_to_ltr::{INLINE_PROPERTY_TO_LTR, PROPERTY_TO_LTR},
  structures::{
    pair::Pair, stylex_options::StyleResolution, stylex_state_options::StyleXStateOptions,
  },
};

pub(crate) fn generate_ltr(pair: &Pair, options: &StyleXStateOptions) -> Pair {
  let enable_logical_styles_polyfill = options.enable_logical_styles_polyfill;
  let style_resolution = &options.style_resolution;
  let key = pair.key.as_str();

  if style_resolution == &StyleResolution::LegacyExpandShorthands {
    if !enable_logical_styles_polyfill {
      return pair.clone();
    }

    if let Some(inline_to_ltr_value) = INLINE_PROPERTY_TO_LTR.get(key) {
      return Pair::new(inline_to_ltr_value.to_string(), pair.value.clone());
    }
  }

  property_to_ltr(pair)
}

fn logical_to_physical_ltr(input: &str) -> Option<&str> {
  match input {
    "start" | "inline-start" => Some("left"),
    "end" | "inline-end" => Some("right"),
    _ => None,
  }
}

fn property_to_ltr(pair: &Pair) -> Pair {
  match pair.key.as_str() {
    "background-position" => {
      let new_val = pair
        .value
        .split_whitespace()
        .map(|word| match word {
          "start" | "insetInlineStart" => "left",
          "end" | "insetInlineEnd" => "right",
          _ => word,
        })
        .collect::<Vec<_>>()
        .join(" ");
      Pair::new(pair.key.clone(), new_val)
    }
    "float" | "clear" => logical_to_physical_ltr(pair.value.as_str())
      .map(|value| Pair::new(pair.key.clone(), value.to_string()))
      .expect("Expected logical value for float/clear property"),
    k => {
      if let Some(&physical) = PROPERTY_TO_LTR.get(k) {
        Pair::new(physical.to_string(), pair.value.clone())
      } else {
        Pair::new(pair.key.clone(), pair.value.clone())
      }
    }
  }
}
