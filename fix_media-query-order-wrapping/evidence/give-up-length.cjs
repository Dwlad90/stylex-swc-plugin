// One ladder length, run through the reference implementation, printed as a
// verdict: did it merge the ladder, hand it back unmerged, or throw?
//
// Run as a subprocess per length so an exponential expansion that never
// returns is a timeout rather than a hang in the bisect.
const path = require('node:path');
const { run } = require(path.join(__dirname, 'ref.cjs'));

const rungs = Number(process.argv[2]);

// The reported shape: exclusive `min-width`/`max-width` rungs from widest to
// narrowest, the last one `max-width` only. Widths are spaced so no two rungs
// touch, which is what makes every distributed branch a contradiction.
function ladder(rungs) {
  const value = {};
  value.default = 'black';
  for (let i = 0; i < rungs - 1; i++) {
    const lower = 1000 - i * 50;
    const upper = 1000 - (i - 1) * 50 - 1;
    value[i === 0 ? `@media (min-width: ${lower}px)` : `@media (min-width: ${lower}px) and (max-width: ${upper}px)`] = `c${i}`;
  }
  value[`@media (max-width: ${1000 - (rungs - 2) * 50 - 1}px)`] = `c${rungs - 1}`;
  return value;
}

const props = { color: ladder(rungs) };
const code = [
  "import * as stylex from '@stylexjs/stylex';",
  `export const styles = stylex.create({ x: ${JSON.stringify(props)} });`,
  '',
].join('\n');

const started = process.hrtime.bigint();
let outcome;
let first = '';
try {
  const { rules } = run(code);
  const preludes = rules
    .map(rule => (rule?.[1]?.ltr ?? '').match(/@media[^{]*/g))
    .filter(found => found !== null)
    .map(found => found[0].trim());
  first = preludes[0] ?? '';
  // A merged first rung carries the expansion: `not all` branches, or the
  // nesting around them. Handed back unmerged, it is the authored query.
  outcome = first.includes('not all') || first.startsWith('@media (') === false || /\)\) or \(/.test(first)
    ? 'merged'
    : first === Object.keys(ladder(rungs))[1] ? 'unmerged' : 'merged';
} catch (error) {
  outcome = 'threw';
  first = error instanceof Error ? `${error.name}: ${error.message.split('\n')[0]}` : String(error);
}
const ms = Number(process.hrtime.bigint() - started) / 1e6;

console.log(JSON.stringify({ rungs, outcome, ms: Math.round(ms), first }));
