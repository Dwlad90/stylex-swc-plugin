# `StyleX Logs`

## Overview

Logging utilities for the StyleX NAPI-RS compiler.

## Contents

- `constants` -- `STYLEX_PREFIX` branded prefix
- `formatter` -- Custom log formatting with ANSI colors and `[StyleX]`
  prefix, color-coded log levels
- `initializer` -- One-time logger and panic hook initialization with
  `STYLEX_DEBUG` environment variable override

## Layer

Layer 0. Dependencies: `stylex-macros`, `ctor`, `colored`, `log`,
`pretty_env_logger`, `env_logger`.
