# 33 — Hash what the harvest check reads

**What to build:** `parity:harvest:check` runs as the `pretest` of
`@stylexswc/rs-compiler` and scans the Rust test sources of every crate. The
Turbo task it sits in front of, `@stylexswc/rs-compiler#test`, declares no
input outside its own package, so a Rust test edit in another crate leaves the
task's hash unchanged. Turbo then replays the cached pass and the `pretest`
never runs — a green local run over a stale corpus, which is the exact failure
mode ticket 21 fixed for `postcss-value-parser` and the reason the root
`turbo.json` names that package's outside input.

The wiring suite in `scripts/git/generated-fixtures.test.mjs` does not report
it, because it looks only at `generate:*` scripts and the harvester is called
`parity:harvest`. So the rule it states is right and its reach is too narrow:
what makes a task need an outside input is that something it runs reads outside
the package, not what that script is named.

This predates the crate split. It got wider with ticket 23, which changed the
harvester from reading two crates to reading all of them.

**Status:** resolved

- [x] `@stylexswc/rs-compiler#test` hashes the Rust sources the harvest check
      reads, and a test edit in another crate re-runs the check
- [x] The wiring suite reaches a generator whatever its script is named, and
      fails when the input is taken away again
- [x] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code

## Comments

Resolved. `@stylexswc/rs-compiler#test` at the root `turbo.json` now declares
`["$TURBO_DEFAULT$", "$TURBO_ROOT$/crates/**/*.rs"]`. The task hashes 1032
files, 886 of them Rust sources -- the same set the harvester walks, since the
`__swc_snapshots__` directories are *named* `*.rs` but hold `.js`, and `target`
is ignored. A probe edit in `crates/stylex-css/src/lib.rs` moves the task hash,
which is the failure this ticket was filed for.

The task-scoped entry repeats `outputs`, because a `package#task` entry
overrides the base `test` task rather than merging with it. It does **not**
repeat `dependsOn`: the crates' shared `crates/*/turbo.json` declares
`test.dependsOn` itself and wins over the root entry, so a copy there would be
inert and would mislead the next reader. A dry run confirms the task still
resolves to `["^do:nothing", "build"]`.

The gate in `scripts/git/lib/generated-fixtures.mjs` now finds a generator by
what the script does. Two signals: the `generate:*` name, which is still
reported with no twin so a half-wired pair cannot hide, and a `:check` twin
that runs the same script file, which is the pair `parity:harvest` forms. The
runner is not read, so a command moved to `tsx` does not make the gate go
quiet, and the value behind an option is stepped over so `--import x.mjs` is
not mistaken for the script. A sweep over all 60 manifests returns exactly the
three real generators.

Known limits, each with a reason to leave it:

- The Turbo-input rule asks only that some declared input names
  `$TURBO_ROOT$`; it does not prove the glob covers what the generator reads.
  Proving that needs the generator's read paths, which only the generator
  knows. Pre-existing, and out of scope here.
- A foreign crate's Rust edit re-runs the whole `vitest` suite (~10s) to
  validate a 0.4s check, against a `.node` that was not rebuilt. A separate
  cached `parity:harvest:check` task that `test` depends on would cut that,
  but it splits the check across two callers and the gate rule with it. Any
  Rust edit already re-runs the workspace `cargo nextest` suite, which is
  minutes, so the 10s is noise. Left alone deliberately.
- `@stylexswc/postcss-plugin#test` in the root `turbo.json` restates the base
  task and drops its `outputs`, so that task has none. Dead config today,
  because the package has no snapshot directory. Not touched here.
