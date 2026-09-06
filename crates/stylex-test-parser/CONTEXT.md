# stylex-test-parser

A developer binary, not a library and not part of the compiler. It walks the
upstream JavaScript StyleX checkout, finds its test files and snapshots, strips
what cannot run here, and writes the result under `./output/__tests__/<package>`
as fixtures for this project's tests to compare against.

## Language

**StyleX path**:
The `--stylex-path` argument (short `-p`), which names the upstream
`stylex/packages` checkout. It defaults to `../../../stylex/packages`. Not a
workspace dependency, so nothing guarantees it is present or current.
_Avoid_: source, upstream dir, repo

**Blacklisted callee**:
A call the rewriter removes rather than translates — `require` and `jest`.
_Avoid_: filtered call, excluded function

**Generated fixture**:
A file written into `output/`. Regenerating overwrites, so a hand edit is lost
on the next run. A fix belongs upstream, or in a hand-written test elsewhere.
_Avoid_: snapshot, golden, test file
