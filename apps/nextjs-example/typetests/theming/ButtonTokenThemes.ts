import type { Theme } from '@stylexjs/stylex';

import { ButtonTokens } from './ButtonTokens.stylex';

import * as stylex from '@stylexjs/stylex';

/* eslint-disable no-unused-vars */

const fdsRed: Theme<typeof ButtonTokens> = stylex.createTheme(ButtonTokens, {
  bgColor: 'red',
  color: 'white',
  height: 'var(--button-height-medium)',
  opacity: '1',
});

const fdsBlue: Theme<typeof ButtonTokens> = stylex.createTheme(ButtonTokens, {
  bgColor: 'blue',
  color: 'white',
  height: 'var(--button-height-medium)',
  opacity: '1',
});

const styles = stylex.create({
  test1: {
    padding: 4,
    color: ButtonTokens.color,
    backgroundColor: `color-mix(in oklch, ${ButtonTokens.bgColor}, 'white')`,
  },
});