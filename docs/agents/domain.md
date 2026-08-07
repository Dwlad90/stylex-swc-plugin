# Domain Docs

How the engineering skills should consume this repo's domain documentation when
exploring the codebase.

This repo is **multi-context**: a pnpm + Cargo monorepo where `crates/`,
`packages/`, and `apps/` each hold separately-modelled contexts.

## Before exploring, read these

- **`CONTEXT-MAP.md`** at the repo root -- one row per context, with its path
  and a one-line statement of what it is responsible for. Use it to pick which
  contexts are relevant; don't read them all.
- **`<context>/CONTEXT.md`** -- the glossary for that crate, package, or app.
- **`docs/adr/`** at the repo root -- decisions that span contexts.
- **`<context>/docs/adr/`** -- decisions scoped to one crate or package.

Read only the ADRs that touch the area you are about to work in.

If any of these files don't exist, **proceed silently**. Don't flag their
absence; don't suggest creating them upfront. Today none of them exist yet --
that is expected. `/domain-modeling` (reached via `/grill-with-docs` and
`/improve-codebase-architecture`) creates them lazily, when a term or decision
actually gets resolved.

## Where they go

A context's docs live inside that context, beside its code:

```
crates/stylex-transform/
├── CONTEXT.md
└── docs/adr/0001-....md
```

Root-level `CONTEXT-MAP.md` and `docs/adr/` hold only what genuinely spans
contexts. Add a `CONTEXT-MAP.md` row whenever a new context grows a
`CONTEXT.md`.

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

> _Contradicts `crates/stylex-transform/docs/adr/0007-...` -- but worth
> reopening because..._
