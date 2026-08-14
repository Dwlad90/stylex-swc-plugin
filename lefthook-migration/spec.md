# Migrate husky + lint-staged + commitizen-via-husky to Lefthook

Status: `resolved` -- applied, issue `03`'s manual TTY checklist is fully
ticked, and the last outstanding item, a rebase run against the lefthook hooks,
is confirmed. Several decisions below were overtaken by what the work found;
each is annotated in place rather than rewritten, since the reasoning is the
record.

The shipped change is **wider than this document originally scoped**, and the
first review of the PR was right to say so. Two things grew it, both recorded
against the decisions they contradict:

1. **Enabling repository-wide checks surfaced gaps the checks themselves had.**
   `shellcheck`, `lefthook validate` and the knip export scan all lived only in
   a hook, so `--no-verify` bought silence rather than deferred feedback. Three
   `basic-checks` legs close that, against decision 5's "the existing entries
   stay as they are".
2. **The `pre-push` follow-up (issue `01`) landed in the same PR**, along with
   the three `post-*` dependency hooks, against decision 6's "exactly three
   hooks". See that issue for the per-job reasoning.

Replace husky (hook runner) and lint-staged (staged-file filtering) with
Lefthook. Commitizen and commitlint are **kept** -- only the machinery that
invokes them changes.

## Decisions

Settled during the design interview; each line is a deliberate choice, not a
default.

| # | Decision |
|---|---|
| 1 | Commitizen stays, invoked from a lefthook `prepare-commit-msg` job with `interactive: true`. `.czrc` is untouched -- it is also the source of truth for commitlint's `type-enum`. |
| 2 | lint-staged is fully translated to native lefthook jobs, not wrapped. |
| 3 | No `core.hooksPath` is set. Lefthook writes to the common git dir, which already covers every worktree. |
| 4 | Lefthook ships as the `lefthook` npm devDep. |
| 5 | ~~CI does not run lefthook. The existing `basic-checks` matrix entries stay as they are~~ -- **half-superseded.** CI still never *runs* hooks; `LEFTHOOK=0` is set workflow-wide. But three legs were added (`lint-shell`, `hooks-validate`, `lint-dead-exports`), each replacing a check that previously existed only inside a hook. `hooks-validate` runs `lefthook validate`, which is schema-checking the config, not executing it. The principle the decision protects -- named per-check signals in the PR UI, and a bypass costing you *later* feedback rather than *no* feedback -- is what drove the additions. |
| 6 | ~~Exactly three hooks, matching today. `pre-push` is a follow-up (issue `01`).~~ **Superseded: seven.** `pre-push` landed in the same PR rather than after it, and `post-checkout` / `post-merge` / `post-rewrite` were added for lockfile-driven reinstalls. The three `post-*` hooks are ergonomics, not gates: git ignores their exit status, so they cannot fail the operation that triggered them, and `STYLEX_SKIP_INSTALL=1` opts out. Holding `pre-push` back would have been the better call for reviewability; it is recorded here rather than reverted because the jobs are covered and documented. |
| 7 | `pre-commit` jobs are non-interactive; the `exec < /dev/tty` from the husky script is dropped. Only `prepare-commit-msg` is interactive. |
| 8 | All downstream references cleaned up in the same change. |
| 9 | `prepare` runs `scripts/git/install-hooks.sh`, which defensively unsets husky's `core.hooksPath` before installing. |
| 10 | `stage_fixed` replaces lint-staged's stash. The partial-staging regression is accepted and documented. |
| 12 | Ordering is expressed as `parallel: true` at the hook level with nested `piped: true` groups. |
| 13 | `pre-commit` gains `skip: [merge, rebase]`. |
| 14 | The `\|\| true` around `cz --hook` is dropped -- aborting the prompt now aborts the commit. |
| 15 | Default lefthook output. |
| 16 | `min_version: 2.1.10`, `assert_lefthook_installed: true`, plus `LEFTHOOK=0` in CI. |
| 17 | `.lefthook/` holds hook-shaped glue only; anything CI also calls stays in `scripts/git/`. **Refined:** "anything CI also calls" was too narrow -- it left `no-merge-conflicts.sh`, `install-changed-deps.mjs` and `audit-rust.sh` looking misplaced. The real discriminator is whether a script can run outside its hook at all: `.lefthook/<hook>/` is for glue that takes the hook's own argv or file list and means nothing without it. `guidelines/git/HOOKS.md` states it that way. |
| 18 | The new `.lefthook/*.sh` scripts are shellcheck-clean rather than excluded. |

## Findings that shaped the design

Each was verified by running lefthook 2.1.10 against a scratch repo, not read
off the docs.

1. **Lefthook installs into the common git dir.** A hook installed from one
   worktree fires in a sibling worktree. No `core.hooksPath` needed; this
   reproduces husky's "install once, all worktrees hooked" behaviour for free.

2. **Lefthook refuses to install while `core.hooksPath` is set.** It prints
   advice and exits **0**. The subsequent commit then runs **no hooks at all**,
   with no error. Since husky leaves `core.hooksPath=.husky/_` in the shared
   bare config, every existing clone would silently lose all checks on the first
   `pnpm install` after this change. This is the migration's single biggest
   breaking point and the entire reason `install-hooks.sh` exists.
   `lefthook install --force` is **not** the fix -- it would write lefthook's
   hooks into `.husky/_`, resurrecting the directory being deleted.

3. **Only `{1}` is substituted in inline `run:` templates.** `{2}` and `{3}`
   pass through as literal text. The commitizen guard reads `COMMIT_SOURCE`
   (`$2`) and `SHA` (`$3`), so it *must* be a script file.

4. **`script:` jobs receive the full git hook argv.** A file under
   `.lefthook/prepare-commit-msg/` was handed
   `[.../COMMIT_EDITMSG] [message] []` -- the guard transplants verbatim.

5. **`script:` jobs get argv, not staged files.** `pre-commit` receives no argv
   from git, so a `script:` job there gets nothing. Jobs needing the file list
   must use `run:` with `{staged_files}`.

6. **Lefthook globs match the full path, lint-staged's matched by basename.**
   `glob: 'package.json'` caught only the root manifest; every workspace
   manifest silently skipped the job *and* was excluded from `data-files`,
   leaving it unformatted. Fixed by globbing both `package.json` and
   `**/package.json`. Verified: all three manifests now route correctly.

7. **`crates/stylex-rs-compiler`'s `lint-staged` devDep is dead.** No script, no
   config, no import references it.

## Behavioural differences from the husky setup

**Two**, both deliberate -- the third was withdrawn on evidence, see below.
Anything else is a bug.

1. ~~**Partial staging.**~~ **Withdrawn -- this difference does not exist.** The
   premise ("lefthook has no stash") was wrong for 2.x: it reverts the unstaged
   hunks of partially staged files before any job runs and restores them after,
   so formatters see staged content only, and it errors with `conflict while
   merging unstaged changes` rather than committing a merge you did not ask
   for. Corrected in `guidelines/git/HOOKS.md` by `4cf802568` and confirmed by
   hand against `git add -p` in issue `03`. Decision 10 above inherits this:
   `stage_fixed` does replace lint-staged's stash, but there is no accepted
   regression attached to it.

   Leaving the original text would have been the costlier error -- it told
   developers to distrust `git add -p` for no reason.

2. **Aborting the commitizen prompt.** Previously `|| true` swallowed it and
   `git commit` fell through to the editor with an empty message. Now Ctrl-C
   aborts the commit.

3. **Merge and rebase.** `pre-commit` no longer runs during merge commits or
   rebase replays. Running formatters with `stage_fixed` while a rebase replays
   commits mutates history mid-flight. `prepare-commit-msg` already opted out of
   merge/squash, so this extends existing intent.

## Files

### Added

- `lefthook.yml`
- `.lefthook/prepare-commit-msg/commitizen.sh`
- `.lefthook/pre-commit/manifests.sh`
- `scripts/git/install-hooks.sh`
- `scripts/git/hooks.test.mjs` -- 20 cases, run by the existing `test:scripts`
  script and so by the `ci-script-tests` matrix entry (issue `02`)
- `guidelines/git/HOOKS.md` -- the hook reference, kept out of `AGENTS.md` so it
  is not loaded into every session

Added past the original list, all of them belonging to the `pre-push` and
`post-*` hooks that decision 6 did not scope:

- `scripts/git/no-merge-conflicts.sh` + `.test.mjs` -- exact-ref conflict-marker
  scan, in `staged` and `pushed` modes
- `scripts/git/install-changed-deps.mjs` + `.test.mjs` -- the `post-*` reinstall
- `scripts/git/audit-rust.sh` and `deny.toml` -- the opt-in supply-chain policy
- `scripts/git/lint-shell.sh` -- the CI counterpart of the `shell` job
- `scripts/git/lefthook-config.test.mjs` +
  `scripts/git/__snapshots__/lefthook-dump.yml` -- the resolved-config golden
  and the `parallel`/`piped` invariant that `lefthook validate` cannot express
- `scripts/git/lib/test-harness.mjs` -- the workspace, stub and `PATH` shape all
  four suites share

### Removed

- `.husky/` (including the generated `_/`)
- `.lintstagedrc.js`

### Edited

| File | Change |
|---|---|
| `package.json` | `prepare`: `husky` -> `./scripts/git/install-hooks.sh`; drop `husky` + `lint-staged` devDeps; add `lefthook` |
| `crates/stylex-rs-compiler/package.json` | drop the dead `lint-staged` devDep |
| `knip.json` | drop `husky`, `lint-staged` from root `ignoreDependencies`; add `lefthook`. Keep `commitizen`, `cz-conventional-changelog`, `@commitlint/cli` -- still invoked, but only from shell knip cannot trace. **Also, unscoped here:** `ignoreExportsUsedInFile` and `tags`, both forced by turning the export scan into a gate -- without them the scan reports file-local exports and `@knipignore`-marked StyleX token objects, and a check that cries wolf on its first run is a check nobody keeps |
| `.github/dependabot.yml` | `code-quality`: drop `lint-staged`. `git-tools`: drop `husky`, add `lefthook` |
| `.github/workflows/pr-validation.yml` | add `LEFTHOOK: '0'` to the workflow `env` |
| `.github/workflows/npm.yml`, `update-visual-snapshots.yml` | add `LEFTHOOK: '0'`, and `--no-verify` on their `git commit` calls -- they were the only committing workflows without it |
| `.gitignore` | add `lefthook-local.yml` |
| `pnpm-workspace.yaml` | `allowBuilds: {lefthook: false}`. Found by CI, not design: pnpm 11 raises `ERR_PNPM_IGNORED_BUILDS` for lefthook's unapproved postinstall. The postinstall is `lefthook install -f` -- the very `--force` finding 2 rules out -- and pnpm runs it *before* the root `prepare`, so approving it would force-write hooks into `.husky/_` on exactly the clones `install-hooks.sh` exists to rescue. Denying is the fix; `pnpm approve-builds` is not. **Widened past one entry:** every existing `allowBuilds` value was re-decided rather than left inherited, and `@swc/core`, `core-js`, `core-js-pure` and `oxc-resolver` flipped `true` -> `false`. This is a runtime-affecting change the spec did not scope. It rides along because the same `ERR_PNPM_IGNORED_BUILDS` diagnostic that forced the `lefthook` entry lists all of them, and a file where `true` sometimes means "audited" and sometimes means "inherited" is not an audit trail. Each entry now carries its reason in a comment; the rule is `true` only where the script produces something the package needs to work |
| `AGENTS.md` | ~~two lines: a Quick Reference entry and a `Detailed Guidelines` link to `guidelines/git/HOOKS.md`~~ -- **both lines are there, but the file was also trimmed around them** (`d69dbbcba`), and `guidelines/SCRIPTS.md` restructured with it. Unrelated to the migration and better as its own commit; recorded rather than reverted |

### Untouched

`.czrc` and `commitlint.config.js`.

Two entries were promised here and not kept:

- **`scripts/git/version-mismatch-check.sh`** -- one line, `pnpm exec syncpack`
  -> `./node_modules/.bin/syncpack`, so the shared script obeys the same
  by-path rule as the hooks that call it. Leaving it would have made the rule
  true of every caller but one.
- **`basic-checks`** -- three legs added, see decision 5.

### Carried along

Two pre-existing defects that the repository-wide checks turned up the moment
they were switched on. Neither belongs to this migration; both are here because
the checks they fail are, and a check that lands red teaches people to ignore
it:

- `scripts/packages/test/index.sh` -- shared cargo arguments moved into an
  array, so a target directory containing a space survives. Found by
  `lint-shell`.
- `packages/postcss-plugin/src/discovery.ts` -- two unreferenced one-line
  wrappers deleted; neither was exported from the package entry point, and both
  only discarded metadata from their `*WithMetadata` counterparts. Found by
  `lint-dead-exports`.

## Rollout

The `core.hooksPath` unset is per **clone**, not per worktree, and
`install-hooks.sh` handles it automatically on the next `pnpm install`. Call it
out in the PR body anyway: a developer who does not reinstall will keep running
husky's hooks against a deleted `.husky/` directory.
