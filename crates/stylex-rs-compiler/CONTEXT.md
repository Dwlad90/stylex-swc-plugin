# stylex-rs-compiler

The NAPI-RS boundary: the whole pipeline packaged as one Node-callable
`transform`, plus the TypeScript wrapper shipped alongside it. Both sides of the
boundary live here, and the vocabulary is where they have to agree.

## Language

**Transform result**:
`StyleXTransformResult` — `{ code, metadata, map }`. The single return shape
every bundler plugin in `packages/` consumes.
_Avoid_: output, compile result, artifact

**Metadata tuple**:
The serialized form of a
[MetaData](../stylex-types/CONTEXT.md) — a three-element array
`[className, { ltr, rtl }, priority]`. `rtl` is always set, and is `null` where
the rule has no mirrored counterpart. A `Const` entry carries `constKey` and
`constVal` as well. An array rather than an object, for parity with the
JavaScript StyleX ecosystem.
_Avoid_: rule, style entry, injected style

**Native binding**:
Three build artifacts, not one. `napi build` emits the addon as
`dist/rs-compiler.<platform>-<arch>.node`, plus a generated loader
`dist/transform.js` and its `.d.ts`, which picks the right addon for the host.
`build:ts` then emits the wrapper `dist/index.js`, the package `main`.
`src/index.ts` imports the loader, never a `.node` directly, and the JS test
suite imports `dist/index.js` — so a stale addon or a stale wrapper makes that
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
`PropertyValidationMode`, because TypeScript erases `const enum`s and ESM
consumers would get no runtime values. Both are `Object.freeze`d and keyed
`Throw` / `Warn` / `Silent` style, but they differ in what they hold:
`SourceMaps` is typed as the native enum, so passing one still typechecks, and
its values are `'True'`, `'False'` and `'Inline'`; `PropertyValidationMode` is
`as const` over the lowercase `'throw'`, `'warn'` and `'silent'`.
_Avoid_: constants, enum export

**File filter**:
`shouldTransformFile` — the include/exclude check applied in TypeScript before
the native call, so a file outside the pattern never crosses the boundary.
_Avoid_: matcher, glob check, guard

**Import elision**:
The type-stripping pass that drops an import specifier nothing in the module
references as a _value_. It is TypeScript's own rule and wrong for a JavaScript
input, which has no type-only imports to remove, so `is_javascript_input`
decides. The pass runs between the resolver and the StyleX transform, so what it
elides the transform never sees. See
[ADR 0001](./docs/adr/0001-a-typescript-module-reads-an-unreferenced-import-as-a-type.md).
_Avoid_: tree shaking, dead import removal, pruning

**Target allocator**:
The `#[global_allocator]` a published `.node` links, which is a property of the
target rather than of the code. `swc_malloc` chooses one for six of the seven
targets `napi.targets` lists and declines every musl target, so
`x86_64-unknown-linux-musl` names its own. `src/tests/allocator_tests.rs` holds
the declaration to the published list, and the workspace manifest carries the
measurement. Adding a musl target is a decision, not a copied line: mimalloc
does not work on ARM64 musl.
_Avoid_: memory allocator setting, malloc flag, mimalloc feature
