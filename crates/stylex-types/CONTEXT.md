# stylex-types

The types that describe compiler _output_: a generated rule, its class name, and
the metadata a host needs to inject it. Everything here is serialized and
crosses into JavaScript, so field names are an interface.

## Language

**Serialized value**:
An authored value written back out as the JavaScript source it becomes, via
`serialize_value_to_json_string`. Not the same as writing it out as JSON: a
value authored as a string is already the source it has to stay, so the quotes
JSON adds come back off, a string that spells a number is that number, and a
string holding a JavaScript object literal is repaired into JSON rather than
emitted as one long escaped string. The empty string is the one exception and
keeps its quotes.
_Avoid_: JSON value, stringified value, dumped value

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

**When marker value**:
`WhenMarkerValue` — the interface [stylex-css](../stylex-css/CONTEXT.md) needs
from whatever occupies the second slot of a `when.*` call. That slot holds
either the options or a marker in one of three shapes: a class-name string, an
import proxy standing in for a marker defined in another file, or a compiled
`$$css` style object. Each accessor answers one of those shapes and yields
nothing when it does not apply, so the marker resolution stays a direct
translation of its JavaScript original. It exists because the evaluated values
live above the CSS layer, which therefore cannot name them.
_Avoid_: marker trait, marker source
