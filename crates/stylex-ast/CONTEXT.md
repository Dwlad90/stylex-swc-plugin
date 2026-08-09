# stylex-ast

Construction and destructuring of SWC AST nodes. The crate is deliberately
two-sided: nothing here inspects StyleX semantics, it only builds nodes and
takes them apart.

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
