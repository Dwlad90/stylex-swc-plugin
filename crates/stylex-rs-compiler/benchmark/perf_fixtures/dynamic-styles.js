/**
 * Dynamic styles: every namespace is a function, so each one leaves the static
 * fold and takes the inline-style path — a CSS custom property per value plus
 * an `@property` rule, and a runtime arrow rebuilt from the evaluated body.
 *
 * The feature under measurement is that path, not size. Eleven namespaces cover
 * the shapes it branches on: one parameter and several, a parameter read twice,
 * a parameter inside a conditional value, a parameter beside static properties,
 * arithmetic on a parameter, and a parameter interpolated into a template.
 *
 * A parameter that *shadows an imported binding* is deliberately not among them:
 * `tests/fixture/dynamic-param-shadows-import{,-edges}` cover that shape as
 * transform fixtures, and one more copy of it would price nothing new — while
 * making this fixture refuse to compile on any revision without the fix, which
 * is exactly the comparison a benchmark fixture should stay out of. Those two
 * are deliberately absent from `fixtures.v1.json` for that same reason; see
 * `guidelines/PERFORMANCE.md`.
 */

import * as stylex from '@stylexjs/stylex';
import { colors } from './colors.stylex';

export const styles = stylex.create({
  width: (w) => ({ width: w }),
  size: (w, h) => ({ width: w, height: h }),
  repeated: (space) => ({ paddingTop: space, paddingBottom: space, gap: space }),
  conditional: (color) => ({
    color: { default: color, ':hover': colors.primary },
  }),
  mixed: (offset) => ({
    position: 'absolute',
    insetInlineStart: offset,
    zIndex: 3,
    backgroundColor: colors.background,
  }),
  arithmetic: (base) => ({ margin: base * 2, padding: base / 2 }),
  template: (radius) => ({ borderRadius: `${radius}px` }),
  nested: (color) => ({
    color: {
      default: color,
      '@media (min-width: 600px)': { default: colors.primary, ':hover': color },
    },
  }),
  transform: (angle) => ({ transform: `rotate(${angle}deg) scale(1.02)` }),
  shorthand: (space) => ({ padding: space, margin: space }),
  many: (a, b, c, d) => ({ top: a, right: b, bottom: c, left: d }),
});
