# 08 — A recursion bound, and the cost of the restored expansion

**What to build:** An author with a breakpoint ladder too deep to expand gets
their queries back unmerged, as they would from the reference implementation —
rather than the compiler taking the whole process down.

The reference implementation wraps its range merge in a catch that returns the
input rules on any throw, and because the merge recurses into itself rather than
through that wrapper, a deep enough ladder raises a call-stack error which that
catch swallows. Its answer for a query too deep to merge is therefore "emit it
unmerged". Rust cannot catch stack exhaustion — it aborts — so the merge gains
an explicit recursion depth bound at the boundary ticket 01 created, returning
its input rules on exceedance. It does not refuse the declaration: the reference
implementation's inner recovery deliberately does not reach its outer refusal.

The bound is a number this compiler chooses, so byte parity past that depth is
unattainable in principle; it is placed at or above ticket 03's measured length,
and both the number and its provenance are recorded next to it. This ticket also
records the measured cost of the expansion that ticket 05 restored, at several
ladder lengths, as the baseline any future fast path must beat — the shortcut
ticket 05 deleted is not coming back without one.

Test the bound at the transform's unit seam only. Its interesting property is
that the input came back unmerged rather than the process aborting, which is
about shape, and forcing it through a higher seam would mean a very large
literal that no reader benefits from.

> **Ticket 03 contradicts this ticket's premise — read
> `../evidence/give-up-length.md` before starting.** There is no ladder length at
> which the reference implementation gives up merging. Its recursion depth is
> linear in ladder length while its branch count is `2^n`, so the stack never
> overflows; what happens instead is the string-length limit or the heap,
> whichever binds first, and neither reaches its `try`/`catch`. Three
> consequences: the bound cannot be placed
> "at or above the measured length" because there is no such length; a depth
> bound alone does not protect this compiler, since a bound permitting 26 rungs
> permits a 63 MB single query, so the number has to be justified against
> output size; and the reference implementation fails rather than degrades,
> which is the answer to this ticket's last acceptance criterion.

> **The boundary ticket 01 created cannot by itself carry the bound.**
> `merge_and_simplify_ranges` is crossed once, from `normalize`, while
> `merge_intervals_for_and` recurses into *itself* (media_query.rs:792-793).
> So a check placed in the boundary sees depth zero and nothing else. The bound
> needs either a depth parameter threaded through the two recursive calls, or a
> pre-pass over the input that measures nesting before the expansion starts.
> The boundary is still where the *recovery* belongs -- return the input rules
> -- which is what ticket 01 bought; it is the *measurement* that has to go
> deeper.

**Blocked by:** 01, 03, 05.

**Status:** done — see `../evidence/expansion-cost.md`

- [x] A ladder past the bound returns its input rules unmerged; a unit-seam test
      asserts this and would fail if the bound were removed.
- [ ] The bound is at or above ticket 03's measured length, and the comment
      beside it states the number, how it was found, and the reference version.
      **Half answered rather than met.** There is no measured length to be at
      or above — ticket 03 established there is none. The comment states the
      number, that it is chosen against output size rather than stack depth,
      the measurement behind it, and the reference version, and says plainly
      that past the bound we stop matching on purpose.
- [x] A ladder past the bound does not abort the process — evidence: the test
      run completes.
- [x] The invalid-media-query-syntax refusal is not raised by exceeding the
      bound.
- [x] Wall-clock cost of the expansion is recorded at several ladder lengths,
      naming the machine and whether the build was warm.
- [x] If the reference implementation survives a length at which this compiler
      still cannot, that is reported rather than left implicit. It does, from
      twenty-one rungs on, and the evidence file says so under its own heading.
