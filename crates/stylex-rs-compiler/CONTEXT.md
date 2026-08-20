# stylex-rs-compiler

The NAPI-RS boundary: the whole pipeline packaged as one Node-callable
`transform`, plus the TypeScript wrapper shipped alongside it. Both sides of the
boundary live here, and the vocabulary is where they have to agree.

## Language

**Transform result**:
`StyleXTransformResult` — `{ code, metadata, map }`. The single return shape
every bundler plugin in `packages/` consumes; a plugin that needs something else
adapts, rather than the boundary growing a variant.
_Avoid_: output, compile result, artifact

**Metadata tuple**:
The serialized form of a
[MetaData](../stylex-types/CONTEXT.md) — `[className, { ltr, rtl? }, priority]`.
An array, not an object, because it is what the JavaScript StyleX ecosystem
already emits.
_Avoid_: rule, style entry, injected style

**Native binding**:
Two artifacts, not one. `napi build` emits the addon itself as
`dist/rs-compiler.<triple>.node`, plus a generated loader `dist/transform.js`
and its `dist/transform.d.ts`; the loader is what picks the right addon for the
host platform. `src/index.ts` imports the loader — never a `.node` directly —
and re-exports its types, so a change to a `#[napi]` struct changes the
published types with no TypeScript edit, and a stale addon makes the JS test
suite meaningless.
_Avoid_: addon, dll, wasm

**Chained map**:
An input source map from earlier tooling, passed as `orig` so the emitted map
describes the authored file rather than the code the transform received.
`sourcesContent` has to be seeded onto it explicitly: on the chained path the
builder returns `orig` verbatim, so nothing it would otherwise have inlined
survives.
_Avoid_: input map, source map, remapping

**Const enum shim**:
The frozen objects `index.ts` exports for `SourceMaps` and
`PropertyValidationMode`. TypeScript erases `const enum`s, so ESM consumers get
no runtime values without them; they are typed as the native enum rather than as
string literals so passing one still typechecks.
_Avoid_: constants, enum export

**File filter**:
`shouldTransformFile` — the include/exclude check applied in TypeScript before
the native call, so a file outside the pattern never crosses the boundary.
_Avoid_: matcher, glob check, guard

**Import elision**:
The type-stripping pass dropping an import specifier nothing in the module
references as a _value_. TypeScript's own rule — such a binding may name a type,
and a type has no module to import at runtime — and wrong for a JavaScript
input, which has no type-only imports to remove. The pass runs between the
resolver and the StyleX transform, so what it elides the transform never sees:
a dynamic style's parameter shadowing an imported name is not a reference, and
until the extension began deciding this the name was gone before anything could
register it. `is_javascript_input` is where the extension decides.
_Avoid_: tree shaking, dead import removal, pruning
