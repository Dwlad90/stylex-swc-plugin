# 94 — Keep the coverage script's statuses to its own table

**What to fix:** two places in `scripts/coverage-missing.sh` where the status a
caller reads was not one the header documents.

**The optional HTML run.** `set -e` is in force and the second
`cargo +nightly llvm-cov nextest ... --html` was unguarded. A failure there
exited with cargo's status -- commonly 101 -- so `exit "$report_status"` was
never reached and a `report_status=1`, a real uncovered region, was silently
replaced. Answered by guarding the run: a failure is reported and the report
status is still what the caller reads.

The `[ "$open" -eq 1 ] && html_flags+=(--open)` line beside it is **not** a
`set -e` bug -- bash exempts the non-final command of an and-or list.

**The no-python3 fallback.** It maps every cargo failure to "uncovered regions".
The direction is safe, but a missing nightly toolchain and a real uncovered
region printed the same code and nothing else. Answered with a distinct message
on the failure branch saying which of the two the output has to be read for.

**Blocked by:** None.

**Status:** resolved

- [x] The HTML run cannot replace the report status
- [x] The fallback says a build failure is not the same as a miss
- [x] `shellcheck` is clean
