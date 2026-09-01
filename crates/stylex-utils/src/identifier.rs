//! Naming one export of one file.

/// The identifier that names `export_name` of `file_name`, with `key` naming
/// one member inside that export where there is one.
///
/// Two files that export the same name never collide, because the file path is
/// part of the name. The un-hashed sibling of [`crate::hash::create_key_hash`]:
/// this one stays readable, and is hashed later where a short name is needed.
pub fn gen_file_based_identifier(file_name: &str, export_name: &str, key: Option<&str>) -> String {
  let key = key.map_or(String::new(), |k| format!(".{}", k));

  format!("{}//{}{}", file_name, export_name, key)
}

#[cfg(test)]
#[path = "tests/identifier_test.rs"]
mod tests;
