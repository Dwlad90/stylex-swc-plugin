# 03 — Transform the prop in every statically known form of a compiled call

**What to build:** A component that survives a JSX-compiling build step keeps
its styles. `jsx("div", { sx })` produces what `jsx("div", { sx: sx })`
produces today, so the form a bundler or minifier happens to emit stops being
something an author has to reason about.

Two forms are added to the scan of a compiled call's props object:

- the object shorthand, whose value is the identifier itself;
- a computed key whose text is statically known — a string literal, or a
  template literal with no expressions.

Two forms are deliberately left alone, because the raw markup path leaves the
constructs they correspond to alone: getter and method properties, which carry
no value expression to forward, in the same way an attribute whose value is not
an expression container is skipped; and spread properties, which are not
inspected, in the same way a spread attribute is not inspected, and whose key
is not knowable at compile time. This ticket pins both as tests, so the
boundary reads as a decision rather than as an absence.

Where a props object names the prop twice the first occurrence wins, matching
the raw markup path, which stops at the first matching attribute. That is not
the last-wins order a runtime would apply; it is left as it is because both
paths agree today.

The host-element guard, the runtime binding resolution, the replacement, and
the cycle the transform runs in are all unchanged. Only the set of matched
property forms grows. If the crate already has a helper that reads a statically
known name from a property key, it is reused rather than a third matcher being
written.

**Blocked by:** 02 — the parity reference it is checked against.

**Status:** done

- [x] The object shorthand transforms, under the default prop name and a
      renamed one
- [x] A computed key with a statically known string transforms. A template
      literal with no expressions is covered too, and a key that is only known
      at run time is pinned as left alone
- [x] A getter or method property is left unchanged, proven by a test. A
      setter is pinned by the same test
- [x] A spread property is left unchanged, proven by a test
- [x] A compiled call naming a component rather than a host element is still
      left unchanged, in the shorthand form as well as the explicit one
- [x] For each new case, the harness from ticket 02 confirms the output carries
      the same props call, binding and injected import as the reference output
      for the paired raw markup. The harness gained a `computed key` case; its
      report is in `sx-parity-after-03.txt`
- [x] The Rust workspace suites pass, and formatting and linting pass

## Comments

- The two matchers the transform carried, one for the key and one for the
  value, became one function. A shape that names the prop but carries no value
  could otherwise be matched by the first and dropped by the second, which is
  how a getter would have read.
- The statically known name is read by the crate helper
  `namespace_name_from_prop_key`, so the computed key needs no new matcher.
  The helper answers for a numeric and a big-integer key as well, which the
  old matcher did not. Nothing reaches that: a prop name is configured as a
  string, and a name made only of digits names no attribute raw markup can
  write. It is left as the helper has it rather than narrowed around it.
- Duplicate-prop order is pinned by a test: the first occurrence wins, and the
  snapshot shows the second one left in place.
- The props object was copied twice for every compiled call with an object
  second argument: once before the scan, and once more inside a copy of the
  whole call. Because that object holds the element children, each copy grew
  with the depth of the tree. The object is now read through a borrow, so a
  call with no such prop copies nothing, and the new call is built argument by
  argument, so a call with one copies the object once.
- A getter, a method and a setter are each pinned, and the spread test spreads
  an object that does name the prop: were the spread inspected, that test would
  transform. A host element carrying a shorthand that names something else is
  pinned too, so both arms of the shorthand match are reached.
