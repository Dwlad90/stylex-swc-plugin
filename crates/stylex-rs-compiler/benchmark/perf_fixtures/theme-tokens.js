/**
 * The theming surface read back: two `createTheme` calls overriding the same
 * variable group, the consts inlined at compile time, and a style object
 * reading tokens, consts and an imported token group together.
 *
 * `defineVars` and `defineConsts` live in `tokens.stylex.js`, since both
 * require a `.stylex.js` module; what is measured here is the theme transformer
 * and the member-expression resolution that reads an imported group.
 */

import * as stylex from '@stylexjs/stylex';
import { colors } from './colors.stylex';
import { spacing, tokens } from './tokens.stylex';

export const dark = stylex.createTheme(tokens, {
  accent: '#8ab4f8',
  surface: '#111318',
  radius: '10px',
  gutter: spacing.loose,
});

export const compact = stylex.createTheme(tokens, {
  gutter: spacing.tight,
  radius: '2px',
  fontSize: {
    default: '0.875rem',
    '@media (min-width: 768px)': '0.9375rem',
  },
});

export const styles = stylex.create({
  card: {
    backgroundColor: tokens.surface,
    color: tokens.accent,
    borderRadius: tokens.radius,
    padding: tokens.gutter,
    gap: spacing.cosy,
    fontSize: tokens.fontSize,
    borderColor: { default: tokens.accent, ':hover': colors['--accent'] },
  },
});
