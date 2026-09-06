import * as stylex from '@stylexjs/stylex';
import { zIndex } from './vars/zIndex.stylex.js';
import { spacing as ünïcödé } from './vars/spacing.stylex.js';
import { firstThatWorks } from './vars/legacy.stylex.js';
import { grid } from './vars/grid.stylex.js';

export const styles = stylex.create({
  // A shadowing parameter whose name is spelled with non-ASCII letters, read
  // beside the same import held as a theme reference.
  unicodeName: { padding: ünïcödé.md },
  unicodeParam: (ünïcödé) => ({ padding: ünïcödé }),

  // The same name written with a unicode escape in the parameter position. The
  // parser folds the escape, so this shadows the very same import.
  escapedParam: (\u00fcn\u00efc\u00f6d\u00e9) => ({ margin: \u00fcn\u00efc\u00f6d\u00e9 }),

  // A parameter shadowing the name of a StyleX helper. The helper is a value
  // here, not a callee, so the parameter wins and the value goes inline.
  helperName: (firstThatWorks) => ({ fontFamily: firstThatWorks }),

  // A shorthand expanded into longhands, each of which has to carry the same
  // inline custom property.
  shorthand: (zIndex) => ({ inset: zIndex, marginInline: zIndex }),

  // A custom property as the key, dynamic in the value.
  customProperty: (zIndex) => ({ '--depth': zIndex, '--nested-depth': zIndex }),

  // A property that expands to vendor-prefixed rules, driven dynamically.
  prefixed: (zIndex) => ({ userSelect: zIndex, appearance: zIndex }),

  // Eight levels of nesting under a single property, each level reading the
  // shadowing parameter, so the custom property is named from the key path.
  deeplyNested: (zIndex) => ({
    zIndex: {
      default: zIndex,
      ':hover': {
        default: zIndex,
        ':focus': {
          default: zIndex,
          '@media (min-width: 600px)': {
            default: zIndex,
            '@supports (display: grid)': {
              default: zIndex,
              ':active': {
                default: zIndex,
                '@media (prefers-color-scheme: dark)': {
                  default: zIndex,
                  ':first-child': zIndex,
                },
              },
            },
          },
        },
      },
    },
  }),

  // The shadowing parameter read through operators and a template literal
  // rather than emitted straight.
  computedFromParam: (zIndex) => ({
    zIndex: zIndex + 1,
    content: `"${zIndex}"`,
    width: `calc(100% - ${zIndex}px)`,
  }),

  // Several parameters, only one of which shadows an import.
  mixedParams: (zIndex, level) => ({ zIndex, order: level }),

  // The import still reads as a theme reference in the same call, after every
  // parameter above has taken its name.
  static: { zIndex: zIndex._10, gridArea: grid.main },
});
