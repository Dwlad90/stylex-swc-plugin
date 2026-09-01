# stylex-test-parser

A developer binary, not a library and not part of the compiler. It walks the
upstream JavaScript StyleX checkout, finds its test files and snapshots, strips
what cannot run here, and writes the result under `./output/__tests__/<package>`
as fixtures for this project's tests to compare against.

## Language

**StyleX path**:
The `--stylex-path` argument: where the upstream `stylex/packages` checkout
lives. A sibling checkout, not a dependency — nothing in the workspace
guarantees it is present or current.
_Avoid_: source, upstream dir, repo

**Blacklisted callee**:
A call the rewriter removes rather than translates — `require`, `jest`. The
binary is not building a test runner; it is salvaging the assertions.
_Avoid_: filtered call, excluded function

**Generated fixture**:
A file written into `output/`. Regenerating overwrites, so an edit made by hand
is lost on the next run — a fix belongs upstream or in a hand-written test
elsewhere.
_Avoid_: snapshot, golden, test file
