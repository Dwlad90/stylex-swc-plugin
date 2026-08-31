# 04 — Extract the state-index crate

**What to build:** The **state manager** composes two lookup structures that let
it answer "which declarator, which call, which span" with one hash probe instead
of a scan of the module. They are pure Rust-only machinery with no counterpart,
and one of them has no internal dependencies at all — the true leaf of the whole
crate.

Give them their own crate. The state manager keeps its fields and composes the
types from across the boundary; its own struct and method surface are untouched,
because it corresponds to a single unit on the comparison side.

This is the tracer bullet for the entire split: it proves the extraction
pipeline end to end — create the crate, move code and its tests, satisfy the
coverage gate, place it in the documented DAG — at the smallest possible stake.

Note that the coverage gate runs across the whole workspace, so this crate is
gated the moment it exists. Note also that the coverage tool keeps only the
best-covered instantiation of a generic, so a generic index can read as fully
covered while one instantiation is untested.

**Blocked by:** 02 — Record the pre-split baseline.

**Status:** ready-for-human

- [x] Both index structures live in the new crate.
- [x] The state manager remains one struct with an unchanged method surface.
- [x] The `Rc`-sharing is preserved — cloning the state manager for a dynamic style's callback must not become a deep copy.
- [x] The unit tests covering the indices move with them.
- [x] The crate reaches zero uncovered lines and zero uncovered regions, with every generic instantiation exercised.
- [x] The crate has a `CONTEXT.md` defining its vocabulary and a row in the context map.
- [x] The crate is placed in the documented layer list.
- [x] Manifest matches the conventions of existing crates; no publish key.
- [x] No re-export facade is added to the transform — call sites use the new crate path.
- [x] Benches diffed against the baseline; no regression outside noise.
- [x] Lockfile regenerated and committed with this change.

## Comments

### The AST key readers had to move first

The ticket says the key-span index "needs only the workspace hashing helper and
AST helpers already owned by `stylex-ast`". They were not owned by `stylex-ast`
— `namespace_name_from_prop_key`, `collect_object_lit_keys` and
`prop_as_key_value` all lived in the transform's `utils::ast::helpers`, which a
layer-6 crate cannot reach. So a prefactor commit moves them down to a new
`stylex_ast::ast::keys` module, and the transform's other call sites import from
there.

`namespace_name_from_member_prop` moved with them although the index never uses
it. It shares the two private helpers `namespace_name_from_expr` and
`namespace_name_from_lit` with `namespace_name_from_prop_key`; leaving it behind
would either duplicate those two across a crate boundary or widen them to `pub`
for one caller. The group is cohesive, so it moves whole.

The move needed a glossary entry: `stylex-ast`'s `CONTEXT.md` opened with
"nothing here inspects StyleX semantics", and the readers carry the
`namespace_name_` prefix. A **Key reader** term now says which name is being
asked for and that the crate still does not read what the name means.

### The public surface is narrower than the move

Only what the transform actually reaches is `pub`. `CandidateIndex::forget`,
`CandidateRank` and its fields, and `NamespaceKeyQuery`'s `sibling_keys` and
`callee` stayed `pub(crate)` — the transform reads a query's `namespace_key`,
`namespace_value_keys` and `target_offset` for the span-cache key and nothing
else. Story 2 asks for a boundary, not for everything the module happened to
expose.

### Coverage needed four tests, and one of them for an odd reason

Three close genuine gaps the transform's suite used to cover from outside:
a property with no readable name, a namespace bound to a reference, and
`CallLookup::wrapped`.

The fourth exists only because the coverage tool counts per generic
instantiation. `CandidateIndex::candidates` takes its key as a closure, so every
call site is its own instantiation, and a site that only ever meets an empty
index leaves the bucket lookup below the short-circuit unexercised *in that
instantiation*. One test asks the same site before and after a record, so one
instantiation walks both paths. Worth remembering for the next generic crate:
merged region coverage can read 100% while llvm-cov's own tally is not zero,
and the gate reads the tally.

### The layer renumber reached further than the layer list

`stylex-state-index` at layer 6 shifts evaluation, CSS, transform and compilers
up by one. Three crate READMEs carry a `**Layer**:` line, and **seventeen** of
them carry the same mermaid dependency graph, byte-identical in every copy. All
were renumbered, the new crate added as a node, and the new crate's own README
given the same block — eighteen identical copies now.

That duplication is itself the problem: one graph maintained in eighteen places
drifts, and it had. **A maintainer should decide** whether it becomes one file
the READMEs link to.

### Two pre-existing doc defects found while renumbering

Recorded rather than folded in, except where leaving them would have made this
ticket's own edit wrong:

- **Fixed, because the count was this ticket's to correct.**
  `stylex-transform/README.md` said "all 14 other internal crates" and listed a
  dependency the manifest does not have (`stylex-logs`) while omitting two it
  does (`stylex-atoms`, `stylex-js`). The list is now derived from the manifest:
  16 crates.
- **Left alone.** The shared mermaid graph has never carried `stylex-atoms`.
  It predates this work and is not what this ticket renumbered.

### Benches

All seven targets re-run against the `pre-split` baseline. The only regressions
reported were the three `ConcatenationChain` sizes, and they are noise: an
earlier run of that same target on the same code measured −3.3% where the
recorded run measured +5.5%, a swing wider than either reading. The target folds
string concatenation and never touches an index. Re-measured on an idle machine
to confirm — see the numbers below.
