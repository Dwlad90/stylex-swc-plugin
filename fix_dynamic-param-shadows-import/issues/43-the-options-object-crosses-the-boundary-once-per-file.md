# The options object crosses the NAPI boundary once per file

Status: triage/perf

## What

Every `transform()` call deserializes the whole `StyleXOptions` object out of
JavaScript — around forty fields, including `importSources`, `aliases`,
`unstable_moduleResolution` and a walk of the `env` `JsObject` — converts it
through `StyleXOptionsParams` into `CoreStyleXOptions`, and throws the result
away. `crates/stylex-rs-compiler/src/lib.rs:202-299`.

The value is constant for an entire build. A project with four thousand modules
marshals it four thousand times.

`input_source_map` has the same shape of cost for a different reason: it
round-trips through a JSON `String` per file.

## Why it is filed rather than fixed

Found during the review of
`fix_dynamic-style-parameter-shadowing-an-imported-binding`, which adds one
scalar to that object and does not make the situation worse. Fixing it is an
API change to the package's entry point, which does not belong in a branch
about reference resolution.

## Shape of a fix

A `#[napi]` struct created once with the options and exposing
`compile(filename, code)`, so the options are marshalled and converted at
construction. That also gives `input_source_map` somewhere to live other than a
per-call string.

The measurement to take first is what the marshalling actually costs per file
against a real corpus — `module_path_bench` already prices the fixed
per-transform overhead a module pays before it imports anything, and this would
be one more candidate in the same shape.
