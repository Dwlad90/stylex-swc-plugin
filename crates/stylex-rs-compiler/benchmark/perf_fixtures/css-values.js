/**
 * Value normalization, which is where a class name's hash comes from: the
 * shapes the scanner and the normalizers branch on. Numbers that take a unit
 * and numbers that do not, leading zeros, timing values, `calc` nesting, colour
 * functions old and new, gradients, `url()`, custom properties, `var()` with
 * fallbacks, content strings with escapes, and the font stacks that get quoted.
 */

import * as stylex from '@stylexjs/stylex';

export const styles = stylex.create({
  values: {
    width: 100,
    lineHeight: 1.5,
    opacity: 0.75,
    zIndex: 42,
    flexGrow: 1,
    transitionDuration: '0.25s',
    transitionDelay: '.5s',
    marginTop: '-0.5em',
    fontSize: '1.0625rem',
    letterSpacing: '.02em',
  },
  functions: {
    width: 'calc(100% - calc(2 * var(--gutter, 8px)))',
    height: 'clamp(1rem, calc(2vw + 1px), 4rem)',
    color: 'rgb(0 0 0 / 50%)',
    backgroundColor: 'color-mix(in srgb, red 40%, blue)',
    borderColor: 'rgba(0, 0, 0, 0.2)',
    outlineColor: 'hsl(210deg 50% 40% / 0.8)',
    backgroundImage:
      'linear-gradient(to bottom, rgba(0, 0, 0, 0) 0%, rgba(0, 0, 0, 0.6) 100%)',
    maskImage: 'url(data:image/svg+xml;base64,AAA=)',
    filter: 'blur(2px) saturate(1.2)',
  },
  text: {
    fontFamily: 'Inter, "Helvetica Neue", system-ui, sans-serif',
    content: '"\\2014 \\00a0"',
    quotes: '"\\201C" "\\201D"',
    whiteSpace: 'pre-wrap',
    '--custom-gutter': '8px',
    '--custom-shadow': '0 1px 2px rgba(0,0,0,.2)',
  },
  positions: {
    backgroundPosition: 'top 0.75rem left 0.625rem',
    objectPosition: '50% bottom',
    translate: '-50% -120%',
    transformOrigin: 'left center',
    gridTemplateColumns: 'repeat(auto-fill, minmax(120px, 1fr))',
    gridTemplateAreas: '"head head" "nav main"',
  },
});
