# stylex-utils

Small helpers over SWC AST, strings, collections and hashing. It is a leaf: no
StyleX domain concept is defined here, only the machinery the domain crates
share.

## Language

**Hash**:
A base-36 murmur2 digest of a string, via `create_hash`. This is what class
names and variable names are built from, so its output is user-visible and
stable across runs — changing the algorithm changes every generated class name.
The digest is taken over the string's **UTF-16 code units, each masked to its
low byte**, not over its UTF-8 bytes: that is how `murmurhash2_32_gc` is
defined, and the two encodings agree only while the input is ASCII. Hashing
bytes instead silently produces a different class name for identical CSS.
_Avoid_: digest, checksum, id

**Short hash**:
`create_short_hash` — the same murmur2 value reduced modulo `62^5` and written
in base 62, used for the object keys of a compiled style. Zero encodes as the
empty string rather than `"0"`, matching `toBase62`.
_Avoid_: small hash, truncated hash

**Key hash**:
`create_key_hash(namespace, key)` — the hash of `namespace.key`, which is how a
style within a `stylex.create` namespace gets its class name.
_Avoid_: style hash, namespace hash

**Blank CSS text**:
A string that spells no CSS value at all — empty, or nothing but whitespace —
via `is_blank_css_text`. Asked of an authored value and of the value it
transforms to, so it lives here rather than on either type. A property whose
value is blank CSS text is left undeclared, because `color:` is not something a
browser accepts. Named against
[CSS text](../stylex-structures/CONTEXT.md), which is what a value spells when
it is not blank.
_Avoid_: empty value, blank string, whitespace check

**JS float read**:
`parse_js_float`, which reads a leading float out of a string exactly as
JavaScript's `parseFloat` does: the longest leading number wins and whatever
trails it is ignored, which is what makes `10px` yield `10`. Rust's
`str::parse` is not a substitute — it rejects any trailing character. Reports
`None` where JavaScript yields NaN, so "there is no number here" cannot pass
for zero.
_Avoid_: float parsing, `parse_f64`, string-to-number

**JS number spelling**:
`to_js_string`, which writes an `f64` back out exactly as JavaScript's
`String(Number)` does, switching to exponential form at the same boundaries
Rust's `Display` never does.
_Avoid_: number formatting, float display

Both are matched to JavaScript because the spelling is **observable**: a
normalized CSS value reaches the emitted stylesheet and feeds the **Hash**
above, so a float read or written one digit differently silently produces a
different class name. Their expectations are generated from a JavaScript
runtime by `scripts/generate-parse-float-cases.mjs` and never written by hand.

**JS string quoting**:
`json_stringify`, which renders a `&str` exactly as JavaScript's
`JSON.stringify` renders a string. `serde_json` does the escaping and the two
agree byte for byte, which is the whole claim the tests exist to hold. Rust's
`{:?}` is not a substitute: it spells a C0 control `\u{1}` where JSON spells
`\u0001`, and escapes an apostrophe JSON leaves alone.
_Avoid_: string escaping, quoting, debug format

Observable for a different reason than the two above: this spelling never
reaches a stylesheet, only a **rejection message** an author reads under
`propertyValidationMode`. Diagnostics that quote a value are built upstream by
interpolating `JSON.stringify`, so a value quoted differently here is a
divergence an author sees even when both compilers refuse the same input.

**Structural hash**:
`stable_hash_unspanned`, a hash of an AST expression that ignores spans, so two
syntactically identical expressions in different source positions collide on
purpose. Callers use it to narrow a candidate set and then confirm with
`eq_ignore_span` — the hash never decides equality by itself.
_Avoid_: ast hash, expression id

**Node kind**:
`get_expr_node_kind`, the ESTree name of an expression node —
`"CallExpression"`, `"ArrowFunctionExpression"`, `"BigIntLiteral"`. It is what
a diagnostic calls the thing the author wrote, so it is **observable**: the
names are the ecosystem's rather than SWC's variant names, and three of them
resolve a disagreement about node boundaries (a logical operator is a
`LogicalExpression`, an optional chain is named by its base, `super.x` is a
`MemberExpression`). Not to be confused with `Expr::get_type`, which answers
the _value_ an expression would produce and is `Unknown` for everything a
static evaluation cannot fold — precisely the cases a diagnostic is written
for.
_Avoid_: expression type, node type, ast kind
