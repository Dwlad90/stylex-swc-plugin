# 18 — Rename the `Unreachable port` domain term

**What to build:** `crates/stylex-css-parser/CONTEXT.md` defines a domain term
called **Unreachable port**: a type in that crate whose reference counterpart
the plugin never runs, so its behaviour cannot be settled by comparing output.
The definition is sound and the term is used. The name is not: it describes the
type by a relationship to another implementation rather than by what the type
is, which is what [ticket 10](./10-regenerate-fixtures-and-close-out.md) closed
out everywhere else.

Find a name that says the same thing from inside this crate — what makes the
type unsettleable is that no run of the plugin can produce evidence about it,
not that it was ported. Then update the term, its `_Avoid_` line, and every
place that uses it.

This is a domain-model change, not a find-and-replace. Read
[docs/agents/domain.md](../../../docs/agents/domain.md) first, and keep the
definition's two worked reasons intact: the plugin normalizes a colour as text
and never rebuilds it from parsed channels, and `Oklch.parser`/`Oklab.parser`
throw on every input.

**Blocked by:** nothing.

**Status:** backlog

- [ ] The term names what the type is, not where it came from.
- [ ] The definition, the `_Avoid_` line and every use site agree.
- [ ] No other artifact still reaches for the old name.
