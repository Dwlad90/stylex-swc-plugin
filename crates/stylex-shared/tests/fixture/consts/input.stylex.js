import * as stylex from '@stylexjs/stylex';

export const breakpoints = stylex.defineConsts({
  small: '@media (max-width: 600px)',
  medium: '@media (min-width: 601px) and (max-width: 1024px)',
  large: '@media (max-width: 1025px)',
});

export const colors = stylex.defineConsts({
  accent: 'hotpink',
  background: 'white',
  foreground: 'black',
});
