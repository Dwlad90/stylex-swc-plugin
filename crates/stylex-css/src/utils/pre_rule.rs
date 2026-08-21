use std::cmp::Ordering;

use crate::utils::pseudo::is_pseudo_element;

/// One run of the pseudo list as [`sort_pseudos`] partitions it.
///
/// A pseudo element stands alone. It names which part of the element the rule
/// targets rather than a state the element is in, so moving one past its
/// neighbours would rewrite what the selector matches; every other key is free
/// to sort among the keys beside it.
enum PseudoRun {
  /// A pseudo element, left where it was written.
  Element(String),
  /// Consecutive keys that sort among themselves. Any length -- a run grows for
  /// as long as no pseudo element interrupts it.
  Sortable(Vec<String>),
}

/// Order a selector's pseudo keys, in the order the class-name hash reads them.
///
/// The list arrives in nesting order, outermost key first, and every key in it
/// is spelled into the hashed selector. Two selectors that differ only in the
/// order the author happened to nest them have to hash the same class name, so
/// the keys are sorted -- but only as far as sorting is safe: a pseudo element
/// pins its position, and the keys on either side of one sort separately.
///
/// A run is sorted **whole**, at whatever length it reached. Sorting each pair
/// as it arrives and appending the next key agrees with sorting the run for one
/// and two keys and diverges from three on, where the third key lands after a
/// pair that is already in order (`:hover` > `:focus` > `:active` reading
/// `:focus:hover:active` rather than `:active:focus:hover`).
///
/// Keys that are neither pseudo class nor pseudo element -- attribute selectors
/// such as `[data-x]` -- join the run they sit in and sort with it.
///
/// A run sorts by its keys' **bytes**, which is not the same order the reference
/// implementation reaches: it compares with `localeCompare`, so the two disagree
/// wherever the first differing position is a letter-case pair or a non-ASCII
/// character (`:HOVER` sorts ahead of `:active` here and behind it there). That
/// is a divergence in the comparator rather than in the grouping, and it is
/// visible at two keys as well as three, so it is neither introduced nor settled
/// by the run partition above. It is measured rather than fixed: matching
/// `localeCompare` means reproducing ICU root collation, which is a dependency
/// decision and not a comparator swap. The cases that pin it are
/// `two_pseudo_names_differing_only_in_case` and
/// `an_uppercase_pseudo_name_sorts_by_its_bytes`, in the transform crate's
/// `nested_pseudo_ordering` suite.
///
/// Nothing here has to be a *stable* sort: a repeated condition key is refused
/// before a key path reaches this function, so no run can hold two equal keys.
pub fn sort_pseudos(pseudos: &[String]) -> Vec<String> {
  if pseudos.len() < 2 {
    return pseudos.to_owned();
  }

  let mut runs: Vec<PseudoRun> = Vec::new();

  for pseudo in pseudos {
    if is_pseudo_element(pseudo) {
      runs.push(PseudoRun::Element(pseudo.clone()));
    } else if let Some(PseudoRun::Sortable(run)) = runs.last_mut() {
      run.push(pseudo.clone());
    } else {
      runs.push(PseudoRun::Sortable(vec![pseudo.clone()]));
    }
  }

  let mut sorted = Vec::with_capacity(pseudos.len());

  for run in runs {
    match run {
      PseudoRun::Element(pseudo) => sorted.push(pseudo),
      PseudoRun::Sortable(mut run) => {
        run.sort_unstable();
        sorted.extend(run);
      },
    }
  }

  sorted
}

pub fn sort_at_rules(at_rules: &[String]) -> Vec<String> {
  let mut unsorted_at_rules = at_rules.to_vec();

  unsorted_at_rules.sort_unstable_by(|a, b| string_comparator(a, b));

  unsorted_at_rules
}

// a comparator function that sorts strings alphabetically
// but where `default` always comes first
fn string_comparator(a: &str, b: &str) -> std::cmp::Ordering {
  if a == "default" {
    return Ordering::Less;
  }
  if b == "default" {
    return Ordering::Greater;
  }
  a.cmp(b)
}
