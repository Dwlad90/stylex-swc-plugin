# Git Hooks

`lefthook`, configured in `lefthook.yml`. A script belongs in
`.lefthook/<hook>/` only if it needs that hook's own argv or file list
(`commitizen.sh`, `manifests.sh`); anything runnable on its own -- by CI, an npm
script, or plain arguments -- lives in `scripts/git/`.

`pnpm install` installs them (`prepare` -> `scripts/git/install-hooks.sh`) into
the common git dir, so one install hooks every worktree -- but jobs resolve
`./node_modules/.bin/<tool>` in the worktree they fire in, so install per
worktree anyway.

## Commit with `-m`, always

A bare `git commit` reaches the `commitizen` prompt, which needs a TTY; without
one the prompt fails and **the commit aborts**. `git commit -m` (or `-F`) sets
the commit source to `message`, which skips the prompt.

## What runs

- `pre-commit` -- conflict markers, lint and format of staged files (JS/TS,
  data, Markdown, manifests, shell, Rust, TOML), version-mismatch check when a
  manifest or lockfile is touched. Skipped during merges and rebases.
- `commit-msg` -- `commitlint`.
- `pre-push` -- conflict markers on the pushed commits, `cargo fmt --check`,
  Markdown over the pushed files, knip dead exports (`pnpm lint:dead-exports`),
  and `pnpm test:scripts` when the push touches `scripts/`, `.github/scripts/`
  or `.lefthook/`. All but the conflict check are skipped on `develop` and
  `master`, where CI runs the same thing. `clippy` and `rust-audit` are opt-in
  behind `STYLEX_SLOW=1`; the audit has no CI counterpart, so a bypass silences
  it completely.
- `post-checkout` / `post-merge` / `post-rewrite` -- reinstall deps when a
  lockfile moved. `STYLEX_SKIP_INSTALL=1` opts out; these cannot fail the
  operation that triggered them.

```sh
STYLEX_SLOW=1 git push               # include the slow pre-push jobs
lefthook run pre-push --job clippy   # run one job on its own
```

Escape hatches: `git commit --no-verify`, `LEFTHOOK=0`, or an untracked
`lefthook-local.yml`.

Partial staging is safe: lefthook hides unstaged hunks while jobs run, so
`git add -p` coexists with the formatters and no unstaged hunk reaches the
commit.

## Gotchas

- Rust formatting uses `rustfmt` on staged files, never `cargo fmt` (no per-file
  mode). rustfmt follows `mod`, so staging a `lib.rs` reformats submodules on
  disk; those stay unstaged, never in the commit.
- The rustfmt edition comes from `rustfmt.toml` and must stay there -- on the
  command line `--edition` overrides the file and goes stale after a bump.
- Every npm tool is addressed as `./node_modules/.bin/<tool>`: lefthook adds
  nothing to `PATH`, and it avoids `pnpm exec`'s ~1s cost. `rustfmt` and `cargo`
  come from the toolchain instead. Safe only while no job sets `root:`.
- `allowBuilds: {lefthook: false}` in `pnpm-workspace.yaml` is deliberate. On
  `ERR_PNPM_IGNORED_BUILDS` leave it denied rather than running
  `pnpm approve-builds` -- lefthook's postinstall runs before the root `prepare`
  and would install hooks into a stale `core.hooksPath`.
- Supply-chain policy lives in `deny.toml` and is green, so any `cargo deny` or
  `cargo audit` finding is new. Both tools are optional installs;
  `pnpm audit:rust` prints an install line instead of failing.

## Editing `lefthook.yml`

The file's own comments explain each job's shape. Traps:

- Ordering comes from `piped: true`, never list position; same-level jobs run
  concurrently. Keep `parallel:` off any `group:` that sets `piped:` -- there
  `parallel` silently wins and discards the ordering.
- Globs match the whole path, not the basename: `package.json` catches only the
  root manifest.
- A glob decides _whether_ a job runs, never what it looks at. Pass
  `{staged_files}` or `{push_files}` to keep a check proportional to the change.
  `rust-fmt` and `dead-exports` cannot -- `cargo fmt` has no per-file mode, and
  an export is dead only relative to every importer -- so their globs are
  triggers alone.
- `{N}` in `run:` expands only if git passed an Nth argument, otherwise the
  literal `{2}` reaches the shell. Use `script:` when git may not supply it.
- Conditions go in `only:`/`skip:` as `run: test -n "$VAR"`; those keys accept
  only `ref:` and `run:`, and an `env:` key is a silent no-op.
- `pre-commit` and `pre-push` skip _every_ job on an empty file list, so
  `pre-push` is a convenience gate, never a guarantee.

Run `pnpm hooks:validate` after editing -- it is schema-only, so every trap
above passes it. Then `pnpm hooks:test`: any config change fails against the
resolved-config golden until you review the diff and accept it with
`pnpm hooks:dump`.
