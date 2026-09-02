# `stylex-diagnostics`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

How StyleX shows an author _where_ a refusal happened. A code frame quotes the
offending line back out of the file the author wrote, which means finding that
line again: what the compiler holds by then is a rewritten tree whose positions
belong to its own source map, not to the text on disk.

- **Code frame** — `CodeFrame`, the quoted line with a caret under the offending
  text, built against a process-wide source map of its own. One entry is
  registered per distinct file _content_, so a watch-mode process does not
  accumulate a copy of each module per save.
- **Expression lookup** — `get_span_from_source_code` finds the position of a
  compiled expression by matching it, structurally, against the module's own
  re-parsed source.
- **Namespace key lookup** — `get_key_span_from_source_code` finds a style
  namespace by its _key_ instead, which survives value-level rewrites an earlier
  loader may have made.
- **Declaration lookup** — a refusal about a binding is framed at that binding's
  declaration, which is the line the author has to go and change.

## Architecture

Of what this crate reads, `stylex-ast` reads expressions back, `stylex-macros`
raises the error a refusal panics with, `stylex-regex` builds the links a
message carries, `stylex-state-index` supplies the key span index, and
`stylex-utils` supplies the stable hash the diagnostic memo is keyed by.
`stylex-state` implements `DiagnosticState` on its state manager, and the
transform and the evaluator reach a code frame through it.

Everything here is best effort. Every lookup sits behind a panic boundary and
degrades to "no code frame", because a compilation must never stop on account of
the aid that explains why it stopped. The process panic hook is replaced once, so
a panic raised inside a boundary is silent while every other panic still reaches
the hook that was there before.

What a diagnostic needs from the compiler's traversal state is declared here as
the `DiagnosticState` trait and implemented by the caller — the same injection
`stylex-atoms` uses — so that building a frame never names the state manager,
which would make the state crate and the diagnostics depend on each other. The
trait is consulted while a diagnostic is being written, never while a module is
being evaluated.

Only what a frame cannot reconstruct is asked through it. What a diagnostic
remembers — the spans it already resolved and the bindings its refusals are
about — is a `DiagnosticMemo`, a type of this crate that the state holds as a
field and never reads itself.

A refused binding is recorded by **name**, not by position: a span from the
compiler's parse indexes the compiler's source map, while the frame's positions
live in the one it built for the file. The name is resolved against the module
the frame re-parsed, and a name that module does not declare falls back to
locating the read.

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
