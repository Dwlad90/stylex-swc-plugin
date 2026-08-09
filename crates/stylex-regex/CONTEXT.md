# stylex-regex

Every regex the compiler pipeline uses, compiled once as `Lazy<Regex>` statics.
Two crates hold their own instead:
[stylex-path-resolver](../stylex-path-resolver/CONTEXT.md) keeps one
extension-matching `Lazy<Regex>` beside the resolver that is its only caller,
and the [stylex-test-parser](../stylex-test-parser/CONTEXT.md) binary builds one
per run. A pattern used by more than one caller belongs here.

## Language

**Pattern**:
A named `Lazy<Regex>` static in `regex.rs`. Patterns are named for what they
recognise (`IS_CSS_VAR`, `NPM_NAME_REGEX`), never for where they are used, so
one pattern serves several callers.
_Avoid_: matcher, rule, expression

**Combinator selector pattern**:
The family that recognises a selector combinator inside a StyleX key —
`ANCESTOR_SELECTOR`, `DESCENDANT_SELECTOR`, `SIBLING_BEFORE_SELECTOR`,
`SIBLING_AFTER_SELECTOR`, `ANY_SIBLING_SELECTOR`. They exist to reject such
keys, not to support them.
_Avoid_: nesting regex, selector matcher
