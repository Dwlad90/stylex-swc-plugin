# `StyleX Structures`

## Overview

Core data structures and configuration types for the StyleX system.

Defines the foundational structs and configuration models used across the StyleX
NAPI-RS compiler pipeline.

## Contents

### Plugin context

- `plugin_pass` -- `PluginPass { cwd, filename }` -- per-file plugin context
  carrying the working directory and source file name
- `uid_generator` -- thread-safe unique identifier generator backed by per-key
  `AtomicUsize` counters; used to produce stable, deterministic class-name
  suffixes

### Style representation

- `dynamic_style` -- `DynamicStyle { expression, key, var_name, path }` -- wraps
  a runtime-dynamic SWC expression together with its CSS variable name and
  source path
- `inline_style` -- `InlineStyle { path, original_expression, expression }` --
  tracks an inline style expression and its normalized form alongside the
  property path

### CSS ordering

- `order` -- `Order` trait -- interface for CSS shorthand expansion;
  implementors return an expansion function for a given property name
- `order_pair` -- `OrderPair(String, Option<String>)` -- a (property, value)
  pair produced by shorthand expansion
- `pair` -- `Pair { key, value }` -- generic string key-value pair
- `property_specificity` -- `PropertySpecificity` -- `Order` implementor (stub;
  specificity expansion is resolved in `stylex-css-order`)
- `shorthands_of_shorthands` -- `ShorthandsOfShorthands` -- `Order` implementor
  (stub; expansion is resolved in `stylex-css-order`)

### Configuration

- `named_import_source` -- `NamedImportSource { as, from }`, `ImportSources`
  (Regular | Named), `RuntimeInjection`, `RuntimeInjectionState` -- StyleX
  import and runtime injection configuration types
- `stylex_env` -- `EnvEntry` (Expr | Function), `JSFunction` -- compile-time
  environment variable entries for the `env` plugin option
- `stylex_options` -- `StyleXOptions` -- full user-facing plugin configuration
  struct (dev mode, test mode, styleResolution, aliases, importSources, …)
- `stylex_state_options` -- `StyleXStateOptions` -- per-transformation-pass
  derived options (flattened and resolved from `StyleXOptions`)
