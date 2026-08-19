# 02 — Resolve a named import by its binding, not by its name

Status: `resolved`
Blocked by: 01

**What to build:** A dynamic style whose parameter shadows an imported binding
compiles, instead of failing the build with `Style value must evaluate to a
static expression.`

Closes GitHub issue
[#1266](https://github.com/Dwlad90/stylex-swc-plugin/issues/1266).

The evaluator looks up an identifier's import by symbol name for a *named*
specifier, while the default and namespace specifiers beside it both compare the
binding — symbol *and* syntax context. The resolver runs before the transform,
so a function parameter carries a context distinct from the module-level import
it shadows, and the ctxt-aware comparison is what already makes a parameter
shadowing a module-level `const` resolve correctly. The named arm is the
outlier.

Consequence today: the parameter resolves to the import, evaluation answers a
*confident* theme reference, and the style-value consumer aborts because a theme
reference has no expression form. The reference implementation never faces the
choice — it resolves through the scope's binding.

Make the named arm compare the binding as its two siblings do. The reported
input and the aliased variant (`{ zIndex as zi }` shadowed by a parameter `zi`,
which fails identically today) both compile to the three rules the reference
implementation produces.

Do not touch the identifier *function map*. It is keyed by name and consulted
first, and the reference implementation does the same — parameters are injected
into that same name-keyed map. Keying it by binding would be a divergence, not
a fix.

- [x] The reported input compiles, matching the reference implementation's rules
- [x] The aliased variant compiles too
- [x] A parameter shadowing a module-level `const` still resolves as before
- [x] Corpus entries for both shapes, plus the prop-order variant, each with the
      verdict they are known to read
- [x] A unit test that pins binding-aware resolution at the lookup itself, not
      only through the transform

## Answer

The named arm compares `named_import.local.eq_ignore_span(ident)`, as its two
siblings do — one line in `crates/stylex-transform/src/shared/utils/common.rs`.
The dead imported-name half beside it is left for ticket 07, and
`functions.identifiers` is untouched.

Measured against `@stylexjs/babel-plugin` 0.19.0, comparing `metadata.stylex`
rule for rule: the reported input, the aliased variant, the prop-order variant,
the nested-condition tree, two parameters shadowing two imports, a non-ASCII
parameter name, a unicode-escaped one, the `const` guard and the unshadowed
control all read `identical`. Eleven corpus entries in
`parity/corpus/modules.json` record that verdict; `parity --set modules` reports
49 subjects, 0 changed.

Nine unit tests sit on `get_import_from` itself, fixing two references with one
symbol at two syntax contexts — including the inverse, so a fix that answered
`None` for everything would fail.

### Beyond the checkboxes, deliberately

Eleven `stylex_test!` snapshots rather than the one the spec's commit 6 lists,
and eight corpus entries past the three this ticket asks for. They are guards on
the shapes the audit touched, each measured against upstream before being
recorded. Two of them — `dynamic_param_shadows_a_namespace_theme_import` and
`dynamic_param_shadows_a_default_theme_import` — pin behaviour that **diverges**
from upstream and says so at the site; ticket 06 will rewrite the default one.

### Three measurements this work surfaced

Each holds with no shadowing at all, so none of them is caused by this fix; each
is filed where it belongs.

| input | Babel 0.19.0 | rs-compiler |
| --- | --- | --- |
| `import tokens from 'tokens.stylex.js'`, read in a style value | imported-file evaluation error | emits the theme ref |
| `import * as tokens from 'tokens.stylex.js'`, read in a style value | `Referenced constant is not defined.` | emits the theme ref |
| `import { "color-lg" as colorLg }`, read in a style value | `A style value can only contain an array, string or number.` | emits nothing |

The first closes ticket 06's measurement. The second and third are new — see
tickets 11 and 12.
