# `StyleX Transformers`

## Overview

Placeholder crate for StyleX API transformer implementations. Currently empty --
transformer code remains in `stylex-transform` due to deep `StateManager`
coupling.

## Future Contents

When the `TransformState` trait hierarchy is introduced, this crate will contain
the implementations for `stylex.create`, `stylex.defineVars`,
`stylex.keyframes`, `stylex.createTheme`, `stylex.firstThatWorks`,
`stylex.types`, `stylex.positionTry`, `stylex.viewTransitionClass`, and
`stylex.defineConsts`.
