# stylex-evaluator

The JavaScript evaluator: what an authored expression folds to, or why it
cannot. A refusal is a normal answer here rather than a failure — a value that
cannot be known at compile time becomes an inline style — so nothing on this
path may abort the process.

The reference implementation answers all of this in one file,
`utils/evaluate-path.js`. Three crates answer it here, so a line-for-line
reading of this crate alone concludes that behaviour is missing when it is one
layer down. This crate holds the dispatcher, the node handlers, the
[engine fold](#engine-fold) and the [chain](#reference-resolution-chain).
[stylex-state](../stylex-state/CONTEXT.md) holds the evaluated value, the
per-file state and which declaration binds a name.
[stylex-diagnostics](../stylex-diagnostics/CONTEXT.md) is where a refusal is
_reported_ — the reference deopts on `binding.path`, so the position a refusal
names is the declaration's and not the read's.

Decisions and the measurements behind them are in [docs/adr](./docs/adr).

## Language

**Declared binding**:
A name this module binds, held under its full SWC `Id` and answered by
`declares_binding`. The one question the chain asks of the reference
implementation's scope chain, and the whole of what it needs, because the
resolver has already given every binding a `SyntaxContext`. Asked of a reference
and never of a name: the global `NaN` beside a `function f(NaN)` matches
nothing. Recorded for every binding form JavaScript spells, a function parameter
included. Held for the module being compiled, so a name bound inside an
_imported_ file falls to the global, which refuses nothing that should fold.
_Avoid_: scope, bound name, symbol table

**Confident**:
Whether an evaluation produced a value the compiler may rely on.
`EvaluateResult { confident, deopt, reason }` is the gate: a non-confident
result carries a deopt expression and a reason instead of a usable value, and
every caller must stop rather than guess.
_Avoid_: successful, resolved, known, static

**Deopt**:
The expression an evaluation fell back to when it could not stay confident — the
code that must be emitted for the runtime to finish the job. Recorded on the
result, not thrown.
_Avoid_: bailout, failure, fallback, error

**Applied global**:
A JavaScript global folded by being _called_ — `String(x)`, `Number(x)`,
`Array(x)`, `Object(x)`. All four are native functions, so they are an engine
fold like any method call: no table of them, and no conversion written out in
Rust. `VALID_CALLEES` names the set. `Math` is one of them, so `Math.round(1.5)`
names a global, but the value it holds is not callable and a bare `Math(x)` is
refused by name.

**Which bindings shadow one depends on where it is written**, and the two rules
go opposite ways. Applied as a function, _every_ [declared
binding](#declared-binding) shadows it (`unshadowed_applied_global`), because
folding the module's own function would name a class hashed from a declaration
the reference compiler never wrote. Read as the receiver of a static, only a
declarator does (`unshadowed_receiver_global`), because the receiver carries no
value across the [transport](#transport): a `function Math() {}` changes nothing
about `Math.max(1, 2)`.

A global written where a _value_ belongs is refused and named —
`['Arial', false].filter(Boolean)` — since it carries nothing across the
transport; `Boolean` is recognised only there, as `VALUE_ONLY_GLOBALS`. An
argument the transport cannot carry hands the call back instead, to the
[conversion behind the fold](#conversion-behind-the-fold).
_Avoid_: callable global, built-in function, wrapper call

**Engine fold**:
Folding a method call by evaluating it in an embedded JavaScript engine (boa)
instead of matching its name against a table. A table is finite, so the method
it does not list is the next bug report; evaluating covers `String.prototype`,
`Array.prototype` and `Object.prototype` at once, the `Math` and `Object`
statics, and a whole chain at once. An expression qualifies when every leaf
resolves to a value the bridge can carry, which is what the
[guard](#fold-guard) decides.

The engine is handed the expression alone and knows nothing of the module, so a
resolved name crosses beside the source as a [transport](#transport) argument.
Nothing is ever registered on it: the instance is leaked per thread and shared
by every file that thread compiles, so a name left behind would be a cross-file
correctness bug. What comes back is the evaluator's own value type, not a syntax
node. Why the engine is permanent is [ADR
0008](./docs/adr/0008-the-fold-guard-reads-values-and-the-engine-is-permanent.md).
_Avoid_: boa fold, reflection, dynamic dispatch

**Fold memo**:
The compiled scripts an engine keeps beside itself, one per distinct printed
expression, so a file writing one shape a thousand times is printed and parsed
once. Keyed by `FoldKey { call, parameters }`, two 128-bit structural hashes, so
the key is in hand before anything is printed. What is memoised is the print and
the parse, never the answer: the script is re-run on every fold, which keeps a
mutating receiver reordering a fresh array. Bounded at `MAX_COMPILED_SCRIPTS`
(2048) and cleared wholesale on overflow, so a watch-mode process does not
accumulate one entry per call site per save. It leaks with the engine, because a
compiled script belongs to a realm.
_Avoid_: cache, arrow cache, script pool

**Fold guard**:
The predicate in front of an engine fold, and the whole of that fold's
behaviour: what it admits is answered by the language, so every boundary the
compiler owns is a refusal the guard states. It reads values, not only syntax —
a leaf qualifies when it is written into the expression, bound by a callback
around it, or a name the module resolves to a carriable value — so naming a
value no longer changes whether the call on it folds. Refusals answerable from a
name alone are applied before the walk, so only an expression the guard intends
to fold pays to have its names read. Reading a name to decide any of this is a
[speculative read](#speculative-read).

Its verdict is one of two. `Decline::NotACandidate` hands the call back and the
dispatch below owns it, which is what leaves the own-keys statics an answer for
a receiver the bridge cannot carry; `Decline::rule(...)` refuses it. The first
is answered before any rule that reads a resolved value, so a call the fold was
never going to claim cannot report a ceiling for a receiver nothing claimed.
Only three refusals sit in front of it, all pure syntax: a locale-sensitive
method name, a receiver written as a number, and an `INVALID_METHODS` member on
an unshadowed global.

Five rule sets, by name:

- **Locale-sensitive** (`LOCALE_SENSITIVE_METHODS`) — the engine has no locale
  data and would answer from the root locale, which is a wrong value rather than
  no value.
- **Escaping** (`ESCAPING_PROPERTIES`: `constructor`, `call`, `apply`, `bind`) —
  each walks off the value that was written and onto the language's function
  graph, where `Function` compiles a string into a body.
- **Length-amplifying** (`LENGTH_AMPLIFYING_METHODS`: `repeat`, `padStart`,
  `padEnd`) — bounded by arithmetic rather than by a shape, because nothing in
  the engine bounds allocation: the guard works out how long a string the call
  would build and refuses past the project's [allocation
  ceilings](../stylex-structures/CONTEXT.md). The count is read rather than
  required to be written, so a name reaches the same bound a literal does, but a
  receiver that is itself a **call** is left unread — resolving it would build
  the very string the bound exists to prevent.
- **Declared length** — see the entry below.
- **Nesting** — bounded on both sides against the configured evaluation depth:
  on the way in because the engine's parser recurses, and on the way out because
  the depth of an answer is not what the width bound measures.

Each applies at every link of a chain, since a chain hides its middle links.
Inside a **callback** body the bound is a product: the guard counts the
[measured receiver](#measured-receiver) the body was written on and multiplies,
so nesting multiplies rather than resets. The guard does not recognise a
callback as a shape — the engine parses it, so destructured parameters and a
block body are the language's business — but it names what the callback binds
and applies every rule above to the body. A statement the body walk does not
read is refused by name, a **loop** above all: the engine's iteration count
lives on the call frame, so a callback invoked per element starts a fresh count.

A **mutating** method is not among the rules. Measured, the reference compiler
refuses none; it disqualifies the **binding** instead, in binding resolution.
_Avoid_: whitelist, allowlist, filter, validator

**Transport**:
How a value the guard resolved reaches the engine. An expression that resolved
names is printed as an arrow taking them as parameters and called with their
values, so `s.toLowerCase()` is handed over as `(s)=>s.toLowerCase()`; one that
resolved none is evaluated as itself, because wrapping it costs a function
object and a VM frame. The author's own name is the parameter name, so nothing
is rewritten and a name the callback binds shadows the value exactly as it does
in the module. Because the value is an argument rather than text, the printed
source stays the size of the expression however large the value is.

A _function_ crosses by the other route: it has no value an argument could
carry, so the declaration it came from is printed as the parameter's default and
`undefined` is passed to hold the position. `['b','a'].map(upper)` is handed
over as `(upper=(p)=>p.toUpperCase())=>['b','a'].map(upper)`.
_Avoid_: value bridge, injection, substitution, interpolation

**Carried value**:
A value the bridge copies inward: a string, a number, a boolean, `null`,
`undefined`, an array, or a plain object, nested to any depth of those.
`undefined` has no literal, so it crosses under its name in both directions.
Everything else the evaluator can answer is **handed back, not refused**. A
callback is the exception: a name holding one either crosses as its declaration
or is refused by a sentence naming that binding.

A [theme reference](../stylex-state/CONTEXT.md) is the one of this compiler's
own values that crosses, as a **group stand-in**: a JS `Proxy` carrying the
group's identity, which answers a member read with the `var(--…)` that member
names. Nothing of the group is copied, because a group stores no members. A
dotted path is the one thing the stand-in cannot work out for itself, since
`colors.brand.primary` is one token rather than a read of a read, so the guard
reads those paths off the source and names them.

What a _name_ may hold is narrower: a number and a boolean cross as an element
or a property, but a name holding one alone is a receiver of its own and
`Number.prototype` is not reachable yet. Bounds are counted on the value rather
than on the syntax that named it, with one running total per direction. A value
crosses in two walks over one traversal: the first only measures, so nothing
oversized reaches an engine; the second builds the engine's values.
_Avoid_: primitive, scalar, serialisable

**Conversion behind the fold**:
What answers `String(x)`, `Number(x)`, `Object(x)` or `Array(x)` when the fold
hands the call back — that is, when the argument is one of this compiler's own
values. Not a second implementation: it reads the same
[coercion bridge](#coercion-bridge) that `+` and a template literal read, so a
value cannot answer one way in a call and another in an interpolation. It
measures a string as it writes it, and raises the refusal itself where it cannot
convert at all.

How much of the argument list it answers for is each conversion's own. `String`,
`Number` and `Object` read the first argument and ignore the rest; `Array` has
no surplus, since every argument is an element, and a style array is a fallback
list. A lone number is the exception, being a length rather than an element:
reaching here means something else in the call declined, so it carries a length
nothing bounded, and it is refused rather than allocated.
_Avoid_: fallback coercion, the Rust conversion, the second bridge

**Engine-callable StyleX function**:
A function of the [folded function map](#folded-function-map) whose answer is a
function of its arguments alone, so the engine may run it. `firstThatWorks` is
the whole of that set: it reorders the fallbacks it was handed and folds the CSS
variables among them into one `var()` chain, touching no state. Every other
function of the map answers by writing into the build, and running one once per
element inside a [speculative read](#speculative-read) would inject what the
source describes once as many times as the receiver is long — which is also why
the map itself cannot cross.

Reached only in the _callee_ position, and only through the name the module
bound. It travels as an ordinary transport parameter. The ordering is shared
Rust rather than written twice, so the engine's answer and the evaluator's
cannot drift apart.
_Avoid_: pure StyleX function, builtin, injected function

**Named callback**:
A function an author declared once and passes to a method by name. It crosses
the transport as the declaration it came from, printed as the default of the
parameter its name became. Which declarations qualify is decided by what the
name resolves to, not by how it was spelled: an arrow with plain parameters and
a single expression body that nothing writes to afterwards, which is the set the
reference compiler folds too. A block body, a destructured, defaulted or rest
parameter, a `function` of either spelling, and a binding written to after
declaration each refuse on both sides, naming the binding rather than the
method.

A name outside that set gets one of two answers, by resolution rather than by
spelling. A binding the module declares as a function is _refused_. A name the
module declares nothing for — a dynamic style's own parameter above all — is
_handed back_, since refusing it would fail a build over a call that was only
ever going to run at runtime.

Calling one through a name is decided by where the call sits: a call _inside_ an
expression the fold already claimed is the fold's, and the _outermost_ call
stays the dispatch below, which resolves the name this compiler's own way and
_applies_ it at the call. An argument with no expression form **binds nothing
and leaves the parameter unbound**, as the language does with an argument nobody
passed: a body that never reads that parameter folds.
_Avoid_: function reference, higher-order argument, callback binding

**Speculative read**:
Resolving a name to decide whether a fold is _possible_, as opposed to folding.
Load-bearing because a refusal raised under one is not the subtree's answer: the
guard hands the call back, the dispatch below evaluates the same name in
earnest, and it has to find the state and the sentence it would have had. So
`speculate()` puts back the evaluation's confidence and deopt, and the per-file
memo withholds the refusal while still memoising a value that did resolve.
_Avoid_: dry run, probe, tentative evaluation, trial fold

**Refused fold**:
A deopt raised by a fold that recognised its callee and will not produce a
value. Every refusal in this area is one — there is no separate error-raising
path — so whether an author sees a failed build or working runtime code depends
on where the call sat: in a static position the `stylex.*` call fails, and in a
dynamic style function the call is left for the runtime. A refusal carries the
rule that refused it, in this compiler's own words: message text is not a parity
obligation, and the harness compares class name, rule text and style-object
shape rather than sentences. A call the guard never recognised is not a refusal.
_Avoid_: fold error, hard error, invalid call

**Hole**:
An array slot with no element in it. Two arrive by different routes. One
`Array(n)` created by counting crosses back as `undefined`, so it joins as
nothing and reaches the style-array check unchanged, where a counted array is
refused and the fold itself succeeds. One an author _wrote_, as in `[, 1]`,
refuses the whole array: the reference implementation evaluates element paths
and a hole's path carries no node, so both compilers answer
`Could not resolve the code being evaluated`. Dropping it is what refusing
replaced — `[, 1].length` answering `1` was a value the source does not
describe. Inside a dynamic style's body the refusal is not an error at all.
_Avoid_: empty slot, gap, undefined element

**Member lookup**:
What a property read asks of a string or an array, decided once for all three
receiver kinds as `ArrayLikeLookup`: `Length`, `Index`, `Missing`, or
`Unreadable` for a computed key with no name the evaluator could read. The
classification carries the key it read, so the refusal a receiver gives names
the same key the classification saw.
_Avoid_: property access, index check, member kind

**Written slot**:
The element count the language reports for an array, read from the literal as
written rather than from what evaluating it produced. A written hole is why:
it occupies a slot the language counts and refuses the array it is in, so the
count is answered from the source ahead of the receiver being evaluated. A
spread never reaches the count, being refused first. The receiver is unwrapped
before it is read, because a parenthesis is not a different receiver.
_Avoid_: array length, element count, size

**Declared length**:
A length a call states in an argument and does not pay for — `Array(n)`, whose
array is sparse, and `Array.from({ length: n })`, which is one property saying
the same thing. It is the only length a fold can be asked to build that nothing
has already been charged for, so the entry ceiling is compared against it where
it is _declared_, before the engine runs, rather than only where an array
crosses back. A length the language itself rejects is not one of these: it
raises before allocating.
_Avoid_: sparse length, array size, allocation hint, entry count

**Measured receiver**:
One reading of the value a callback's call was written on, answering the three
things the amplification bounds need together, so they cannot disagree: how many
**elements** it holds, and so how many times the body runs; what the widest of
them **renders** to, which bounds a length read off the parameter handed the
element; and the largest **index** it has, which bounds a count written as
`i + 1`. The reading belongs to the receiver rather than to the method, so a
method nobody wrote down is measured like every other — the method is asked one
question only, which parameter is handed the element. Only a reducer (the
second) and a comparator (none, since the language runs one more often than the
array is long) answer it other than plainly. `Array.from` measures the value it
iterates. A receiver the reading cannot resolve leaves the callback
**unmeasured**, which is the blanket refusal.
_Avoid_: callback size, receiver length, repeat count

**Spread refusal**:
The single answer every spread in a value position earns —
`Unsupported expression: SpreadElement` — given before the operand is evaluated,
and the same whatever the operand is and wherever the spread sits. Upstream gets
this for free by evaluating elements as _paths_, where a spread is a node kind
with no fold; this evaluator reads through to the operand, so it refuses
explicitly. One constant, `SPREAD_ELEMENT`.
_Avoid_: unsupported spread, illegal spread

**Own enumerable properties**:
What a spread operand contributes to the object it is spread into —
`Object.assign` semantics, which is what the reference implementation calls. A
number, a boolean, `null`, `undefined` and a function have none, so spreading
one adds nothing and is not an error; a string and an array have their indices.
Two readings are refused because this evaluator cannot write them down: an
astral string, whose code units are lone surrogates no Rust string holds, and a
value held in a representation of the evaluator's own.
_Avoid_: spread properties, object keys, assigned properties

**Object method receiver**:
What the argument of `Object.keys` / `values` / `entries` reads as. Four
answers, not two, because an absent object spells three different things. A
receiver that is not an object folds to `[]`, as `Object.keys(5)` does. One
holding an element with no expression form refuses, since `[]` would write a
shorter list than the source describes. A nullish receiver refuses under the
sentence the language itself raises, which complains about the receiver rather
than the list. A [folded function map](#folded-function-map) is a receiver like
any other and reads through its object form.
_Avoid_: object argument, keys source

**Winning operand**:
The side of `||`, `&&` or `??` the fold keeps, returned as the value it already
was rather than re-created as a literal — so a winning object stays an object.
The three are decided on their own node, because they answer with an operand
where every other binary operator answers with a coercion of both.
_Avoid_: result, chosen branch, short-circuit value

**Dead operand**:
The side of `||`, `&&`, `??` or `?:` the language never evaluates. Neither the
evaluator nor the guard enters one, and for the guard that is correctness rather
than a saving: reading a leaf queues a theme reference's compensating import and
can refuse the whole call, while a speculative read puts back the evaluation's
state but not the module's. So a token read behind a compile-time-false guard
would leave an import behind for a value no stylesheet holds. Which operand is
dead is the operator's own rule, read from the one place that states it — and
read from the module only outside a callback body, since inside one the engine
binds the names and both sides have to carry.

Not entered is not the same as not printed: the engine decides the short circuit
itself, so a dead operand crosses whole and the printer and parser both descend
through it. The guard therefore measures its [text nesting](#text-nesting) where
it declines to enter it.
_Avoid_: unreachable branch, pruned side, skipped operand

**Coercion bridge**:
A helper that answers one of the [coercion crate](../stylex-js/CONTEXT.md)'s
questions over `EvaluateResultValue` rather than over an expression, deciding
for each evaluator-only variant what the value it stands for would have
answered. Exactly one per question, and every caller goes through it, because a
private copy of a question is how two call sites come to disagree.
_Avoid_: adapter, wrapper, converter

**String operand**:
An evaluated side of `+` that already _is_ a string, which is what decides
whether the operator concatenates or adds — either side being one is enough. The
question is asked of the operands, never of whether coercing them to numbers
happened to succeed: `'1'` coerces perfectly well.
_Avoid_: stringable operand, non-numeric operand

**Reference resolution chain**:
The one ordered question the evaluator asks of an identifier it could not fold
from the injected function map: which binding does this name, and what does that
binding fold to. Eight steps in the reference implementation's order, each
citing the line range it mirrors, because both compilers agree on every step's
verdict and the sequence is the only thing left for them to disagree by. One
step is deliberately absent and says so at its position. The rest ask for an
[import specifier kind](../stylex-state/CONTEXT.md), a
[declared binding](#declared-binding), a
[binding write](../stylex-state/CONTEXT.md) and an
[early reference](#early-reference). Why the order is upstream's is
[ADR 0003](./docs/adr/0003-one-ordered-chain-resolves-a-reference.md).
_Avoid_: identifier lookup, binding resolver, evaluate fallback

**Folded function map**:
What a reference resolves to when its name is a key of the injected function
map, which is consulted ahead of the chain and keyed by name rather than by
binding — so a dynamic style's parameter spelling `stylex` folds to the
namespace import's map and not to the parameter. Deliberate on both compilers:
an arrow parameter is injected into that same map so a nested `create()` can see
it, which is why the map cannot be keyed by binding.

The map holds the StyleX API surface reachable off one namespace, which is not
all functions: `when` is a config and `env` is an object of the `env` option's
entries. `env` is registered into the fold only where a `create` call sets its
evaluation up, so a bare `stylex` written where a static value belongs refuses
rather than materializing and dropping the declaration.

The fold carries no expression form, so every position needing one
**materializes** it as the object it stands for — its keys, each carrying a
function — through one function, so the sentence a build stops on cannot depend
on which position asked. Which keys depends on what the name was registered as,
mirroring upstream's registration rather than this compiler's types: a map
stands for one key per entry, a single function config for `{ fn }`, and a
config holding a marker map for the marker names. Every other evaluated shape
with no expression form is refused rather than materialized, a theme reference
among them.
_Avoid_: shadowed namespace, identifier map hit, function config fold

**Early reference**:
A reference that begins before the declarator naming it ends, so the program
does not hold the value where it is read. The sibling of a binding write: both
make the declaration initializer an unsound stand-in, and both deopt.
Declarations are collected module-wide with no notion of position, so this is
decided by comparing the parser's byte positions, and a
[synthesized node](../stylex-ast/CONTEXT.md) is never one.
_Avoid_: forward reference, hoisted read, out-of-order declaration

**Evaluation depth**:
How many levels of the fold are currently standing, counted on the
[state manager](../stylex-state/CONTEXT.md) rather than on the evaluation,
because the evaluation's confidence forks while the stack it accounts for does
not. Crossing the ceiling is a refused fold, not an abort. Always in **fold
levels**, never in levels of source nesting: a member read spends two, an array
element spends one for the array, and a parenthesis spends none. The ceiling is
`maxEvaluationDepth`, resolved in
[stylex-structures](../stylex-structures/CONTEXT.md). Why the fold owns both a
ceiling and a stack is
[ADR 0004](./docs/adr/0004-the-fold-owns-its-own-ceiling-and-its-own-stack.md).
_Avoid_: recursion limit, nesting level, stack depth

**Grown stack**:
Room a descent is given rather than room it inherited. Several descents recurse
without a budget of their own — the evaluator's walk, the guard's walk, the
carriage of a value each way, SWC's print of the source the engine is handed,
and the engine's parse of it. Overflowing any aborts the process from inside an
evaluation that is allowed to refuse, so none may run on whatever the thread had
left over. Nothing is allocated when the room is already underfoot.

Two modes. **Asking by the level** spends one level and claims headroom for the
next, so a walk that stops early pays only for what it descended; every walk
this compiler writes asks this way. A **claim** is the whole descent at once,
for the two callers that will not ask again — SWC's printer and the engine's
parser — sized from the [text nesting](#text-nesting) of what they are handed,
and made after the guard has admitted a call. Whether a depth may be claimed at
all is **carriable**: the deepest carried nesting and the limit the evaluation
depth is clamped to are one constant. Past it the caller refuses.
_Avoid_: stack growth, thread stack, recursion budget

**Text nesting**:
How deeply an expression nests, counted at the three node kinds that nest
without bound — an expression, a statement and a binding pattern. Everything
between two of them is a fixed number of frames, so counting the three counts
the descent a printer or a parser makes through the text. Distinct from
evaluation depth, which counts the levels a _fold_ spends: an operand a short
circuit never reaches costs the fold nothing and the parser its whole height.
_Avoid_: expression depth, source depth, nesting level

**Measured string**:
A string the evaluator grew with `+` or a template interpolation, together with
the count of UTF-16 code units it was measured to. The count is what
`maxFoldedCharacters` is spent in, and it travels with the text, so a chain of
`+` measures each operand once: the link above **adopts** the buffer below it,
text and count together. A measured string that goes through the
[memo](../stylex-state/CONTEXT.md) comes back as a plain string literal and is
measured again, because the tree has nowhere to carry a count.
_Avoid_: grown string, sized string, string with a length
