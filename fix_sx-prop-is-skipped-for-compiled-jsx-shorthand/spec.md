# sx prop is skipped for compiled JSX shorthand and for modules with no StyleX import

Status: ready-for-agent
GitHub issue: https://github.com/Dwlad90/stylex-swc-plugin/issues/1307

## Problem Statement

A developer sets `sxPropName: 'sx'` and writes `<div sx={sx} />`. In two
situations the element keeps a native `sx` attribute and the forwarded styles
are never applied. Nothing is logged; the page is simply unstyled.

1. **The module reaches the compiler already JSX-compiled.** An earlier esbuild
   or SWC pass turns `<div sx={sx} />` into `jsx("div", { sx })`. The compiler
   recognises the same call written as `jsx("div", { sx: sx })` and rewrites it,
   but the object shorthand form is left alone.
2. **The module reaches a bundler plugin with no StyleX import.** A component
   that only forwards an `sx` prop has nothing to import. Every bundler plugin
   scans the source text for an import source before it calls the compiler, so
   such a module is dropped before the compiler sees it. Calling the compiler
   directly on the same module works, which is what makes the failure confusing.

The two are independent: adding a side-effect import does not fix the shorthand,
and wrapping the value in an array does not get the module past the plugin
filter.

## Solution

The `sx` prop is transformed wherever its key can be known at compile time, and
a module that uses the `sx` prop is never dropped before the compiler sees it.

After the fix, `jsx("div", { sx })` produces the same output as
`jsx("div", { sx: sx })`, and a bundler plugin hands a module that mentions the
configured prop name to the compiler even when the module imports nothing.

## User Stories

1. As a component author, I want `<div sx={sx} />` to keep working after my
   build pipeline compiles JSX, so that my styles do not silently disappear when
   I change bundler or target.
2. As a component author, I want the object shorthand `{ sx }` and the explicit
   property `{ sx: sx }` to behave identically, so that a minifier's choice of
   output form is not something I have to reason about.
3. As a component author, I want a computed but statically known key
   `{ ["sx"]: value }` to transform, so that output from a toolchain that emits
   computed keys is handled like any other.
4. As a component author, I want a leaf component that only forwards an `sx`
   prop to be styled, so that I do not have to add an unused import to make the
   bundler notice it.
5. As a TypeScript author, I want a type-only StyleX import that an earlier pass
   erased not to change whether my module is processed, so that the transform
   does not depend on what survives type stripping.
6. As a Vite user, I want the plugin and the compiler to agree about which
   modules need processing, so that debugging with the compiler directly
   reflects what my build does.
7. As a webpack, Rspack, Next.js, Rollup, Turbopack or PostCSS user, I want the
   same behaviour as the Vite user, so that the fix does not depend on which
   bundler I picked.
8. As a developer who renamed the prop with `sxPropName: 'css'`, I want both
   fixes to follow the configured name, so that the feature is not quietly
   hardcoded to `sx`.
9. As a developer who set `sxPropName: false`, I want module selection to stay
   exactly as it is today, so that turning the feature off costs nothing.
10. As a developer with a large project, I want modules that use neither StyleX
    nor the `sx` prop to keep being skipped without being compiled, so that the
    fix does not slow my build down.
11. As a maintainer, I want one implementation of the module-selection
    predicate rather than one per plugin, so that a future fix lands everywhere
    at once.
12. As a maintainer, I want the boundary of what the compiled-call path handles
    to be written down, so that the forms it deliberately does not handle do not
    read as oversights.

## Implementation Decisions

### Compiled-call prop matching

- The props object of a recognised compiled JSX call is scanned for the
  configured prop name. Today only a key-value property with an identifier or
  string key is matched. The scan gains two further forms, and the value it
  extracts follows the same rules for each:
  - **Object shorthand** — the property is the prop name and its value is the
    identifier itself.
  - **Computed key with a statically known string** — a string literal, or a
    template literal with no expressions.
- Two forms are deliberately not handled, because the raw JSX attribute path
  does not handle the constructs they correspond to:
  - **Getter and method properties** carry no value expression to forward, in
    the same way that a JSX attribute whose value is not an expression container
    is left alone.
  - **Spread properties** are not inspected, in the same way that a JSX spread
    attribute is not inspected. The key is not knowable at compile time.
- **First match wins.** Where a props object names the prop twice, the first
  occurrence is transformed, matching the raw JSX attribute path, which stops at
  the first matching attribute. This is not the last-wins semantics a runtime
  would apply; it is recorded here rather than changed, because both paths agree
  today and no real toolchain emits a duplicated key.
- The lowercase host-element guard, the runtime binding resolution, the replacement
  with a spread of the props call, and the `Discover` cycle gating are all
  unchanged. Only the set of matched property forms grows.
- Where the crate already has a helper that reads a statically known key from a
  property name, that helper is reused rather than a third key matcher written.

### Module selection in the bundler plugins

- The source-text scan that decides whether a module is handed to the compiler
  currently looks only for an import source substring. It gains a second
  condition: when the prop name is configured, a module that mentions the prop
  name in a prop-like position is also handled.
- The prop-name pattern is built from the **resolved** option value and is
  regex-escaped, so a renamed prop is honoured and a name containing regex
  metacharacters cannot corrupt the pattern.
- When the prop name option is `false`, the second condition is skipped
  entirely and module selection is exactly what it is today.
- The pattern matches the name followed by `=`, `:`, `,` or `}`, with optional
  whitespace, which covers `sx={…}`, `sx: …`, `{ sx }` and `{ sx, … }`.
- This is a text scan, not a parse. A mention inside a string or a comment
  matches. That trade is deliberate: a false positive costs one compile that
  produces unchanged output, a false negative costs a silently unstyled
  element.
- The import-source half of the predicate is **not** changed. Tightening it
  would drop modules that are processed today.
- The predicate is **extracted into `@stylexswc/plugin-shared`** and exported
  from a new leaf subpath, following that package's existing convention of leaf
  subpath exports so that a consumer does not pull in the plugin core. All five
  copies — unplugin, rollup-plugin, the shared loader used by webpack, Rspack
  and Next.js, turbopack-plugin and postcss-plugin — call it.
- `rollup-plugin` gains a dependency on `@stylexswc/plugin-shared`. The catalog
  entry already exists, so no workspace change is needed.

### Commits

Three conventional commits, each passing the full gate on its own:

1. the compiled-call prop matching, in the transform crate;
2. extraction of the module-selection predicate into the shared package, with no
   behaviour change and five call sites converging on it;
3. the prop-name condition added to that predicate.

Splitting 2 from 3 keeps the behaviour change readable as a few lines rather
than hidden inside a five-file move.

## Testing Decisions

A good test here asserts the transformed output for a given input, not the
shape of the visitor or the predicate's internals. The compiled-call cases are
behavioural by construction: input source in, output source compared.

Three seams, two of them existing:

1. **The transform crate's snapshot tests.** New cases sit beside the existing
   compiled-call cases in the `sx` attribute test module, which already cover
   the explicit key form, the array value, the component-not-host case, the Vue
   element-block form and the Vite runtime call. New cases: object shorthand;
   computed key with a string literal; a getter property left unchanged; a
   spread property left unchanged; and a renamed prop in shorthand form.
2. **The unplugin `transform` hook.** The existing test that asserts a module
   with no StyleX import is skipped is extended rather than replaced: a module
   using the prop with no import is now transformed; a module using neither is
   still skipped; and with the prop name disabled a module using the prop is
   still skipped.
3. **A new unit test on the extracted predicate**, in the shared package.
   Table-driven over the cases that are awkward to reach through a plugin: a
   renamed prop, a name needing regex escaping, the disabled option, and each
   of the four prop-like positions. This is the one new seam, taken because
   pinning the pattern through five plugins would test the import statement
   rather than the behaviour.

4. **A parity harness against the reference implementation**, run from the
   tracker directory and never committed. For every new and existing `sx` case
   it takes the **raw JSX** source, runs it through the reference plugin with
   the same `sxPropName`, and compares the result with this compiler's output
   for that same raw JSX. Those must be identical.

   For the compiled-call cases the comparison is indirect, and deliberately so:
   the reference plugin has no compiled-call path, so it returns the compiled
   input unchanged. Asserting output equality there would assert the defect.
   Instead each compiled case is paired with the raw JSX it was produced from,
   and the check is that this compiler's compiled-call output carries the same
   props call, the same resolved binding and the same injected import as the
   reference output for that raw JSX — the compiled call shape being the only
   difference.

   The harness resolves the reference plugin and Babel through a package that
   has them, since the tracker directory has no `node_modules` of its own. It
   stays in the tracker directory so that no reference to another
   implementation enters the repository.

The other four plugin packages get no new tests. They call the same function,
and a test per call site would assert that the import resolves.

No end-to-end test is added. Reproducing the reporter's flow would mean
building the native addon and driving a bundler plugin to assert what the three
seams above already pin, and the JavaScript suites only carry meaning after a
rebuild.

## Out of Scope

- **Raw JSX attribute gaps.** A namespaced attribute name and a string-literal
  attribute value are both left untouched today and stay that way.
- **Numeric property keys.** Reachable only if the prop name is configured to a
  numeric string; no change.
- **Spread properties and spread attributes.** Resolving what a spread carries
  is a new capability, not a gap being closed.
- **The `.as` versus `.from` asymmetry** in the import-source scan: unplugin and
  rollup-plugin inspect only `from` on an object import source, while the
  loaders inspect both. A real inconsistency, a different defect, not this one.
- **Tightening the import-source scan** from a substring to a structural match.
- **Duplicate-key precedence.** Recorded above, not changed.

## Further Notes

The reported failure is reproduced at HEAD in this worktree before any change is
made, not against published packages. The compiled input is **generated**, not
hand-written: the tool the reporter used to produce it is already a development
dependency of the unplugin package, so the exact reported input can be
reproduced rather than approximated.

The compiler itself does not drop a module that uses only the `sx` prop: the
`sx` transform runs during `Discover` and registers an import path when it
injects the runtime binding, which is what carries the module past the
module-level bail. The second defect is entirely in the bundler plugins.
