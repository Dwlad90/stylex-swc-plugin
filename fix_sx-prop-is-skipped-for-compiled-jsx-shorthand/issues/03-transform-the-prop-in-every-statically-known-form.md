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

**Status:** ready-for-agent

- [ ] The object shorthand transforms, under the default prop name and a
      renamed one
- [ ] A computed key with a statically known string transforms
- [ ] A getter or method property is left unchanged, proven by a test
- [ ] A spread property is left unchanged, proven by a test
- [ ] A compiled call naming a component rather than a host element is still
      left unchanged
- [ ] For each new case, the harness from ticket 02 confirms the output carries
      the same props call, binding and injected import as the reference output
      for the paired raw markup
- [ ] The Rust workspace suites pass, and formatting and linting pass
