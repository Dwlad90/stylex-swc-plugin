# stylex-styleq

Rust port of the runtime [`styleq`](https://github.com/necolas/styleq)
class-name merger. It merges compiled style objects into one `className`, so its
vocabulary is the runtime's, not the compiler's.

## Language

**Styleq value**:
Anything a compiled style object can hold at a property — a class name, `null`,
or the `$$css` boolean. Modelled as the `StyleqValue` trait rather than a
concrete enum, so a caller can merge its own value type without converting.
_Avoid_: style value, class value

**Styleq argument**:
One item in the merge list: a style map, a nested list of arguments, or
something to skip. The `StyleqArgument` trait, whose `cache_key` returns an
identity only when the allocation outlives the cache.
_Avoid_: input, style arg

**Mix**:
The pass that lets a later argument override an earlier one property by
property, as against concatenating class names. Disabled by `disable_mix`.
_Avoid_: merge, override pass

**Class-name chunk**:
A whitespace-separated run in a value that is itself several class names.
`dedupe_class_name_chunks` decides whether a repeated chunk is dropped.
_Avoid_: class token, class fragment
