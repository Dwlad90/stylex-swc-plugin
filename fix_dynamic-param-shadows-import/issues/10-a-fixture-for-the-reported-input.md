# 10 — A fixture for the reported input

Status: `ready-for-agent`
Blocked by: 02

**What to build:** The reported module pinned as emitted text, in both
development and production mode, not only as rule metadata.

The corpus compares rule metadata between the two compilers, which is the right
question for a divergence but says nothing about the code we emit around it. A
shadowed dynamic parameter is exactly the shape where the emitted module
matters: the parameter has to survive into the runtime function, while the
import stays a theme reference for the static prop beside it.

Add the reported input as a fixture case, following the existing ones — a
`.stylex.js` input with a development-mode and a production-mode expected
output, picked up by the fixture runner without registration.

- [ ] The fixture case exists with both expected outputs
- [ ] Both are checked in as generated, not hand-edited to what looks right
- [ ] The dynamic function keeps its parameter, and the static prop beside it
      keeps its theme reference
