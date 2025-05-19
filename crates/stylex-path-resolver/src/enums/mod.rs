use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ExportsType {
  Simple(String),
  Complex(FxHashMap<String, String>),
}
