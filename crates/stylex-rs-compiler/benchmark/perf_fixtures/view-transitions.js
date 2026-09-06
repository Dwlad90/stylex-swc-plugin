/**
 * The two newest transformers: `viewTransitionClass`, whose groups each become
 * their own rule, and `positionTry`, which emits an `@position-try` at-rule and
 * is read back through `positionAnchor`.
 */

import * as stylex from '@stylexjs/stylex';

const fade = stylex.viewTransitionClass({
  group: { animationDuration: '0.3s' },
  imagePair: { borderRadius: '4px' },
  old: { animationTimingFunction: 'ease-out' },
  new: { animationTimingFunction: 'ease-in' },
});

const slide = stylex.viewTransitionClass({
  group: { animationDuration: '0.2s', animationTimingFunction: 'linear' },
  old: { opacity: 0 },
  new: { opacity: 1 },
});

const flip = stylex.positionTry({
  positionAnchor: '--anchor',
  top: 'anchor(bottom)',
  insetInlineStart: 'anchor(start)',
  width: '120px',
});

export const styles = stylex.create({
  transitioning: {
    viewTransitionClass: fade,
    viewTransitionName: 'card',
  },
  sliding: {
    viewTransitionClass: slide,
  },
  anchored: {
    positionAnchor: '--anchor',
    positionTryFallbacks: flip,
    position: 'absolute',
  },
});
