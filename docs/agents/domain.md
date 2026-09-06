# Domain Docs

How the engineering skills should consume this repo's domain documentation when
exploring the codebase.

This repo is **multi-context**: a pnpm + Cargo monorepo where `crates/` and
`packages/` each hold separately-modelled contexts.

## Before exploring, read these

- **`CONTEXT-MAP.md`** at the repo root -- one row per context, with its path
  and a one-line statement of what it is responsible for. Use it to pick which
  contexts are relevant; don't read them all.
- **`<context>/CONTEXT.md`** -- the glossary for that crate or package.
- **`docs/adr/`** at the repo root -- decisions that span contexts.
- **`<context>/docs/adr/`** -- decisions scoped to one crate or package.

Read only the ADRs that touch the area you are about to work in.

`CONTEXT-MAP.md` is the inventory: what it lists, exists. An absent file means
nothing has needed defining there yet, so **proceed silently** past it.
`/domain-modeling` (reached via `/grill-with-docs` and
`/improve-codebase-architecture`) adds files lazily, when a term or decision
actually gets resolved.

**Staleness is a delete signal.** When an entry stops matching the code, remove
the term -- and the file, and its map row, once nothing true is left. A wrong
definition gets believed; a missing one sends the reader to the source, which
is always right. Rewrite an entry only when you have just confirmed the new
wording against the code.

## Where they go

A context's docs live inside that context, beside its code:

```
crates/stylex-transform/
├── CONTEXT.md
└── docs/adr/0001-....md
```

Root-level `CONTEXT-MAP.md` and `docs/adr/` hold only what genuinely spans
contexts. The map moves with the files: a row appears when a context grows a
`CONTEXT.md`, and goes when one is deleted.

## Use the glossary's vocabulary

When your output names a domain concept (in an issue title, a refactor proposal,
a hypothesis, a test name), use the term as defined in the relevant
`CONTEXT.md`. Don't drift to synonyms the glossary explicitly avoids.

If the concept you need isn't in the glossary yet, that's a signal -- either
you're inventing language the project doesn't use (reconsider) or there's a real
gap (note it for `/domain-modeling`).

Because contexts are modelled separately, one term can legitimately mean
different things in two of them -- what a "style object" is at the Rust
transform boundary need not match the Node-facing package API. Define such a
term in its own context's `CONTEXT.md`; promote it to a shared definition only
when every context actually agrees.

## Flag ADR conflicts

If your output contradicts an existing ADR, surface it explicitly rather than
silently overriding, and name the file you mean -- a root ADR and a
context-scoped one can disagree, and either may be the newer decision:

> _Contradicts `crates/stylex-rs-compiler/docs/adr/0001-...` -- but worth
> reopening because..._
