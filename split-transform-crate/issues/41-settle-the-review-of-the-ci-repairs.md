# 41 — Settle the review of the CI repairs

**What to build:** A two-axis review of everything since `517cb0d95` — the
tickets 21, 37, 38, 39 and 40 work — reported findings on both axes. Six
survived verification; five did not, and the reasons are recorded below so the
same arguments are not re-run.

**Blocked by:** None.

**Status:** resolved

- [x] `scripts/packages/test/index.sh` searches the directories a crate has.
      Almost no crate has a `tests/` directory, a missing path is an error to
      grep, and that error outranked the match found in `src` beside it — so
      the runner reported "no tests" for every crate and skipped the suite in
      silence
- [x] The same script no longer folds `basename`'s trailing newline into the
      target directory name
- [x] The script says why it has no caller, and `guidelines/SCRIPTS.md`
      repeats it
- [x] The allocator checks read `package.json` with `serde_json` rather than by
      scanning for the first `"targets"` key, which removes the `.expect()`
      calls `guidelines/stack/RUST.md` rules out
- [x] The Windows binding-name rule is folded once, in `bindingPathKey`
- [x] `stylex-utils` strips a trailing carriage return, and the
      generated-fixture gate states the rule
- [x] The workspace gate is green in **debug** — never `--release`:
      `format:check`, `lint:check`, `lint:shell`, `typecheck`, `pnpm test` and
      `pnpm test:crates:workspace`, each run directly rather than piped into a
      pager, whose exit code would mask a failure. Re-run `typecheck` after
      committing, because the pre-commit hook rewrites code

## Comments

**What each fix is worth.** The test-runner fault is the serious one: it is the
"a green run is not proof tests ran" shape, and it applied to every crate. The
rest are correctness in a reader, or one rule kept in one place.

**New cases, per fix.** The runner: tests in `src` alone, in `tests` alone, in
both and in neither, each of the four markers, a marker in a file that is not
Rust, an argument holding a space, a crate name a path cannot carry, a crate of
two thousand files. The manifest reader: a second `targets` key ahead of
`napi`, a comma inside a name, a JSON-escaped name, a non-string entry, a list
that is not an array, no `napi` block, an empty manifest, twenty thousand keys.
The binding name: upper case, a long-path prefix and its UNC form, forward
separators, another package's addon, POSIX case kept, a backslash as an
ordinary character, a path two thousand levels deep. The gate: a check reading
bytes, the flag in three positions, a `diff` in a later pipeline stage, a check
with no `diff`, and an argument merely ending in `diff`.

**Findings not taken, with the reason.**

- *`--profile ci` in the shared script reverses a documented rule.* It does
  not. `develop` carries `--profile ci` in `test:crates:workspace:regular`, and
  ticket 37 asked for exactly that arrangement back. The rewritten
  `guidelines/SCRIPTS.md` states the restored behaviour and its local cost
  honestly.
- *`pnpm-workspace.yaml` carries 243 lines of quote churn.* `oxfmt` demands the
  new form; reverting it fails `format:check`. Tooling-enforced, so out of
  scope for a review.
- *`realPathOf` is a Middle Man.* It is the one place the decision to use the
  operating system's resolver rather than the JavaScript one is written down,
  for three callers. Inlining it deletes the reason and keeps the call.
- *The allocator's `#[cfg]` scanner is Speculative Generality.* The half that
  read `package.json` was, and is gone. The half that reads the attribute out
  of `lib.rs` is the load-bearing one: it is what makes a new musl target a
  decision rather than a line to copy, and reading it off the attribute the
  compiler obeys is the point.
- *Ticket 20 is untouched.* It is `backlog`, and its criteria need a release
  run observed end to end. Not a gap. One thing was owed and is now paid: the
  measurement the musl work observed is recorded there.
