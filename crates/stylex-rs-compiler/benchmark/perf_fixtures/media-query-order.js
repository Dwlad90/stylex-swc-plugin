/**
 * The at-rule ordering machinery, priced on its own.
 *
 * `enableMediaQueryOrder` is what rewrites each media condition into one that
 * excludes every condition ranked above it, so a stylesheet orders correctly
 * without relying on source order. The work is quadratic in the number of
 * conditions declared on a single property -- every rule carries a `not (...)`
 * for each of its betters -- so the shape that prices it is many conditions on
 * one property, repeated across properties.
 *
 * `media-queries.js` covers the canonicalizer's branches: it is broad and
 * shallow, one or two conditions per shape. This file is the opposite, and the
 * two are not interchangeable. The condition shapes here are still mixed on
 * purpose -- `min-`/`max-` pairs, range syntax, `and`/`or`, negation, media
 * types, container and feature queries -- because ranking has to reduce each to
 * a comparable form before it can exclude anything.
 *
 * Registered twice in `fixtures.v1.json`: once under the default (ordering on)
 * and once with `enableMediaQueryOrder: false`. Neither number means anything
 * alone; the pair is the price of the feature.
 */

import * as stylex from '@stylexjs/stylex';

export const styles = stylex.create({
  typeScale: {
    fontSize: {
      default: '1rem',
      '@media (min-width: 320px)': '1.05rem',
      '@media (min-width: 480px)': '1.1rem',
      '@media (min-width: 640px)': '1.15rem',
      '@media (width >= 768px)': '1.2rem',
      '@media (min-width: 1024px) and (max-width: 1279px)': '1.25rem',
      '@media (1280px <= width <= 1535px)': '1.3rem',
      '@media (min-width: 1536px)': '1.35rem',
    },
    lineHeight: {
      default: 1.4,
      '@media (min-width: 480px)': 1.45,
      '@media (min-width: 768px)': 1.5,
      '@media (min-width: 1024px)': 1.55,
      '@media (min-width: 1440px)': 1.6,
      '@media (orientation: landscape)': 1.35,
    },
  },
  spacing: {
    padding: {
      default: 4,
      '@media (min-width: 360px)': 6,
      '@media (min-width: 600px)': 8,
      '@media (min-width: 900px)': 12,
      '@media (min-width: 1200px)': 16,
      '@container (min-width: 400px)': 10,
      '@container card (min-width: 700px)': 20,
    },
    marginInline: {
      default: 0,
      '@media (min-width: 600px) and (orientation: portrait)': 8,
      '@media (min-width: 600px) and (orientation: landscape)': 12,
      '@media (min-width: 1200px)': 24,
      '@media print': 0,
    },
  },
  chrome: {
    display: {
      default: 'block',
      '@media print': 'none',
      '@media (any-hover: hover)': 'flex',
      '@media not all and (monochrome)': 'grid',
      '@media screen and (min-width: 900px)': 'inline-flex',
      '@supports (display: grid)': 'grid',
    },
    color: {
      default: 'black',
      '@media (prefers-color-scheme: dark)': 'white',
      '@media (prefers-contrast: more)': '#000',
      '@media (forced-colors: active)': 'CanvasText',
      '@media (min-width: 800px)': '#111',
      '@media (min-width: 1400px)': '#222',
    },
  },
  motion: {
    transitionDuration: {
      default: '200ms',
      '@media (prefers-reduced-motion: reduce)': '0s',
      '@media (min-width: 700px)': '250ms',
      '@media (min-width: 1100px)': '300ms',
      '@media (update: slow)': '0s',
      '@media (any-pointer: coarse)': '150ms',
    },
  },
});
