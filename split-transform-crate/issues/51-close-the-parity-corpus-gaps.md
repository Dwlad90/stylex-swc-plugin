# 51 — Close the parity-corpus gaps outside the evaluator

**What to build:** Five shapes the Rust suites do not hold a case for, found
while reviewing [ticket 15](./15-cover-the-evaluator-crate.md). None of them
belongs to `stylex-evaluator`, which is why they are a ticket rather than
branch work. Each is a shape only the child-process parity harness would catch
today, and the harness runs per pull request rather than per commit.

1. **A framed declaration preceded by an astral character**
   (`crates/stylex-rs-compiler/parity/corpus/positions.json`).
   `stylex-diagnostics`' `code_frame_test.rs` pins CRLF and a byte-order mark
   but no character outside the Basic Multilingual Plane, so a column counted
   in bytes rather than in UTF-16 code units is caught nowhere in Rust.
2. **A condition key as the whole style value**
   (`modules-null-as-a-whole-condition-value`, expected to diverge
   structurally). Rust holds only the property-wrapped form.
3. **`codePointAt`.** Zero occurrences in `stylex-evaluator` or `stylex-js`.
   It belongs beside `charCodeAt` in
   `crates/stylex-evaluator/src/evaluate/tests/unsupported_shape_tests.rs`.
4. **`hyphenateCharacter` bypassing normalization.** Zero hits in any crate.
5. **Three degenerate whole values with no Rust row**: `color: '\u{3000}'`,
   `color: '\u{a0}'` and `color: '\u{1}'`.

**Also**, the `lone surrogate in a name` refusal family holds exactly two rows,
both written by hand in `corpus/modules.json`. A family of two rows is one
deletion away from failing the unreached-family gate, which is the failure mode
recorded in
[ticket 23](./23-fix-the-parity-harvest.md): fixing the harvester starved a
family and the gate reported the fix as the fault.

**Blocked by:** None.

**Status:** resolved

## Comments

**2026-09-13, resolved.** All five gaps hold a Rust case now, and the refusal
family is no longer one deletion from starving.

1. **An astral character before a framed declaration** --
   `a_declaration_after_an_astral_character_is_framed_on_its_own_line` in
   `stylex-diagnostics`. The run beside it is all `λ`, which is two bytes and
   one UTF-16 unit, so it separated bytes from characters and nothing else. An
   astral character is four bytes, two units and one character, which is the one
   that separates all three.
2. **A condition key as the whole style value** --
   `a_condition_key_as_the_whole_value_declares_nothing`, with the structural
   divergence written beside it.
3. **`codePointAt`** -- `code_point_at_reads_the_whole_code_point`, beside
   `charCodeAt`, including the row where the two part: the first unit of an
   astral character.
4. **`hyphenateCharacter` bypassing normalization** --
   `keeps_a_hyphenate_character_value_as_it_was_written`, both spellings of the
   key.
5. **The three degenerate whole values** --
   `declares_a_value_that_only_looks_blank` for the two spaces the language does
   not call whitespace, and `declares_nothing_for_a_control_character` for the
   third. The reference implementation fails inside its own value parser on that
   one, reading a property of `undefined`, so there is no answer of its to agree
   with.

**The `lone surrogate in a name` family had already lost a row**, which is the
failure ticket 23 records and this ticket predicted. Ticket 49 replaced the
`String value contains invalid UTF-8 encoding.` refusal for a key with `The key
has no name at compile time.`, and the family claims its rows by that sentence.
It now claims either, because a name is decoded in two places -- where the text
is read, and where a key is named -- and the reason is the same for both. Two
rows were added beside the existing two.
