use log::warn;
use napi::{
  Env, Error, JsValue, NapiRaw, Unknown,
  bindgen_prelude::{FromNapiValue, ToNapiValue},
  sys::{napi_env, napi_value},
};
use napi_derive::napi;
use stylex_regex::regex::NPM_NAME_REGEX;
use stylex_structures::named_import_source::NamedImportSource;

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

#[derive(Debug, Clone)]
pub enum RuntimeInjectionUnion {
  Boolean(bool),
  Regular(String),
}

impl FromNapiValue for RuntimeInjectionUnion {
  unsafe fn from_napi_value(env: napi_env, value: napi::sys::napi_value) -> Result<Self, Error> {
    // Try to parse as boolean first
    if let Ok(bool_value) = unsafe { bool::from_napi_value(env, value) } {
      return Ok(RuntimeInjectionUnion::Boolean(bool_value));
    }

    // Fall back to string
    let js_unknown = unsafe { Unknown::from_napi_value(env, value) }?;
    let js_str = unsafe { js_unknown.cast::<napi::JsString>() }?;
    let string_value = js_str.into_utf8()?.as_str()?.to_owned();

    Ok(RuntimeInjectionUnion::Regular(string_value))
  }
}

impl ToNapiValue for RuntimeInjectionUnion {
  unsafe fn to_napi_value(env: napi_env, value: Self) -> Result<napi_value, Error> {
    match value {
      RuntimeInjectionUnion::Boolean(b) => unsafe { bool::to_napi_value(env, b) },
      RuntimeInjectionUnion::Regular(s) => {
        let env = Env::from_raw(env);
        let js_str = env.create_string(&s)?;
        Ok(js_str.raw())
      },
    }
  }
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
    let js_unknown = unsafe { Unknown::from_napi_value(env, value) }?;
    // SAFETY: This cast will fail if the value is not actually a JsObject,
    // which is handled by the Err branch below.
    let js_obj = unsafe { js_unknown.cast::<napi::JsObject>() };

    match js_obj {
      Ok(obj) => match unsafe { ImportSourceInput::from_napi_value(env, obj.raw()) } {
        Ok(value) => {
          validate_import_path(&value.from).map_err(Error::from_reason)?;
          Ok(ImportSourceUnion::Named(NamedImportSource {
            r#as: value.as_,
            from: value.from,
          }))
        },
        Err(_) => {
          let js_unknown = unsafe { Unknown::from_napi_value(env, value) }?;
          let js_str = unsafe { js_unknown.cast::<napi::JsString>() }?;
          let import_path = js_str.into_utf8()?.as_str()?.to_owned();

          validate_import_path(&import_path).map_err(Error::from_reason)?;
          Ok(ImportSourceUnion::Regular(import_path))
        },
      },
      Err(_) => {
        let js_unknown = unsafe { Unknown::from_napi_value(env, value) }?;
        let js_str = unsafe { js_unknown.cast::<napi::JsString>() }?;
        let import_path = js_str.into_utf8()?.as_str()?.to_owned();

        validate_import_path(&import_path).map_err(Error::from_reason)?;
        Ok(ImportSourceUnion::Regular(import_path))
      },
    }
  }
}

impl ToNapiValue for ImportSourceUnion {
  unsafe fn to_napi_value(env: napi_env, value: Self) -> Result<napi_value, Error> {
    match value {
      ImportSourceUnion::Regular(s) => {
        let env = Env::from_raw(env);
        let js_str = env.create_string(&s)?;
        Ok(js_str.raw())
      },
      ImportSourceUnion::Named(named) => {
        let env = Env::from_raw(env);
        let mut js_obj = env.create_object()?;

        let as_str = env.create_string(&named.r#as)?;
        let from_str = env.create_string(&named.from)?;

        js_obj.set_named_property("as", as_str)?;
        js_obj.set_named_property("from", from_str)?;

        Ok(unsafe { js_obj.raw() })
      },
    }
  }
}

#[napi(string_enum)]
#[derive(Debug)]
pub enum PropertyValidationMode {
  #[napi(value = "throw")]
  Throw,
  #[napi(value = "warn")]
  Warn,
  #[napi(value = "silent")]
  Silent,
}

/// Represents the `sxPropName` option: a string name for the sx prop, or
/// `false` to disable.
#[derive(Debug, Clone)]
pub enum SxPropNameUnion {
  /// Disables the `sx` prop feature
  Disabled,
  /// A string name for the sx prop (e.g. `"sx"` or `"css"`)
  Name(String),
}

impl FromNapiValue for SxPropNameUnion {
  unsafe fn from_napi_value(env: napi_env, value: napi::sys::napi_value) -> Result<Self, Error> {
    // Try to parse as boolean first
    if let Ok(bool_value) = unsafe { bool::from_napi_value(env, value) } {
      // Only allow `false` to disable the feature - `true` is an error
      if bool_value {
        return Err(Error::from_reason(
          "sxPropName does not accept `true` - use `false` to disable or provide a string prop name",
        ));
      }
      return Ok(SxPropNameUnion::Disabled);
    }

    // Fall back to string
    let js_unknown = unsafe { Unknown::from_napi_value(env, value) }?;
    let js_str = unsafe { js_unknown.cast::<napi::JsString>() }?;
    let string_value = js_str.into_utf8()?.as_str()?.to_owned();

    Ok(SxPropNameUnion::Name(string_value))
  }
}

impl ToNapiValue for SxPropNameUnion {
  unsafe fn to_napi_value(env: napi_env, value: Self) -> Result<napi_value, Error> {
    match value {
      SxPropNameUnion::Disabled => unsafe { bool::to_napi_value(env, false) },
      SxPropNameUnion::Name(s) => {
        let env = Env::from_raw(env);
        let js_str = env.create_string(&s)?;
        Ok(js_str.raw())
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn validate_import_path_accepts_valid_npm_names() {
    assert!(validate_import_path("@scope/pkg-name").is_ok());
    assert!(validate_import_path("stylex").is_ok());
  }

  #[test]
  fn validate_import_path_rejects_too_long_values() {
    let long_path = "a".repeat(MAX_IMPORT_PATH_LENGTH + 1);
    let error = validate_import_path(&long_path).unwrap_err();
    assert!(error.contains("too long"));
  }

  #[test]
  fn validate_import_path_rejects_invalid_pattern() {
    let error = validate_import_path("Invalid Package Name!").unwrap_err();
    assert!(error.contains("required pattern"));
  }
}
