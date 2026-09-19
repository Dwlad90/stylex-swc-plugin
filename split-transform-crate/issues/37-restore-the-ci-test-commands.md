# 37 — Restore the CI test commands the base branch had

**What to build:** [Ticket 21](./21-restore-the-local-test-gate.md) made the
root `test` script an aggregate — `test:scripts && test:crates:workspace &&
turbo run test` — and every CI job that called `pnpm run test` inherited legs
it cannot run. The repair taken there was to bypass the script in CI, so four
`Test bindings` jobs and the `tests-nodejs` leg called
`pnpm exec turbo run test` directly, and the `tests-rust` leg carried
`NEXTEST_PROFILE=ci` on its command line.

The maintainer's call: CI runs the scripts this repo ships, in the
`pnpm run <script>` form it had on `develop`, and no new script is added to
make that work. That fixes the root `test` script back to the JavaScript half
and leaves the Rust half where every crate's skip line already points —
`pnpm test:crates:workspace`, which is what the `tests-rust` and
`tests-rust-doc` legs run and the only place `cargo-nextest` is installed.

**Blocked by:** None.

**Status:** resolved

- [~] `.github/workflows/pr-validation.yml` is byte-identical to `develop`.
      `npm.yml` no longer is: `8689f614e` landed after this ticket closed and
      added `--allow-base-refusals` to both paired benchmark legs. That change
      is [ticket 38](./38-let-the-release-gate-read-a-published-base.md), which
      owns it; nothing here is owed
- [x] No CI step calls `pnpm exec turbo run test`
- [x] The root `test` script is `turbo run test --continue`, so
      `pnpm run test --filter=<pkg>` resolves to the task graph `develop` had
- [x] `test:crates:workspace:regular` carries `--profile ci` again, and the
      `NEXTEST_PROFILE` declaration it made necessary is gone from `turbo.json`
- [x] `//#test:scripts` is gone from `turbo.json`; `test:scripts` runs outside
      Turbo, as the `ci-script-tests` leg and the `pre-push` hook already ran it
- [x] `scripts/packages/test/index.sh` is restored
- [x] The crates keep their skip line and are exercised only by the workspace
      suite
- [x] The agent instructions, the testing guidelines and the scripts
      guidelines describe what actually runs
- [x] No check `develop`'s CI ran has been dropped

## Comments

**Resolved.** At the time of closing, the two workflow files matched `develop`
byte for byte, so the question "did a check change?" was answered by inspection
rather than by argument.

**That is no longer the whole answer, and the criterion says so.** `npm.yml`
gained `--allow-base-refusals` afterwards. The inspection test still works, it
just needs one ticket read beside it: `git diff origin/develop HEAD --
.github/workflows/` should show that change and nothing else.

Three things ticket 21 introduced were kept, because none of them changes what
CI checks — each closes a cache hole or narrows a hash:

- `@stylexswc/rs-compiler#test` keeps `$TURBO_ROOT$/crates/**/*.rs` in its
  inputs, so a Rust test edit in another crate still invalidates the
  `parity:harvest:check` pretest (ticket 33).
- `@stylexswc/postcss-value-parser#test` keeps the cross-crate corpus in its
  inputs, so the generated-fixture check cannot replay a cached pass over a
  stale `cases.rs`.
- The four root Rust tasks keep `crates/**`, `Cargo.lock`,
  `rust-toolchain.toml` and `.config/nextest.toml` instead of hashing the whole
  tree.

**The local-gate loss ticket 21 named is back, deliberately.** `pnpm test` runs
the JavaScript suites and a skip line per crate; it does not run the Rust
suites. The guidelines and the agent instructions now name both commands
together, so the documented gate is `pnpm test` plus
`pnpm test:crates:workspace` rather than one command.

**`scripts/packages/test/index.sh` has no caller.** The crates keep the skip
line, so nothing invokes `scripty` for `test`. It was restored on the
maintainer's instruction and stands ready for the crate that points its `test`
back at `scripty`, or for a direct run from a crate directory. Its siblings
`coverage.sh` and `flamegraph.sh` are reached by `test:coverage` and
`test:flamegraph` and always had callers.
