# 11 — Decide what a namespace import of a theme file means

Status: `resolved`
Blocked by: 04 — it changes the same chain step, and landing both at once is how
the two refusals stay distinguishable.

**What to build:** A verdict, and then either a refusal or a recorded reason not
to refuse.

```js
import * as tokens from 'tokens.stylex.js';
export const styles = stylex.create({ wrapper: { color: tokens.color } });
```

The reference implementation refuses this with `Referenced constant is not
defined.` We accept it and emit `.x…{color:var(--xe7srj8)}` — the same rule a
named import of the same variable produces.

Measured while implementing 02, with no shadowing anywhere in the module, so this
is about the import kind and nothing else. Found because 02's audit put a
namespace import through both compilers for the first time; the spec's audit
table does not have this row.

Unlike the default-import case in 06, it is not obvious which side is right. The
reference implementation reads a namespace object as a value and finds no
constant behind it; we resolve the member through the theme file, which is what
an author writing `tokens.color` means. Refusing would break modules that compile
today. So the deliverable is the decision, argued, before any code.

- [x] The two behaviours are stated with the exact message text, and the reason
      the reference implementation refuses is read out of `evaluate-path.js`
      rather than guessed
- [x] A decision: mirror the refusal, or keep accepting and record why at the
      chain step, the way 06 asks for
- [x] Corpus entry with the verdict it reads, either way
- [ ] ~~If we keep accepting~~ — we do not; this branch is closed

## Answer

**Mirror the refusal.** The ticket opened expecting this to be the close call of
the two import kinds, with "refusing would break modules that compile today" on
one side. What decided it was measuring what those modules compile *to*, and the
answer removed the trade: they compile to a custom property nothing defines.

### The reference implementation's reason, read out of the source

`evaluate-path.js` 0.19.0 line 600-605 guards step 1 with

```js
!bindingPath.isImportDefaultSpecifier() &&
!bindingPath.isImportNamespaceSpecifier() &&
bindingPath.isImportSpecifier()
```

and the two exclusions are not the same kind of exclusion. A default specifier
gets a refusal of its own at 652-654 — a deliberate verdict on the import kind,
the one 06 mirrored. A namespace specifier gets none. It is excluded because the
step's body reads `importSpecifierNode.imported` (line 609), a field an
`ImportNamespaceSpecifier` does not carry; the exclusion is a guard on the step's
input. The reference then falls through `constantViolations`, `isMutated`, the
position comparison and `hasValue`, reaches `path.resolve()` at 685, and deopts
at 687 with `UNDEFINED_CONST` — `Referenced constant is not defined.`, which is
the message the ticket reports. `resolve()` cannot cross the module boundary:
`@babel/traverse`'s `_resolve` returns early on `binding.kind === "module"`, so
`resolved === path` and the terminal refusal is the only answer left.

One name gets a different one. The globals step at 670-683 sits ahead of
`resolve()`, so `import * as NaN from 'tokens.stylex.js'` never reaches the
tail — a binding exists for the name, and it refuses with `UNINITIALIZED_CONST`
instead. Both are refusals and the chain here answers the same way, from
`declares_binding`; it is the one namespace shape whose *sentence* is not the
terminal one.

### What we did instead, and what it produced

The chain's namespace arm synthesized the reference's own **local alias** as the
export name. Measured on both compilers under the parity harness's configuration,
with `vars.stylex.js` exporting `stylex.defineVars({ color: 'red' })` as `vars` —
both compilers agree the file defines `--x19b9nh9`:

| module | rs-compiler | Babel 0.19.0 |
| --- | --- | --- |
| `import { vars }` → `vars.color` | `var(--x19b9nh9)` ✔ | same |
| `import * as tokens` → `tokens.color` | `var(--x1fz91o1)` | refuses |
| `import * as tokens` → `tokens.vars.color` | `var(--x18rg9xe)` | refuses |
| `import * as vars` → `vars.color` | `var(--x19b9nh9)` ✔ | refuses |
| both imports in one module | **two different variables for one token** | refuses |

So the ticket's premise — "the same rule a named import of the same variable
produces" — holds only in row 4, where the alias happens to be spelled like the
exported group. That is a coincidence, not a resolution. Row 3 is the spelling an
author reaching for a namespace import would actually write, and it was already
broken. Row 5 is the whole argument in one module: the same token, named twice,
emitting two custom properties, one of which nothing defines.

A `var()` nothing defines renders as nothing and reports as nothing. What the arm
was protecting was a silent wrong render, not a capability — the variable is
reachable through the named import both compilers resolve. So the arm gives up
the resolution and takes the fall-through.

### What landed

`binding.rs` step 1 now answers all three specifier kinds: a named one resolves,
a default one refuses with `IMPORT_FILE_EVAL_ERROR`, a namespace one resolves
nothing and falls through to the chain's terminal `UNDEFINED_CONST`. No new
constant and no new step — the arm is an absence, which is what upstream's is.

Sixteen consumer positions were put through both compilers to check that the
refusal really belongs to the specifier and not to whatever sits around it: a
computed key, a template literal, a fallback array, a spread, five conditions
deep, a custom property, a vendor-prefixed property, an expanding shorthand, an
unknown property, `firstThatWorks`, `createTheme`, `keyframes`, `defineConsts`,
`viewTransitionClass`, `positionTry`, and a non-ASCII and a unicode-escaped
alias. All sixteen agree on acceptance with `@stylexjs/babel-plugin` 0.19.0, and
all but one agree on the sentence too — a read through optional chaining, which
upstream refuses as `Unsupported expression: OptionalMemberExpression` before it
reaches the identifier at all. That gap predates this change and is not this
seam's. Twelve of the sixteen are asserted in
`validation_stylex_create_test::invalid_values`.

Two refusals given for the wrong reason were replaced along the way, both now
reading upstream's sentence for upstream's reason:

- a namespace import of a **non-theme** file read as
  `Could not resolve the path to the imported file.` — the same wrong reason the
  default-import case read before 06;
- a namespace group read where a value belongs read as
  `A style value can only contain an array, string or number.` This is what
  `modules-1266-a-theme-namespace-import-read-as-a-style-value` was waiting on,
  and its note no longer defers to this ticket.

Corpus entries, all reading their recorded verdict against
`@stylexjs/babel-plugin` 0.19.0, harness reporting `changed 0`:

| entry | verdict |
| --- | --- |
| `modules-1266-a-namespace-theme-import` | `both reject` |
| `modules-1266-a-namespace-theme-import-read-through-its-group` | `both reject` |
| `modules-1266-a-namespace-theme-import-beside-a-named-one` | `both reject` |
| `modules-1266-a-namespace-theme-import-aliased-to-the-export-name` | `both reject` |
| `modules-1266-a-namespace-import-of-a-non-theme-file` | `both reject` |
| `modules-1266-a-namespace-theme-import-shadowed-by-a-dynamic-param` | `identical` |

Tests: eight unit cases beside the chain covering the arm and its ordering, four
refusals in `validation_stylex_create_test::invalid_values`, one rewritten in
`theme_reference_style_values`. `dynamic_param_shadows_a_namespace_theme_import`
no longer reads the import outside the dynamic style, so — exactly as 06 did for
the default arm — it guards the `Namespace` arm harder than the accepting
snapshot did: a regression to a name match now refuses instead of emitting
different CSS. Its comment no longer says "not a parity claim".

ADR 0003 records the decision and carries the rejected option (keep accepting)
with the measurement that rejected it. `CONTEXT.md`'s **import specifier kind**
entry now gives three kinds three answers instead of two.

The one thing worth flagging for a release note: this is a breaking change for
any module reading a theme through a namespace import. Every such module was
already emitting at least one variable nothing defines unless its alias was
spelled like the exported group; the fix in each case is the named import.
