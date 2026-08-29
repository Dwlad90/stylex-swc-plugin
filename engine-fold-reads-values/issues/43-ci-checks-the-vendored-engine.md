# 43 — CI checks the vendored engine against its pinned commit

**What to build:** The vendored engine tree is verified against the commit it
claims to be, by a job rather than by a person remembering to run a command.

**The check exists and is manual.** `vendor/boa/README.md` asks whoever bumps the
tree to `diff -rq` it against `VENDORED_COMMIT`. Nothing in `.github/workflows/`
does. The tree is correctly pinned, its licences are carried, and `deny.toml`
covers the patched graph with per-ID ignores — the pinning is not the gap. The
gap is that nothing notices when the tree stops matching the pin.

**It has already failed once**, and the README records it: this repo's own lint
autofix silently rewrote five files under the engine's benches before anyone
noticed. That is the benign version. The same silence covers a merge artefact or
a hand edit in the evaluator, in a dependency that runs the user's own source
text at build time.

ADR 0008 argues the engine is permanent and vendored rather than taken from the
registry. A vendored dependency trades the registry's integrity guarantee for a
local one, and the local one is currently a sentence in a README.

**Blocked by:** none — can start immediately.

**Status:** wontfix

- [ ] A CI job verifies the vendored tree against `VENDORED_COMMIT` and fails on
      any difference
- [ ] The job is in the required gate, not advisory
- [ ] Its failure message says what to do — re-vendor, or update the pin
      deliberately — since a legitimate bump will trip it
- [ ] The README's manual instruction points at the job rather than duplicating it

## Comments

**Closed by 48, without being built.** The vendored tree it guards is gone: the
engine's 0.22.0 release asks for the ICU line this workspace was already on, so
the copy under `vendor/boa` and the `[patch.crates-io]` section that reached it
were deleted rather than pinned. A check that fetches a commit to compare a
directory against has nothing left to compare.

The reasoning survives in the record rather than in a job: ADR 0008 now carries
the sentence this ticket was written to argue -- that a vendored dependency
trades the registry's integrity guarantee for a local one, and that the local one
was a person remembering to run `diff`, which had already failed once. Anything
carried under `vendor/` again should arrive with that job attached.
