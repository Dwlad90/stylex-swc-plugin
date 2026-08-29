# 40 — The engine is proved one per thread

**What to build:** A test that compiles on many threads at once and asserts each
one gets its own answer, so the invariant the whole engine design rests on is
observed rather than assumed.

**What rests on it.** The engine is a `ManuallyDrop<Engine>` in a thread-local,
deliberately leaked, because boa's garbage collector lives in a thread-local of
undefined drop order. The compiled-arrow memo is leaked with it. The transport
was chosen over registering globals on the engine *specifically* because the
engine is one leaked instance per thread shared across every file that thread
compiles, and a name left behind or shadowed would be a cross-file correctness
bug. The NAPI host compiles on a worker pool. Every one of those sentences is an
argument about threads.

`thread::spawn` appears in exactly one test in the whole workspace, and it is the
uid generator's. Nothing exercises this.

**What a test would catch.** A value left on the engine's global object between
folds; a memo entry keyed in a way that collides across differently-bound values;
a `Context` reachable from two threads at once. The first is the one the design
document says it is protecting against, and it is the one no current test can
see, because every existing test folds on one thread in sequence — where a leaked
global and a correct fold are indistinguishable.

**A second question the same test answers.** One engine is leaked per thread,
forever. A host that retires and respawns workers leaks a `Context`, its source
interner and its never-evicted memo per retired thread. Whether the napi pool is
long-lived rather than churning is a fact this branch depends on and does not
record.

**Blocked by:** none — can start immediately.

**Status:** resolved

- [x] Eight threads, fifty folds each, half folding one receiver and half
      another; every thread gets its own answer every time
- [x] A shape that would show a leaked global — a fold whose result would differ
      if a name from the other thread's fold were still bound
- [x] Whether the NAPI worker pool is long-lived is established and written down
      where the leak is argued
**Resolution:** `thread_isolation_tests` is the observation, beside
`source_evaluation`, which now carries `folded_in_a_module_binding` because a
second suite needs it. Eight threads released together through a barrier fold
fifty times each -- the even ones on a string receiver, the odd ones on an array
-- with the marker bound to the thread's own index rather than written into the
call, so every thread prints the same source for its half and differs only in
the value travelling beside it. An answer carrying another thread's marker could
only have come from another thread's engine.

The leaked-name case asks the engine rather than the answer, because the answer
cannot tell: a leaked name and a name that was never written fold to the same
value. `holds_a_global` reads an own property of the thread's global object, and
each folding thread asserts the marker is not on it while seven others are
folding markers of their own. The probe is controlled by `Object`, a name the
language does bind, so a probe that saw nothing at all could not pass. The guard
half sits beside it: the threads that bind nothing refuse both the source their
neighbour is folding and the one they would fold themselves, which is what keeps
a free name from ever being printed. Written and read are each refused, and the
two together are the checkbox.

A third case reads the slot itself: a thread arriving after eight others have
each built an engine finds it empty and builds its own on the first fold.

The pool question is settled rather than assumed, and settled only as far as
this repository can answer it. `transform` is the one exported binding, it is
synchronous, and it takes `napi::Env`, which is not `Send` -- so it folds on the
JavaScript thread that called it and cannot be dispatched to libuv's pool. No
package here spawns a thread of its own. What that establishes is the *shape* of
the cost, which is per retired thread rather than per file; how long a host
keeps its workers is the host's answer, and `jest-worker` is named in ADR 0008
as the one in this dependency graph that retires them. The claim that bundler
pools are long-lived was checked and cut rather than kept.
