use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct InjectableStyleBase {
    pub(crate) rtl: Option<String>,
    pub(crate) ltr: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct InjectableStyle {
    pub(crate) priority: Option<u16>,
    pub(crate) ltr: String,
    pub(crate) rtl: Option<String>,
}

impl From<InjectableStyle> for InjectableStyleBase {
    fn from(style: InjectableStyle) -> Self {
        // Assuming InjectableStyleBase and InjectableStyle have similar fields
        InjectableStyleBase {
            ltr: style.ltr,
            rtl: style.rtl,
        }
    }
}
