/**
 * The engine-backed fold: a method call that carries its own value is printed
 * as source, evaluated by a JavaScript engine and read back as a value. What
 * this prices is the round trip — the guard walk, the print, the engine's parse
 * and evaluation, and the conversion of the answer back into the compiler's own
 * value — once per call site, over the shapes that cost differently: a string
 * method, an array method, a callback the engine invokes per element, a chain
 * that folds at every link, and an answer that comes back as an array rather
 * than as a scalar.
 *
 * Two shapes the fold does take are deliberately absent, both for the same
 * reason. `bench:revisions` sanity-checks every fixture against the revision
 * before the change as well as against this one, so a shape only the newer
 * subject compiles throws on the base subject and takes the whole leg down
 * before a single measurement — see `guidelines/PERFORMANCE.md`, and
 * `perf_fixtures/dynamic-styles.js` for the same omission made for the same
 * reason.
 *
 * The first is a receiver reached through a *name*: `const s = 'ABC';
 * s.toLowerCase()`. Every receiver here is written out in full instead. The
 * second is a mutating method — `reverse`, `sort`, `push` — which folds on a
 * temporary nothing can name afterwards and so is as safe as any other, but
 * only since the change under measurement. Both belong to
 * `crates/stylex-transform` as correctness fixtures, and earn a place here once
 * both subjects of a comparison compile them.
 */

import * as stylex from '@stylexjs/stylex';

export const styles = stylex.create({
  strings: {
    // One string method per shape the printer has to emit: an argumentless
    // call, one string argument, two numeric arguments, and an amplifying call
    // whose length the guard has to read before it prints anything.
    content: '  "read"  '.trim(),
    fontFamily: 'inter, system-ui'.toUpperCase(),
    gridArea: 'head-head'.replace('-', ' '),
    borderRadius: '4px 8px 4px 8px'.slice(0, 7),
    borderWidth: '1px '.repeat(4).trim(),
  },
  arrays: {
    // Array receivers, including the two callback shapes: one value per
    // element, and one value folded across all of them.
    fontFamily: ['Inter', 'system-ui', 'sans-serif'].join(', '),
    padding: [4, 8, 12, 16].map(step => step + 'px').join(' '),
    transitionProperty: ['opacity', 'transform'].concat(['filter']).join(','),
    zIndex: [1, 2, 3].reduce((total, step) => total + step, 0),
    gridTemplateColumns: ['1fr', '2fr'].filter(track => track !== '3fr').join(' '),
  },
  chains: {
    // A chain folds at every link, and the whole chain is printed once. These
    // are what two separate method tables could not agree on.
    borderColor: '  rgba(0,0,0,.2)|rgba(0,0,0,.4)  '.trim().split('|').join(' '),
    boxShadow: ['0 1px 2px', '0 2px 4px'].map(shadow => shadow + ' rgba(0,0,0,.2)').join(', '),
    fontFamily: '  helvetica neue  '.trim().split(' ').join('-'),
  },
  fallbacks: {
    // The answer comes back as an array rather than as a scalar, so the
    // outward half of the bridge is what runs: StyleX reads it as a fallback
    // list and emits one declaration per entry.
    position: ['-webkit-sticky'].concat(['sticky']),
    display: ['flex', '-webkit-box'].slice(0, 2),
  },
});
