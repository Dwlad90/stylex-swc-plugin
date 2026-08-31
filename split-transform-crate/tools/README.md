# Move tooling

Throwaway scripts written during
[ticket 08](../issues/08-move-evaluator-core.md) and kept because tickets 12 and
13 are the same kind of mechanical move. Run them from the code worktree root.
Read the caveats before trusting any of them.

## `rewrite_uses.py`

Expands nested `use` trees to leaf paths, re-groups the leaves by which crate now
owns them, and rewrites the statement.

**Why a plain regex is not enough.** The moved module name is almost always
_inside_ a brace group, not in the statement's prefix, so a literal
search-and-replace on `crate::a::b` misses `use crate::{a::{b, c}}` entirely and
reports a false clean. On ticket 08 a literal pass rewrote **0** files where this
rewrote 78.

Edit the `MOVED` map before each run. It also handles a crate's _external_ name
(`stylex_transform::...`), which is how integration tests under `tests/` reach in.

## `visibility.py`

Gives every item in a freshly extracted crate the narrowest visibility that still
compiles. `demote` first, then run bare to iterate: it demotes everything to
`pub(crate)`, then promotes back only what the workspace fails to reach, asking
the compiler rather than guessing.

**Run it with `--tests`.** Without that, test-only consumers in other crates are
invisible and the loop settles on a surface that breaks the moment you build
tests. Its error parsing handles `E0603`/`E0616`/`E0624`; `E0451` (private fields
in a struct literal) is worded differently and needs a separate pass.

**A `#[cfg(test)]` item cannot serve another crate's tests** — a cfg set while
compiling this crate is not set while compiling theirs. `StateManager::for_test`
had to lose its `cfg` and become `pub`.

## `renest.py` — buggy, read this first

Re-nests the flat leaf lists `rewrite_uses.py` produces, and merges duplicate
statements, so the result matches the surrounding style.

**It corrupts two shapes and you must exclude them by hand:**

- a `use` statement containing a **comment** — the comment is spliced into the
  middle of the re-emitted list;
- a statement containing a **glob** (`prelude::*`) — the glob is dropped
  silently, which compiles until something needs the prelude.

On ticket 08 it damaged 11 files this way. Find them before running:

```sh
git grep -l -E 'use (crate|stylex_[a-z_]+)::[^;]*(//|::\*)' -- '*.rs'
```

It also merges statements in files that had nothing to do with the move, which
inflates the diff. Revert any file whose diff contains no reference to the crate
you are extracting.
