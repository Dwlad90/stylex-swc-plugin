# stylex-types

The types that describe compiler _output_: a generated rule, its class name, and
the metadata a host needs to inject it. Everything here is serialized and
crosses into JavaScript, so field names are an interface.

## Language

**Injectable style**:
One generated rule ready to be injected — `ltr`, an optional `rtl`, and a
[priority](../stylex-constants/CONTEXT.md). The `Const` variants additionally
carry the `const_key`/`const_value` pair that `defineConsts` produced.
_Avoid_: rule, css rule, style object

**Metadata**:
`MetaData` — the triple `(class name, injectable style, priority)` that is
handed to the host for injection. This is what a bundler plugin receives and
what a snapshot test compares.
_Avoid_: injected style, css metadata, output

**Class name**:
`ClassName`, a newtype over the generated atomic class string. Newtyped rather
than aliased so it cannot be swapped with a rule key by accident.
_Avoid_: css class, atom, selector

**Rule key**:
`RuleKey`, a newtype over the key a rule is deduplicated under. Two rules with
the same key are the same rule regardless of where they were authored.
_Avoid_: style key, hash, id

**Style options trait**:
`StyleOptions` — the interface the CSS layer needs from whatever is holding
state: the resolved options, the seen-property map, and the injected-rules map.
It exists so [stylex-css](../stylex-css/CONTEXT.md) can be given the transform's
`StateManager` without depending on it.
_Avoid_: state trait, context trait

**When marker value**:
`WhenMarkerValue` — the interface [stylex-css](../stylex-css/CONTEXT.md) needs
from whatever occupies the second slot of a `when.*` call. That slot holds
either the options or a marker in one of three shapes: a class-name string, an
import proxy standing in for a marker defined in another file, or a compiled
`$$css` style object. Each accessor answers one of those shapes and yields
nothing when it does not apply, so the marker resolution stays a direct
translation of its JavaScript original. It exists for the same reason as
`StyleOptions`: the evaluated values live in the transform crate, which the CSS
layer sits below.
_Avoid_: marker trait, marker source
