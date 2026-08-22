# A namespace import of a theme file resolves nothing

**Status:** accepted

```js
import * as tokens from 'colors.stylex.js';
export const styles = stylex.create({ wrapper: { color: tokens.primary } });
```

This used to compile, emitting `color:var(--x…)`. It is refused now, with
`Referenced constant is not defined.` — the sentence
`@stylexjs/babel-plugin` 0.19.0 gives the same input.

Refusing input that compiled before needs an argument, and this one is not the
"mirror upstream" argument. It is that what the input compiled _to_ was wrong.

## What upstream does, read out of the source rather than guessed

`evaluate-path.js` 0.19.0 guards its import step at lines 600-605 with

```js
!bindingPath.isImportDefaultSpecifier() &&
  !bindingPath.isImportNamespaceSpecifier() &&
  bindingPath.isImportSpecifier();
```

and the two exclusions are not the same kind of exclusion.

A **default** specifier gets a refusal of its own at 652-654 — a deliberate
verdict on the import kind, which
[ADR 0003](./0003-one-ordered-chain-resolves-a-reference.md)'s step 2 mirrors
with `IMPORT_FILE_EVAL_ERROR`.

A **namespace** specifier gets none. It is excluded because the step's body reads
`importSpecifierNode.imported` (line 609), a field an `ImportNamespaceSpecifier`
does not carry — the exclusion is a guard on the step's _input_, not a verdict on
the kind. The reference then falls through `constantViolations`, `isMutated`, the
position comparison and `hasValue`, reaches `path.resolve()` at 685 and deopts at
687 with `UNDEFINED_CONST`. `resolve()` cannot cross a module boundary:
`@babel/traverse`'s `_resolve` returns early on `binding.kind === "module"`, so
`resolved === path` and the terminal refusal is the only answer left.

## What this compiler used to do, and what it produced

The chain's namespace arm synthesized the reference's own **local alias** as the
export name. Measured on both compilers with `vars.stylex.js` exporting
`stylex.defineVars({ color: 'red' })` as `vars`, where both agree the file
defines one variable:

| module                                     | before                          | Babel 0.19.0 |
| ------------------------------------------ | ------------------------------- | ------------ |
| `import { vars }` → `vars.color`           | resolves ✔                      | same         |
| `import * as tokens` → `tokens.color`      | a _different_ variable          | refuses      |
| `import * as tokens` → `tokens.vars.color` | a third variable                | refuses      |
| `import * as vars` → `vars.color`          | resolves ✔                      | refuses      |
| both imports in one module                 | **two variables for one token** | refuses      |

Row 4 is the only one where the old behaviour matched a named import, and it
matched because the alias happened to be spelled like the exported group. That is
a coincidence, not a resolution. Row 3 is the spelling an author reaching for a
namespace import would actually write, and it was already emitting a variable
nothing defines. Row 5 is the argument in one module: the same token, named
twice, emitting two custom properties, one of which nothing defines.

A `var()` nothing defines renders as nothing and reports as nothing. So the arm
was not protecting a capability — the variable is reachable through the named
import both compilers resolve — it was protecting a silent wrong render. It gives
up the resolution and takes the fall-through.

## What landed

`binding.rs` step 1 answers all three specifier kinds: a named one resolves, a
default one refuses with `IMPORT_FILE_EVAL_ERROR`, and a namespace one resolves
nothing and falls through to the chain's terminal `UNDEFINED_CONST`. No new
constant and no new step — the arm is an _absence_, which is what upstream's is.

**One namespace shape gets a different sentence, and it is not an exception.**
The globals step (670-683) sits ahead of the initializer read, so
`import * as NaN from 'colors.stylex.js'` meets it on the way down: a binding
exists for the name, and the step refuses with `UNINITIALIZED_CONST` —
`Referenced constant is not initialized.` Both compilers answer that, for the
same reason in the same order, and
`a_namespace_alias_spelled_like_a_global_is_not_initialized` pins it beside the
cases that get the terminal sentence.

## Consequences

**This is a breaking change for modules that compiled before.** They compiled to
CSS referencing an undefined custom property, so the styles they were meant to
carry were already absent at runtime; the change turns a silent nothing into a
build error naming the reference. The fix for such a module is a named import,
which both compilers resolve, and which is what row 1 shows working.

**Sixteen consumer positions were measured, not just the bare read.** A computed
key, a template literal, a fallback array, a spread, five conditions deep, a
custom property, a vendor-prefixed property, an expanding shorthand, an unknown
property, `firstThatWorks`, `createTheme`, `keyframes`, `defineConsts`,
`viewTransitionClass`, `positionTry`, and a non-ASCII and a unicode-escaped
alias. All sixteen agree with 0.19.0 on acceptance, and all but one on the
sentence: a read through optional chaining, which upstream refuses as
`Unsupported expression: OptionalMemberExpression` before reaching the identifier
at all. That gap predates this decision and is not this seam's. Twelve of the
sixteen are asserted in `validation_stylex_create_test::invalid_values`.

**Two refusals that were already happening for the wrong reason were corrected**
on the way, and now read upstream's sentence for upstream's reason: a namespace
import of a **non-theme** file, which read `Could not resolve the path to the
imported file.`; and a namespace group read where a value belongs, which read
`A style value can only contain an array, string or number.`
