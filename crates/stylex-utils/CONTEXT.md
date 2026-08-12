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

**Structural hash**:
`stable_hash_unspanned`, a hash of an AST expression that ignores spans, so two
syntactically identical expressions in different source positions collide on
purpose. Callers use it to narrow a candidate set and then confirm with
`eq_ignore_span` — the hash never decides equality by itself.
_Avoid_: ast hash, expression id
