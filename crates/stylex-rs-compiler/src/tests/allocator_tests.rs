//! The allocator of every target this package publishes, checked against the
//! manifests.
//!
//! `swc_malloc` chooses one for six of the seven targets `napi.targets` lists.
//! It declines every musl target at once, so a musl target is the one that can
//! ship the system allocator with nobody deciding that. The difference is not
//! small: the paired release gate read the musl artifact 1.13-1.53x slower than
//! the previous release while every glibc artifact read 0.33-0.82x faster.
//!
//! So `lib.rs` names the musl targets it answers for, and this holds that list
//! against the list the package publishes. A musl target in one and not the
//! other fails here, which is the point: mimalloc segfaults on ARM64 musl
//! (microsoft/mimalloc#556), so a new musl target is a decision to make rather
//! than a line to copy.
//!
//! The check lives in this crate because the allocator does: `lib.rs` declares
//! it, this manifest depends on it, and this package's `napi.targets` says which
//! targets receive it.

use std::{
  collections::BTreeSet,
  fs,
  path::{Path, PathBuf},
};

use serde_json::Value;

/// This crate's directory, which holds every file the checks below read.
fn crate_dir() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

/// Reads one file of this crate as text.
fn crate_file(relative: &str) -> String {
  let path = crate_dir().join(relative);

  match fs::read_to_string(&path) {
    Ok(source) => source,
    Err(error) => panic!("{} is not readable: {error}", path.display()),
  }
}

/// The `target_arch` values of every musl `#[global_allocator]` in `source`.
///
/// Read off the attribute rather than from a list beside it. The attribute is
/// what the compiler obeys, and a list beside it is a second place to forget.
///
/// One `#[cfg(...)]` on one line is the form this reader understands, which is
/// the form the source uses. A predicate split over several lines answers
/// nothing here, and the check that reads the real file would then fail rather
/// than pass on a list it could not see.
fn musl_arches_declared_in(source: &str) -> BTreeSet<String> {
  let mut found = BTreeSet::new();
  let mut predicate: Option<&str> = None;

  for line in source.lines() {
    let line = line.trim();

    if let Some(rest) = line.strip_prefix("#[cfg(") {
      predicate = Some(rest);
      continue;
    }

    if line == "#[global_allocator]" {
      if let Some(cfg) = predicate.take()
        && cfg.contains(r#"target_env = "musl""#)
      {
        found.extend(arches_in(cfg));
      }
      continue;
    }

    // A comment or a blank line sits between attributes without separating
    // them. Anything else ends the run, so a `cfg` on unrelated code cannot
    // reach an allocator further down the file.
    if !line.is_empty() && !line.starts_with("//") && !line.starts_with("#[") {
      predicate = None;
    }
  }

  found
}

/// Every `target_arch = "..."` value in one `cfg` predicate.
fn arches_in(predicate: &str) -> Vec<String> {
  let mut found = Vec::new();
  let mut rest = predicate;

  while let Some(start) = rest.find(r#"target_arch = ""#) {
    rest = &rest[start + r#"target_arch = ""#.len()..];
    match rest.find('"') {
      Some(end) => {
        found.push(rest[..end].to_string());
        rest = &rest[end + 1..];
      },
      None => break,
    }
  }

  found
}

/// The targets `napi.targets` lists, in the order the manifest writes them.
///
/// Parsed rather than scanned, so the answer follows the manifest and not the
/// way the manifest is written. A list this reader cannot find is a failure
/// rather than an empty answer: an empty answer agrees with a `lib.rs` that
/// declares nothing, which is the one reading these checks must never give.
fn published_targets(manifest: &str) -> Vec<String> {
  let parsed: Value = match serde_json::from_str(manifest) {
    Ok(value) => value,
    Err(error) => panic!("package.json is not readable as JSON: {error}"),
  };

  match parsed.pointer("/napi/targets") {
    Some(Value::Array(targets)) => targets
      .iter()
      .filter_map(|target| target.as_str().map(str::to_string))
      .collect(),
    _ => panic!("package.json declares napi.targets as an array"),
  }
}

/// The `<arch>` of every published `<arch>-unknown-linux-musl` target.
fn published_musl_arches(manifest: &str) -> BTreeSet<String> {
  published_targets(manifest)
    .into_iter()
    .filter(|target| target.ends_with("-linux-musl"))
    .filter_map(|target| target.split('-').next().map(str::to_string))
    .collect()
}

#[test]
fn every_published_musl_target_declares_an_allocator() {
  let declared = musl_arches_declared_in(&crate_file("src/lib.rs"));
  let published = published_musl_arches(&crate_file("package.json"));

  assert_eq!(
    declared, published,
    "the musl targets `lib.rs` declares an allocator for must be the musl \
     targets `napi.targets` publishes"
  );
}

#[test]
fn the_manifest_takes_mimalloc_for_the_targets_lib_declares() {
  // The dependency and the declaration are two halves of one statement: a
  // `cfg` with no dependency does not compile, and a dependency with no `cfg`
  // links an allocator nothing chose.
  let manifest = crate_file("Cargo.toml");

  for arch in musl_arches_declared_in(&crate_file("src/lib.rs")) {
    // Split on the start of a table and not on the text "[target.". A split on
    // the text gives each part to the end of the file, so a dependency in a
    // later table would answer for this one.
    let section = manifest.split("\n[").find(|section| {
      section.starts_with("target.")
        && section.contains(r#"target_env = "musl""#)
        && section.contains(&format!(r#"target_arch = "{arch}""#))
    });

    let table = match section {
      Some(table) => table,
      None => panic!("Cargo.toml has no target table for musl {arch}"),
    };

    assert!(
      table.contains("mimalloc.workspace = true"),
      "the musl {arch} target table must depend on mimalloc"
    );
  }
}

#[test]
fn the_other_published_targets_keep_swc_malloc() {
  assert!(
    crate_file("src/lib.rs").contains("use swc_malloc as _;"),
    "the six targets `swc_malloc` answers for must still link it"
  );
}

mod reading_a_declaration {
  use super::musl_arches_declared_in;

  #[test]
  fn names_the_arch_of_a_musl_declaration() {
    let source = r#"
      #[cfg(all(target_os = "linux", target_env = "musl", target_arch = "x86_64"))]
      #[global_allocator]
      static A: mimalloc::MiMalloc = mimalloc::MiMalloc;
    "#;

    assert_eq!(
      musl_arches_declared_in(source),
      ["x86_64".to_string()].into()
    );
  }

  #[test]
  fn a_declaration_for_another_libc_is_not_a_musl_declaration() {
    let source = r#"
      #[cfg(all(target_os = "linux", target_env = "gnu", target_arch = "x86_64"))]
      #[global_allocator]
      static A: mimalloc::MiMalloc = mimalloc::MiMalloc;
    "#;

    assert!(musl_arches_declared_in(source).is_empty());
  }

  #[test]
  fn a_cfg_that_carries_no_allocator_is_not_read_as_one() {
    // The failure this reader is most prone to: a `cfg` elsewhere in the file
    // that happens to name musl.
    let source = r#"
      #[cfg(all(target_env = "musl", target_arch = "aarch64"))]
      fn only_on_musl() {}

      #[global_allocator]
      static A: mimalloc::MiMalloc = mimalloc::MiMalloc;
    "#;

    assert!(musl_arches_declared_in(source).is_empty());
  }

  #[test]
  fn a_comment_between_the_two_attributes_does_not_separate_them() {
    let source = r#"
      #[cfg(all(target_env = "musl", target_arch = "x86_64"))]
      // Why this target needs one.
      #[global_allocator]
      static A: mimalloc::MiMalloc = mimalloc::MiMalloc;
    "#;

    assert_eq!(
      musl_arches_declared_in(source),
      ["x86_64".to_string()].into()
    );
  }

  #[test]
  fn two_declarations_are_both_read() {
    let source = r#"
      #[cfg(all(target_env = "musl", target_arch = "x86_64"))]
      #[global_allocator]
      static A: mimalloc::MiMalloc = mimalloc::MiMalloc;

      #[cfg(all(target_env = "musl", target_arch = "aarch64"))]
      #[global_allocator]
      static B: mimalloc::MiMalloc = mimalloc::MiMalloc;
    "#;

    assert_eq!(
      musl_arches_declared_in(source),
      ["aarch64".to_string(), "x86_64".to_string()].into()
    );
  }

  #[test]
  fn one_cfg_naming_two_arches_is_read_as_both() {
    let source = r#"
      #[cfg(all(target_env = "musl", any(target_arch = "x86_64", target_arch = "aarch64")))]
      #[global_allocator]
      static A: mimalloc::MiMalloc = mimalloc::MiMalloc;
    "#;

    assert_eq!(
      musl_arches_declared_in(source),
      ["aarch64".to_string(), "x86_64".to_string()].into()
    );
  }

  #[test]
  fn a_source_that_declares_nothing_answers_nothing() {
    assert!(musl_arches_declared_in("").is_empty());
    assert!(musl_arches_declared_in("fn main() {}").is_empty());
  }

  #[test]
  fn an_allocator_with_no_cfg_at_all_names_no_arch() {
    let source = "#[global_allocator]\nstatic A: System = System;";

    assert!(musl_arches_declared_in(source).is_empty());
  }

  #[test]
  fn a_source_far_larger_than_this_crate_is_still_read() {
    let filler = "// a comment line that names nothing\n".repeat(50_000);
    let source = format!(
      "{filler}#[cfg(all(target_env = \"musl\", target_arch = \"x86_64\"))]\n\
       #[global_allocator]\nstatic A: mimalloc::MiMalloc = mimalloc::MiMalloc;\n{filler}"
    );

    assert_eq!(
      musl_arches_declared_in(&source),
      ["x86_64".to_string()].into()
    );
  }
}

mod reading_the_published_targets {
  use super::{published_musl_arches, published_targets};

  #[test]
  fn reads_every_target_of_a_multiline_array() {
    let manifest = r#"{
      "napi": {
        "binaryName": "rs-compiler",
        "targets": [
          "x86_64-unknown-linux-gnu",
          "x86_64-unknown-linux-musl"
        ]
      }
    }"#;

    assert_eq!(
      published_targets(manifest),
      vec![
        "x86_64-unknown-linux-gnu".to_string(),
        "x86_64-unknown-linux-musl".to_string()
      ]
    );
  }

  #[test]
  fn reads_an_array_written_on_one_line() {
    let manifest =
      r#"{ "napi": { "targets": ["aarch64-apple-darwin", "aarch64-unknown-linux-musl"] } }"#;

    assert_eq!(
      published_musl_arches(manifest),
      ["aarch64".to_string()].into()
    );
  }

  #[test]
  fn names_only_the_musl_targets() {
    let manifest =
      r#"{ "napi": { "targets": ["x86_64-pc-windows-msvc", "x86_64-unknown-linux-musl"] } }"#;

    assert_eq!(
      published_musl_arches(manifest),
      ["x86_64".to_string()].into()
    );
  }

  #[test]
  fn a_second_targets_key_elsewhere_in_the_manifest_is_not_the_published_list() {
    // The reader that scanned for the first `"targets"` answered from whichever
    // key came first in the file. A pointer names the one key that decides what
    // is published.
    let manifest = r#"{
      "browserslist": { "targets": ["aarch64-unknown-linux-musl"] },
      "napi": { "targets": ["x86_64-unknown-linux-musl"] }
    }"#;

    assert_eq!(
      published_musl_arches(manifest),
      ["x86_64".to_string()].into()
    );
  }

  #[test]
  fn a_target_name_holding_an_escape_is_read_as_the_name_it_spells() {
    // JSON may write any character as an escape. The scan compared the raw
    // bytes, so an escaped hyphen hid a musl target from a check whose whole
    // job is to find one.
    let manifest = r#"{ "napi": { "targets": ["x86_64\u002dunknown\u002dlinux\u002dmusl"] } }"#;

    assert_eq!(
      published_targets(manifest),
      vec!["x86_64-unknown-linux-musl".to_string()]
    );
    assert_eq!(
      published_musl_arches(manifest),
      ["x86_64".to_string()].into()
    );
  }

  #[test]
  fn a_comma_inside_a_target_name_does_not_split_it() {
    // What the scan got wrong: it cut the array on every comma, so a name
    // holding one became two names and neither was a target.
    let manifest = r#"{ "napi": { "targets": ["odd,name", "x86_64-unknown-linux-musl"] } }"#;

    assert_eq!(
      published_targets(manifest),
      vec![
        "odd,name".to_string(),
        "x86_64-unknown-linux-musl".to_string()
      ]
    );
  }

  #[test]
  fn an_entry_that_is_not_a_string_is_passed_over() {
    let manifest =
      r#"{ "napi": { "targets": [null, 7, { "name": "x" }, "x86_64-unknown-linux-musl"] } }"#;

    assert_eq!(
      published_targets(manifest),
      vec!["x86_64-unknown-linux-musl".to_string()]
    );
  }

  #[test]
  fn an_empty_array_publishes_no_musl_target() {
    let manifest = r#"{ "napi": { "targets": [] } }"#;

    assert!(published_targets(manifest).is_empty());
    assert!(published_musl_arches(manifest).is_empty());
  }

  #[test]
  fn a_manifest_far_larger_than_this_package_is_still_read() {
    let filler: Vec<String> = (0..20_000)
      .map(|index| format!(r#""key-{index}": "value-{index}""#))
      .collect();
    let manifest = format!(
      r#"{{ {}, "napi": {{ "targets": ["x86_64-unknown-linux-musl"] }} }}"#,
      filler.join(", ")
    );

    assert_eq!(
      published_musl_arches(&manifest),
      ["x86_64".to_string()].into()
    );
  }

  #[test]
  #[should_panic(expected = "package.json declares napi.targets as an array")]
  fn a_manifest_with_no_target_list_fails_rather_than_answers_nothing() {
    // An empty answer would agree with a `lib.rs` that declares nothing, so a
    // manifest this reader cannot read has to stop the test.
    published_targets(r#"{ "napi": { "binaryName": "rs-compiler" } }"#);
  }

  #[test]
  #[should_panic(expected = "package.json declares napi.targets as an array")]
  fn a_target_list_that_is_not_an_array_fails_as_well() {
    published_targets(r#"{ "napi": { "targets": "x86_64-unknown-linux-musl" } }"#);
  }

  #[test]
  #[should_panic(expected = "package.json declares napi.targets as an array")]
  fn a_manifest_with_no_napi_block_fails() {
    published_targets(r#"{ "name": "@stylexswc/rs-compiler" }"#);
  }

  #[test]
  #[should_panic(expected = "package.json is not readable as JSON")]
  fn an_unclosed_array_fails_as_well() {
    published_targets(r#"{ "napi": { "targets": ["x86_64-unknown-linux-musl" "#);
  }

  #[test]
  #[should_panic(expected = "package.json is not readable as JSON")]
  fn an_empty_manifest_fails() {
    published_targets("");
  }
}

/// Every criterion bench states which allocator it measures.
///
/// The shipped addon runs mimalloc, and a bench that links nothing measures the
/// system allocator instead. Rust links a dev-dependency only where a target
/// names it, so a manifest entry alone does nothing: the one line that makes
/// the allocator real is `use swc_malloc as _;` in the bench itself.
///
/// A bench may measure the system allocator, but it must say so. Write
/// `ALLOCATOR: system` with the reason in the bench, and this reader accepts
/// it. Silence is what this refuses.
#[cfg(test)]
mod bench_allocator {
  use std::{fs, path::PathBuf};

  /// The `crates` directory that holds every crate of this workspace.
  fn crates_dir() -> PathBuf {
    match super::crate_dir().parent() {
      Some(parent) => parent.to_path_buf(),
      None => panic!("the crate directory has no parent"),
    }
  }

  /// Every crate directory of this workspace.
  ///
  /// Both readers below walk the same directory, and each held its own copy of
  /// this read. A copy that fails differently from its twin is how one reader
  /// starts answering for a workspace the other cannot see.
  fn crate_directories() -> Vec<PathBuf> {
    let entries = match fs::read_dir(crates_dir()) {
      Ok(entries) => entries,
      Err(error) => panic!("the crates directory is not readable: {error}"),
    };

    entries.flatten().map(|entry| entry.path()).collect()
  }

  /// Every `benches/*.rs` file in the workspace, as (name, source).
  fn benches() -> Vec<(String, String)> {
    let mut found = Vec::new();

    for crate_directory in crate_directories() {
      let benches = crate_directory.join("benches");

      let files = match fs::read_dir(&benches) {
        Ok(files) => files,
        // A crate with no bench directory has no bench to answer for.
        Err(_) => continue,
      };

      for file in files.flatten() {
        let path = file.path();

        if path.extension().is_none_or(|extension| extension != "rs") {
          continue;
        }

        let name = path
          .strip_prefix(crates_dir())
          .unwrap_or(&path)
          .display()
          .to_string();

        match fs::read_to_string(&path) {
          Ok(source) => found.push((name, source)),
          Err(error) => panic!("{name} is not readable: {error}"),
        }
      }
    }

    found.sort();
    found
  }

  /// Whether a bench states the allocator it measures.
  ///
  /// The statement must start a line. The comment beside it names the same
  /// crate, so a bench that copied the comment and dropped the statement would
  /// answer yes to a plain search of the whole text.
  fn states_its_allocator(source: &str) -> bool {
    source
      .lines()
      .any(|line| line.trim_start() == "use swc_malloc as _;" || line.contains("ALLOCATOR: system"))
  }

  /// How many `[[bench]]` tables the crate manifests declare in total.
  fn declared_bench_count() -> usize {
    crate_directories()
      .into_iter()
      .map(
        |crate_directory| match fs::read_to_string(crate_directory.join("Cargo.toml")) {
          Ok(manifest) => manifest.matches("[[bench]]").count(),
          // A directory with no manifest declares no bench.
          Err(_) => 0,
        },
      )
      .sum()
  }

  #[test]
  fn every_bench_says_which_allocator_it_measures() {
    let silent: Vec<String> = benches()
      .into_iter()
      .filter(|(_, source)| !states_its_allocator(source))
      .map(|(name, _)| name)
      .collect();

    assert_eq!(
      silent,
      Vec::<String>::new(),
      "these benches measure the system allocator without saying so; add \
       `use swc_malloc as _;`, or write `ALLOCATOR: system` with the reason"
    );
  }

  #[test]
  fn the_reader_finds_every_bench_the_manifests_declare() {
    // A reader that finds nothing agrees with a workspace where every bench is
    // silent. The manifests say how many there should be, so a bench this
    // reader cannot reach -- one a `[[bench]]` table points outside `benches`,
    // for example -- fails here rather than going unchecked.
    let declared = declared_bench_count();

    // Both readers walk the same directory, so both give zero for a directory
    // they cannot find, and the comparison below would then pass while nothing
    // was measured. This holds the pair to a workspace that has benches.
    assert!(
      declared > 0,
      "no crate manifest declares a bench; the reader is looking in the wrong \
       place"
    );

    assert_eq!(benches().len(), declared);
  }

  #[test]
  fn a_bench_that_only_names_the_rule_does_not_state_it() {
    assert!(!states_its_allocator(
      "// Read about `use swc_malloc as _;` elsewhere.\n"
    ));
    assert!(!states_its_allocator("fn main() {}\n"));
    assert!(states_its_allocator("use swc_malloc as _;\n"));
    assert!(states_its_allocator("  use swc_malloc as _;\n"));
    assert!(states_its_allocator(
      "// ALLOCATOR: system -- measured on purpose.\n"
    ));
  }
}
