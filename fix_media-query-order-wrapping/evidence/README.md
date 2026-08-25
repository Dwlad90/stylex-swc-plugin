# Evidence for the media-query-order wrapping work

Recorded output, not tickets. The tickets are in `../issues/`; these are what
the measurement ones produced.

Kept out of `../issues/` on purpose: that directory is one file per ticket, and
an earlier draft of these files used the same `NN-slug.md` naming, which
collided outright — `02-divergence-table.md` existed in both places at once.
Names here describe the artefact instead, with no number.

| File                  | Ticket | What it holds                       |
| --------------------- | ------ | ----------------------------------- |
| `divergence-table.md` | 02     | Every expectation re-derived        |
| `give-up-length.md`   | 03     | Reference behaviour as ladders grow |
| `ladder-expansion.md` | 05     | The one expectation 02 could not carry |
| `collision.md`        | 06     | Why the declaration loss could not land red |
| `expansion-cost.md`   | 08     | What the restored expansion costs, and its bound |
| `sweep.md`            | 09     | At-rule order, the refusal gate, the ordering option |
| `gate.md`             | 12     | Every gate, with what it said |
| `edge-cases.md`       | --     | One recorded run per edge-case expectation |
| `grammar.md`          | 13     | Which media queries are valid, decided |

## Reproducing

Node scripts, run from anywhere — `ref.cjs` resolves the repository by absolute
path, so there is no working directory to get wrong. They need a built
`crates/stylex-rs-compiler/dist/`, because that is the artefact the comparison
measures rather than the Rust sources.

```sh
# ticket 02
node evidence/emit-table.cjs > evidence/divergence-table.md

# ticket 03, one length per process
node evidence/give-up-length.cjs <rungs>
```

| Script               | Role                                        |
| -------------------- | ------------------------------------------- |
| `ref.cjs`            | Loads both compilers and the shared options |
| `subjects.cjs`       | The expectation corpus, one entry per row   |
| `emit-table.cjs`     | Runs the corpus and writes the table        |
| `give-up-length.cjs` | One ladder length, printed as a verdict     |
| `expansion-cost.cjs` | One ladder length through this compiler     |
| `collision-search.cjs` | Every ordered value map over a collision alphabet |
| `sweep.cjs`          | The three ticket 09 comparisons, as a report |
| `edge-cases.cjs`     | Fifteen edge-case subjects through both compilers |
| `disjunct-cost.cjs`  | One ladder widened by comma disjuncts        |
| `grammar-sweep.cjs`  | Every combinator shape through both compilers |
| `nesting-depth.cjs`  | One parenthesis nesting depth, per compiler   |

A run past roughly 26 rungs costs minutes and gigabytes; 28 took 435 s and
about 7.4 GB resident. One process per length keeps a run that dies from taking
the rest of the sweep with it.
