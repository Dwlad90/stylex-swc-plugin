# 61 — Say in the release notes that `StyleqValue` broke

**What to build:** One line in the release notes for the next `stylex_styleq`
release. `StyleqValue::is_undefined` is a new method on a `pub trait` with no
default body, so every implementation outside this workspace stops compiling
until it adds one. That is a semver break, and nothing says so yet.

**The method stays as it is.** Its rustdoc
(`crates/stylex-styleq/src/types.rs:13-23`) argues the case and the argument
holds: a default of `false` is the right answer for a value type that has no
such state and the wrong one for a type that has it and forgot to say so, and
the two read the same from inside the merge -- the property is written, and held
against every style after it. Asking every type makes the answer a decision
rather than an omission. So this ticket adds a note, not a default.

**What the note has to carry**, so a reader can fix a build from it alone:

- The trait and the method: `StyleqValue::is_undefined`.
- What it answers: whether the value stands for a property that was not given.
  An inline style skips such a property completely -- it writes nothing, defines
  nothing, and leaves the property for a later style to declare.
- What an implementer does: answer `true` for the type's own "not given" state,
  or `false` if the type has none.

**Where it goes.** The repo keeps no changelog file, so the note belongs
wherever the release for this crate is written up. `crates/stylex-styleq/README.md`
is the fallback if the release notes have no home yet.

**Found by:** the review of `fix_benchmarks`.

**Blocked by:** None.

**Status:** ready-for-human

- [ ] The next `stylex_styleq` release note names `is_undefined` as a breaking
      change on a `pub trait`
- [ ] The note says what the method answers and what an implementer writes
- [ ] The method keeps no default body
