# Ticket 12 — the full repository gate

Everything run, with what it said. Nothing skipped except where stated.

- machine: Apple M1 Max, 64 GB, macOS
- Node v24.11.0, pnpm 11, cargo test on the workspace with `--all-features`
- the JS suites ran against a `pnpm build` taken after the last Rust change,
  because they exercise `dist/*.node` rather than the Rust sources

## Versions the parity run resolved

Printed by the harness before anything else, because the reference is held by
`pnpm-lock.yaml` rather than by an exact range in the dependency catalog — it
moves under a `pnpm update` without anything in the parity directory changing.

```text
Subjects
  @stylexswc/rs-compiler   v0.18.4
  @stylexjs/babel-plugin   v0.19.0
  @babel/core              v8.0.1
  style resolution         property-specificity
```

## Results

| Gate                                     | Result                          |
| ---------------------------------------- | ------------------------------- |
| `pnpm typecheck`                          | 40 of 40 tasks                  |
| `pnpm format:check`                       | 20 of 20 tasks                  |
| `pnpm lint:check`                         | pass                            |
| `pnpm lint:shell`                         | pass                            |
| `pnpm lint:type-aware`                    | pass                            |
| `cargo fmt --all --check`                 | pass                            |
| `cargo clippy --workspace --all-features --all-targets` | no warnings, no errors |
| `cargo test --workspace --all-features`   | 7150 passed, 0 failed, 55 ignored |
| `pnpm test`                               | 86 of 86 tasks                  |
| `parity-values.ts`                        | 1103 subjects, 0 changed, 0 unexpected |
| `evidence/sweep.cjs`                      | 27 of 30 comparisons agree; the three that do not are the grammar decision |
| `evidence/grammar-sweep.cjs`              | 45 of 50 combinator shapes agree; the five that do not are the two deliberate differences |
| `parity-positions.ts`                     | 13 of 18 point at the same place, 5 point nowhere on either side, 0 unexpected |
| `fuzz-pseudo-order.ts`                    | 1000 compared, 0 disagreements  |

The 55 ignored Rust tests predate this work and are unrelated to it.

Re-run after ticket 13 and its review fixes, both of which landed after the
first pass of this gate.

## Two things worth stating rather than omitting

**The harvested parity corpus had to be regenerated twice.** It is scanned out
of the Rust test suites, so every commit here that added a declaration moved it,
and `parity:harvest:check` fails rather than waiting for someone to notice. Both
regenerations are in the commits that caused them.

**One JS task failed once and did not reproduce.** `@stylexswc/webpack-plugin`'s
"second build from a cold filesystem cache still emits the full CSS" failed on a
single run and passed on its own and in every full run after. It touches a
filesystem cache under concurrent turbo tasks and nothing in this work reaches
it. Recorded because a flake that goes unrecorded is indistinguishable from one
nobody ran.

**Ticket 11 was drafted, not filed.** Filing two reports against
facebook/stylex is outward-facing and irreversible; the maintainer was asked and
chose to review the drafts and open them personally. Nothing about that changes
code, so the gate did not wait on it. The drafts are in `../upstream/`.

## What is not covered here

`fuzz-shorthand-split.ts` is the nightly sweep — roughly 154k subjects and about
97 seconds — and is not part of this gate. It guards where a value is cut, which
no change here touches.
