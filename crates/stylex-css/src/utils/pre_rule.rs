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
/// A run sorts with [`pseudo_comparator`], which is the ASCII half of the
/// ordering the reference implementation reaches through `localeCompare`. What
/// it covers, what it does not, and why it is not the same comparator
/// [`sort_at_rules`] uses are all in that function's own documentation.
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
        run.sort_unstable_by(|a, b| pseudo_comparator(a, b));
        sorted.extend(run);
      },
    }
  }

  sorted
}

/// Order a run of pseudo keys the way the reference implementation's
/// `localeCompare` orders them, over the inputs an author can reach.
///
/// **Three comparators, not one.** Upstream sorts pseudo keys with
/// `String.prototype.localeCompare` -- ICU root collation -- and at-rules with a
/// bare `.sort()`, which is UTF-16 code-unit order. This function is the first;
/// [`string_comparator`] is the second. They were one function here, which is
/// how the pseudo side came to be sorted by bytes.
///
/// **Root collation, over ASCII, is two passes.** A letter's *primary* weight
/// ignores its case, so `:HOVER` compares as `hover` and sorts after `:active`
/// and before `:italic`; letters rank above digits, and digits above
/// punctuation. Case is a *tertiary* difference, read only when the primary pass
/// ties, and lowercase ranks below uppercase -- so `:a` precedes `:A`, and
/// `:hover` precedes `:HOVER`. Both passes are here: the loop compares
/// ASCII-case-folded bytes and remembers the first position where the two
/// differed only in case, and the fold is what lifts every letter above the
/// `[ \ ] ^ _ ` ` block that sits between the two ASCII cases, which is where a
/// byte comparison put an uppercase letter below punctuation.
///
/// Length settles a tie before case does, because a string that runs out of
/// characters has run out of *primary* weights: `:a` precedes `:aB`, and the
/// case difference further along never gets read.
///
/// **What it does not cover: anything outside ASCII.** Root collation gives `ä`
/// the primary weight of `a` and orders it by the accent only when the base
/// letters tie, so upstream sorts `:ä` ahead of `:z`; every byte at or above
/// `0x80` sorts above every ASCII character here, so this puts `:z` first.
/// Closing that needs decomposition and a weight table -- a collation dependency
/// rather than a comparator -- to serve an author who both writes an accented
/// pseudo name and nests a second key beside it. The divergence is left, named,
/// and measured: `stylex-transform`'s `nested_pseudo_ordering` suite pins the
/// ASCII cases as agreeing and the non-ASCII ordering as it stands.
pub fn pseudo_comparator(a: &str, b: &str) -> Ordering {
  let (left, right) = (a.as_bytes(), b.as_bytes());
  let mut case_tiebreak = Ordering::Equal;

  for (x, y) in left.iter().zip(right.iter()) {
    let folded = x.to_ascii_lowercase().cmp(&y.to_ascii_lowercase());

    if folded != Ordering::Equal {
      return folded;
    }

    // Equal once folded and unequal before: an ASCII letter against its other
    // case, and the lowercase one -- the larger byte -- ranks first. Only the
    // earliest such position is kept, which is what "the first differing
    // tertiary weight decides" means.
    if case_tiebreak == Ordering::Equal && x != y {
      case_tiebreak = y.cmp(x);
    }
  }

  left.len().cmp(&right.len()).then(case_tiebreak)
}

/// Order at-rules the way the reference implementation's bare `.sort()` orders
/// them: by code unit, with `default` pulled to the front.
///
/// Not [`pseudo_comparator`]. Upstream reaches for `localeCompare` on the pseudo
/// side and for nothing at all on this one, so matching it here means *keeping*
/// the plain comparison -- a locale-aware at-rule sort would be a new divergence
/// rather than a fix.
///
/// A `String` compares by UTF-8 bytes and a JavaScript string by UTF-16 code
/// units. The two agree for every character in the basic multilingual plane,
/// because both are code-point order there; they disagree only where one at-rule
/// carries a supplementary character (`U+10000` and above, which UTF-16 spells
/// as a surrogate pair beginning below `U+E000`) at the position that decides
/// the comparison, against a private-use or specials character in the other. No
/// media query, container query or supports condition has a use for either.
pub fn sort_at_rules(at_rules: &[String]) -> Vec<String> {
  let mut unsorted_at_rules = at_rules.to_vec();

  unsorted_at_rules.sort_unstable_by(|a, b| string_comparator(a, b));

  unsorted_at_rules
}

/// Sorts strings by their bytes, with `default` always first.
///
/// The `default` arm has no counterpart upstream, whose `sortAtRules` passes no
/// comparator at all. It is unreachable from [`sort_at_rules`]: the key path is
/// filtered to `@`-prefixed keys before it arrives, and `default` is neither
/// at-rule nor const rule by then. Kept because the cost of the branch is a
/// pointer comparison and the cost of removing it is arguing about a key path
/// filter from the other end of the crate.
fn string_comparator(a: &str, b: &str) -> Ordering {
  if a == "default" {
    return Ordering::Less;
  }
  if b == "default" {
    return Ordering::Greater;
  }
  a.cmp(b)
}
