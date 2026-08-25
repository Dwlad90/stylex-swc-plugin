# 01 — A named boundary for the range merge

**What to build:** Nothing changes for an author. The range merge inside media
query canonicalization is reached through a named, fallible boundary that
mirrors the reference implementation's wrapper around the same call, instead of
being invoked directly from normalization. This is prefactoring: it creates the
one place where "give up merging and return the input rules" can live, so that
the recursion bound in ticket 08 is a small change rather than a restructure,
and so the inner recovery is visibly distinct from the outer refusal that turns
an unparseable query into the invalid-media-query-syntax error.

**Blocked by:** None — can start immediately.

**Status:** done

- [x] Normalization reaches the range merge only through the new boundary.
- [x] The boundary's doc comment names the reference-implementation wrapper it
      mirrors and says what its two callers' failure modes are for.
- [x] No test expectation anywhere in the repository changes, and no snapshot is
      regenerated — evidence: the Rust and JS suites pass untouched.
- [x] The parity harness reports the same rows before and after.
