/**
 * The calls that fold nothing: what the fold's guard costs on ordinary input.
 *
 * `engine-fold.js` is all folds, so on it the round trip dominates and hides
 * everything around it. Real modules are the other way round — almost every
 * call expression the evaluator visits is one no engine can answer, and it
 * still pays for the guard walk that decides so. This fixture is that side of
 * the split: many call expressions, not one of them foldable, so the price of
 * declining is measured on its own.
 *
 * Every call here has a leaf the guard cannot resolve — a parameter of a
 * dynamic style function — so the walk descends the whole expression before it
 * refuses. Where the refusal is decided differs on purpose: a global callee, a
 * method on a named receiver, an array element, a callback body, and a chain
 * that only refuses at its last link.
 *
 * A call the guard would admit is deliberately absent: it would fold, which is
 * the other fixture's measurement. So is an unresolvable call in a *static*
 * value, which both revisions refuse by failing the module rather than by
 * emitting anything. The `stylex.firstThatWorks` calls are written out of
 * literals and are neither: the fold never sees a StyleX function.
 *
 * The `+` chains and the template interpolations are not calls and are not
 * incidental. They are the string the evaluator grows, and every module in the
 * world pays for it whether or not anything folds, so the cost belongs in the
 * same number.
 *
 * The module compiles to the same CSS on the revision before the fold as on
 * this one — the point of a no-fold fixture — which is also what
 * `guidelines/PERFORMANCE.md` requires of every fixture registered here. One
 * shape is missing for that reason and no other: a `filter` whose receiver is a
 * call nothing can resolve, as in `[1, 2].map(i => i * step).filter(...)`. It
 * compiles here and throws `Expr is not a literal` on the revision before,
 * which takes the whole benchmark leg down before a single measurement. A
 * `map` on such a receiver is fine, and so is a conversion global inside a
 * callback — both are written above. Check a new shape against both subjects
 * before adding it, rather than reasoning from these.
 *
 * `the_no_fold_benchmark_fixture_holds_no_foldable_call`, in
 * `crates/stylex-transform`, reads this file by path and fails if any call in
 * it folds. Cargo knows nothing about that edge, so moving or renaming this
 * file breaks a test in another crate.
 */

import * as stylex from '@stylexjs/stylex';

import { colors } from './colors.stylex';

// A ten-operand concatenation and a template holding the same pieces: the two
// spellings reach the same grown string, and both are re-measured per level.
const FONT_STACK =
  'Inter' + ', ' + 'system-ui' + ', ' + 'Segoe UI' + ', ' + 'Roboto' + ', ' + 'Helvetica Neue';
const GRID = `${'[full-start]'} ${'minmax(1rem, 1fr)'} ${'[content-start]'} ${'minmax(0, 60rem)'}`;

// A long chain, because the cost of growing a string this way is not linear in
// its length: every level re-measures what the level below it had already
// measured. Twenty operands is a plausible transition list and still inside the
// nesting the evaluator allows by default, which a chain twice as long is not.
const TRANSITIONS =
  'opacity, ' +
  'transform, ' +
  'filter, ' +
  'color, ' +
  'background-color, ' +
  'border-color, ' +
  'box-shadow, ' +
  'translate, ' +
  'rotate, ' +
  'scale, ' +
  'inset, ' +
  'margin, ' +
  'padding, ' +
  'width, ' +
  'height, ' +
  'flex-basis, ' +
  'font-size, ' +
  'letter-spacing, ' +
  'outline-color, ' +
  'z-index';
const OFFSETS = `${4}% ${8}% ${12}% ${16}% ${20}% ${24}% ${28}% ${32}% ${36}% ${40}% ${44}% ${48}%`;

export const motion = stylex.create({
  animated: {
    transitionProperty: TRANSITIONS,
    transitionDuration: '150' + 'ms',
    transitionTimingFunction: 'cubic-bezier(' + 0.4 + ', ' + 0 + ', ' + 0.2 + ', ' + 1 + ')',
    backgroundPosition: OFFSETS,
  },
});

export const statics = stylex.create({
  text: {
    fontFamily: FONT_STACK + ', ' + 'Arial' + ', ' + 'sans-serif',
    letterSpacing: '0' + '.' + '01' + 'em',
    lineHeight: `${1}.${5}`,
  },
  layout: {
    gridTemplateColumns: GRID + ' ' + '[content-end]' + ' ' + 'minmax(1rem, 1fr)' + ' [full-end]',
    padding: `${8}px ${12}px ${8}px ${12}px`,
    borderRadius: '4' + 'px' + ' ' + '8' + 'px' + ' ' + '4' + 'px' + ' ' + '8' + 'px',
  },
  surface: {
    backgroundColor: colors.background,
    boxShadow: `0 ${1}px ${2}px rgba(${0}, ${0}, ${0}, 0.${2})`,
    borderColor: colors.primary,
  },
  fallbacks: {
    position: stylex.firstThatWorks('sticky', '-webkit-sticky', 'fixed'),
    display: stylex.firstThatWorks('grid', '-ms-grid', 'block'),
    height: stylex.firstThatWorks('100dvh', '100vh', 'auto'),
  },
});

// One namespace per shape the guard refuses on, each refusing because a
// parameter of the enclosing function reaches the expression.
//
// The expressions are denser than a hand-written stylesheet's, and on purpose.
// Every value a dynamic namespace holds costs a custom property and an
// `@property` rule to emit, which is work the guard has nothing to do with, so
// a fixture written at one call per value would report the inline-style path
// and call it the cost of declining. Stacking the calls inside a few values
// keeps that overhead flat while the number of call expressions the evaluator
// visits is what grows.
export const dynamics = stylex.create({
  // A global callee whose argument does not resolve, nested into itself.
  clamped: size => ({
    width: Math.max(Math.min(Math.round(size), 960), Math.abs(Math.trunc(size))),
    height: Math.min(Math.max(Math.ceil(size), 8), Math.floor(Math.sqrt(size))),
    zIndex: Math.round(Math.max(Math.sign(size), Math.min(size, 8))),
  }),
  // A conversion global, refused for its argument rather than for its name.
  converted: size => ({
    flexBasis: Number(String(parseInt(size, 10))),
    order: parseInt(String(Number(size)), 10),
    opacity: parseFloat(String(parseFloat(size))),
    lineHeight: Number(parseFloat(String(size))),
  }),
  // A method on a named receiver the module cannot read.
  trimmed: label => ({
    content: label.trim().replace('-', ' ').toUpperCase(),
    gridArea: label.trim().replace('_', ' ').toLowerCase().slice(0, 8),
    fontFamily: label.trim().padEnd(8, ' ').toUpperCase().trimEnd(),
  }),
  // A chain that only refuses at its last link.
  chained: label => ({
    borderColor: label.trim().split('|').join(' ').replace('  ', ' ').trim(),
    transitionProperty: label.trim().toLowerCase().split(' ').join(',').replace(',,', ','),
  }),
  // An array whose receiver is written out but whose elements are not.
  joined: step => ({
    padding: [step, 8, step, 8].join('px ').concat('px').trim(),
    margin: ['0', step].concat(['auto']).join(' ').trim(),
    inset: [step, step].concat([step]).slice(0, 3).join(' '),
  }),
  // A callback the engine would have to run, over an element it cannot see.
  mapped: step => ({
    transitionDuration: [1, 2, 3]
      .map(index => index * step)
      .map(value => String(value).concat('ms').trim())
      .join(', '),
    gridTemplateRows: [1, 2]
      .map(index => `${index * step}px`)
      .concat(['auto'])
      .join(' '),
    borderWidth: [1, 2, 3, 4].reduce((total, index) => total + Math.min(index * step, 4), 0),
  }),
  // A call whose receiver is a template the parameter is interpolated into.
  interpolated: size => ({
    width: `calc(100% - ${Math.max(size, 0)}px)`.replace('- -', '+ '),
    maxWidth: `min(${Math.min(size, 960)}px, 100%)`.trim(),
    translate: `${Math.round(size)}px ${Math.round(size / 2)}px`.trimStart(),
  }),
  // A concatenation the parameter joins, beside the calls that read it.
  concatenated: size => ({
    marginBlock: String(size).concat('px ').concat(String(size)).concat('px'),
    outlineOffset: String(Math.max(size, 1)).concat('px'),
    outlineWidth: (size + 1).toString().concat('px').trim(),
  }),
  // A theme member beside the parameter: the read resolves, the call does not.
  themed: size => ({
    color: colors.primary,
    borderWidth: Math.max(Math.min(size, 4), Number(size)),
    backgroundImage: `linear-gradient(${Math.round(size)}deg, ${colors.background}, transparent)`
      .replace('  ', ' ')
      .trim(),
  }),
  // Nested callbacks: the inner one reads the outer one's binding.
  nested: step => ({
    gridTemplateAreas: [1, 2]
      .map(row => [1, 2].map(column => `a${Math.round(row * column * step)}`).join(' '))
      .join(' '),
    scale: [1, 2]
      .map(index => [index, step].join('.'))
      .map(value => Number(value).toFixed(2))
      .join(' '),
  }),
});

export const wrapper = stylex.props(statics.text, statics.layout, statics.surface);
export const overlay = stylex.props(statics.surface, statics.fallbacks);
export const cell = stylex.attrs(statics.layout, statics.text);
