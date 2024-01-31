use std::{fmt::Debug, sync::Arc};

use serde::{Deserialize, Serialize};

use super::pre_rule::{PreRule, PreRules};

#[derive(Debug, Clone)]
pub(crate) struct Pair {
    pub(crate) key: String,
    pub(crate) value: String,
}
