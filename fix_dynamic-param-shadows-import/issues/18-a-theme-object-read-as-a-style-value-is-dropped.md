# 18 — A theme object read as a style value is dropped

Status: `resolved`
Blocked by: None

**What was found:** A theme import read as a style value *without* a member
access compiles to nothing. The reference implementation refuses it. We emit no
rule, no error, and no warning -- the declaration disappears.

```js
import { zIndex } from './vars/zIndex.stylex.js';
export const styles = stylex.create({ w: { zIndex: zIndex } });
```

| | |
| --- | --- |
| Babel 0.19.0 | `A style value can only contain an array, string or number.` |
| here | compiles, emits nothing |

Measured against `@stylexjs/babel-plugin` 0.19.0 under the parity harness's
configuration -- `haste` resolution, one source string.

| input, inside `create({ … })` | Babel 0.19.0 | here |
| --- | --- | --- |
| `w: { zIndex: zIndex }` | refuses | `ok`, no rule |
| `w: { color: 'red', zIndex: zIndex }` | refuses | `ok`, only the `color` rule |
| `w: { zIndex: { default: zIndex } }` | refuses | `ok`, no rule |
| `w: { margin: zIndex }` | refuses | `ok`, no rule |
| `w: { zIndex: \`${zIndex}\` }` | `z-index:x1q8i56t` | `ok`, no rule |
| `w: { zIndex: t }`, `import * as t` | `Referenced constant is not defined.` | `ok`, no rule |
| `dyn: (a) => ({ zIndex: zIndex, color: a })` | refuses | `Style value must evaluate to a static expression.` |

The mechanism is one line. `nodes/object_expression.rs:288` answers a
`ThemeRef` with `None`:

```rust
EvaluateResultValue::ThemeRef(_) => None,
```

and the caller reads `if let Some(value) = value` before pushing the property --
so `None` means "no property", not "refuse". Every arm beside it either folds to
an expression or deopts; this is the only one that drops the declaration on the
floor. A member read (`zIndex._10`) never reaches it, which is why this went
unnoticed: that is the shape every test and every fixture writes.

The one row where the reference implementation *emits* rather than refuses is
the template, and it emits the theme's hash as a bare string -- `z-index:
x1q8i56t`. Worth confirming that is intended upstream before mirroring it;
refusing may be the better answer even though it diverges.

Found while reviewing [08](./08-reject-a-folded-map-as-a-namespace.md), whose
`materialize_style_value` docstring claimed the style-value fall-through was
reachable by two shapes. A theme reference is the third, and it is the one the
spec opens with -- so this is the same seam, reached by a shape ticket 08 did not
audit. Related to [14](./14-an-array-style-value-inside-a-dynamic-style.md) and
[15](./15-the-function-map-read-where-it-is-not-a-map.md): all three are
"an evaluated shape with no expression form, read where a style value belongs".

Worth deciding together with 15 case 1, which is the same consumer
(`object_expression.rs`) refusing with the wrong words rather than not refusing
at all.

- [x] A bare theme object read as a style value refuses, with the reference
      implementation's text
- [x] A member read off the same import still resolves to its theme reference
- [x] The dropped-declaration path is gone: no arm of that match answers `None`
      in a way the caller reads as "no property"
- [x] The namespace-import spelling is decided too -- upstream reads
      `Referenced constant is not defined.`
- [x] The template row is measured against upstream intent before it is mirrored
- [x] Corpus entries for each row above, with the verdict each is known to read

## Resolution

The arm is gone, and with it the shape that made it possible. Every arm of the
style-value match in `nodes/object_expression.rs` now answers an expression or
refuses, so the property is pushed unconditionally -- there is no longer a value
the caller can read as "no property". A theme reference falls through to
`ILLEGAL_PROP_VALUE`, which is the reference implementation's sentence for the
same input: it folds the group to an object its namespace validation refuses
because it is not a plain object.

The dynamic style's body reads the same value through the other consumer,
`materialize_style_value` in `core/evaluate_stylex_create_arg.rs`, which reported
a sentence about a static expression. It refuses with the same sentence now.
Materializing a theme reference is not an option there the way a folded function
map was: the group's keys live in another file.

Both refusals report at the value the author wrote, with a code frame. The
dynamic consumer aborted without one, which
`docs/adr/0002-a-refusal-and-a-broken-invariant-are-separate-constructs.md`
reserves for an invariant the code established itself -- so the arm beside it,
which every array in that position already reached, is reported the same way now.

Measured on every row of the tables above plus the positions around them, against
`@stylexjs/babel-plugin` 0.19.0 under the parity harness's configuration. All six
create-position rows read `both-reject` now; the five rows that recorded the drop
as `acceptance-divergent` or as a wording divergence -- the bare read, the read
beside a sibling, the read at depth and in a shorthand, the namespace spelling,
the dynamic style -- were updated in place rather than duplicated. Pinned by
message in
`validation_stylex_create_test::theme_reference_style_values`, which the corpus
cannot do (`both-reject` compares acceptance, not wording -- issue
[17](./17-the-corpus-cannot-report-a-changed-refusal.md)).

### The two decisions this needed

**The namespace spelling.** Kept as a refusal of the value, not of the
resolution. Upstream cannot resolve a namespace import of a theme file at all
and says `Referenced constant is not defined.`; this compiler resolves it, and
member reads off it work. Matching upstream's sentence would mean giving up that
resolution, which is [11](./11-refuse-a-namespace-theme-import.md)'s question,
not this one. Both compilers refuse; only the wording differs, and the corpus row
holds the outcome.

*Superseded by [11](./11-refuse-a-namespace-theme-import.md).* The resolution
went: it hashed the reference's own local alias as if it were an export name, so
the member reads that "work" produced a variable the theme file defines only
when the alias happens to be spelled like the exported group. This row now reads
upstream's sentence for upstream's reason. Nothing about the decision made here
was wrong at the time — the question really was 11's — and the arm this refusal
guards is still the one it was written to guard.

**The template row.** Measured and deliberately not mirrored. Upstream coerces
the group through its own `toString` and declares `z-index:x1q8i56t` -- the
var-group hash as a z-index, meaningless CSS that hashes a class name all the
same. It is not upstream *intent*, it is JavaScript's coercion reaching a
position nobody designed for it. The seam is not this arm either: the template
evaluator drops any interpolation with no literal form, which is a silent empty
string for far more than a theme reference. Filed whole as
[23](./23-an-interpolation-with-no-string-form-contributes-nothing.md), which
also closes [15](./15-the-function-map-read-where-it-is-not-a-map.md) case 4.

### The divergence this created, deliberately

`nodes/object_expression.rs` is the general object evaluator, so the refusal
reaches every call that reads an object through it -- and upstream validates a
namespace only in `create`. Six calls beside it were measured. In four, a theme
reference read as a value is *dropped* upstream:

| input | Babel 0.19.0 | here |
| --- | --- | --- |
| `keyframes({ from: { zIndex: zIndex }, … })` | `@keyframes …{from{}to{…}}` | refuses |
| `positionTry({ top: zIndex })` | `@position-try --x {}` | refuses |
| `viewTransitionClass({ group: { zIndex: zIndex } })` | selector, empty body | refuses |
| `createTheme(zIndex, { ten: zIndex })` | compiles, no rule for the key | refuses |

Taken as decided rather than narrowed. Every *other* shape with no value form is
already refused in those four positions by the arm next door, so the theme
reference was the single exception; and the silent drop is the defect this ticket
exists to remove, not a behaviour to keep for four callers out of five. Each is
pinned by a test beside its call and recorded as `acceptance-divergent` with the
reason, so a decision to narrow it later starts from a measurement.

The other two calls that read the same evaluator -- `defineVars` and
`defineConsts` -- refuse in *both* compilers, so nothing changed there. They were
the two positions the first pass of this work neither measured nor tested, found
by review; upstream refuses a `defineVars` value with `Default value is not
defined for a variable.` because the group it folds to has no `default` key. Both
now carry a test, with the member read beside them as the guard. Measuring a
`defineVars` value needs a `*.stylex.js` filename -- otherwise the call refuses
for the filename it cannot hash, and a value question is answered as a path
question. The corpus cannot ask it for that reason: every module subject is
handed the same filename on purpose, so the two rows written for it reported
`both-reject` about the filename and measured nothing. They were dropped and the
limit is written down in the harness README; `defineConsts`, which hashes
nothing, keeps its row.

### Found while measuring, not fixed here

- A theme reference read as a *computed key* is refused here and declares a
  property named after the group's hash upstream (`.x12l9qay{x1q8i56t:1px}`).
  Recorded as a decided divergence; upstream's answer is not a property.
- A theme reference *spread* into a style object compiles to an empty namespace
  upstream and refuses here. The two agree about the CSS and disagree about
  whether saying so is worth stopping for. Recorded, not changed.
- `firstThatWorks(zIndex, 1)` refuses in both, and neither says the same thing:
  the argument check answers first here, the array check upstream.
