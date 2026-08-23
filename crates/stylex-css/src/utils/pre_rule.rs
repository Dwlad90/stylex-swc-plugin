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

  // With no element in the list the partition below produces exactly one
  // `Sortable` run covering every key, so this is the same answer for three
  // fewer allocations -- and it is the shape almost every key path has:
  // `[':hover']`, `[':hover', ':focus']`, `['[data-x]', ':hover']`.
  if !pseudos.iter().any(|pseudo| is_pseudo_element(pseudo)) {
    let mut sorted = pseudos.to_owned();

    sorted.sort_unstable_by(|a, b| pseudo_comparator(a, b));

    return sorted;
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

/// The order `localeCompare` puts printable ASCII in, one character per
/// position, with a letter's two cases sharing a position because case is a
/// tiebreak rather than an identity.
///
/// Read out of `String.prototype.localeCompare` rather than off the collation
/// charts: sorting the 95 printable ASCII characters with it produces exactly
/// this sequence. Its shape is root collation's -- whitespace, then punctuation
/// and symbols, then digits, then letters -- and its detail is not byte order
/// anywhere: `_` leads `-`, `$` trails every other symbol, and `{ | } ~` sit
/// below every letter where their bytes sit above every one.
pub(super) const ASCII_PRIMARY_ORDER: &[u8] =
  b" _-,;:!?.'\"()[]{}@*/\\&#%`^+<=>|~$0123456789abcdefghijklmnopqrstuvwxyz";

/// `ASCII_PRIMARY_ORDER` inverted: a byte to its position in it, or
/// [`UNRANKED`] for a byte the order does not name.
pub(super) const ASCII_PRIMARY_RANK: [u8; 128] = build_ascii_primary_rank();

/// The rank of a byte [`ASCII_PRIMARY_ORDER`] does not name: a control
/// character, `DEL`, or any byte of a non-ASCII character.
pub(super) const UNRANKED: u8 = u8::MAX;

/// Invert [`ASCII_PRIMARY_ORDER`] into a lookup table.
///
/// Visible to this module's tests, and callable at runtime as well as in a
/// `const`, so the invariants the table rests on can be asserted rather than
/// argued: a rank is one-based so nothing collides with a zero fill, a letter's
/// two cases share one, and every byte the order does not name stays
/// [`UNRANKED`]. Every class name carrying a pseudo selector is hashed off this
/// ordering, so those three are load-bearing rather than tidy.
pub(super) const fn build_ascii_primary_rank() -> [u8; 128] {
  let mut table = [UNRANKED; 128];
  let mut index = 0;

  while index < ASCII_PRIMARY_ORDER.len() {
    let character = ASCII_PRIMARY_ORDER[index];
    // One-based, so no character can hold the rank a `[0u8; _]` initialiser
    // would have handed every byte the order does not name.
    let rank = (index + 1) as u8;

    table[character as usize] = rank;

    if character.is_ascii_lowercase() {
      table[character.to_ascii_uppercase() as usize] = rank;
    }

    index += 1;
  }

  table
}

/// One byte's primary weight, widened so that the bytes the order does not name
/// still rank above every one it does -- and still rank apart from each other.
///
/// Collapsing them onto a single weight would make two distinct keys compare
/// `Equal`, which `sort_unstable_by` is entitled to resolve either way. Adding
/// the byte keeps the answer total: a non-ASCII character sorts by its UTF-8
/// bytes, which for UTF-8 is code-point order, and that is exactly the
/// behaviour the non-ASCII cases pin.
///
/// It is also a **known divergence**, and the consequence is worth stating
/// plainly: root collation places an accented letter beside its base letter,
/// where this places every non-ASCII byte above all of printable ASCII. So a
/// key path nesting `[data-état]` beside `[data-f]` sorts one way here and the
/// other way in Babel — and since the sorted path feeds the class-name hash,
/// the two compilers emit *different class names* for the same source. That is
/// a mixed-toolchain hazard rather than an ordering curiosity, and it is what
/// decides whether taking on a real collation dependency is ever worth it.
#[inline]
fn primary_weight(byte: u8) -> u16 {
  match ASCII_PRIMARY_RANK.get(byte as usize) {
    Some(&rank) if rank != UNRANKED => u16::from(rank),
    _ => u16::from(byte) + 256,
  }
}

/// What closing the non-ASCII half of [`pseudo_comparator`] costs, either way.
///
/// Carried as a module rather than as prose above the comparator so that
/// [`pseudo_comparator`] can link to it and `cargo doc` renders it beside the
/// function it decides. It holds nothing: the decision is the content.
///
/// Every number below was taken from a build on this repository rather than
/// estimated, because the choice turns on the size of one of them.
///
/// **Which characters a condition key can carry, argued rather than asserted.**
/// [`sort_pseudos`] is handed pseudo-classes, pseudo-elements and attribute
/// selectors. Every pseudo-class and pseudo-element name CSS defines is ASCII,
/// so nothing non-ASCII can arrive that way. It arrives only through an
/// attribute selector, in two positions: the attribute *name*, which for a
/// `data-*` attribute is an XML Name and admits letters from every script; and a
/// quoted attribute *value*, which is arbitrary text. The second is what settles
/// the question — the set is not bounded in principle, so any generated range
/// leaves a remainder, and the remainder is unbounded. Latin-1 Supplement plus
/// Latin Extended-A would serve a Western-European author; Greek, Cyrillic, CJK
/// and emoji are each one quoted attribute value away.
///
/// **Option A — `icu_collator`.** Exact: it places `[data-état]` between
/// `[data-e]` and `[data-f]`, and wiring it in as a probe turned the parity
/// corpus's one `divergent` row into `identical`, read from a run.
///
/// - *Dependency surface*: 33 crates in its tree, of which **26 are already in
///   this workspace's lockfile** — SWC reaches the ICU4X normalizer, properties,
///   provider, collections and the zerovec/yoke family through `idna_adapter`.
///   Six are new: `icu_collator`, `icu_collator_data`, `icu_locale_fallback`,
///   `icu_locale_fallback_data`, `utf16_iter`, `write16`.
/// - *Compile time*: **2.74s of CPU** across those six, 2.04s of it
///   `icu_collator` itself, from `cargo build --timings`. On top of that, once:
///   adding it moves twelve already-present crates forward inside their semver
///   ranges and hands `icu_normalizer` two features it did not have, which
///   invalidates `idna`, `url` and the SWC stack for one rebuild.
/// - *Binary size*: **+1 222 800 bytes on the `.node` addon**, 9 777 328 to
///   11 000 128 — 12.5%. The same delta to within 16 bytes on a standalone
///   binary, so it is the CLDR tables rather than shared code.
///
/// **Option B — a generated weight table.** The table is not the problem: a
/// dense per-code-point rank derived from `Intl.Collator` the way
/// [`ASCII_PRIMARY_ORDER`] was derived is 256 bytes for Latin-1 Supplement, 384
/// for Latin Extended-A, 880 with combining diacritics and 2 560 with Greek and
/// Cyrillic. Three orders of magnitude below the ICU data.
///
/// The problem is that it does not work. Comparing that table's comparator
/// against `localeCompare` over 200 000 random pairs drawn from each range:
/// printable ASCII, which is what ships today, disagrees on **0**; Latin-1
/// Supplement on **0.50%**; Latin Extended-A on **0.29%**; add combining
/// diacritics and it is **9.95%**. Three structural reasons, each observed
/// rather than reasoned about:
///
/// 1. *Secondary weights.* `é` and `e` share a primary weight, and root
///    collation settles the tie on an accent pass that sits between the primary
///    pass and case. This comparator has no such pass — primary, then length,
///    then case — so every pair whose primary weights tie on an accented letter
///    sorts wrong. Eight of the first ten disagreements.
/// 2. *Completely ignorable characters.* `U+00AD` SOFT HYPHEN carries no weight
///    at all in root collation. A dense rank must give it one, which shifts
///    every position after it.
/// 3. *Expansions.* `æ` weighs as two primary weights, `a` then `e` — measured:
///    `localeCompare('æ', 'az')` is negative. One character cannot carry two
///    ranks in a per-code-point table.
///
/// So the generated table has no defensible edge. Reaching the ordering needs
/// decomposition, a secondary pass, ignorable handling and expansions — which is
/// the collation algorithm, and it then has to be fed a weight table anyway.
///
/// **Decision: take the dependency.** 1.17 MiB and six crates, four of them from
/// a family the workspace already carries, against a divergence that costs a
/// class name and has no cheaper closing.
///
/// **What stays uncovered, and how a reader will know.** Root collation is what
/// upstream reaches when `localeCompare` is called with no locale *on a machine
/// whose default locale does not tailor the characters involved*. It does not
/// call it with `'und'`: it calls it bare, so the runtime's default locale
/// decides. Measured — `en-US`, `de-DE` and `tr-TR` all agree with root on `ö`,
/// `ä`, `å`, `ø`, `ü` and `é`, while `sv-SE` and `da-DK` sort `ö` **after** `z`.
/// So upstream on a Swedish-locale build machine names a different class than
/// upstream on an American one, from the same source. This compiler will always
/// sort as root, which is the majority answer and the only one a compiler can
/// pick without reading the build machine's environment — but it means the
/// divergence is closed for most build machines rather than all of them.
///
/// That remainder is not left to memory. It is named here, and the property test
/// beside the comparator checks it against the reference over random pairs — so
/// the day the reference's own answer moves under it, the check says so rather
/// than a reader having to remember that it might.
pub(super) mod collation_cost {}

/// Order a run of pseudo keys the way the reference implementation's
/// `localeCompare` orders them, over every ASCII input.
///
/// **Three comparators, not one.** Upstream sorts pseudo keys with
/// `String.prototype.localeCompare` -- ICU root collation -- and at-rules with a
/// bare `.sort()`, which is UTF-16 code-unit order. This function is the first;
/// [`at_rule_comparator`] is the second. They were one function here, which is
/// how the pseudo side came to be sorted by bytes.
///
/// **Root collation is two passes, and neither is a byte comparison.** The
/// *primary* pass weighs each character by [`ASCII_PRIMARY_ORDER`]: whitespace,
/// then punctuation and symbols, then digits, then letters, with a letter's case
/// ignored -- so `:HOVER` weighs as `hover` and sorts after `:active`, and
/// `:{` sorts *below* `:z` although its byte is above. Case is a *tertiary*
/// difference, read only when the primary pass ties, and lowercase ranks below
/// uppercase, so `:a` precedes `:A`. Length settles a tie before case does,
/// because a key that runs out of characters has run out of primary weights:
/// `:a` precedes `:aB`, and the case difference further along is never read.
///
/// **The table is measured, and so is the whole comparator.** The order was read
/// out of `localeCompare` by sorting the printable ASCII characters with it, and
/// this comparator was then checked against it on 200 000 random ASCII pairs and
/// on every pair drawn from a list of realistic condition keys -- zero
/// disagreements. `pre_rule_test.rs` pins the pairs that decide it and
/// `stylex-transform`'s `nested_pseudo_ordering` suite pins the class names
/// they hash to.
///
/// **What it does not cover: anything that is not printable ASCII.** A control
/// character, `DEL`, and every byte of a non-ASCII character rank above all of
/// the above, where root collation weighs an accented letter beside its base
/// letter, a symbol below every letter, and a control character not at all. So
/// `[data-état]` nested beside `[data-f]` sorts one way here and the other way
/// upstream, and since the sorted path feeds the class-name hash the two
/// compilers name different classes for the same source. That is the one
/// remaining divergence in the whole parity harness that costs a class name.
///
/// **The two ways of closing it, costed against this repository.** See
/// [`collation_cost`] below. The short of it: a real collation crate costs six
/// new crates and 1.17 MiB of binary and is exact; a generated weight table
/// costs under 3 KiB and is wrong, because the thing it would have to reproduce
/// is not a per-character rank. The dependency is the decision. Until it lands
/// the divergence is left named and measured, in both test suites and in the
/// parity corpus.
pub(crate) fn pseudo_comparator(a: &str, b: &str) -> Ordering {
  let (left, right) = (a.as_bytes(), b.as_bytes());
  let mut case_tiebreak = Ordering::Equal;

  for (x, y) in left.iter().zip(right.iter()) {
    let primary = primary_weight(*x).cmp(&primary_weight(*y));

    if primary != Ordering::Equal {
      return primary;
    }

    // Equal primary weight and unequal bytes: an ASCII letter against its other
    // case, which is the only pair the table gives one rank to. The lowercase
    // one -- the larger byte -- ranks first, and only the earliest such position
    // is kept, which is what "the first differing tertiary weight decides"
    // means.
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

  unsorted_at_rules.sort_unstable_by(|a, b| at_rule_comparator(a, b));

  unsorted_at_rules
}

/// Sorts strings by their bytes, with `default` always first.
///
/// The `default` arm does have a counterpart upstream — it is just on the other
/// comparator. `stringComparator` (`shared/utils/rule-utils.js:51-56`, 0.19.0)
/// pulls `default` to the front, and `sortPseudos` (`:38`) is what passes it;
/// `sortAtRules` (`:46`) passes no comparator at all. So the placement here is
/// upstream's mirrored onto the wrong function.
///
/// Inert on both sides, which is why it is left where it is rather than moved:
/// the arm is unreachable from [`sort_at_rules`], whose key path is filtered to
/// `@`-prefixed keys before it arrives, and equally unreachable from
/// [`sort_pseudos`], whose keys are filtered to `:` and `[`. `default` is
/// neither by the time either is called. Moving it would be a change to
/// unreachable code, and asserting the move changed nothing is the same work as
/// reading this comment.
fn at_rule_comparator(a: &str, b: &str) -> Ordering {
  if a == "default" {
    return Ordering::Less;
  }
  if b == "default" {
    return Ordering::Greater;
  }
  a.cmp(b)
}
