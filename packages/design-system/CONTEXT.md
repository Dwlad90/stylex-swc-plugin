# @stylexswc/design-system

The tokens and one component the example apps share, so the examples exercise
cross-package StyleX usage: a `defineVars` import that must be resolved and
followed into another package. Consumers reach them through the
`./tokens.stylex` and `./consts.stylex` subpath exports.

## Language

**Token**:
A variable from `src/tokens.stylex.ts`, declared with `defineVars`. It compiles
to a CSS custom property.
_Avoid_: variable, theme value, constant

**Const**:
An entry in `src/consts.stylex.ts`, declared with `defineConsts` — a media query
string here. Inlined at compile time rather than emitted as a custom property.
_Avoid_: token, variable, breakpoint
