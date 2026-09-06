# stylex-styleq

Rust port of the runtime [`styleq`](https://github.com/necolas/styleq)
class-name merger. It merges compiled style objects into one `className`.

## Language

**Styleq value**:
Anything a compiled style object can hold at a property — a class name, `null`,
or the `$$css` boolean. The `StyleqValue` trait asks three questions —
`as_class_name`, `is_null`, `is_true_bool` — so a caller can merge its own value
type without converting.
_Avoid_: style value, class value

**Styleq argument**:
One item in the merge list: a style map, a nested list of arguments, or
something to skip. The merge walks the list **right to left**, so the last
argument wins. `cache_key` returns an identity only when the allocation outlives
the cache and no transform is configured; otherwise the key is a hash of the
style, since caching an identity across a transform would serve pre-transform
output.
_Avoid_: input, style arg

**Mix**:
The bookkeeping that lets a non-compiled style suppress a class name an earlier
argument set. `disable_mix` takes such styles off it and merges them into the
inline style whole, so an inline property no longer removes a class name.
_Avoid_: merge, override pass

**Class-name chunk**:
The run of class names one compiled style argument contributes, built fresh per
argument and prepended to the result. `dedupe_class_name_chunks` decides whether
a repeated chunk is dropped, by a **substring** test rather than a whole-run
comparison. It defaults to `false`, but the compile-time caller in
[stylex-transform](../stylex-transform/CONTEXT.md) sets it `true`.
_Avoid_: class token, class fragment
