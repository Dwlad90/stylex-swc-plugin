# @stylexswc/design-system

The tokens and one component the example apps share. It exists so the examples
exercise cross-package StyleX usage — a `defineVars` import that must be
resolved and followed into another package — not to be a real design system.

## Language

**Token**:
A variable from `tokens.stylex.ts`, declared with `defineVars`. Compiles to a
CSS custom property, and resolving one across a package boundary is the case the
examples exist to cover.
_Avoid_: variable, theme value, constant

**Const**:
An entry in `consts.stylex.ts`, declared with `defineConsts` — a media query
string here. Inlined at compile time rather than emitted as a custom property,
which is the whole difference from a token.
_Avoid_: token, variable, breakpoint
