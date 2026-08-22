/**
 * Dynamic styles: every namespace is a function, so each one leaves the static
 * fold and takes the inline-style path — a CSS custom property per value plus
 * an `@property` rule, and a runtime arrow rebuilt from the evaluated body.
 *
 * The feature under measurement is that path, not size. Twelve namespaces cover
 * the shapes it branches on: one parameter and several, a parameter read twice,
 * a parameter inside a conditional value, a parameter beside static properties,
 * and a parameter that shadows an imported binding (issue #1266).
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
  shadowsImport: (colors) => ({ color: colors }),
  many: (a, b, c, d) => ({ top: a, right: b, bottom: c, left: d }),
});
