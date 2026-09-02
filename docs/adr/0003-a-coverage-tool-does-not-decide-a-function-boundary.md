# A coverage tool does not decide a function boundary

**Status:** accepted

A function is split when the extracted half has a name and a contract worth
stating on its own. It is not split to move a region the coverage gate reports
as unreached.

The question came up because the crate split lifted two error arms out of the
diagnostics code frame -- `warn_no_code_frame` out of `build_code_frame_error`,
and `warn_unparseable` out of `parse_and_normalize_program`. Each arm chooses
between a debug message that prints the whole expression and a shorter warning
that names the file, and each is now a named reporter a unit test can call
directly. Both splits stand, and both stand on testability rather than on
coverage: a reporter with a stated contract -- "say the frame could not be
built, at the level the reader asked for" -- is a thing to test, and the two
log levels are two behaviours rather than two lines.

The distinction is not academic. Splitting for coverage produces a helper with
no name of its own, called once, whose only reason to exist is that the tool
counts it separately -- and the tool then reports full coverage over a shape
that was arranged for it. Splitting for testability produces a seam, and the
coverage that follows is a consequence rather than the goal. Reaching the same
branch through a whole-transform case would also have coloured the region, and
would have proved less: such a case shows the message and hides which arm
produced it.

The tension worth naming is that this decision landed in the same work that
exempted two crates from the coverage gate entirely. A tool that may be
suspended for a whole crate is plainly not an authority over one function's
shape. The exemptions are a bounded, ticketed debt; they are not a licence to
arrange code around the tool where it does run.

## Considered options

**Let the tool decide, and treat a red region as a licence to split.** Rejected.
It reports full coverage over a shape that was arranged for it, so the number
stops measuring what it claims to measure -- and the helper it produces has no
name of its own and one call site.

**Forbid a split during a move, with no exception.** Rejected as the rule to
write down. It is the right default and the spec already states it, but stated
absolutely it would also forbid the two splits above, which are improvements a
reviewer would ask for on their own terms.

**Ask what the extracted half is called.** Accepted. It separates the two cases
with one question a reviewer can answer from the diff.

## How to tell them apart in review

Ask what the extracted half is called and what its contract is. If the answer
only makes sense as "the part the tool could not reach", the split is the wrong
fix -- write the test that reaches the branch where it lives, or state why the
branch is unreachable and remove it.

## Consequences

- The two diagnostics reporters keep their own doc comments, because a split
  that cannot be described is a split that was made for the tool.
- A region the gate reports as unreached is answered with a test, an argument
  that it is unreachable, or a recorded exemption -- never with a new function
  boundary.
