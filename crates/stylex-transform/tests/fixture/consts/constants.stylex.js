import * as stylex from '@stylexjs/stylex';

export const constants = stylex.defineConsts({
  INPUT_HEIGHT: '10',
  YELLOW: 'yellow',
  ORANGE: 'var(--orange)',
  mediaBig: '@media (max-width: 1000px)',
  mediaSmall: '@media (max-width: 500px)'
});
export const C = constants;
export const vars = stylex.defineVars({
  blue: 'blue'
});
