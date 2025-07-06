import * as stylex from '@stylexjs/stylex';

export const constants = stylex.defineConsts({
  YELLOW: 'yellow',
  ORANGE: 'var(--orange)',
  mediaBig: '@media (max-width: 1000px)',
  mediaSmall: '@media (max-width: 500px)'
});
export const vars = stylex.defineVars({
  blue: 'blue'
});