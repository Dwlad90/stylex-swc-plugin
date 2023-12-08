use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StyleWithDirections {
    pub rtl: Option<String>,
    pub ltr: String,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct MetaData(pub String, pub StyleWithDirections, pub u16);
