# The printed fold reads and calls through a check

**Status:** accepted

A computed property the guard cannot name is printed as `__sxRead(o, k)` rather
than as `o[k]`, and a call through a name the printed source binds itself is
printed as `__sxCall(f)(x)` rather than as `f(x)`. Both are parameters of the
arrow the fold is printed into. The reader refuses the union of
`ESCAPING_PROPERTIES` and `BLOCKED_PROPERTIES` on the coerced key; the caller
refuses the language's `Function`, anything inheriting directly from it, and
`eval`, and answers the callee so the engine performs the call.

This is the third turn of one argument.
[0008](./0008-the-fold-guard-reads-values-and-the-engine-is-permanent.md)
removed the engine's method tables and put a guard in front of it.
[0009](./0009-a-static-is-called-only-from-an-allowlist.md) brought one list
back, for the one surface where a missing name is an open door. This record says
what the guard cannot answer at all, and where the answer had to move to.

## What the guard cannot answer

The guard reads the source. It refuses `o.constructor` and `o['constructor']`,
because both spell a name, and the dispatch below the fold refuses a key the
evaluator resolves. Neither reading reaches the third case: an expression that
crosses into the engine **whole**, carrying a key that is a value.

```js
const a = '__proto__';
const seed = {};

String(
  [seed[a].constructor.constructor]
    .map(F => {
      const h = F(
        'String.prototype.toUpperCase = function () { return "PWNED" }'
      );
      return h();
    })
    .join('')
);
```

Every step is admitted. `seed[a]` is a computed key the syntax does not spell,
so no name is there to refuse. The read is then performed by the engine's own
index operator, under no rule of this compiler. The constructor is laundered
through a callback parameter, which turns a value no rule could name back into a
call through a bare name — and a call through a bare name was admitted without
asking what the name held.

Measured, this compiled `'clean'.toUpperCase()` in a **later, unrelated**
declaration to `PWNED`, because the engine is one per thread and a prototype
written in one fold is read by every fold after it. `Date.now()` is reachable
the same way, which moves a class name between builds.

## Why not refuse the shape

The obvious guard is to refuse any computed key the syntax does not spell. It
is not shippable: `['a','b','c'][i]` and `{a:'X',b:'Y'}[k]` both fold today and
both are ordinary. Refusing them would trade a security fix for a large,
silent loss of folding.

## Why a check in the printed source

The rule is about the key's **name**, and the name exists only after the engine
has evaluated the key. So the check has to run there. Printing a call to a
reader is the smallest way to put it there: the rewrite is local to one node
kind, the refusal comes from the same table the guard reads, and every fold that
needs no check is printed exactly as before — an expression with no computed key
and no call through a name pays nothing, which keeps the cheapest fold cheap.

The alternative was to neutralise the escape routes inside the engine, by
overwriting `Function.prototype.constructor` and the global bindings in the
prelude. That is a denylist of gadgets against an engine whose object graph is
its own, which is the mistake this record exists to stop repeating.

## Which calls the caller stands in front of

A value the engine produced is invoked in one of three ways: through a member,
through an expression, or through a name.

A member call spells its method name, and the guard reads it — `call`, `apply`,
`bind` and `constructor` are refused there. An expression callee is not a
candidate for a fold at all. A name is either bound inside the printed source,
which is the shape the escape used, or free — and a free name is a parameter of
the printed arrow, holding a value or a function this compiler carried, or one
of the globals the guard lets stand free, which is the applied-global
allowlist. Neither can be a function that compiles code.

So the caller stands in front of exactly one shape, and that shape is the whole
set. `String(x)` keeps the language's own call and costs what it always cost,
which matters because the arrow this record adds is otherwise the wrapping the
fold measured at +44% on its cheapest leg. The names the printed source binds
are collected from the printed tree in one pass, without a scope stack: a name
bound in one callback makes the same spelling in another go through the check
as well, which costs one call and answers the same value.

This is the one place the record reasons about routes rather than closing them,
so it is written where a reader will meet it: if the guard is ever taught to
leave a new kind of name free, this has to be read again.

## Why two checks and not one

The reader closes the route that was found. The caller closes the route that has
not been. A reader alone is one unknown gadget away from an escape, and a
gadget nobody has thought of is exactly what the reader cannot be written
against. The reference implementation ships both for the same reason, and words
the second refusal as `BLOCKED_FUNCTION_CALL`, which this compiler now shares.

The caller names the async, generator and async-generator constructors one by
one as well as testing the inheritance the reference implementation tests. That
is not a second list for its own sake: this engine answers
`Object.getPrototypeOf` of its `GeneratorFunction` with `Function.prototype`
rather than with `Function`, so the inheritance test alone lets one through.

## Why parameters and not globals

A global is a name the printed source could point somewhere else. A parameter of
the arrow the fold is printed into is not reachable from the source at all.

A parameter is still a name, and an inner binding of the same spelling would
shadow it — so the guard refuses a binding that spells either name, and refuses
a module name of either spelling as well. Every name the walk binds passes
through one function, which is what makes the two parameters unshadowable rather
than merely unlikely to be shadowed.

## What it costs

A read through the reader is one call frame and a set lookup instead of an index
operation; a call through the caller is one call frame and a set lookup. Both
are paid only by the folds that print one, and the rewrite itself runs only
where the fold memo missed, so it is a cost per printed shape rather than per
fold. The arrow is printed with two extra parameters, and two extra arguments
cross with it.

## What this record does not change

The guard is unchanged in what it admits. A name the syntax spells is still
refused by the guard, with the sentence written for it, and the checks never see
it. `member_prop_name` still answers a string key, so `o['constructor']` is
refused before anything is printed; a numeric key keeps the index operator,
because no number spells a refused name.
