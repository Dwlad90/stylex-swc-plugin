//! The crate layer ladder in `guidelines/STRUCTURE.md`, checked against the
//! manifests.
//!
//! The ladder is what this crate links: a layer is the longest path from a crate
//! down to one with no workspace dependency, and a crate the addon never reaches
//! has no rung at all. Both facts belong to this crate, which is why the check
//! lives here rather than in a crate the ladder merely mentions.
//!
//! Every hand-maintained copy of this data has drifted before. The mermaid
//! diagrams that preceded the ladder drew edges that never existed, and the
//! ladder itself documented five crates above their real depth -- one of them
//! seven rungs high, which told a reader that CSS generation may call the
//! evaluator. Nothing warned about either, because nothing read the manifests.

use std::{
  collections::{BTreeMap, BTreeSet},
  fs,
  path::{Path, PathBuf},
};

/// The document that carries the ladder, relative to the workspace root.
const LADDER_DOCUMENT: &str = "guidelines/STRUCTURE.md";

/// The workspace root, resolved from this crate's own directory rather than from
/// the working directory, which `cargo test` does not promise.
fn workspace_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .ancestors()
    .nth(2)
    .expect("this crate sits two levels below the workspace root")
    .to_path_buf()
}

/// This crate's own directory name, which is the top of the ladder.
fn addon_directory() -> String {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .file_name()
    .and_then(|name| name.to_str())
    .expect("the crate directory has a name")
    .to_string()
}

/// The `[dependencies]` lines of a manifest, without the dev, build and target
/// tables.
///
/// Dev dependencies are deliberately out: a test that reaches sideways says
/// nothing about what the compiler links, and counting one would put a crate
/// above the layer its shipped code sits at.
fn dependency_lines(manifest: &str) -> String {
  let mut lines = String::new();
  let mut inside = false;

  for line in manifest.lines() {
    if line.starts_with('[') {
      // `[dependencies.name]` is a legal spelling this reader does not
      // understand, and skipping it would silently drop an edge.
      assert!(
        !line.starts_with("[dependencies."),
        "{line} is a dependency table this reader cannot parse; teach it the form"
      );

      inside = line.starts_with("[dependencies]");
      continue;
    }

    if inside {
      lines.push_str(line);
      lines.push('\n');
    }
  }

  lines
}

/// Whether `lines` declares `package` as a dependency.
///
/// The name has to be followed by `=` or `.`, so a package whose name is the
/// prefix of another's is not read as an edge: `stylex_state_index = { … }`
/// leaves `_index = { … }` behind and answers `false` for `stylex_state`.
fn names_dependency(lines: &str, package: &str) -> bool {
  lines.lines().any(|line| {
    line.trim_start().strip_prefix(package).is_some_and(|rest| {
      let rest = rest.trim_start();
      rest.starts_with('=') || rest.starts_with('.')
    })
  })
}

/// Every crate under `crates/`, as directory name to the directory names it
/// depends on.
fn read_edges(crates: &Path) -> BTreeMap<String, BTreeSet<String>> {
  let mut manifests = BTreeMap::new();

  let entries = fs::read_dir(crates)
    .unwrap_or_else(|error| panic!("cannot read {}: {error}", crates.display()));

  for entry in entries {
    let entry = entry.unwrap_or_else(|error| panic!("cannot read a crate directory: {error}"));
    let manifest_path = entry.path().join("Cargo.toml");

    if !manifest_path.is_file() {
      continue;
    }

    let directory = entry
      .file_name()
      .to_str()
      .unwrap_or_else(|| panic!("{} is not a usable directory name", entry.path().display()))
      .to_string();
    let manifest = fs::read_to_string(&manifest_path)
      .unwrap_or_else(|error| panic!("cannot read {}: {error}", manifest_path.display()));
    let package = manifest
      .lines()
      .find_map(|line| line.strip_prefix("name = \""))
      .and_then(|rest| rest.strip_suffix('"'))
      .unwrap_or_else(|| panic!("{} declares no package name", manifest_path.display()))
      .to_string();

    manifests.insert(directory, (package, dependency_lines(&manifest)));
  }

  assert!(
    !manifests.is_empty(),
    "found no crate manifests under {}",
    crates.display()
  );

  edges_of(&manifests)
}

/// The dependency edges of already-read manifests, keyed by directory name.
///
/// Split from [`read_edges`] so the cases below can hand it manifests of their
/// own instead of a directory tree.
fn edges_of(manifests: &BTreeMap<String, (String, String)>) -> BTreeMap<String, BTreeSet<String>> {
  manifests
    .iter()
    .map(|(directory, (_, lines))| {
      let named = manifests
        .iter()
        .filter(|(other, (package, _))| *other != directory && names_dependency(lines, package))
        .map(|(other, _)| other.clone())
        .collect();

      (directory.clone(), named)
    })
    .collect()
}

/// The longest path from each crate down to a dependency-free one.
///
/// Longest rather than shortest, because the rule the ladder states is that a
/// crate depends only on lower layers: it has to sit above the deepest thing it
/// reaches, not above the shallowest.
///
/// `Err` names a crate a cycle reaches. Cargo rejects a cycle, so this cannot
/// happen against the real manifests -- it is here so a walk over a bad set
/// stops instead of recursing until the stack runs out.
fn layers_of(
  edges: &BTreeMap<String, BTreeSet<String>>,
) -> Result<BTreeMap<String, usize>, String> {
  let mut layers = BTreeMap::new();

  for crate_name in edges.keys() {
    let mut walking = BTreeSet::new();
    depth_of(crate_name, edges, &mut layers, &mut walking)?;
  }

  Ok(layers)
}

fn depth_of(
  crate_name: &str,
  edges: &BTreeMap<String, BTreeSet<String>>,
  layers: &mut BTreeMap<String, usize>,
  walking: &mut BTreeSet<String>,
) -> Result<usize, String> {
  if let Some(known) = layers.get(crate_name) {
    return Ok(*known);
  }

  if !walking.insert(crate_name.to_string()) {
    return Err(format!("a dependency cycle reaches {crate_name}"));
  }

  let mut deepest = 0;

  for dependency in edges.get(crate_name).into_iter().flatten() {
    deepest = deepest.max(depth_of(dependency, edges, layers, walking)? + 1);
  }

  walking.remove(crate_name);
  layers.insert(crate_name.to_string(), deepest);

  Ok(deepest)
}

/// The crates `root` links, itself included.
fn reachable_from(edges: &BTreeMap<String, BTreeSet<String>>, root: &str) -> BTreeSet<String> {
  let mut reached = BTreeSet::new();
  let mut pending = vec![root.to_string()];

  while let Some(crate_name) = pending.pop() {
    if !reached.insert(crate_name.clone()) {
      continue;
    }

    for dependency in edges.get(&crate_name).into_iter().flatten() {
      pending.push(dependency.clone());
    }
  }

  reached
}

/// The ladder the manifests describe: layer number to the crates on that rung.
fn ladder_of(
  edges: &BTreeMap<String, BTreeSet<String>>,
  root: &str,
) -> Result<BTreeMap<usize, BTreeSet<String>>, String> {
  assert!(edges.contains_key(root), "no crate directory named {root}");

  let layers = layers_of(edges)?;
  let linked = reachable_from(edges, root);
  let mut rungs: BTreeMap<usize, BTreeSet<String>> = BTreeMap::new();

  for (crate_name, layer) in layers {
    if linked.contains(&crate_name) {
      rungs.entry(layer).or_default().insert(crate_name);
    }
  }

  Ok(rungs)
}

/// The crate names in backticks, taken from the odd halves of a split on the
/// backtick.
fn backticked(text: &str) -> BTreeSet<String> {
  text
    .split('`')
    .skip(1)
    .step_by(2)
    .map(str::to_string)
    .collect()
}

/// The ladder the document states.
///
/// A rung opens with `- **<number> -- `, and its crate names follow the bold
/// title, wrapped over as many lines as the prose formatter needed. Names are
/// read from after the title so a backtick inside a title cannot be mistaken
/// for a crate.
fn documented_ladder(document: &str) -> BTreeMap<usize, BTreeSet<String>> {
  let mut rungs = BTreeMap::new();
  let mut lines = document.lines().peekable();

  while let Some(line) = lines.next() {
    let Some((number, after_number)) = line
      .strip_prefix("- **")
      .and_then(|rest| rest.split_once(" -- "))
    else {
      continue;
    };

    let Ok(layer) = number.parse::<usize>() else {
      continue;
    };

    let mut item = after_number.to_string();

    // A wrapped continuation is indented and is not itself a list item.
    while let Some(next) = lines.peek() {
      if !next.starts_with("  ") || next.trim_start().starts_with("- ") {
        break;
      }

      item.push(' ');
      item.push_str(next.trim());
      lines.next();
    }

    let names = item
      .split_once("**")
      .map_or(item.as_str(), |(_, tail)| tail);

    rungs.insert(layer, backticked(names));
  }

  rungs
}

#[test]
fn the_documented_ladder_matches_the_manifests() {
  let root = workspace_root();
  let document = root.join(LADDER_DOCUMENT);
  let text = fs::read_to_string(&document)
    .unwrap_or_else(|error| panic!("cannot read {}: {error}", document.display()));

  let edges = read_edges(&root.join("crates"));
  let ladder = ladder_of(&edges, &addon_directory()).expect("the crate graph is acyclic");

  assert_eq!(
    documented_ladder(&text),
    ladder,
    "{LADDER_DOCUMENT} no longer describes the manifests. A rung is the longest \
     path from a crate down to one with no workspace dependency, which \
     `cargo tree -p <crate> -e normal` prints; a crate this addon never reaches \
     has no rung."
  );
}

/// The synthetic cases below hand the readers manifests and prose of their own,
/// so a failure names the reader rather than the repository.
mod reading_a_ladder {
  use super::*;

  fn manifests(crates: &[(&str, &[&str])]) -> BTreeMap<String, (String, String)> {
    crates
      .iter()
      .map(|(directory, dependencies)| {
        let lines = dependencies
          .iter()
          .map(|dependency| {
            format!(
              "{} = {{ path = \"../{dependency}\" }}\n",
              package(dependency)
            )
          })
          .collect::<String>();

        (directory.to_string(), (package(directory), lines))
      })
      .collect()
  }

  fn package(directory: &str) -> String {
    directory.replace('-', "_")
  }

  fn rungs(crates: &[(&str, &[&str])]) -> BTreeMap<usize, BTreeSet<String>> {
    ladder_of(&edges_of(&manifests(crates)), "stylex-rs-compiler")
      .expect("the synthetic graph is acyclic")
  }

  fn rung(layer: usize, names: &[&str]) -> (usize, BTreeSet<String>) {
    (
      layer,
      names.iter().map(|name| (*name).to_string()).collect(),
    )
  }

  #[test]
  fn a_chain_puts_one_crate_on_each_rung() {
    let ladder = rungs(&[
      ("leaf", &[]),
      ("middle", &["leaf"]),
      ("stylex-rs-compiler", &["middle"]),
    ]);

    assert_eq!(
      ladder,
      BTreeMap::from([
        rung(0, &["leaf"]),
        rung(1, &["middle"]),
        rung(2, &["stylex-rs-compiler"]),
      ])
    );
  }

  #[test]
  fn the_longest_path_decides_a_layer() {
    // The addon reaches `leaf` directly and again through two crates above it.
    // The shortest path would put the addon at 1.
    let ladder = rungs(&[
      ("leaf", &[]),
      ("middle", &["leaf"]),
      ("upper", &["middle"]),
      ("stylex-rs-compiler", &["leaf", "upper"]),
    ]);

    assert_eq!(
      ladder,
      BTreeMap::from([
        rung(0, &["leaf"]),
        rung(1, &["middle"]),
        rung(2, &["upper"]),
        rung(3, &["stylex-rs-compiler"]),
      ])
    );
  }

  #[test]
  fn a_crate_the_addon_never_reaches_has_no_rung() {
    let ladder = rungs(&[
      ("leaf", &[]),
      ("stylex-rs-compiler", &["leaf"]),
      ("stylex-test-parser", &["leaf"]),
    ]);

    assert_eq!(
      ladder,
      BTreeMap::from([rung(0, &["leaf"]), rung(1, &["stylex-rs-compiler"])])
    );
  }

  #[test]
  fn several_crates_share_one_rung() {
    let ladder = rungs(&[
      ("zebra", &[]),
      ("apple", &[]),
      ("stylex-rs-compiler", &["zebra", "apple"]),
    ]);

    assert_eq!(
      ladder,
      BTreeMap::from([
        rung(0, &["apple", "zebra"]),
        rung(1, &["stylex-rs-compiler"]),
      ])
    );
  }

  #[test]
  fn a_package_name_that_prefixes_another_is_not_an_edge() {
    // `stylex_state` is a prefix of `stylex_state_index`. A prefix match would
    // reverse the real edge and report a cycle.
    let ladder = rungs(&[
      ("stylex-state-index", &[]),
      ("stylex-state", &["stylex-state-index"]),
      ("stylex-rs-compiler", &["stylex-state"]),
    ]);

    assert_eq!(
      ladder,
      BTreeMap::from([
        rung(0, &["stylex-state-index"]),
        rung(1, &["stylex-state"]),
        rung(2, &["stylex-rs-compiler"]),
      ])
    );
  }

  #[test]
  fn a_cycle_is_named_instead_of_exhausting_the_stack() {
    let cyclic = manifests(&[
      ("left", &["right"]),
      ("right", &["left"]),
      ("stylex-rs-compiler", &["left"]),
    ]);

    let answer = ladder_of(&edges_of(&cyclic), "stylex-rs-compiler");

    assert_eq!(answer, Err("a dependency cycle reaches left".to_string()));
  }

  #[test]
  fn a_chain_of_hundreds_of_crates_is_laddered() {
    let names: Vec<String> = (0..500).map(|index| format!("crate-{index:03}")).collect();
    let mut crates: Vec<(&str, Vec<&str>)> = vec![(names[0].as_str(), vec![])];

    for pair in names.windows(2) {
      crates.push((pair[1].as_str(), vec![pair[0].as_str()]));
    }
    crates.push(("stylex-rs-compiler", vec![names[499].as_str()]));

    let borrowed: Vec<(&str, &[&str])> = crates
      .iter()
      .map(|(name, dependencies)| (*name, dependencies.as_slice()))
      .collect();
    let ladder = rungs(&borrowed);

    assert_eq!(ladder.len(), 501);
    assert_eq!(
      ladder.get(&0),
      Some(&BTreeSet::from(["crate-000".to_string()]))
    );
    assert_eq!(
      ladder.get(&500),
      Some(&BTreeSet::from(["stylex-rs-compiler".to_string()]))
    );
  }

  #[test]
  fn the_addon_alone_is_a_ladder_of_one_rung() {
    let ladder = rungs(&[("stylex-rs-compiler", &[])]);

    assert_eq!(ladder, BTreeMap::from([rung(0, &["stylex-rs-compiler"])]));
  }

  #[test]
  fn a_dev_dependency_does_not_raise_a_crate() {
    let manifest = "\
[package]
name = \"stylex_leaf\"

[dependencies]

[dev-dependencies]
stylex_helper = { path = \"../stylex-helper\" }
";

    assert!(!names_dependency(
      &dependency_lines(manifest),
      "stylex_helper"
    ));
  }

  #[test]
  fn a_target_table_is_not_read_as_a_dependency_table() {
    let manifest = "\
[dependencies]
stylex_utils = { path = \"../stylex-utils\" }

[target.'cfg(unix)'.dependencies]
stylex_unix = { path = \"../stylex-unix\" }
";
    let lines = dependency_lines(manifest);

    assert!(names_dependency(&lines, "stylex_utils"));
    assert!(!names_dependency(&lines, "stylex_unix"));
  }

  #[test]
  fn a_dotted_dependency_is_an_edge() {
    let lines = dependency_lines("[dependencies]\nstylex_macros.workspace = true\n");

    assert!(names_dependency(&lines, "stylex_macros"));
  }
}

mod reading_the_document {
  use super::*;

  #[test]
  fn reads_a_rung_written_on_one_line() {
    let document = "- **1 -- Macros**: `stylex-macros`, `stylex-styleq`\n";

    assert_eq!(
      documented_ladder(document),
      BTreeMap::from([(
        1,
        BTreeSet::from(["stylex-macros".to_string(), "stylex-styleq".to_string()])
      )])
    );
  }

  #[test]
  fn reads_a_rung_wrapped_over_several_lines() {
    let document = "\
- **2 -- Domain leaves**: `stylex-ast`, `stylex-enums`,
  `stylex-js`,
  `stylex-logs`
- **3 -- Structures**: `stylex-structures`

Prose that follows the list.
";

    assert_eq!(
      documented_ladder(document),
      BTreeMap::from([
        (
          2,
          BTreeSet::from([
            "stylex-ast".to_string(),
            "stylex-enums".to_string(),
            "stylex-js".to_string(),
            "stylex-logs".to_string(),
          ])
        ),
        (3, BTreeSet::from(["stylex-structures".to_string()])),
      ])
    );
  }

  #[test]
  fn reads_a_rung_whose_title_carries_a_parenthesis() {
    let document = "- **0 -- Primitives** (no internal dependencies): `stylex-utils`\n";

    assert_eq!(
      documented_ladder(document),
      BTreeMap::from([(0, BTreeSet::from(["stylex-utils".to_string()]))])
    );
  }

  #[test]
  fn ignores_a_bullet_that_is_not_a_rung() {
    let document = "\
- `plugin-shared` -- the shared core, on `rs-compiler`
- **Note** -- not a numbered rung: `stylex-utils`
- **4 -- Types**: `stylex-types`
";

    assert_eq!(
      documented_ladder(document),
      BTreeMap::from([(4, BTreeSet::from(["stylex-types".to_string()]))])
    );
  }

  #[test]
  fn reads_no_rung_from_a_document_that_states_none() {
    assert!(documented_ladder("# Project Structure\n\nNo ladder here.\n").is_empty());
    assert!(documented_ladder("").is_empty());
  }
}
