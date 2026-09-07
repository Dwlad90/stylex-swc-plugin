# Dynamic style functions in nested scopes

Status: done

Tracks GitHub issue
[#1303](https://github.com/Dwlad90/stylex-swc-plugin/issues/1303). Spec only --
no ticket files. Branch: `fix_dynamic-functions-become-non-callable-in-nested-scopes`,
cut from `develop`.

Reference: `~/Projects/Facebook/stylex` @ `@stylexjs/babel-plugin` 0.19.0,
`src/visitors/stylex-create.js` and `src/utils/ast-helpers.js`.

## Context

A dynamic `stylex.create` entry declared inside a nested scope compiles to a
plain object. The call site then fails at run time:

```ts
import * as stylex from '@stylexjs/stylex';

export namespace Demo {
  const styles = stylex.create({
    color: (value: string) => ({ color: value }),
  });

  export function render() {
    return stylex.props(styles.color('red'));
  }
}
```

```
TypeError: styles.color is not a function
```

Reproduced on `HEAD` (ec63fb87c) with `dev: false, runtimeInjection: false`.
Function, IIFE and block scopes reproduce it too. Module scope does not.

## Cause

`transform_stylex_create_call/mod.rs` hoists the compiled object with
`path_replace_hoisted` before it applies `apply_dynamic_style_functions`. A
nested call is hoisted to an identifier, and the helper only rewrites an object
expression, so every dynamic entry keeps its compiled static shape.

The reference visitor maps the `fns` over the result object first, registers
the styles, and hoists once at the end. The static fragments of each dynamic
entry are hoisted from inside the `fns` branch.

## Change

- Apply `apply_dynamic_style_functions` to the compiled object, then hoist the
  result once in the caller.
- Remove the helper's final `path_replace_hoisted` and its `is_program_level`
  parameter. Keep the hoist of static fragments inside the helper.
- `register_styles` keeps its position. It receives the hoisted identifier and
  the dynamic-applied object as the fallback.
- `style_map` and `dynamic_style_namespaces` stay program-level only, as
  upstream does with `isTopLevel`.

## Acceptance criteria

- [x] Snapshot tests in `transform_stylex_create_test/dynamic_styles.rs` for
      a namespace, a function, an IIFE and a block scope, with a static
      sibling and a runtime-injection variant, match the reference output.
- [x] A vitest spec in `crates/stylex-rs-compiler/__test__/` compiles the
      namespace and IIFE cases, executes the output, and asserts that the
      dynamic entry is callable, returns per-argument values, and leaves the
      static sibling intact.
- [x] `pnpm test:crates:workspace`, `pnpm test`, `pnpm typecheck`,
      `pnpm lint:all` and `pnpm format:check` pass.

## Comments

- 2026-09-07: Done in 2789cf914 (fix) and dc229267c (execution spec). Two
  pre-existing follow-ups are recorded in ticket 01: the nested-scope `$$css`
  debug line number, and the loop-invariant hashes in `register_styles`.
- 2026-09-08: Follow-ups closed in 4bcfa0ec0, ca89d1313, c00bb72d1,
  37dca4ce6 and 197f446bd. See the comments in tickets 01 and 02.
