# stylex-regex

Every regex the compiler pipeline uses, compiled once as `Lazy<Regex>` statics
over `fancy_regex`. Two crates hold their own instead:
[stylex-path-resolver](../stylex-path-resolver/CONTEXT.md), beside the resolver
that is its only caller, and the
[stylex-test-parser](../stylex-test-parser/CONTEXT.md) binary. Those are the
only two `Regex::new` sites outside this crate.

## Language

**Pattern**:
A named `Lazy<Regex>` static in `regex.rs`, named for what it recognises
(`IS_CSS_VAR`, `NPM_NAME_REGEX`) and never for where it is used, so one pattern
serves several callers. `IS_CSS_VAR` writes its class out as `[a-zA-Z0-9-_]`
rather than `[\w-]`, because `\w` is Unicode-aware here and would accept
`var(--épaisseur)`.
_Avoid_: matcher, rule, expression

**Combinator selector pattern**:
The family recognising a selector combinator inside a StyleX key —
`ANCESTOR_SELECTOR`, `DESCENDANT_SELECTOR`, `SIBLING_BEFORE_SELECTOR`,
`SIBLING_AFTER_SELECTOR`, `ANY_SIBLING_SELECTOR`. They support the relational
selectors of `when.*`, and [stylex-css](../stylex-css/CONTEXT.md) prices each
into a [priority](../stylex-constants/CONTEXT.md). Any-sibling must be tested
before the two individual ones.
_Avoid_: nesting regex, selector matcher
