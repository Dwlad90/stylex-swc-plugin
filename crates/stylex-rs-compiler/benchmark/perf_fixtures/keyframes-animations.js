/**
 * Keyframes and animation values: several `keyframes` calls, one composed of
 * many stops, plus the shapes that read them back — a shorthand `animation`,
 * an animation named in a conditional value, and `firstThatWorks` around
 * vendor-prefixed properties.
 */

import * as stylex from '@stylexjs/stylex';

const fadeIn = stylex.keyframes({
  from: { opacity: 0, transform: 'translateY(4px)' },
  to: { opacity: 1, transform: 'translateY(0)' },
});

const pulse = stylex.keyframes({
  '0%': { transform: 'scale(1)' },
  '25%': { transform: 'scale(1.02)' },
  '50%': { transform: 'scale(1.04)' },
  '75%': { transform: 'scale(1.02)' },
  '100%': { transform: 'scale(1)' },
});

const slide = stylex.keyframes({
  from: { insetInlineStart: 0 },
  to: { insetInlineStart: '100%' },
});

export const styles = stylex.create({
  entering: {
    animationName: fadeIn,
    animationDuration: '0.2s',
    animationTimingFunction: 'ease-in-out',
    animationFillMode: 'both',
  },
  shorthand: {
    animation: `${pulse} 1.5s ease-in-out infinite`,
  },
  conditional: {
    animationName: {
      default: fadeIn,
      ':hover': pulse,
      '@media (prefers-reduced-motion: reduce)': 'none',
    },
  },
  moving: {
    animationName: slide,
    animationDuration: '0.4s',
    position: 'relative',
    overflow: stylex.firstThatWorks('clip', 'hidden'),
    inset: 0,
  },
});
