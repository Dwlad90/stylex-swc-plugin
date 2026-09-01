# Issue tracker: Local Markdown

Issues and specs for this repo live as markdown files in `.scratch/`.

## Worktree layout -- read this first

This repo is a bare checkout with sibling worktrees:

```
stylex-swc-plugin.git/
├── .bare/            <- the bare git dir
├── scratch/          <- the tracker: a worktree on orphan branch scratch
├── develop/          <- worktree, contains .scratch -> ../scratch
├── master/           <- worktree, contains .scratch -> ../scratch
└── <branch>/         <- worktree, contains .scratch -> ../scratch
```

`scratch/` sits next to `.bare`, **outside every code worktree**. It is itself a
worktree, checked out from the orphan branch `scratch`, which shares no history
with `develop` or `master`. Each code worktree gets a relative symlink
`.scratch -> ../scratch`, so:

- One tracker, shared across all branches -- an issue filed while on `develop`
  is visible from a feature worktree.
- Deleting a worktree never deletes tracker state.
- The path stays valid if the whole tree is moved, because the link is
  relative.

Always read and edit tracker files through the in-worktree path `.scratch/...`,
never through the parent directory. Committing is the one exception -- see
below.

### Why it never lands on a code branch

Tracker state **is** committed -- but only to the orphan branch `scratch`, and
only from the `scratch/` worktree. From a code worktree, never `git add`
anything under `.scratch`: the path there is a symlink into another branch's
checkout, and staging it would carry tracker state onto `develop` or `master`.

Two independent guards keep that from happening by accident, deliberately kept
separate:

- **`.bare/info/exclude`** carries `/.scratch`. This lives in the shared git
  dir, so it covers **every** worktree immediately, including ones created
  before the ignore rule was written and ones on branches where the committed
  `.gitignore` doesn't have the entry yet.
- **`/.gitignore`** also carries `/.scratch`, for plain (non-worktree) clones
  where `info/exclude` doesn't exist.

Both are anchored with a leading `/` so they match only the repo-root entry,
never a `crates/*/.scratch` or `packages/*/.scratch`.

If you add the tracker to a repo where neither guard is present, add one before
creating any files -- not after.

### After creating a new worktree

`git worktree add` does not create the symlink. Run this from inside the new
worktree; it is idempotent and safe to re-run:

```sh
cd "$(git rev-parse --show-toplevel)"
[ -e .scratch ] || [ -L .scratch ] || ln -s ../scratch .scratch
```

The guard matters: a bare `ln -s ../scratch .scratch` run when `.scratch` is
already a **directory** does not fail -- it silently creates `.scratch/.scratch`
inside it. `ln -sfn` does not help either; `-f` replaces an existing file or
symlink, but a real directory still absorbs the link.

Check the result with `readlink .scratch` (expect `../scratch`). If `.scratch`
turns out to be a real directory rather than a link, that worktree has forked
tracker state -- move its contents into the shared directory and replace it
with the link, rather than leaving two trackers.

## Conventions

- One feature per directory: `.scratch/<feature-slug>/`
- The spec is `.scratch/<feature-slug>/spec.md`
- Implementation issues are one file per ticket at
  `.scratch/<feature-slug>/issues/<NN>-<slug>.md`, numbered from `01` -- never a
  single combined tickets file
- Triage state is recorded as a `Status:` line near the top of each issue file
  (see [triage-labels.md](./triage-labels.md) for the role strings)
- Acceptance criteria are a checklist. A criterion is `[ ]` when it is not met,
  `[x]` when it is met, and `[~]` when it is met in part. A `[~]` or a `[ ]`
  criterion must say on the same lines what remains, and must name the ticket
  that closes it. The marker alone tells a reader that a criterion moved; the
  sentence after it tells the reader what to do
- Comments and conversation history append to the bottom of the file under a
  `## Comments` heading

## When a skill says "publish to the issue tracker"

Create a new file under `.scratch/<feature-slug>/` (creating the directory if
needed).

## When a skill says "fetch the relevant ticket"

Read the file at the referenced path. The user will normally pass the path or
the issue number directly.

## Wayfinding operations

Used by `/wayfinder`, which adds a map-and-children structure on top of the
Conventions above. The **map** is a file with one **child** file per ticket.

- **Map**: `.scratch/<effort>/map.md` -- the Notes / Decisions-so-far / Fog
  body.
- **Child ticket**: a file under `.scratch/<effort>/issues/`, named per the
  Conventions above. The question goes in the body. A `Type:` line records the
  ticket type (`research`/`prototype`/`grilling`/`task`); the `Status:` line
  records `claimed`/`resolved`.
- **Blocking**: a `Blocked by: NN, NN` line near the top. A ticket is unblocked
  when every file it lists is `resolved`.
- **Frontier**: scan `.scratch/<effort>/issues/` for files that are open,
  unblocked, and unclaimed; first by number wins.
- **Claim**: set `Status: claimed` and save before any work.
- **Resolve**: append the answer under an `## Answer` heading, set
  `Status: resolved`, then append a context pointer (gist + link) to the map's
  Decisions-so-far in `map.md`.

## Relationship to GitHub Issues

The repo has a public GitHub remote, but it is **not** the tracker -- nothing is
filed there automatically. Publishing an issue publicly is a deliberate act:
run `gh issue create` yourself and record the resulting number in the local
file.
