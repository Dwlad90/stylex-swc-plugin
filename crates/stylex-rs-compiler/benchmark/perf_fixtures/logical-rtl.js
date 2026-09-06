/**
 * Logical properties and the values that get flipped for right-to-left: the
 * inline/block axes, the physical properties that have an RTL counterpart, and
 * the shorthands whose four sides are reordered rather than renamed.
 *
 * Measured under the legacy shorthand-expansion and legacy value-flipping
 * options as well as plain, so the cost of emitting a second direction shows up
 * as its own number. Not under `enableLogicalStylesPolyfill`: it was measured
 * and changes nothing on this fixture, so no manifest entry names it.
 */

import * as stylex from '@stylexjs/stylex';

export const styles = stylex.create({
  logical: {
    marginInlineStart: 8,
    marginInlineEnd: 4,
    paddingInline: '4px 8px',
    paddingBlock: '2px 6px',
    insetInlineStart: 0,
    insetBlockEnd: 2,
    borderInlineStartWidth: 1,
    borderInlineEndColor: 'grey',
    textAlign: 'start',
  },
  physical: {
    marginLeft: 8,
    marginRight: 4,
    paddingLeft: 12,
    paddingRight: 6,
    left: 0,
    right: 'auto',
    borderLeftWidth: 1,
    borderRightColor: 'grey',
    float: 'left',
    clear: 'right',
  },
  shorthands: {
    margin: '1px 2px 3px 4px',
    padding: '4px 8px',
    inset: '0 2px 4px 6px',
    borderWidth: '1px 2px 3px 4px',
    borderRadius: '1px 2px 3px 4px',
    borderColor: 'red blue green yellow',
  },
  transforms: {
    transform: 'translateX(4px) rotate(45deg)',
    backgroundPosition: 'left 4px top 8px',
    boxShadow: '4px 2px 0 0 rgba(0,0,0,0.2)',
    textShadow: '2px 1px 0 grey',
  },
});
