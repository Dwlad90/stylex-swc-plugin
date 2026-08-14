# Manual verification checklist

Status: `resolved`
Type: `task`

Run once, on a real terminal, before merging. Everything here needs a TTY or a
real git state machine, which is why it is not automated.

**All boxes ticked -- the migration is verified end to end.** Two things this
checklist produced that outlive it, both recorded in place below:

1. Ctrl-C aborting the commitizen prompt is the one intentional behavioural
   break from husky. It wants a changelog line, not a fix.
2. The generated hooks hardcode an absolute path into *this* worktree's
   `node_modules`. Siblings resolve the lefthook binary through it, so removing
   or re-installing this worktree changes their behaviour.

~~**The interactive commitizen prompt is the one claim in this migration that was
never verified by running it**~~ -- retired. It was designed from the lefthook
docs (`interactive: true` opens `/dev/tty`) and from confirming that scripts
receive the hook argv; both have since been observed on a real terminal.

Confirmed against commit `828703bbf`, which was made through the real hooks:
install, `pre-commit` across all six jobs on 20 files, `commit-msg`, and the
`prepare-commit-msg` guard declining a `-F` commit.

Second round, on top of `52fbc0d27`, all driven off one staged mismatch
(`@stylexjs/babel-plugin: ^0.19.0 -> ^0.18.0` in `packages/unplugin`):
`version-mismatch` failing the commit, the `rebase` skip, `--amend` and a merge
commit not prompting, the bare-`git commit` prompt, both escape hatches, the
`lefthook-local.yml` override, and the sibling-worktree split above.

## Install

- [x] `git config --get core.hooksPath` shows `.husky/_` beforehand
- [x] `pnpm install` prints the "Removing husky's core.hooksPath" line
- [x] `git config --get core.hooksPath` is now empty
- [x] `<bare>/hooks/` contains `pre-commit`, `commit-msg`, `prepare-commit-msg`
- [x] a **sibling worktree** inherits the hooks and the lefthook binary with no
      install of its own; jobs needing workspace tooling still require
      `pnpm install` there. *Reworded from "runs the hooks without its own
      install", which conflated two things -- the plumbing is inherited, the
      tooling is not:*
      - **Hooks fire.** `core.hooksPath` is unset, so git falls back to
        `$GIT_COMMON_DIR/hooks` = `.bare/hooks/`, shared by every worktree in
        this layout. A detached sibling added with no `pnpm install` and no
        `node_modules` at all still ran `pre-commit` and was correctly blocked
        by `merge-conflicts` on a staged conflict marker (`exit status 1`).
      - **Jobs that shell out to `./node_modules/.bin/*` do not.** The same
        sibling then failed a *clean* commit at `commit-msg`:
        `sh: ./node_modules/.bin/commitlint: No such file or directory`,
        `exit status 127`. A sibling cannot complete a commit without its own
        install. Not a lefthook regression -- husky needed the same install for
        lint-staged -- but the original wording promised more than holds.

      Needs no merge to `develop`: verified with `git worktree add --detach`
      off this branch, removed afterwards.

      One coupling this surfaced, worth recording. `.bare/hooks/pre-commit`
      hardcodes an **absolute path** into *this* worktree's install --
      `.../lefhook/node_modules/.pnpm/lefthook-darwin-arm64@2.1.10/.../lefthook`
      -- and siblings resolve the binary through it. Remove this worktree, or
      reinstall it at a different lefthook version, and siblings fall through to
      the `$dir/node_modules/...` branches, which need their own install.


## Commit message

- [x] bare `git commit` -> commitizen prompt appears and is usable. Run on a
      real terminal; the resulting commit is `bb2518495` in the reflog. This
      retires the caveat at the top of this file -- `interactive: true` opening
      `/dev/tty` is now an observed fact, not a reading of the docs.
- [x] Ctrl-C at the prompt -> **commit aborts** (changed behaviour). Confirmed
      on a real terminal. This is the one intentional behavioural difference
      from husky in the migration, and it is now observed rather than argued:
      the abort is the desired outcome, so it needs no follow-up work -- but it
      does need to reach the changelog, since anyone used to the old escape
      will read it as the hook breaking.
- [x] `git commit -m "feat(x): y"` -> no prompt, commitlint passes
- [x] `git commit -m "nope"` -> commitlint rejects it
- [x] `git commit --amend` -> no prompt (real terminal). Git passes the existing
      SHA as `$3`, so `commitizen.sh` exits at the `[ -n "$SHA" ]` guard before
      reaching `cz` -- the argv assumption the tests encode is the real one.
- [x] a merge commit -> no prompt

Both guard branches are covered by `scripts/git/hooks.test.mjs` (issue `02`), and
git is now confirmed to pass the arguments those tests assume -- `$3` for the
amend/rebase path, `COMMIT_SOURCE=merge` for the merge path.

A rebase replay exercises the same `$3` branch: `prepare-commit-msg` *does* fire
per replayed commit, but git hands it the original SHA and the guard bails in
0.01s. The hook running is expected; prompting would not be.

## pre-commit

- [x] a staged `.ts` file is linted then formatted, and the fix is staged
- [x] two staged `package.json` files in different workspaces both format
      (this was a real bug, caught and fixed -- confirm it stays fixed)
- [x] a staged `.md` file is formatted before `check-markdown.mjs` reads it
- [x] a version mismatch fails the commit. Staging
      `@stylexjs/babel-plugin: ^0.19.0 -> ^0.18.0` in `packages/unplugin` gave
      `🥊 version-mismatch (0.13 seconds)` / `exit status 1`, whole hook 0.35s.
      The glob gate held: `merge-conflicts`, `data-files` and `manifests` ran;
      `js-ts`, `rust`, `toml`, `shell` and `markdown` reported no files.
- [x] a rebase that replays commits does **not** run pre-commit. Tested with the
      mismatch above *committed*, then `git rebase -i --no-ff HEAD~1` -- pre-commit
      would have hard-failed the replay had it run. No pre-commit banner at all,
      `Successfully rebased`. `skip: [merge, rebase]` holds.

## Partial staging (no longer a regression)

- [x] `git add -p` one hunk of a file needing formatting, commit, and confirm
      the unstaged hunks were swept in. Confirmed on a real terminal -- and the
      premise turned out to be obsolete. **They are not swept in.** Lefthook
      2.x reverts the unstaged hunks of partially staged files before any job
      runs and restores them afterwards, so formatters only ever see staged
      content. `4cf802568` corrected `guidelines/git/HOOKS.md` accordingly;
      this heading and `spec.md`'s decision 10 / behavioural difference 1 are
      the last places still framing it as an accepted loss.

      So the sign-off this box asked for is moot: there is nothing to sign off.
      The migration has **two** behavioural differences from husky, not three.

## Escape hatches

- [x] `git commit --no-verify` skips everything
- [x] `LEFTHOOK=0 git commit -m "..."` skips everything. The `LEFTHOOK=0` guard
      is the third statement of each generated hook, ahead of any binary
      lookup, which is also why it is the safe way to script around the hooks.
- [x] a `lefthook-local.yml` override is honoured and stays untracked

## Answer

Every box passes. The migration behaves as designed on a real terminal, and the
two claims that were previously argued from documentation rather than observed
-- the interactive commitizen prompt, and `skip: [merge, rebase]` -- are now
run. Nothing here blocks the merge.

Two outputs outlive the checklist:

- **Changelog, not code.** Ctrl-C at the commitizen prompt now aborts the
  commit. It is the only intentional behavioural break from husky, and it will
  read as a malfunction to anyone who relied on the old escape.
- **A coupling to keep in mind.** The generated hooks in `.bare/hooks/` embed an
  absolute path into this worktree's `node_modules`. Sibling worktrees resolve
  the lefthook binary through it, which is why they run hooks with no install of
  their own -- and why removing or re-installing this worktree changes their
  behaviour. Worth a line in the migration notes so the next person does not
  rediscover it from a broken sibling.

One box was reworded rather than ticked as written: "a sibling worktree runs the
hooks without its own install" conflated inherited plumbing with inherited
tooling. Hooks fire; jobs shelling out to `./node_modules/.bin/*` do not. The
entry now says so.

No `map.md` exists for this effort, so the Decisions-so-far pointer the tracker
asks for has nowhere to go; `spec.md` is the only sibling document.
