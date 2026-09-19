# 01 — Share the module selection predicate across the bundler plugins

**What to build:** Nothing changes for a user of any bundler plugin. Every
plugin decides which modules to hand to the compiler exactly as it does today,
but it asks one shared function instead of carrying its own copy of the answer.

Five packages each hold their own copy of the same source-text scan: the Vite
and Rollup plugins, the shared loader behind webpack, Rspack and Next.js, the
Turbopack loader, and the PostCSS plugin. The copies have already drifted —
two of them inspect only one field of an object-shaped import source while the
others inspect both. This ticket makes them one implementation so the next
change to module selection lands everywhere at once.

The function goes in the shared plugin package, exported from a new leaf
subpath so that a consumer gets the predicate without pulling in the plugin
core. The one plugin that does not yet depend on that package gains the
dependency; the version catalog entry already exists, so no workspace change is
needed.

This is a prefactor. It carries no behaviour change of its own — it exists to
make ticket 04 a few lines rather than a five-file edit.

**Blocked by:** None — can start immediately.

**Status:** done

- [x] One implementation of the predicate lives in the shared package and is
      reachable from a leaf subpath export
- [x] All five plugin packages call it; no package retains a private copy
- [~] The drift between copies is resolved by the shared version, and the
      resolution is toward the more permissive behaviour, so no module that is
      processed today stops being processed. Two blank-value cases are
      narrowed instead — see Comments. Both are misconfigurations, and neither
      needs a later ticket
- [x] The holdout package's dependency on the shared package is declared
- [x] Every existing plugin test passes unchanged — no test is edited in this
      ticket. One new test file is added for the shared predicate; ticket 04
      extends it with the prop-name cases
- [x] Type checking, linting, formatting and the JavaScript suites pass

## Comments

The shared predicate is the union of the five copies, with two deliberate
exceptions where the union would have been absurd. Both are recorded here
because the acceptance criterion above asks for the more permissive answer.

1. **An import source with no local name.** The webpack and Turbopack loaders
   read `as` without a guard, so a missing `as` made them search the module for
   the text `undefined`. Any module that uses the word — common in compiled
   JavaScript — was compiled although the import source never matched. The
   shared version ignores a missing name. The performance review measured the
   removed full-source scan as the one real speed-up in the change.

2. **A blank import source.** `importSources: ['']` made `includes('')` true in
   four of the five copies, so every module in the project was compiled. The
   PostCSS copy already refused a blank entry. The shared version follows
   PostCSS.

Both are narrowings, so each is a decision rather than an oversight. Neither
can drop a module that a correct configuration selects: a blank entry and a
missing local name carry no specifier to match.

One behaviour widens beyond the named drift. The PostCSS copy read the
specifier out of an import source that arrived as its own JSON text, for hosts
that pass the options through a serialising layer. The other four searched the
module for the JSON text itself, which never matches. The shared version keeps
the PostCSS reading everywhere. It only ever adds modules, never removes them.
