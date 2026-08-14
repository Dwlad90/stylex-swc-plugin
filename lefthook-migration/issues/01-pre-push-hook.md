# Add a `pre-push` hook

Status: `resolved`
Type: `task`
Blocked by: the lefthook migration landing

Deliberately held out of the migration PR so that any regression there is
attributable to "same behaviour, new runner" and nothing else.

## Scope

Two jobs, both fast and turbo-cached:

- `cargo clippy --all-targets --all-features --workspace -- -D warnings`
  (already exposed as `lint:crates:workspace`)
- `pnpm run test:scripts`

> [!NOTE]
> "Both fast and turbo-cached" was wrong about both jobs, and the measurements
> are below. The scope is unchanged -- both jobs exist -- but neither runs
> unconditionally, which is the correction this section needs and did not get
> before the first review.

## Explicitly rejected

`pnpm lint:type-aware`. Its findings are real -- `CLAUDE.md` notes they are
errors invisible to `lint:check` -- but it runs `turbo run build` first. A
multi-minute pre-push gets disabled by the team within a week, which is worse
than not having the hook.

## Notes

Gate on branch so pushes to `develop`/`master` are not double-checked against
what CI is about to run anyway.

## Answer

Landed as the `pre-push` block in `lefthook.yml`. The hook budget is 10s, and
holding to it is what shaped every deviation below.

### Both jobs are conditional, neither is unconditional

- **`clippy`** is not turbo-cached -- `lint:crates:workspace` shells straight
  out to `cargo`. Measured 0.56s fully warm but **42.86s on a cold populate**,
  and touching any crate pushes it back toward the cold figure. Gated behind
  `STYLEX_SLOW=1`, which is the same opt-in `rust-audit` uses. CI runs clippy on
  every PR regardless, so nothing is lost but the local minute.
- **`test:scripts`** is `node --test`, not turbo, so there is no cache at all:
  **~15s** every time. Glob-gated to `scripts/**`, `.github/scripts/**`,
  `.lefthook/**` and `lefthook.yml` -- the only paths that can regress it. CI
  runs it unconditionally as `ci-script-tests`.

A conditional job was the honest reading of "fast". An unconditional 40s-plus
push is the thing this issue's own "rejected" section rules out for
`lint:type-aware`, and it would earn the same `--no-verify` habit.

### The branch gate

`skip: [{ref: develop}, {ref: master}]`, carried on a YAML anchor so the four
gated jobs cannot drift apart. Applied to `rust-fmt`, `markdown`,
`dead-exports` and `script-tests` -- each has a CI counterpart
(`format-check`, `lint-dead-exports`, `ci-script-tests`), which is the premise
the note above rests on.

**`merge-conflicts` is deliberately not gated.** It is a single
`git diff --check` and the one check here with no CI equivalent on a direct
push to a shared branch, so gating it would trade the cheapest check for the
one case it most needs to cover.

### Jobs added past the original two

Recorded here rather than in a separate issue, because they arrived with the
same hook and answer to the same 10s budget:

| Job | Why |
|---|---|
| `merge-conflicts` | Exact-ref scan of the pushed commits; no CI counterpart |
| `rust-fmt` | `cargo fmt --all -- --check`; catches what per-file staging misses |
| `markdown` | `check-markdown.mjs {push_files}`, the `--no-verify` backstop |
| `dead-exports` | Knip export scan; cheaper to hear now than in review |
| `rust-audit` | `cargo deny` + `cargo audit`, opt-in with `clippy` |
