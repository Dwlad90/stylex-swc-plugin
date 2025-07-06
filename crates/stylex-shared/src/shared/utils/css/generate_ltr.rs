use crate::shared::{
  constants::logical_to_ltr::{INLINE_TO_LTR, LOGICAL_TO_LTR},
  structures::{
    pair::Pair, stylex_options::StyleResolution, stylex_state_options::StyleXStateOptions,
  },
};

pub(crate) fn generate_ltr(pair: &Pair, options: &StyleXStateOptions) -> Pair {
  dbg!(&pair);
  let enable_logical_styles_polyfill = options.enable_logical_styles_polyfill;
  let style_resolution = &options.style_resolution;
  let key = pair.key.as_str();

  if style_resolution == &StyleResolution::LegacyExpandShorthands {
    if !enable_logical_styles_polyfill {
      return pair.clone();
    }

    if let Some(inline_to_ltr_value) = INLINE_TO_LTR.get(key) {
      return Pair::new(inline_to_ltr_value.to_string(), pair.value.clone());
    }
  }

  if let Some(pair) = convert_to_standard_properties(pair.key.as_str(), pair.value.as_str()) {
    return pair;
  }

  property_to_ltr(pair)
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
    k => {
      if let Some(&physical) = LOGICAL_TO_LTR.get(k) {
        Pair::new(physical.to_string(), pair.value.clone())
      } else if k == "float" || k == "clear" {
        match logical_to_physical_ltr(pair.value.as_str()) {
          Some(value) => Pair::new(pair.key.clone(), value.to_string()),
          None => pair.clone(),
        }
      } else {
        Pair::new(pair.key.clone(), pair.value.clone())
      }
    }
  }
}

fn logical_to_physical_ltr(input: &str) -> Option<&str> {
  match input {
    "start" => Some("left"),
    "end" => Some("right"),
    _ => None,
  }
}

// Helper function to convert logical values to standard values
fn logical_to_standard_value(val: &str) -> &str {
  match val {
    "start" => "inline-start",
    "end" => "inline-end",
    _ => val,
  }
}

pub(crate) fn convert_to_standard_properties<'a>(key: &'a str, val: &'a str) -> Option<Pair> {
  match key {
    "float" | "clear" => Some(Pair::new(
      key.to_string(),
      logical_to_standard_value(val).to_string(),
    )),
    _ => None,
  }
}
