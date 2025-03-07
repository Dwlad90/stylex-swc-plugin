use core::panic;
use indexmap::IndexSet;
use log::{debug, warn};
use once_cell::sync::Lazy;
use path_clean::PathClean;
use regex::Regex;
use rustc_hash::FxHashMap;
use std::path::{Path, PathBuf};
use swc_core::{
  common::FileName,
  ecma::loader::{
    resolve::{Resolution, Resolve},
    resolvers::node::NodeModulesResolver,
  },
};

use std::fs;

use crate::{
  file_system::{get_directories, get_directory_path_recursive},
  package_json::{
    PackageJsonExtended, find_closest_node_modules, find_closest_package_json_folder,
    get_package_json, get_package_json_with_deps, recursive_find_node_modules,
    resolve_package_from_package_json,
  },
  utils::{contains_subpath, relative_path},
};

mod tests;

pub const EXTENSIONS: [&str; 8] = [".tsx", ".ts", ".jsx", ".js", ".mjs", ".cjs", ".mdx", ".md"];

pub static FILE_PATTERN: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"\.(jsx?|tsx?|mdx?|mjs|cjs)$"#).unwrap());

pub fn resolve_path(
  processing_file: &Path,
  root_dir: &Path,
  package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
) -> String {
  if !FILE_PATTERN.is_match(processing_file.to_str().unwrap()) {
    let processing_path = if cfg!(test) {
      processing_file
        .strip_prefix(root_dir.parent().unwrap().parent().unwrap())
        .unwrap()
        .to_path_buf()
    } else {
      processing_file.to_path_buf()
    };

    panic!(
      r#"Resolve path must be a file, but got: {}"#,
      processing_path.display()
    );
  }

  let cwd = if cfg!(test) {
    root_dir.to_path_buf()
  } else {
    "cwd".into()
  };

  let mut path_by_package_json =
    match resolve_from_package_json(processing_file, root_dir, &cwd, package_json_seen) {
      Ok(value) => value,
      Err(value) => return value,
    };

  if path_by_package_json.starts_with(&cwd) {
    path_by_package_json = path_by_package_json
      .strip_prefix(cwd)
      .unwrap()
      .to_path_buf();
  }

  let resolved_path_by_package_name = path_by_package_json.clean().display().to_string();

  if cfg!(test) {
    let cwd_resolved_path = format!("{}/{}", root_dir.display(), resolved_path_by_package_name);

    assert!(
      fs::metadata(&cwd_resolved_path).is_ok(),
      "Path resolution failed: {}",
      resolved_path_by_package_name
    );
  }

  resolved_path_by_package_name
}

fn resolve_from_package_json(
  processing_file: &Path,
  root_dir: &Path,
  cwd: &Path,
  package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
) -> Result<PathBuf, String> {
  let resolved_path = match processing_file.strip_prefix(root_dir) {
    Ok(stripped) => stripped.to_path_buf(),
    Err(_) => {
      let processing_file_str = processing_file.to_string_lossy();

      if let Some(node_modules_index) = processing_file_str.rfind("node_modules") {
        // NOTE: This is a workaround for the case when the file is located in the node_modules directory and pnpm is package manager

        let resolved_path_from_node_modules = processing_file_str
          .split_at(node_modules_index)
          .1
          .to_string();

        if !resolved_path_from_node_modules.is_empty() {
          return Err(resolved_path_from_node_modules);
        }
      }

      let relative_package_path = relative_path(processing_file, root_dir);

      get_package_path_by_package_json(cwd, &relative_package_path, package_json_seen)
    }
  };

  Ok(resolved_path)
}

fn get_package_path_by_package_json(
  cwd: &Path,
  relative_package_path: &Path,
  package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
) -> PathBuf {
  let (resolver, package_dependencies) = get_package_json_with_deps(cwd, package_json_seen);

  let mut potential_package_path: PathBuf = PathBuf::default();

  for (name, _) in package_dependencies.iter() {
    let file_name = FileName::Real(cwd.to_path_buf());

    let potential_path_section = name.split("/").last().unwrap_or_default();

    if contains_subpath(relative_package_path, Path::new(&potential_path_section)) {
      let relative_package_path_str = relative_package_path.display().to_string();

      let potential_file_path = relative_package_path_str
        .split(potential_path_section)
        .last()
        .unwrap_or_default();

      if !potential_file_path.is_empty()
        || relative_package_path_str.ends_with(format!("/{}", potential_path_section).as_str())
      {
        let resolved_node_modules_path =
          get_node_modules_path(&resolver, &file_name, name, package_json_seen);

        if let Some(resolved_node_modules_path) = resolved_node_modules_path {
          if let FileName::Real(real_resolved_node_modules_path) =
            resolved_node_modules_path.filename
          {
            potential_package_path = resolve_exports_path(
              &real_resolved_node_modules_path,
              Path::new(potential_file_path),
              package_json_seen,
            );
          }
        }

        if potential_package_path.as_os_str().is_empty() {
          potential_package_path =
            PathBuf::from(format!("node_modules/{}{}", name, potential_file_path));
        }

        break;
      }
    }
  }

  potential_package_path
}

fn resolve_exports_path(
  real_resolved_node_modules_path: &Path,
  potential_file_path: &Path,
  package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
) -> PathBuf {
  let (potential_package_json, _) =
    get_package_json(real_resolved_node_modules_path, package_json_seen);

  match &potential_package_json.exports {
    Some(exports) => resolve_package_json_exports(
      potential_file_path,
      exports,
      real_resolved_node_modules_path,
    ),
    None => {
      let node_modules_regex = Regex::new(r".*node_modules").unwrap();

      node_modules_regex
        .replace(
          real_resolved_node_modules_path
            .display()
            .to_string()
            .as_str(),
          "node_modules",
        )
        .to_string()
        .into()
    }
  }
}

pub(crate) fn get_node_modules_path(
  resolver: &NodeModulesResolver,
  file_name: &FileName,
  name: &str,
  package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
) -> Option<swc_core::ecma::loader::resolve::Resolution> {
  {
    match resolver.resolve(file_name, name) {
      Ok(resolution) => {
        if let FileName::Real(real_filename) = &resolution.filename {
          if real_filename.to_string_lossy().contains("node_modules/") {
            return Some(resolution);
          }
        }
        None
      }
      Err(_) => get_potential_node_modules_path(file_name, name, package_json_seen),
    }
  }
}

fn get_potential_node_modules_path(
  file_name: &FileName,
  name: &str,
  package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
) -> Option<Resolution> {
  let file_name_real = if let FileName::Real(real_filename) = file_name {
    real_filename
  } else {
    return None;
  };

  let potential_package_path = PathBuf::from(format!(
    "{}/{}",
    find_closest_node_modules(file_name_real)
      .unwrap_or(file_name_real.clone())
      .to_string_lossy(),
    name
  ));

  if let Some(resolved_potential_package_path) =
    get_directory_path_recursive(&potential_package_path)
  {
    let (potential_package_json, _) =
      get_package_json(&resolved_potential_package_path, package_json_seen);

    let package_name = potential_package_json
      .name
      .unwrap_or_else(|| panic!("Package name is not found in package.json of '{}'", name))
      .clone();

    let potential_import_path_segment = name.split(&package_name).last().unwrap_or_default();

    let potential_package_path = resolve_exports_path(
      &resolved_potential_package_path,
      Path::new(potential_import_path_segment),
      package_json_seen,
    );

    let file_name_real_lossy = file_name_real.to_string_lossy();
    let root_subst_file_name = file_name_real_lossy.split("node_modules").next().unwrap();

    let path = Path::new(&potential_package_path);

    let stripped_path = path.strip_prefix(root_subst_file_name).unwrap_or(path);

    return Some(Resolution {
      filename: FileName::Real(stripped_path.to_path_buf()),
      slug: None,
    });
  }

  None
}

fn resolve_package_json_exports(
  potential_import_segment_path: &Path,
  exports: &FxHashMap<String, String>,
  resolved_node_modules_path: &Path,
) -> PathBuf {
  let mut result: PathBuf = PathBuf::default();

  let import_segment_path_without_extension = PathBuf::from(potential_import_segment_path)
    .with_extension("")
    .display()
    .to_string();

  let mut exports_values: Vec<&String> = exports.values().collect();

  exports_values.sort_by_key(|k| (k.to_string(), k.len()));

  let resolved_package_path = find_closest_package_json_folder(resolved_node_modules_path)
    .unwrap_or_else(|| {
      panic!(
        "package.json not found near: {}",
        resolved_node_modules_path.display()
      )
    });

  for export_value in exports_values {
    if export_value.contains(&import_segment_path_without_extension) {
      result = resolved_package_path.join(export_value);

      break;
    }
  }

  if result.components().count() == 0 {
    let mut keys: Vec<&String> = exports.keys().collect();
    keys.sort_by_key(|k| (k.to_string(), k.len()));

    for key in keys {
      if key.contains(&import_segment_path_without_extension) {
        result = resolved_package_path.join(exports.get(key).unwrap());
      }
    }
  }

  if result.components().count() == 0 {
    warn!(
      "Unfortunatly, the exports field is not yet fully supported, so path resolving may work not as expected"
    );
    // TODO: implement exports field resolution
  }

  result
}

pub fn resolve_file_path(
  import_path_str: &str,
  source_file_path: &str,
  root_path: &str,
  aliases: &FxHashMap<String, Vec<String>>,
  package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
) -> std::io::Result<PathBuf> {
  let source_file_dir = Path::new(source_file_path).parent().unwrap();
  let root_path = Path::new(root_path);

  let cwd_path = Path::new(root_path);

  let resolved_file_paths = if import_path_str.starts_with('.') {
    if FILE_PATTERN.is_match(import_path_str) {
      vec![PathBuf::from(resolve_path(
        &source_file_dir.join(import_path_str),
        root_path,
        package_json_seen,
      ))]
    } else {
      EXTENSIONS
        .iter()
        .map(|ext| {
          let import_path_str_with_ext = format!("{}{}", import_path_str, ext);

          PathBuf::from(resolve_path(
            &source_file_dir.join(import_path_str_with_ext),
            root_path,
            package_json_seen,
          ))
        })
        .collect()
    }
  } else if import_path_str.starts_with('/') {
    vec![root_path.join(import_path_str)]
  } else {
    let mut possible_file_paths = possible_aliased_paths(import_path_str, aliases)
      .iter()
      .map(PathBuf::from)
      .collect::<IndexSet<PathBuf>>();

    if !import_path_str.is_empty() {
      possible_file_paths.insert(Path::new("node_modules").join(import_path_str));

      let closest_node_modules_paths = recursive_find_node_modules(source_file_dir, None);

      for closest_node_modules_path in closest_node_modules_paths.iter() {
        let closest_node_modules_path = closest_node_modules_path.join(import_path_str);

        possible_file_paths.insert(closest_node_modules_path.clone());

        possible_file_paths.extend(resolve_package_with_node_modules_path(
          closest_node_modules_path,
          import_path_str,
          source_file_dir,
          package_json_seen,
        ));
      }
    }

    if let Ok(resolved_node_modules_path_buf) =
      resolve_node_modules_path_buff(cwd_path, import_path_str, package_json_seen)
    {
      possible_file_paths.extend(resolve_package_with_node_modules_path(
        resolved_node_modules_path_buf,
        import_path_str,
        source_file_dir,
        package_json_seen,
      ));
    }

    possible_file_paths.into_iter().collect()
  };

  let resolved_potential_file_paths = resolved_file_paths
    .iter()
    .filter(|path| path.as_path() != Path::new("."))
    .collect::<Vec<&PathBuf>>();

  debug!(
    "Resolved potential paths: {:?} for import `{}`",
    resolved_potential_file_paths, import_path_str
  );

  for ext in EXTENSIONS.iter() {
    for resolved_file_path in resolved_potential_file_paths.iter() {
      let mut resolved_file_path = resolved_file_path.clean();

      if let Some(extension) = resolved_file_path.extension() {
        let subpath = extension.to_string_lossy();
        if EXTENSIONS
          .iter()
          .all(|ext| !ext.ends_with(subpath.as_ref()))
        {
          resolved_file_path.set_extension(format!("{}{}", subpath, ext));
        }
      } else {
        resolved_file_path.set_extension(ext.trim_start_matches("."));
      }

      let cleaned_path = resolved_file_path.to_string_lossy().to_string();

      let path_to_check: PathBuf;
      let node_modules_path_to_check: PathBuf;

      if !cleaned_path.contains(root_path.to_str().expect("root path is not valid")) {
        if !cleaned_path.starts_with("node_modules") {
          node_modules_path_to_check = cwd_path.join("node_modules").join(&cleaned_path);
        } else {
          node_modules_path_to_check = cwd_path.join(&cleaned_path);
        }
        path_to_check = cwd_path.join(cleaned_path);
      } else {
        path_to_check = PathBuf::from(&cleaned_path);
        node_modules_path_to_check = path_to_check.clone();
      }

      if fs::metadata(&path_to_check).is_ok() {
        return Ok(path_to_check.to_path_buf().clean());
      }

      if fs::metadata(&node_modules_path_to_check).is_ok() {
        return Ok(node_modules_path_to_check.to_path_buf().clean());
      }
    }
  }

  Err(std::io::Error::new(
    std::io::ErrorKind::NotFound,
    "File not found",
  ))
}

fn resolve_package_with_node_modules_path(
  mut resolved_node_modules_path_buf: PathBuf,
  import_path_str: &str,
  source_file_dir: &Path,
  package_json_seen: &mut std::collections::HashMap<
    String,
    PackageJsonExtended,
    rustc_hash::FxBuildHasher,
  >,
) -> IndexSet<PathBuf> {
  let mut aliased_file_paths: IndexSet<PathBuf> = IndexSet::new();

  let (mut package_json, _) = get_package_json(&resolved_node_modules_path_buf, package_json_seen);

  let package_name = package_json.name.clone().unwrap_or_else(|| {
    panic!(
      "Package name is not found in package.json of '{}'",
      import_path_str
    )
  });

  if let Some((pnpm_package_json, pnpm_package_path)) = resolve_package_with_pnpm_path(
    source_file_dir,
    &package_name,
    import_path_str,
    package_json_seen,
  ) {
    package_json = pnpm_package_json;
    resolved_node_modules_path_buf = pnpm_package_path;
  }

  let potential_import_path_segment = import_path_str
    .split(&package_name)
    .last()
    .unwrap_or_default();

  let import_path_segment = if potential_import_path_segment.is_empty() {
    package_name
  } else {
    potential_import_path_segment.to_string()
  };

  if let Some(exports) = &package_json.exports {
    let potential_package_path = resolve_package_json_exports(
      Path::new(&import_path_segment),
      exports,
      &resolved_node_modules_path_buf,
    );

    if !potential_package_path.as_os_str().is_empty() {
      aliased_file_paths.insert(Path::new(&potential_package_path).to_path_buf().clean());
    }
  }

  if !resolved_node_modules_path_buf.as_os_str().is_empty()
    && !resolved_node_modules_path_buf.ends_with("node_modules")
  {
    aliased_file_paths.insert(resolved_node_modules_path_buf.clean());
  }

  aliased_file_paths
}

fn resolve_package_with_pnpm_path(
  source_file_dir: &Path,
  package_name: &String,
  import_path_str: &str,
  package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
) -> Option<(PackageJsonExtended, PathBuf)> {
  let closest_package_json_path = find_closest_package_json_folder(&PathBuf::from(source_file_dir));
  if let Some(closest_package_json_directory) = closest_package_json_path {
    if let Ok(directories) =
      get_directories(&closest_package_json_directory.join("node_modules/.pnpm"))
    {
      let normalized_name = if package_name.starts_with('@') {
        package_name.replace('/', "+")
      } else {
        package_name.to_string()
      };

      for path in directories.iter() {
        if path.to_string_lossy().contains(&normalized_name) {
          if let Ok(resolved_node_modules_path_buff) =
            resolve_node_modules_path_buff(path, import_path_str, package_json_seen)
          {
            let (package_json, _) = get_package_json(path, package_json_seen);

            return Some((package_json, resolved_node_modules_path_buff));
          };
        }
      }
    }
  };

  None
}

fn resolve_node_modules_path_buff(
  path: &Path,
  import_path_str: &str,
  package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
) -> Result<PathBuf, std::io::Error> {
  let (resolver, _) = get_package_json_with_deps(path, package_json_seen);

  if let Some(package_resolution) = resolve_package_from_package_json(
    &resolver,
    &FileName::Real(path.to_path_buf()),
    import_path_str,
    package_json_seen,
  ) {
    if let FileName::Real(resolved_node_modules_path_buf) = package_resolution.filename {
      return Ok::<PathBuf, std::io::Error>(resolved_node_modules_path_buf);
    }
  }

  Result::Err(std::io::Error::new(
    std::io::ErrorKind::NotFound,
    "File not found",
  ))
}

fn possible_aliased_paths(
  import_path_str: &str,
  aliases: &FxHashMap<String, Vec<String>>,
) -> Vec<PathBuf> {
  let mut result = vec![PathBuf::from(import_path_str)];

  if aliases.is_empty() {
    return result;
  }

  for (alias, values) in aliases.iter() {
    if let Some((before, after)) = alias.split_once('*') {
      if import_path_str.starts_with(before) && import_path_str.ends_with(after) {
        let replacement_string =
          &import_path_str[before.len()..import_path_str.len() - after.len()];
        for value in values {
          result.push(PathBuf::from(value.replace('*', replacement_string)));
        }
      }
    } else if alias == import_path_str {
      result.extend(values.iter().map(PathBuf::from));
    }
  }

  result
}
