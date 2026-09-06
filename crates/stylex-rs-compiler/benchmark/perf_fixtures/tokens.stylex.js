/**
 * A token file for the theming fixtures: `defineVars` and `defineConsts` both
 * require a `.stylex.js` module, so the two transformers that own them are
 * measured here and the theme overriding them is measured next door in
 * `theme-tokens.js`.
 */

import * as stylex from '@stylexjs/stylex';
import { colors } from './colors.stylex';
import { sizes } from './sizes.stylex';

export const spacing = stylex.defineConsts({
  tight: '4px',
  cosy: '8px',
  loose: '16px',
  gutterQuery: '@media (min-width: 768px)',
});

export const tokens = stylex.defineVars({
  accent: colors['--accent'],
  surface: colors['--background-wash-plain'],
  radius: sizes.borderRadiusMedium,
  gutter: spacing.cosy,
  fontSize: {
    default: '1rem',
    '@media (min-width: 768px)': '1.125rem',
  },
});
