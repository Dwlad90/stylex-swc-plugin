use std::{collections::HashMap, path::PathBuf};

use swc_core::common::FileName;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PluginPass {
    // Assuming BabelFile is a struct in your code
    // file: &BabelFile;
    // pub(crate) key: String,
    // pub(crate) opts: HashMap<String, String>, // Assuming opts is a HashMap
    pub cwd: Option<PathBuf>,
    pub filename: FileName,
}

trait TraitName {
    fn get(&self, key: &str) -> Option<&str>; // Assuming the key is a string and the value is a string
    fn set(&mut self, key: &str, value: &str); // Assuming the key is a string and the value is a string
}

impl TraitName for PluginPass {
    fn get(&self, key: &str) -> Option<&str> {
        Option::None
    } // Assuming the key is a string and the value is a string
    fn set(&mut self, key: &str, value: &str) {} // Assuming the key is a string and the value is a string
}

impl Default for PluginPass {
    fn default() -> Self {
        Self {
            cwd: Option::None,
            filename: FileName::Anon,
        }
    }
}
