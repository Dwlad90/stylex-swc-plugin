# 03 — Refuse a reference used before its declaration

Status: `resolved`
Blocked by: 02 — a textual gate only: both edit the identifier path in the same
function. The two are logically independent and could land in either order.

**What to build:** A style value that reads a binding declared *later* in the
module fails the build, instead of quietly compiling to CSS the reference
implementation refuses.

```js
export const styles = stylex.create({ a: { color: c } });
const c = 'red';
```

The reference implementation refuses this with `Referenced value is used before
declaration.` We emit `.x…{color:red}`. This is the only case in the whole
identifier audit where the divergence is *wrong output* rather than a differing
refusal, which is why it is its own ticket: a bisect deserves to land on it.

The cause is that declarations are collected module-wide with no position check,
so an initializer is inlined at a use site that precedes it. The reference
implementation compares the reference's start offset against the binding's end
offset. Mirror that with the parser's byte positions, ahead of the initializer
read.

**Skip the check when either span is dummy.** A synthesized node — shorthand
expansion produces them — has no authored position, so any comparison against it
is meaningless. Skipping degrades to today's behaviour for those nodes rather
than inventing a refusal.

The message text is already written and commented out beside the other
evaluation errors, byte-identical to the reference implementation's output;
revive it rather than composing a new string.

- [x] The example above fails with the reference implementation's exact text
- [x] A binding declared *before* the `create()` call still inlines
- [x] A synthesized node with no authored position is unaffected
- [x] Corpus entry with the verdict it is known to read
- [x] A unit test covering the dummy-span skip, which the corpus cannot reach

## Comments

Landed as `05c8e32b9`.

`reads_before_its_declaration` in `js/evaluate/mod.rs` compares
`reference.span.lo < declarator.span.hi`, off the declarator
`get_var_decl_by_ident` already returns, so the check costs a comparison rather
than a second scan of the declaration list. The `NON_CONSTANT` probe above it is
untouched, ordering rationale included.

Measured against `@stylexjs/babel-plugin` 0.19.0 directly, six inputs, all
agreeing:

| input | Babel 0.19.0 | rs-compiler now |
| --- | --- | --- |
| `create({a:{color:c}})` then `const c` | refuses | refuses, same text |
| the same pair in program order | folds | folds |
| `const c = c` then a read of `c` | refuses | refuses |
| `create({a:{margin:m}})` then `const m` | refuses | refuses |
| `props(redTheme)` above `const redTheme` | runtime call | runtime call |
| `props(styles.a)` above `const styles` | folds to a class | folds to a class |

The last two are why `tests/fixture/buttons-demo/output.js` changed: a bare theme
reference read above its declaration now falls to the runtime, which is what the
reference compiler has always emitted there, and a member read of a later
`create()` still folds. Both are recorded as corpus entries so the asymmetry is
measured. Six entries added to `parity/corpus/modules.json`; `--set modules`
reports 0 changed verdicts over 55 subjects.

Two glossary terms added to `crates/stylex-transform/CONTEXT.md` -- **Early
reference**, beside its sibling **Binding write**, and **Synthesized node**, the
thing exempt from every position question. The seam name ticket 04 owes that file
is a separate entry.

### Left undone, deliberately

`parity/corpus/harvested.json` is stale, and was already stale at `b69f28d94`:
regenerating at HEAD adds six rows, and regenerating with this ticket's tests
applied adds the same six and no more -- the new tests carry identifier values,
which the harvest scan does not take. Regenerating here would fold six rows of
someone else's drift into this commit and rewrite
`crates/postcss-value-parser/src/tests/cases.rs` wholesale. Worth its own commit.

Done separately in `32554e1a2`.

### Noted for ticket 04

Upstream's comparison fires for *any* binding, hoisted `function` and `class`
declarations included; `get_var_decl_from` only sees `VarDeclarator`s, so those
still fall through to `check_ident_declaration`. That is what the spec asks for
(step 8 keeps `check_ident_declaration` as-is, and the corpus lists
function/class references among the agreeing cases), and it is now written down
in `reads_before_its_declaration`'s doc comment rather than only here.
