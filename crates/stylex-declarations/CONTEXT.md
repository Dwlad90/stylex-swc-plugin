# stylex-declarations

What a name resolves to, against the declarations the
[state](../stylex-state/CONTEXT.md) recorded. The state below records _that_ a
file declares something. This crate answers the next two questions: which
declaration binds a given identifier, and what that declaration says. Both the
visitor and the evaluator above ask them. That is why this is a layer of its
own, and not part of either.

Nothing here folds an expression. A conversion that must evaluate one -- a
binary expression read as a number, a template read through its substitutions --
stays above this crate. That split keeps this half out of the evaluation cycle.

## Language

**Declaration lookup**:
Which declaration binds a name, asked of the indices the
[state manager](../stylex-state/CONTEXT.md#state-manager) fills while it walks
the file. Four readers over one idea: the declarator a name is bound by, the
import declaration and [import specifier
kind](../stylex-state/CONTEXT.md#import-specifier-kind) that bound it, and the
two parts of a declarator -- its span and its initializer -- that a caller
reading it actually needs. A lookup only _matches_; what the matched declaration
means is the caller's question. It answers the first steps of the
[reference resolution chain](../stylex-evaluator/CONTEXT.md#reference-resolution-chain)
without being that chain, which also probes writes and positions this crate
knows nothing about.

The declarator lookup falls through to the injected function map where the state
recorded nothing, and synthesizes a declarator from a mapper entry. That is one
reader and not two, because a caller asking "what binds this name" cannot say in
advance which of the two recorded it.
_Avoid_: binding resolver, identifier lookup, declaration finder, symbol table

**Spelled value**:
What an expression says when read literally, with no fold: the string a literal
or a chain of identifiers spells, the expression a declaration was initialized
with, a template with each substituted identifier replaced by its initializer.
Reading stops at the first thing that is neither a literal nor another
identifier, and answers _nothing_ there rather than refusing -- what a
non-literal means belongs to the caller, and a step of an animation that
declares nothing and a namespace name that is a hard error cannot both be
decided here.

A literal read as an authored style value keeps its JavaScript type: a numeric
literal stays a number, and everything else with a string form becomes one,
because that distinction is what decides whether a unit suffix is appended
later. So `42` and `"42"` are two answers, not one.
_Avoid_: literal value, static value, constant folding, resolved value
