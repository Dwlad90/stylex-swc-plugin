/**
 * Conditional values nested several levels deep: pseudo-classes inside
 * at-rules inside pseudo-elements, with `default` arms at every level.
 *
 * What this measures is the pseudo/at-rule *ordering* work — the run
 * comparator, the collation-aware key sort, and the modifier string a class
 * name hashes — rather than the number of declarations. Nesting order is
 * deliberately not alphabetical, since that is the case an order-insensitive
 * comparator gets wrong.
 */

import * as stylex from '@stylexjs/stylex';

export const styles = stylex.create({
  deep: {
    color: {
      default: 'black',
      ':hover': {
        default: 'blue',
        '@media (min-width: 600px)': {
          default: 'navy',
          ':active': 'darkblue',
          ':focus': 'steelblue',
        },
      },
      ':active': 'red',
      '@supports (display: grid)': { default: 'green', ':hover': 'darkgreen' },
    },
    backgroundColor: {
      default: 'white',
      '@media (prefers-color-scheme: dark)': {
        default: 'black',
        ':hover': { default: '#111', ':focus': '#222' },
      },
    },
  },
  pseudoElements: {
    '::before': { content: '""', display: 'block', width: 8 },
    '::after': { content: '""', display: 'block', width: 4 },
    '::placeholder': { color: { default: 'grey', ':hover': 'darkgrey' } },
    '::selection': { backgroundColor: 'yellow' },
  },
  runs: {
    opacity: { default: 1, ':hover': 0.9, ':focus': 0.8, ':active': 0.7 },
    borderColor: {
      default: 'grey',
      ':hover': 'black',
      ':focus-visible': 'blue',
      ':disabled': 'lightgrey',
    },
  },
  attributeSelectors: {
    display: {
      default: 'block',
      '[data-state="open"]': 'flex',
      '[data-State="Closed"]': 'none',
      '[aria-hidden="true"]': 'none',
    },
  },
});
