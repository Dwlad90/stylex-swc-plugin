import * as stylex from '@stylexjs/stylex';

export const breakpoints = stylex.defineConsts({
  mobile: '@media (max-width: 600px)',
  desktop: '@media (min-width: 601px)',
});