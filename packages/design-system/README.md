# @stylexswc/design-system

A design system built with StyleX for workspace examples, providing reusable
components and design tokens.

## Installation

This package is private and intended for internal use within the StyleX
workspace examples.

```bash
pnpm add @stylexswc/design-system
```

## Usage

### Importing Components

```tsx
import { Text } from '@stylexswc/design-system';
```

### Using Design Tokens

```tsx
import tokens from '@stylexswc/design-system/tokens.stylex';

// Use tokens in your StyleX styles
const styles = stylex.create({
  container: {
    backgroundColor: tokens.pink7,
  },
});
```
