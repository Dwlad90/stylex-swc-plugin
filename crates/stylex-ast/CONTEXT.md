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
