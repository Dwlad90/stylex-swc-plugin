# `stylex-types`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

Injectable style types and metadata structures for the StyleX compiler. This
crate defines the `InjectableStyle` family of structs and enums, the `MetaData`
output type, and the `WhenMarkerValue` trait that lets the layers below the
evaluator read a `stylex.when` marker. It is a separate crate so that the six
crates that need compiled-style representations do not depend on transform
logic.

- **Injectable styles** — `InjectableStyle`, `InjectableConstStyle` and their
  `Base` counterparts provide LTR/RTL CSS content with optional priority and
  const-variable tracking
- **Enum wrappers** — `InjectableStyleKind` and `InjectableStyleBaseKind`
  distinguish regular styles from const-referencing styles
- **Metadata** — `MetaData` pairs a CSS class name with its injectable style and
  priority, supporting custom serialisation
- **Trait interface** — `WhenMarkerValue` is an object-safe trait that reads the
  second argument of `stylex.when.*` without naming the evaluated-value types
- **Type alias** — `InjectableStylesMap`
  (`IndexMap<RuleKey, Rc<InjectableStyleKind>>`) provides ordered,
  reference-counted style storage with typed lookup keys

## Architecture

### `WhenMarkerValue` Trait

The `WhenMarkerValue` trait solves a layering problem: `stylex-css` must test
what the second argument of a `stylex.when.*` call is, but it sits below the
evaluator and cannot name the evaluated-value types.

```text
┌──────────────┐         ┌──────────────────────┐
│  stylex-css  │──uses──▶│  dyn WhenMarkerValue │
└──────────────┘         │  (object-safe trait) │
                         └──────────┬───────────┘
                                    │ implements
              ┌─────────────────────┴─────────────────┐
              │                                       │
  ┌───────────▼──────────┐            ┌───────────────▼──────────┐
  │  StyleXStateOptions  │            │  EvaluateResultValue     │
  │  (here)              │            │  (stylex-state)          │
  └──────────────────────┘            └──────────────────────────┘
```

Key methods on the trait:

- `as_str_value(&self)` — the marker written as a literal class name
- `is_proxy(&self)` / `as_proxy_string(&self)` — an import proxy, tested and
  then resolved
- `first_css_key(&self)` — the class name a compiled `$$css` marker carries
- `class_name_prefix(&self)` — present only on the options, so it tells a
  marker from the options

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
