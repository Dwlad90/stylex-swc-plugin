// Does the bound cap one query, or the whole compile? A comma query is several
// `and` lists, and the boundary is crossed once per list.
const path = require('node:path');
const EV = '/Users/vladislavbuinovski/Projects/Facebook/stylex-swc-plugin.git/bugfix/.scratch/fix_media-query-order-wrapping/evidence';
const { runRust } = require(path.join(EV, 'ref.cjs'));

const disjuncts = Number(process.argv[2]);
const rungs = Number(process.argv[3] ?? 20);

// One ladder, but the first key is a comma query of `disjuncts` disjuncts, so
// the transform builds that many `and` lists each carrying the whole chain.
function ladder() {
  const value = { default: 'black' };
  const heads = [];
  for (let d = 0; d < disjuncts; d++) heads.push(`(min-width: ${1000 + d}px)`);
  value[`@media ${heads.join(', ')}`] = 'c0';
  for (let i = 1; i < rungs - 1; i++) {
    const lower = 1000 - i * 50;
    value[`@media (min-width: ${lower}px) and (max-width: ${1000 - (i - 1) * 50 - 1}px)`] = `c${i}`;
  }
  value[`@media (max-width: ${1000 - (rungs - 2) * 50 - 1}px)`] = `c${rungs - 1}`;
  return value;
}

const code = `import * as stylex from '@stylexjs/stylex';\nexport const styles = stylex.create({ x: ${JSON.stringify({ color: ladder() })} });\n`;
const t = process.hrtime.bigint();
let out;
try {
  const { rules } = runRust(code);
  const first = (rules.map(r => (r?.[1]?.ltr ?? '').match(/@media[^{]*/g)).filter(Boolean)[0] ?? [''])[0];
  out = { chars: first.length };
} catch (e) { out = { threw: String(e.message).split('\n')[0].slice(0, 60) }; }
console.log(JSON.stringify({ disjuncts, rungs, ms: Math.round(Number(process.hrtime.bigint() - t) / 1e6), rssMB: Math.round(process.memoryUsage().rss / 1e6), ...out }));
