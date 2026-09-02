# 24 — Close the proven test holes in the new crates

**What to build:** Several of the new crates sit at 100% region coverage and
are still wrong, because coverage measures lines rather than behaviour. The
review proved four such holes by reading the tests.

The candidate index is generic and production instantiates it six ways, but
its tests instantiate three — and one of those carries a comment claiming to
stand for indices that are really keyed by something else. The coverage tool
keeps only the best-covered instantiation, so the generic reads as fully
covered while three real ones never run.

The file-offset helper has a saturating path that release ships, but every
test builds offsets through a test-only constructor that bypasses both it and
its debug assertion.

The diagnostics code-frame entry points are all generic over an injected
state trait, and the only implementation the crate can see is its own test
double — so the gate measures the double exclusively while the real
implementation lives in an excluded crate.

Three panic tests assert that a panic happened but not which message, while
the code under test routes three different inputs to three distinct messages;
one non-export code path is never asserted; and two evaluator error arms have
no direct test.

Finally, the corpus holds roughly 169 normalization-shaped CSS values that are
asserted only through whole-transform integration tests plus a generated
fixture, with no direct unit test anywhere.

**Blocked by:** 21

If this ticket overflows a single context, split the corpus normalization
criterion out first — it is the largest and the least coupled to the rest.

**Status:** ready-for-agent

- [ ] The candidate index's untested production instantiations are exercised,
      including that dummy spans collide and that one name under two syntax
      contexts stays apart
- [ ] The file-offset saturating path is asserted, with its debug assertion
      covered too
- [ ] The three message-blind panic tests assert their distinct messages
- [ ] The non-export pattern-bound-call path is asserted
- [ ] The evaluator's refused result and its two distinct declaration-check
      messages have direct tests
- [ ] The nine diagnostics trait methods are exercised through a real state
      manager, including the seen-module source round-trip and the one rule
      the test double encodes: the key-span index is dropped when the module
      is replaced
- [ ] The corpus's normalization values gain direct unit tests, with expected
      values taken from a harness run rather than from the review document
- [ ] The review's own unclosed check is closed: the three rewritten test
      files are byte-diffed against the base branch to confirm no test *input*
      changed when its cases were renamed. The review matched them by scenario
      and assertion count only, so a subtle input change inside a rewritten
      case would not have been caught — and these tests are the refactor's
      central invariant
- [ ] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code
