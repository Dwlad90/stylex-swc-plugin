# `stylex-diagnostics`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

Code frames for StyleX compiler errors. A code frame quotes the offending line
from the file the author wrote and puts a caret under it. By the time an error
is raised, the compiler holds a rewritten tree whose positions belong to its own
source map, so this crate finds the authored line again.

- **Code frame** — `CodeFrame` builds the quoted line and the caret against a
  process-wide source map. It registers one entry per distinct file _content_,
  so a watch-mode process does not accumulate a copy of each module per save.
- **Expression lookup** — `get_span_from_source_code` matches a compiled
  expression structurally against the re-parsed source of the module.
- **Namespace key lookup** — `get_key_span_from_source_code` finds a style
  namespace by its _key_, which survives value rewrites made by an earlier
  loader.
- **Declaration lookup** — an error about a binding is framed at the declaration
  of that binding.

## Architecture

`stylex-ast` reads expressions back, `stylex-macros` raises the error,
`stylex-regex` builds the links a message carries, `stylex-state-index` supplies
the key span index, and `stylex-utils` supplies the stable hash the memo is
keyed by. `stylex-state` implements `DiagnosticState`; the transform and the
evaluator reach a code frame through it.

Every lookup is best effort. Each one sits behind a panic boundary and degrades
to "no code frame", so a compilation never stops because the aid that explains a
refusal failed. The process panic hook is replaced once: a panic inside a
boundary is silent, and every other panic still reaches the previous hook.

The caller implements `DiagnosticState`, the same injection `stylex-atoms` uses.
This keeps `stylex-state` and this crate free of a dependency cycle. The trait
is read while a diagnostic is written, never while a module is evaluated.
`DiagnosticMemo` holds the resolved spans and the refused bindings; the state
manager stores it as a field and never reads it.

A refused binding is recorded by **name**, not by position, because frame
positions belong to a different source map than compiler spans. The name is
resolved against the re-parsed module. A name that module does not declare falls
back to the position of the read.

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
