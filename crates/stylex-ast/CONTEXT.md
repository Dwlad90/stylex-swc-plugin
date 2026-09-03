# stylex-ast

Construction and destructuring of SWC AST nodes. The crate is deliberately
two-sided: nothing here inspects StyleX semantics, it only builds nodes, takes
them apart, and reads the names written on them.

## Language

**Factory**:
A `create_*` function that builds an AST node — `create_key_value_prop`,
`create_object_expression`. Every node the compiler emits is built through one,
so spans and contexts are set in exactly one place.
_Avoid_: builder, constructor, helper

**Convertor**:
A `convert_*` function that reads a value back out of an AST node —
`convert_lit_to_string`, `convert_key_value_to_str`. The inverse direction to a
factory, and the two are named as a pair on purpose.
_Avoid_: extractor, parser, getter, reader

**Key reader**:
A `*_from_prop_key` / `*_from_member_prop` function that answers the name a
property or a member is written under — `namespace_name_from_prop_key`,
`collect_object_lit_keys`. Distinct from a convertor, which reads a _value_: a
key is written in five shapes that all name the same property at run time, so a
reader collapses them to one `Atom` and answers with nothing where the shape
carries no static name. The `namespace_name_` prefix says which name is being
asked for, not that the crate knows what a namespace is — nothing here reads
what the name means.
_Avoid_: key extractor, key convertor, name getter

**Own-key order**:
The order JavaScript enumerates an object's own properties in, applied by
`order_own_keys`: every **array-index key** first in ascending numeric order,
then every other key in insertion order. Not a spelling detail — the order the
properties come out in is the order their declarations reach the stylesheet, so
it decides which of two rules at equal specificity wins.

The same order applies to anything else that stands for a JavaScript object,
whether or not it is a property list. A `create` call's namespace map is one,
and `order_own_map_keys` applies the order to it. The two share the rule and not
the mechanics: a list is split in one pass, and an ordered map, which cannot be
split where it stands, is sorted.
_Avoid_: sort keys, property order, key ordering

**Array-index key**:
A key that JavaScript counts as an array index: the canonical decimal spelling
of an integer below 2^32 - 1. Canonical is what makes `0` one and `00`, `01` and
`+0` not — those round-trip to a different string, so the language enumerates
them in insertion order as ordinary string keys.
_Avoid_: numeric key, index, integer key

**Synthesized node**:
An AST node this compiler built rather than read, carrying `DUMMY_SP` because no
source text spells it. Shorthand expansion and injected function mappers both
produce them. Every question answered from a position has to exempt them: byte
zero sorts before every authored node, so comparing one answers a fact about its
having been built.
_Avoid_: generated node, dummy node, fake node
