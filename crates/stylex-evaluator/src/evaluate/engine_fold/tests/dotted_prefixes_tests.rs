use swc_core::atoms::Atom;

use super::DottedPrefixes;

fn prefixes_of(prefixes: &DottedPrefixes, name: &str) -> Vec<String> {
  let mut paths: Vec<String> = prefixes
    .get(&Atom::from(name))
    .into_iter()
    .flatten()
    .map(|path| path.to_string())
    .collect();
  paths.sort();
  paths
}

/// A rollback removes the prefixes added after the mark and keeps the ones from
/// before it, also when the walk read one of those again.
#[test]
fn a_rollback_keeps_the_prefixes_from_before_the_mark() {
  let mut prefixes = DottedPrefixes::default();
  let theme = Atom::from("theme");

  prefixes.insert(&theme, Atom::from("colors"));
  let mark = prefixes.mark();
  prefixes.insert(&theme, Atom::from("colors"));
  prefixes.insert(&theme, Atom::from("colors.brand"));
  prefixes.insert(&Atom::from("other"), Atom::from("space"));

  prefixes.roll_back(mark);

  assert_eq!(prefixes_of(&prefixes, "theme"), ["colors"]);
  assert!(prefixes_of(&prefixes, "other").is_empty());
}

/// Prefixes added after a rollback are kept, and a second rollback to the same
/// mark removes only those.
#[test]
fn a_walk_after_a_rollback_records_again() {
  let mut prefixes = DottedPrefixes::default();
  let theme = Atom::from("theme");

  let mark = prefixes.mark();
  prefixes.insert(&theme, Atom::from("a"));
  prefixes.roll_back(mark);
  prefixes.insert(&theme, Atom::from("b"));

  assert_eq!(prefixes_of(&prefixes, "theme"), ["b"]);

  prefixes.roll_back(mark);

  assert!(prefixes_of(&prefixes, "theme").is_empty());
}
