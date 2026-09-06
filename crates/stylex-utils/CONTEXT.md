# stylex-utils

Small helpers over SWC AST, strings, collections and hashing. It is a leaf: no
StyleX domain concept is defined here, only the machinery the domain crates
share.

## Language

**Hash**:
A base-36 murmur2 digest of a string, via `create_hash`, with **seed 1**. Class
names and variable names are built from it, so its output is user-visible and
stable across runs. The digest is taken over the string's **UTF-16 code units,
each masked to its low byte**, not over its UTF-8 bytes: that is how the
upstream `murmurhash2_32_gc` is defined, and the two agree only while the input
is ASCII. Changing the encoding or the seed renames every generated class.
_Avoid_: digest, checksum, id

**Short hash**:
`create_short_hash` — the same murmur2 value reduced modulo `62^5` and written
in base 62, used for the object keys of a compiled style. Zero encodes as the
empty string rather than `"0"`, matching `toBase62`; `to_base36` does write
`"0"`.
_Avoid_: small hash, truncated hash

**Key hash**:
`create_key_hash(namespace, key)` — the hash of `namespace.key`, in that order,
which is how a style within a `stylex.create` namespace gets its class name.
_Avoid_: style hash, namespace hash

**Blank CSS text**:
A string that spells no CSS value at all, via `is_blank_css_text`. The test is
every byte at or below 32, so a C0 control is blank as well as a space. A
property whose value is blank CSS text is left undeclared, because `color:` is
not something a browser accepts. Named against
[CSS text](../stylex-structures/CONTEXT.md), which is what a value spells when
it is not blank.
_Avoid_: empty value, blank string, whitespace check

**JS float read**:
`parse_js_float`, which reads a leading float out of a string exactly as
JavaScript's `parseFloat` does: the longest leading number wins and what trails
it is ignored, which is what makes `10px` yield `10`. Rust's `str::parse` is not
a substitute — it rejects any trailing character. It reports `None` where
JavaScript yields NaN, so "there is no number here" cannot pass for zero.
_Avoid_: float parsing, `parse_f64`, string-to-number

**JS number spelling**:
`to_js_string`, which writes an `f64` back out exactly as JavaScript's
`String(Number)` does, switching to exponential form at boundaries Rust's
`Display` never does.
_Avoid_: number formatting, float display

Both are matched to JavaScript because the spelling is **observable**: a
normalized CSS value reaches the stylesheet and feeds the **Hash** above, so a
float read or written one digit differently produces a different class name.
Their cases are generated from a JavaScript runtime by
`scripts/generate-parse-float-cases.mjs` and never written by hand.

**JS string quoting**:
`json_stringify`, which renders a `&str` exactly as JavaScript's
`JSON.stringify` renders a string. Rust's `{:?}` is not a substitute: it spells
a C0 control as `\u{1}` where JSON spells `\u0001`, and it escapes an apostrophe
JSON leaves alone. This spelling reaches no stylesheet, only a **rejection
message** an author reads under `propertyValidationMode` — and upstream builds
those by interpolating `JSON.stringify`, so a value quoted differently here is a
divergence an author sees.
_Avoid_: string escaping, quoting, debug format

**Structural hash**:
`stable_hash_unspanned`, a **128-bit** xxh3 hash of an AST expression that
ignores spans, so two identical expressions in different source positions
collide on purpose. Some callers narrow a candidate set and then confirm with
`eq_ignore_span`; the evaluator's memo and the before-declaration injection slot
act on a hash hit alone, so for those the key _is_ the equality test, and its
width is what stands between a collision and a wrong folded value.
`stable_hash_wide` is the same 128 bits over anything `Hash`. Past
`MAX_UNSPANNED_HASH_COLLECTION_LEN` a collection is hashed through a `drop_span`
clone instead.

The hash walks the whole subtree and the evaluator takes one per level, which is
why folding a deep expression is quadratic
([ADR 0005](../stylex-evaluator/docs/adr/0005-the-memo-key-is-a-whole-subtree-hash.md)).
_Avoid_: ast hash, expression id

**File-based identifier**:
`gen_file_based_identifier(file, export, key)` — the `file//export.key` string
that names one export of one file, so that two files exporting the same name
never collide. The un-hashed sibling of **Key hash**: this one stays readable,
and is hashed later where a short name is needed.
_Avoid_: qualified name, export id, var key

**Node kind**:
`get_expr_node_kind`, the ESTree name of an expression node —
`"CallExpression"`, `"BigIntLiteral"`. It is what a diagnostic calls the thing
the author wrote, so it is **observable**: the names are the ecosystem's rather
than SWC's variant names, and three resolve a disagreement about node boundaries
(a logical operator is a `LogicalExpression`, an optional chain is named by its
base, `super.x` is a `MemberExpression`). `get_stmt_node_kind` is the statement
half. Not `Expr::get_type`, which answers the _value_ an expression produces.
_Avoid_: expression type, node type, ast kind
