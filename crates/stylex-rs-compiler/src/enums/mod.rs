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

  if !NPM_NAME_REGEX.is_match(path) {
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
