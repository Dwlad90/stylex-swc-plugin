# 09 — An unpaired surrogate aborts a string fold

Status: needs-triage
Phase: Phase 1

**What to build:** A string fold reads its receiver's code units, so a literal
holding an unpaired surrogate refuses instead of aborting the build.

`"\uD83D"` is a legal JavaScript string of length 1. It has no UTF-8 form, so
`convert_atom_to_string` reaches `stylex_panic!(INVALID_UTF8)`:

```rust
pub fn convert_atom_to_string(atom: &Wtf8Atom) -> String {
  match atom.as_str() {
    Some(value) => value.to_string(),
    None => stylex_panic!("{}", INVALID_UTF8),
  }
}
```

Measured, by evaluating each expression and catching the unwind:

| input                    | result       |
| ------------------------ | ------------ |
| `"\uD83D".charCodeAt(0)` | **panics**   |
| `"\uD83D".concat("a")`   | **panics**   |
| `"a" + "\uD83D"`         | refuses      |
| `"\uD83D".length`        | folds to `1` |

This is exactly the family
[02](./02-separate-the-two-kinds-of-evaluation-failure.md) closed for
`evaluate/`, reached one layer down in a convertor that 02's audit did not
cover: the call sites are already `deopt_unsupported!`-shaped, but the convertor
they call aborts before the refusal can be recorded. Put an unpaired surrogate on
the right of an `&&` and the whole build stops, which is
[#1265](https://github.com/Dwlad90/stylex-swc-plugin/issues/1265) again with a
different input.

`length` is the row that already works, because
[01](./01-length-on-a-string-evaluates-to-the-receiver.md) added
`atom_utf16_length`, which counts code units and never needs a scalar. That is
the shape of the fix, not the fix: the other folds need the receiver's _text_,
not its length, so each has to decide whether it can answer at all. `charCodeAt`
can — a code unit is exactly what it returns, and
`stylex_utils::string::char_code_at` already indexes by code unit; it is only the
`&str` it is handed that cannot carry the input. `concat` cannot answer with a
`String` at all.

**Audit `convert_atom_to_string`'s callers before choosing.** There are more than
these two, and the answer differs per caller, so the ticket is the audit plus a
convertor that refuses rather than one that panics.

Not a wrong value — every row above either works or stops the build, so nothing
ships incorrect CSS. It needs no release note, and unlike 01 it can ride along
with a fix that has to go out quickly.
