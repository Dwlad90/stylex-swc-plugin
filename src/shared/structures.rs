use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct StyleWithDirections {
    pub(crate) rtl: Option<String>,
    pub(crate) ltr: String,
}

#[derive(Debug, Serialize, Deserialize)]

pub(crate) struct MetaData(
    pub(crate) String,
    pub(crate) StyleWithDirections,
    pub(crate) u16,
);
