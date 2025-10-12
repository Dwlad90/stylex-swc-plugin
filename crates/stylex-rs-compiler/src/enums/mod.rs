use fancy_regex::Regex;
use log::warn;
use napi::{
  Env, Error, JsUnknown, NapiRaw,
  bindgen_prelude::{FromNapiValue, ToNapiValue},
  sys::{napi_env, napi_value},
};
use napi_derive::napi;
use stylex_shared::shared::{
  regex::NPM_NAME_REGEX, structures::named_import_source::NamedImportSource,
};

#[napi(object)]
#[derive(Debug, Clone)]
pub struct StyleXModuleResolution {
  pub r#type: String,
  pub root_dir: Option<String>,
  pub theme_file_extension: Option<String>,
}

#[napi(string_enum)]
#[derive(Debug)]
pub enum SourceMaps {
  True,
  False,
  Inline,
}

#[napi(object)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ImportSourceInput {
  #[serde(rename = "as")]
  pub as_: String,
  pub from: String,
}

#[derive(Debug, Clone)]
pub enum ImportSourceUnion {
  Regular(String),
  Named(NamedImportSource),
}

static MAX_IMPORT_PATH_LENGTH: usize = 214;

fn validate_import_path(path: &str) -> Result<(), String> {
  if path.len() > MAX_IMPORT_PATH_LENGTH {
    return Err(format!(
      "Import path is too long (max {} characters)",
      MAX_IMPORT_PATH_LENGTH
    ));
  }

  if !NPM_NAME_REGEX.is_match(path).unwrap_or_else(|err| {
    warn!(
      "Error matching NPM_NAME_REGEX for '{}': {}. Skipping pattern match.",
      path, err
    );

    false
  }) {
    return Err("Import path does not match required pattern".to_string());
  }
  Ok(())
}

impl FromNapiValue for ImportSourceUnion {
  unsafe fn from_napi_value(env: napi_env, value: napi::sys::napi_value) -> Result<Self, Error> {
    let js_unknown = unsafe { JsUnknown::from_napi_value(env, value) }?;
    let js_obj = js_unknown.coerce_to_object()?;

    match unsafe { ImportSourceInput::from_napi_value(env, js_obj.raw()) } {
      Ok(value) => {
        validate_import_path(&value.from).map_err(Error::from_reason)?;
        Ok(ImportSourceUnion::Named(NamedImportSource {
          r#as: value.as_,
          from: value.from,
        }))
      }
      Err(_) => {
        let js_unknown = unsafe { JsUnknown::from_napi_value(env, value) }?;

        let js_str = js_unknown.coerce_to_string()?;
        let import_path = js_str.into_utf8()?.as_str()?.to_owned();

        validate_import_path(&import_path).map_err(Error::from_reason)?;
        Ok(ImportSourceUnion::Regular(import_path))
      }
    }
  }
}

impl ToNapiValue for ImportSourceUnion {
  unsafe fn to_napi_value(env: napi_env, value: Self) -> Result<napi_value, Error> {
    match value {
      ImportSourceUnion::Regular(s) => {
        let env = unsafe { Env::from_raw(env) };
        let js_str = env.create_string(&s)?;
        Ok(unsafe { js_str.raw() })
      }
      ImportSourceUnion::Named(named) => {
        let env = unsafe { Env::from_raw(env) };
        let mut js_obj = env.create_object()?;

        let as_str = env.create_string(&named.r#as)?;
        let from_str = env.create_string(&named.from)?;

        js_obj.set_named_property("as", as_str)?;
        js_obj.set_named_property("from", from_str)?;

        Ok(unsafe { js_obj.raw() })
      }
    }
  }
}

#[derive(Debug, Clone)]
pub enum PathFilterUnion {
  Glob(String),
  Regex(String),
}

impl PathFilterUnion {
  pub fn from_string(pattern: &str) -> Self {
    if pattern.starts_with('/') && pattern.len() > 2 {
      // Find the last unescaped slash to handle patterns like /path\/to\/file/
      let mut last_slash_pos = None;
      let chars: Vec<char> = pattern.chars().collect();

      for i in (1..chars.len()).rev() {
        if chars[i] == '/' {
          // Check if this slash is escaped (preceded by odd number of backslashes)
          let mut backslash_count = 0;
          let mut j = i;
          while j > 0 && chars[j - 1] == '\\' {
            backslash_count += 1;
            j -= 1;
          }

          // If even number of backslashes (including 0), the slash is not escaped
          if backslash_count % 2 == 0 {
            last_slash_pos = Some(i);
            break;
          }
        }
      }

      if let Some(last_slash) = last_slash_pos {
        // Extract the regex pattern (without the surrounding slashes)
        let regex_pattern = &pattern[1..last_slash];
        let flags = &pattern[last_slash + 1..];

        // Validate regex flags (only valid JS regex flags: gimsuy)
        if flags
          .chars()
          .all(|c| matches!(c, 'g' | 'i' | 'm' | 's' | 'u' | 'y'))
        {
          // Try to validate the regex pattern
          if Regex::new(regex_pattern).is_ok() {
            // Convert JS flags to inline modifiers for Rust regex
            let mut inline_flags = String::new();
            if flags.contains('i') {
              inline_flags.push('i');
            }
            if flags.contains('m') {
              inline_flags.push('m');
            }
            if flags.contains('s') {
              inline_flags.push('s');
            }

            let final_pattern = if !inline_flags.is_empty() {
              format!("(?{}){}", inline_flags, regex_pattern)
            } else {
              regex_pattern.to_string()
            };

            return PathFilterUnion::Regex(final_pattern);
          }
        }
      }
    }

    // Default to glob pattern
    PathFilterUnion::Glob(pattern.to_string())
  }
}
