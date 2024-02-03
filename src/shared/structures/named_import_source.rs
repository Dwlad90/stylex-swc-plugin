use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct NamedImportSource {
    pub r#as: String,
    pub from: String,
}

#[derive(Deserialize, Clone, Debug)]
pub enum ImportSources {
    Regular(String),
    Named(NamedImportSource),
}

impl ImportSources {
    pub fn is_named_export(&self) -> bool {
        match self {
            ImportSources::Regular(_) => false,
            ImportSources::Named(named) => true,
        }
    }
}

