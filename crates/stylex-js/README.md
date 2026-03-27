# `StyleX JS`

## Overview

JavaScript evaluation helpers for the StyleX system.

Provides utilities for evaluating and processing JavaScript expressions within
the StyleX NAPI-RS compiler.

## Contents

- `helpers` -- JS expression inspection utilities:
  - `is_valid_callee` / `get_callee_name` -- validates and extracts the name of
    a static identifier callee (checks against `VALID_CALLEES`)
  - `is_invalid_method` -- checks whether a member property is in the
    `INVALID_METHODS` deny-list
  - `is_mutating_object_method` / `is_mutating_array_method` -- detects
    `Object.assign`, `Array.push`, `Array.splice`, and similar mutating built-in
    calls
  - `is_mutation_expr` -- returns `true` for assignment to a member expression,
    update expressions on a member, or `delete` operations
  - `get_method_name` -- extracts the method name from a member expression
  - `is_id_prop` -- checks whether a member property is a plain identifier
