# 46 — Compare the four coverage exclusion lists

**What to build:** The crates excluded from coverage are named in four places,
and nothing compares them. Ticket 11 took `stylex_state` off three of them; every
local check stayed green and the push failed against the fourth, which is a list
in a test file nobody thinks of as a list.

The four, and how each spells a crate:

| List                                                       | Spelling             |
| ---------------------------------------------------------- | -------------------- |
| `test:coverage:workspace` in `package.json`                | Cargo package name   |
| `EXCLUDED_CRATES` in `scripts/coverage-missing.sh`         | Cargo package name   |
| the `case` in `scripts/packages/test/coverage.sh`          | crate directory name |
| `EXCLUDED` in `scripts/git/crate-coverage-runner.test.mjs` | crate directory name |

The two spellings are not the same string, and not the same string with the
hyphens swapped: `stylex-rs-compiler` is the crate `stylex_compiler_rs`. So a
comparison has to read each crate's own `Cargo.toml` rather than transform the
name.

**Blocked by:** None.

**Status:** done

- [x] A suite in `scripts/git` reads all four lists and names the one that
      disagrees.
- [x] A list that cannot be read is a fault, never an empty list — an empty list
      agrees with everything.
- [x] A row naming a crate the workspace does not hold is a fault, which is what
      a rename or a deletion leaves behind.
- [x] `guidelines/STRUCTURE.md` and both script comments say what compares them.
- [x] The script suite and the workspace suite stay green.

## Comments

**2026-09-06, closed.** `scripts/git/coverage-exclusions.test.mjs` over
`scripts/git/lib/coverage-exclusions.mjs`, in the shape
`generated-fixtures.test.mjs` already uses: the rule is stated against a
synthetic tree, and three cases run against the real repository so it is
load-bearing. `pnpm run test:scripts` went from 292 to 304.

Each list gets its own parser, and each raises rather than answering an empty
list when it cannot find what it is looking for. That is the whole design
question here: a parser that answered `[]` for a file whose array had moved would
make every other list agree with it, which is the one answer this suite must
never give.

Faults are compared against the first readable list rather than pairwise, so the
regression that prompted this — a row on one list of four — reads as one
sentence naming that list, not six naming each other.

Checked against the real fault: the tree at ticket 11's parent, with only
`crate-coverage-runner.test.mjs` still holding `stylex_state`, answers

```
scripts/git/crate-coverage-runner.test.mjs excludes `stylex_state`, which package.json does not
```

and dropping `stylex_transform` from each of the four in turn is caught in all
four directions.

`guidelines/STRUCTURE.md` no longer says nothing compares them; it names the
suite, states the two spellings and says to edit the four together.
