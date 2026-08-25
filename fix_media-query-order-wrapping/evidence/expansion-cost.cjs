// One ladder length through this compiler: wall clock and first-rung query size.
const path = require('node:path');
const EV = '/Users/vladislavbuinovski/Projects/Facebook/stylex-swc-plugin.git/bugfix/.scratch/fix_media-query-order-wrapping/evidence';
const { runRust } = require(path.join(EV, 'ref.cjs'));

const rungs = Number(process.argv[2]);

function ladder(rungs) {
  const value = { default: 'black' };
  for (let i = 0; i < rungs - 1; i++) {
    const lower = 1000 - i * 50;
    const upper = 1000 - (i - 1) * 50 - 1;
    value[i === 0 ? `@media (min-width: ${lower}px)` : `@media (min-width: ${lower}px) and (max-width: ${upper}px)`] = `c${i}`;
  }
  value[`@media (max-width: ${1000 - (rungs - 2) * 50 - 1}px)`] = `c${rungs - 1}`;
  return value;
}

const code = [
  "import * as stylex from '@stylexjs/stylex';",
  `export const styles = stylex.create({ x: ${JSON.stringify({ color: ladder(rungs) })} });`,
  '',
].join('\n');

const started = process.hrtime.bigint();
let out = {};
try {
  const { rules } = runRust(code);
  const first = (rules.map(r => (r?.[1]?.ltr ?? '').match(/@media[^{]*/g)).filter(Boolean)[0] ?? [''])[0].trim();
  out = { chars: first.length, merged: first.includes('not all') || first.length > 40 };
} catch (error) {
  out = { threw: String(error.message ?? error).split('\n')[0] };
}
const ms = Number(process.hrtime.bigint() - started) / 1e6;
console.log(JSON.stringify({ rungs, ms: Math.round(ms), rssMB: Math.round(process.memoryUsage().rss / 1e6), ...out }));
