# 08 — Drop declarations with empty values

**What to build:** An empty or whitespace-only style value produces no
declaration, so the emitted stylesheet stays valid CSS.

Today `stylex.create({ a: { color: '' } })` compiles to `.x1tfe9bt{color:}` — a
declaration with no value, which is invalid CSS and which a browser discards.
The reference implementation does not produce that, and the two compilers
diverge.

Found while measuring the coercion work: an empty string was briefly mistaken
for a zero-argument `String()` crash. It is not — the same output reproduces
from a plain empty string literal with no coercion anywhere, which is precisely
what makes it a separate defect rather than part of the fold.

## What upstream does

Deliberately, in the cases it handles:

- `create({ color: null })` — no rules
- `create({ color: [''] })` — no rules
- `defineVars({ background: '' })` — emits the empty custom property, and is
  unaffected by this change

And accidentally, in the remaining case: `create({ color: '' })` and
`create({ color: ' ' })` both dereference a null inside its value normaliser,
which reads the first node of an empty parse. That is a defect, not a contract,
and is not reproduced. Converging on "no declaration" matches what upstream
already does deliberately in the neighbouring cases and produces valid CSS in
the one it crashes on.

Locate where valueless declarations are already dropped and route
whitespace-only values there, rather than adding a second parallel check. The
spec assumes that lives in the CSS crate; if it turns out to sit in the
transform crate, follow the code and adjust the scope.

**Blocked by:** None — can start immediately. Fully independent of the fold.

**Status:** done

- [x] An empty style value emits no declaration
- [x] A whitespace-only style value emits no declaration
- [x] `defineVars` still emits its empty custom property — the change belongs to
      declaration emission, not custom-property emission
- [x] Pinned in the value-normalisation transform directory alongside the
      existing normalisation cases
- [x] No existing fixture shifts

## Outcome

The drop is one decision in one place: `convert_style_to_class_name` returns
`None` when the transformed value carries no CSS text, and
`StylesPreRule::compiled` maps that to `CompiledResult::Null` — the result the
existing `NullPreRule` already produces for an authored `null`. So a blank value
reaches the same outcome as `null` by reaching the same result type, without a
second parallel check anywhere.

Judging the value **after** transformation rather than before is what makes it
correct rather than merely close. `content: ''` transforms to `content:""`,
which is a real declaration the reference implementation emits; a check on the
authored value would have dropped it and shifted an existing fixture. Blank
entries of a fallback array drop at the same site, before `variable_fallbacks`
composes the `var()` chain, so the class name is hashed from the entries that
survive — `[' ', 'red']` yields the class name a lone `'red'` yields.

`is_blank_css_text` in `stylex-utils` names the predicate so the definition of
"blank" has one home; **Blank value** is now a term in the transform glossary.

### Measured against the reference implementation

`@stylexjs/babel-plugin` 0.19.0, 34 inputs, `create`/`defineVars`/`keyframes`
across all three style resolutions:

- **21 match** exactly on class name, rule text, priority and compiled object —
  every input upstream handles deliberately. Two of these were divergences
  before this change: under `legacy-expand-shorthands`, `margin: null` and
  `borderColor: null` emitted four empty declarations here, because that
  expansion turns an absent part into `Some("")` rather than propagating the
  absence. Both now drop, as upstream drops them.
- **11 are inputs upstream crashes on** — the whole blank family inside
  `create`, including the blank-in-array and blank-nested-branch shapes. All now
  compile to valid CSS, converging on what upstream does deliberately for
  `null`.
- **2 diverge**, both recorded below.

### Divergences left standing

- `borderColor: ''` under `legacy-expand-shorthands` only. Upstream emits four
  invalid declarations (`border-top-color:` …) because that expansion splits a
  blank into blank parts; here they drop. Emitting invalid CSS is the defect
  this ticket exists to remove, so it is not reproduced — and it is unreachable
  under the other two resolutions, where upstream crashes instead.
- `keyframes({ from: { color: null } })` fails here with `Expected a string
  value but received a different type.` where upstream compiles it with the
  declaration dropped. **Pre-existing and unrelated**: the keyframes path
  touches neither changed function. It is a fourth shape of the defect 09
  already covers, and has been recorded there.

### Found but not filed

`positionTry` diverges twice, in neither case about blank values: `top: null`
fails the build where upstream drops the declaration, and the at-rule is
serialized as `@position-try --x{…}` against upstream's `@position-try --x {…}`.
Untouched by this ticket — `positionTry` shares none of the code involved.

### Review outcome

Both review axes flagged the same structural point, and it was right: an
earlier draft had two checks — a pre-transform filter in
`flatten_raw_style_object` for array entries and the post-transform check for
lone values — which the ticket explicitly warned against. Moving the array
filter to the post-transform site collapsed them into one, left
`flatten_raw_style_object` unmodified, and closed a gap the two-check version
had: a blank surviving in a mixed array would have emitted `color:;color:red`.

Also applied: one fixture now exports its styles, so the compiled object is
pinned as well as the CSS — `{ kMwMTN: null, kWkggS: "xrkmrrc" }`, which is what
lets a later namespace revert the property, and is byte-identical to upstream.
The glossary entry was corrected twice, ending up simpler than either draft
because the restructure removed the asymmetry it had been describing.
