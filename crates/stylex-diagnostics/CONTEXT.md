# stylex-diagnostics

How StyleX shows an author _where_ a refusal happened. A code frame quotes the
offending line back out of the file the author wrote, which means finding that
line again: what the compiler holds by then is a rewritten tree whose positions
belong to its own source map, not to the text on disk.

Everything here is best effort. Every lookup sits behind a panic boundary and
degrades to "no code frame", because a compilation must never stop on account of
the aid that explains why it stopped.

## Language

**Code frame**:
The quoted source line a diagnostic points at, with a caret under the offending
text. Built against a source map of its own -- shared by the whole process and
never cleared -- rather than against the one the module was compiled in, so a
position taken from the compiler's own parse means nothing here and has to be
found again. What is registered with that map is one entry per distinct file
_content_: registering per lookup is how a watch-mode process accumulated a full
copy of each module per save. _Avoid_: error frame, source frame, snippet

**Diagnostic state**:
What a diagnostic asks of the compiler's traversal state -- the filename, the
memoized module and its text, the key span index, and the [diagnostic
memo](#diagnostic-memo). A trait owned here and implemented by the caller, so
that building a frame never names the state manager, which would make the state
crate and the diagnostics depend on each other. Only what a frame cannot
reconstruct is asked: what the diagnostics remember is their own type, which
the state merely stores. Consulted while a diagnostic is being written, never
while a module is being evaluated, so the dispatch costs nothing measurable.
_Avoid_: state adapter, frame context, diagnostic context

**Diagnostic memo**:
What the diagnostics remember about one file: the spans they already resolved,
and the [framed declarations](#framed-declaration) their refusals recorded. A
type owned here, held as a field by the compilation state because that is what
lives as long as the file does, and read or written by nothing but the
diagnostics. Keyed by 128 bits, because the read side acts on a hit alone and a
collision would annotate one style with another style's line number. _Avoid_:
span cache, diagnostic cache, frame cache

**Memoized module**:
The module's own source, re-read and re-parsed into the [code
frame](#code-frame)'s source map, held on the [diagnostic
state](#diagnostic-state) so the whole module is read, parsed and normalized once
per file rather than once per diagnostic. Normalized on the way in -- syntax
contexts dropped, types stripped, template literals folded -- so that an
expression from the compiled tree can be matched against it by structure.
_Avoid_: seen module, cached source, parsed source

**Framed declaration**:
The binding a refusal is _about_, recorded so its frame names the line that
binding was declared on rather than the line it was read from -- the line an
author has to go and change. A **name** is recorded rather than a position,
because a span from the compiler's parse indexes the compiler's source map while
the frame's positions live in its own; the name is resolved against the
[memoized module](#memoized-module) instead. Recorded per refused expression,
because a refused dynamic style falls through to an inline style instead of
stopping the build, and a later diagnostic must not inherit an earlier refusal's
position. A name that module does not declare falls back to locating the read.
_Avoid_: deopt span, declaration span cache, reported position

**Declaration span**:
Where a name is declared in the [memoized module](#memoized-module): the whole
declarator for a `var` / `let` / `const`, the whole declaration for a hoisted
`function` or `class`, the local specifier for an import, and the binding itself
for a parameter, a `catch` binding or a name inside a destructuring pattern. A
**module-level** declaration wins over one nested inside a function or a block,
whichever comes first in the file, because the chain that raised the refusal
resolves bindings module-wide with no scope of its own. Among declarations at the
same level the first in source order wins, which is the only order a module-wide
resolution can be said to have. _Avoid_: binding position, declaration location,
symbol span

**Panic boundary**:
What every span lookup runs inside. Locating a position re-reads, re-prints and
re-parses a module purely to improve a message, and a panic anywhere in there --
a byte offset landing inside a multi-byte character is the usual one -- must
degrade to "no code frame", never abort the compilation. The process panic hook
is replaced once, so a panic raised inside a boundary is silent while every other
panic still reaches the hook that was there before. _Avoid_: catch_unwind guard,
error boundary, safety net
