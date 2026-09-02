# 21 — Restore the local test gate

**What to build:** `pnpm test` currently echoes a skip line for every crate
and runs no Rust test at all, so the gate this repo documents as the thing to
run before calling any task complete proves nothing locally. CI still runs the
workspace suites separately, so this is a local-gate loss rather than a CI
one. Make the documented command exercise the Rust suites again — either by
restoring the per-crate test scripts or by having the root command chain the
workspace suite — and pick one deliberately rather than leaving both
half-wired. Two generated-fixture drift checks were collateral damage: both
were dropped from the scripts that used to call them and are now invoked by
nothing, so a stale generated fixture is unguarded.

**Blocked by:** None — can start immediately. This ticket gates most of the
others, because until it lands they cannot verify themselves locally.

**Status:** resolved

- [x] `pnpm test` runs the Rust test suites, or the root `test` script chains
      the workspace suite — one or the other, deliberately chosen. The root
      script chains: `test:scripts && test:crates:workspace && turbo run test`
- [x] The value-parser-cases check and the parse-float-cases check are each
      invoked by a pretest, turbo task or hook. Each is its crate's `pretest`,
      and the value-parser task names the cross-crate corpus in its `inputs`
      so the Turbo cache cannot replay a pass over stale rows
- [x] The unreachable `|| true` after the echo is gone
- [x] The root agent instructions, the testing guidelines and the scripts
      guidelines all describe what actually runs. `STRUCTURE.md` needed the
      same correction and got it
- [x] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the newly-restored test command, each run
      directly rather than piped into a pager, whose exit code would mask a
      failure. Re-run `typecheck` after committing, because the pre-commit
      hook rewrites code

## Comments

**Resolved.** The choice is one workspace run, not per-crate runs.

`cargo nextest run --workspace` compiles the shared dependency graph once and
runs 8,188 tests in 17.5 s. Twenty-five per-crate `cargo` invocations would
re-resolve and re-link the same graph twenty-five times for the same tests, so
the crate packages keep a skip line and the root script chains the workspace
suite ahead of the JS suites:

```
test:scripts && test:crates:workspace && turbo run test --continue
```

`test:scripts` came back into the chain because `SCRIPTS.md`, `HOOKS.md` and
the CI workflow all describe `pnpm test` as running it first; it left the
script in `40571d6f8`, a commit about benchmark reporting that said nothing
about it. The Rust leg runs before the JS leg because it needs no `.node`
build, so the cheap leg fails first.

The two generated-fixture checks are each their crate's `pretest`, which is
the shape `stylex-rs-compiler` already uses for `parity:harvest:check`. Both
were proved to fail on a deliberately stale fixture and to pass once restored.

**One unrelated artifact travelled with this.**
`parity/corpus/harvested.json` records the source line of each harvested
declaration, and `767a99dc1` — a documentation commit — moved three lines in
`crates/stylex-css/src/css/tests/normalize_value_test.rs` without
regenerating. The stale corpus was invisible while `pnpm test` ran nothing;
restoring the gate exposed it, so the regenerated corpus lands here. Row order
and every id are unchanged, so `postcss-value-parser/src/tests/cases.rs` is
untouched. This is the drift [ticket 23](./23-fix-the-parity-harvest.md)
describes, not a fix for it.

**Review found one hole and it is closed.** A `pretest` only runs when Turbo
runs the `test` script, and Turbo hashes a package's own files. The
value-parser fixture is generated from the parity corpus in a *different*
package, so a harvest could leave `cases.rs` stale and the gate would replay a
cached pass over it. `@stylexswc/postcss-value-parser#test` now declares

```
"$TURBO_ROOT$/crates/stylex-rs-compiler/parity/corpus/*.json"
```

as an input. Proved by mutating one corpus value: before, a cache hit and no
check; after, a cache miss and the check fails. The `stylex-utils` generator
reads only its own package, so its task needs nothing extra.

**One performance win taken.** The script suite is 15.5 s and ran
unconditionally outside Turbo. It is now the `//#test:scripts` task, so an
unchanged tree replays it in 0.34 s. Invalidation was proved with a content
change under `scripts/git/`; `pnpm test:scripts` still runs the suite directly
for CI and the pre-push hook.

**Both remaining performance findings were taken after all.**

- The four root Rust tasks hashed the whole tree plus every `*.rs` and
  `*.toml`. Each now hashes `crates/**`, `Cargo.lock`, `rust-toolchain.toml`
  and, for the test tasks, `.config/nextest.toml`. A false cache hit is the
  risk, so every direction was measured: a source, a fixture, `Cargo.lock`,
  the nextest config and the toolchain file each still miss; a root markdown
  file, a guidelines page and a TypeScript source now hit. The hashed set is
  byte-identical across a full gate run.
- `test:crates:workspace:regular` no longer passes `--profile ci`, so a local
  failure is reported at once instead of after two retries. CI sets
  `NEXTEST_PROFILE=ci` on its own command, and the variable is declared on the
  Turbo task, so a run under one profile never replays under the other.

**A suite now holds the wiring open.**
`scripts/git/generated-fixtures.test.mjs` asserts that each generator has a
`:check`, that `test` or `pretest` reaches it, and that a generator reading
another package has a Turbo input to match. It was proved to fail on both
regressions this ticket fixed.

**One review finding was declined by the maintainer.** The 24 identical skip
lines are duplicated, and the reviewer proposed collapsing or removing them.
The crates keep their `test` scripts.
