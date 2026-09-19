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

**Status:** resolved

- [x] The term names what the type is, not where it came from.
- [x] The definition, the `_Avoid_` line and every use site agree.
- [x] No other artifact still reaches for the old name.

## What landed

The term is **Unwitnessed parser**: a parser that no plugin run enters, so no
output is evidence about it and its behaviour cannot be settled by comparing
output. The name says what makes the parser unsettleable and holds inside the
crate, with no reference to where the code came from. The old name is now on the
`_Avoid_` line, so a reader who reaches for it is sent to the new one.

`parser` rather than `type` for the head noun. The crate already defines **CSS
type** for a grammar production, and the members of this set are wider than
that: a CSS type, a property parser, or a hand-written reader one of them is
built from, such as `parse_alpha_token`. The definition names all three, so no
use site is covered by inference alone.

The crate header had to move with the term. It said every CSS type and property
parser has no caller outside the crate, "which is what unwitnessed type below is
about" -- but callerlessness is the mechanism, not the rule. The header now
states the chain: no plugin run enters these parsers, so nothing the plugin
emits is evidence about one. The entry closes on the same distinction, because
some colour behaviour _does_ show in emitted text through the text path and was
settled end to end; the parsers below it stay unwitnessed.

The definition carries both worked reasons again. The second one --
`Oklch.parser`/`Oklab.parser` refuse every input -- was dropped by the glossary
shortening in `72e0ce69a`, and was confirmed against the reference source before
it went back. The cause is also corrected: the old entry blamed the lightness
reader's own optional whitespace prefix, but the sequence carries an optional
whitespace separator between its elements, and for `oklch(0.7 0.15 180)` that
separator consumes the space before the reader's prefix is ever consulted. The
sequence then demands whitespace as an element and finds none. Either step alone
gives the same refusal, so only the named cause changes.

Three use sites agree with the new name: the crate header, the doc comment on
`parse_alpha_token`, and the characterization note on the `lch(50% 100 270deg)`
case. The last one also carried a relative link one directory too shallow, so
the reader could not reach the definition; the depth is corrected. That fix and
the header rewording are both outside the rename proper, and are recorded here
rather than left to be found.

Two artifacts still spell the old name, and both are records of finished work
rather than live vocabulary: this epic's ticket 10 close-out, which says why the
rename needed its own ticket, and the plan for the media-query bounds fix, which
records the term as it stood then. Rewriting either would falsify a history, and
the `_Avoid_` line catches a reader who searches for the old name.
