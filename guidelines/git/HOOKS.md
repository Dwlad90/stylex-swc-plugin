# Git Hooks

Driven by `lefthook` and configured in `lefthook.yml`. Hook scripts live in
`.lefthook/<hook>/`; anything CI also invokes stays in `scripts/git/`.

Installed by `pnpm install`, via `prepare` -> `scripts/git/install-hooks.sh`.
Lefthook writes into the common git dir, so one install covers every worktree.

## The hooks

- `prepare-commit-msg` -- the `commitizen` prompt, skipped on amend and on
  `message`/`merge`/`squash` commit sources. Aborting the prompt aborts the
  commit.
- `pre-commit` -- lint and format of staged files (JS/TS, data files, Markdown,
  manifests, shell, Rust, TOML) plus the version-mismatch check on commits that
  touch a manifest or the lockfile. Skipped during merges and rebases.
- `commit-msg` -- `commitlint`.
- `pre-push` -- `cargo fmt --check` and the full Markdown sweep. Budget is 10s.
- `post-checkout` / `post-merge` / `post-rewrite` --
  `scripts/git/install-changed-deps.mjs`, which reinstalls only when a lockfile
  actually moved.

Escape hatches: `git commit --no-verify`, `LEFTHOOK=0`, or an untracked
`lefthook-local.yml` for per-developer overrides.

## Rust and the slow opt-in

`rustfmt` formats staged `*.rs` files, never `cargo fmt` -- `cargo fmt` has no
per-file mode, so with `stage_fixed` it would widen a commit with unrelated
files that merely happened to be unformatted.

One caveat: rustfmt follows `mod` declarations, so staging a `lib.rs` or
`mod.rs` reformats its submodules on disk too. Those rewrites are never
staged -- `stage_fixed` re-adds only the paths lefthook passed in -- so they
appear as ordinary unstaged changes rather than in your commit.
`--skip-children` would suppress it but is nightly-only.

The edition comes from `rustfmt.toml`, and has to: a directly invoked `rustfmt`
never reads `Cargo.toml`, and with no config at all it falls back to edition
2015 and rejects valid 2024 code as a parse error. Do not pass `--edition` on
the command line -- it overrides the config file, which is how the hook would
end up formatting against a stale edition after a bump.

`clippy` is opt-in on `pre-push` because it measures 0.56s cached and ~43s cold:

```sh
STYLEX_SLOW=1 git push          # run it with the push
lefthook run pre-push --job clippy   # run it on its own
```

CI runs clippy on every PR regardless, so the opt-in only affects how early you
hear about it.

## Dependency reinstalls

`install-changed-deps.mjs` diffs `pnpm-lock.yaml` and `Cargo.lock` across the
ref move (`HEAD@{1}`..`HEAD@{0}`) and runs, respectively,
`pnpm install --prefer-offline --prefer-frozen-lockfile` and `cargo fetch`. The
two graphs are independent, so a failing install does not stop the other.
`cargo fetch`, never `cargo build` -- fetch is network-only and cheap.

These are `post-*` hooks: git ignores their exit status, so they cannot fail the
operation that triggered them. Set `STYLEX_SKIP_INSTALL=1` to opt out.

## Binaries are addressed by path

Every npm-provided tool is invoked as `./node_modules/.bin/<tool>`. Lefthook
adds nothing to `PATH`, so a bare `oxlint` resolves from whatever the caller
happens to have -- fine in an interactive shell, missing in a GUI git client or
an IDE commit dialog, and in the `shellcheck` case it silently picked up the
Homebrew binary instead of the pinned devDependency. It is also the largest
single performance win, since `pnpm exec` costs ~1s per invocation.

This is safe only because no job sets `root:`; a job that does needs an absolute
path instead. `rustfmt` and `cargo` are the deliberate exceptions -- they come
from the Rust toolchain, not from `node_modules`.

## Committing without a terminal

A bare `git commit` reaches the commitizen prompt, which needs a TTY. Without
one -- a script, an agent, some IDE integrations -- the prompt has nothing to
read, commitizen exits non-zero, and **the commit aborts**. The husky hook
swallowed that failure and fell through to the editor; this one does not, so
that abandoning the prompt is a real abort rather than a silent empty message.

Pass a message instead: `git commit -m` or `-F` sets the commit source to
`message`, which the guard skips before commitizen is ever reached.

## Partial staging

Safe. On every `pre-commit` invocation lefthook reverts the unstaged hunks of
partially staged files before any job runs, so formatters see staged content
only; it restores them afterwards. If the hook's edits conflict with the hidden
changes it reverts everything and errors with `conflict while merging unstaged
changes` rather than committing a merge you did not ask for.

`stage_fixed` still runs a whole-file `git add`, but since the file on disk
holds only what you staged at that point, the net effect is that an unstaged
hunk cannot be swept into the commit. `git add -p` and a formatter coexist.

(This section previously said the opposite. It described lefthook before 2.1.7,
which is where the stash guard landed.)

## Changing the config

`lefthook.yml` carries comments explaining why each job is shaped the way it is;
several encode bugs that were already fixed once. Constraints worth knowing
before editing:

- Job ordering that matters is expressed with `piped: true`, never by list
  position. Jobs at the same level run concurrently.
- Globs match the whole path, unlike lint-staged's basename matching. A bare
  `package.json` catches only the root manifest.
- Never add `parallel:` to a `group:` that sets `piped:`. At hook level the pair
  is a runtime error; on a group it is accepted and `parallel` silently wins,
  discarding the ordering the group exists to express.
- `{N}` in a `run:` template expands only when git actually passed an Nth
  argument -- otherwise the literal `{2}` reaches the shell. A hook that needs
  arguments git may not supply has to be a `script:`, which receives the full
  unmodified argv.
- Environment conditions go in `only:`/`skip:` as `run: test -n "$VAR"`. `skip`
  and `only` accept only `ref:` and `run:`; an `env:` key passes validation and
  is a silent no-op.
- `pre-commit` and `pre-push` skip _every_ job when the resolved file list is
  empty, keyed on the hook name rather than on whether a job has a glob. There
  is no config-level opt-out, so `pre-push` is a convenience gate, never a
  guarantee.

Run `pnpm hooks:validate` after editing, and note what it does not catch: it is
pure JSON-schema validation, so every semantic trap above passes it cleanly.

The guard logic is covered by `scripts/git/hooks.test.mjs` and
`scripts/git/install-changed-deps.test.mjs`, which run under `pnpm hooks:test`
and `pnpm test:scripts`.
