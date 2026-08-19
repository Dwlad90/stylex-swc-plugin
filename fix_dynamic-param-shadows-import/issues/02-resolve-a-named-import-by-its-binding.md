# 02 — Resolve a named import by its binding, not by its name

Status: `ready-for-agent`
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

- [ ] The reported input compiles, matching the reference implementation's rules
- [ ] The aliased variant compiles too
- [ ] A parameter shadowing a module-level `const` still resolves as before
- [ ] Corpus entries for both shapes, plus the prop-order variant, each with the
      verdict they are known to read
- [ ] A unit test that pins binding-aware resolution at the lookup itself, not
      only through the transform
