/**
 * Media and container queries: many distinct conditions on one property, which
 * is what the media-query canonicalizer and the at-rule priority order have to
 * sort. Range syntax, legacy `min-`/`max-` pairs, feature queries and container
 * queries are all spelled here because each takes a different branch.
 */

import * as stylex from '@stylexjs/stylex';

export const styles = stylex.create({
  responsive: {
    fontSize: {
      default: '1rem',
      '@media (min-width: 320px)': '1.05rem',
      '@media (min-width: 480px)': '1.1rem',
      '@media (width >= 600px)': '1.15rem',
      '@media (min-width: 768px) and (max-width: 1023px)': '1.2rem',
      '@media (400px <= width <= 700px)': '1.25rem',
      '@media screen and (orientation: landscape)': '1.3rem',
      '@media (prefers-reduced-motion: reduce)': '1rem',
      '@supports (font-size: clamp(1rem, 2vw, 2rem))': 'clamp(1rem, 2vw, 2rem)',
    },
    padding: {
      default: 4,
      '@media (min-width: 480px)': 8,
      '@media (min-width: 1024px)': 16,
      '@container (min-width: 400px)': 12,
      '@container card (min-width: 600px)': 20,
    },
    display: {
      default: 'block',
      '@media print': 'none',
      '@media (any-hover: hover)': 'flex',
      '@media not all and (monochrome)': 'grid',
    },
  },
});
