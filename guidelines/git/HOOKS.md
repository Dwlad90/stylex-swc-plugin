# Git Hooks

Driven by `lefthook` and configured in `lefthook.yml`. Hook scripts live in
`.lefthook/<hook>/`; anything CI also invokes stays in `scripts/git/`.

Installed by `pnpm install`, via `prepare` -> `scripts/git/install-hooks.sh`.
Lefthook writes into the common git dir, so one install covers every worktree.

## The hooks

- `prepare-commit-msg` -- the `commitizen` prompt, skipped on amend and on
  `message`/`merge`/`squash` commit sources. Aborting the prompt aborts the
  commit.
- `pre-commit` -- version-mismatch check plus lint and format of staged files.
  Skipped during merges and rebases.
- `commit-msg` -- `commitlint`.

Escape hatches: `git commit --no-verify`, `LEFTHOOK=0`, or an untracked
`lefthook-local.yml` for per-developer overrides.

## Committing without a terminal

A bare `git commit` reaches the commitizen prompt, which needs a TTY. Without
one -- a script, an agent, some IDE integrations -- the prompt has nothing to
read, commitizen exits non-zero, and **the commit aborts**. The husky hook
swallowed that failure and fell through to the editor; this one does not, so
that abandoning the prompt is a real abort rather than a silent empty message.

Pass a message instead: `git commit -m` or `-F` sets the commit source to
`message`, which the guard skips before commitizen is ever reached.

## Partial staging caveat

Lefthook has no equivalent of lint-staged's stash. Formatters see the
working-tree file and `stage_fixed` stages the whole file -- so if you
`git add -p` a subset of hunks and a formatter touches that file, the hunks you
left out get committed too. There is no rollback on failure either. Stage whole
files when a formatter is likely to run.

## Changing the config

`lefthook.yml` carries comments explaining why each job is shaped the way it is;
several encode bugs that were already fixed once. Two constraints worth knowing
before editing:

- Job ordering that matters is expressed with `piped: true`, never by list
  position. Jobs at the same level run concurrently.
- Globs match the whole path, unlike lint-staged's basename matching. A bare
  `package.json` catches only the root manifest.

The guard logic is covered by `scripts/git/hooks.test.mjs`, which runs under
`pnpm test:scripts`.
