// Ticket 06 -- search for a collision the two compilers order or count
// differently. Run with a key count and optionally `nested`:
//
//   node collision-search.cjs 4
//   node collision-search.cjs 3 nested
//
// Alphabet includes queries that canonicalize to text another authored key is
// already spelled as, which is the shape the reference's delete-then-assign
// loop treats differently from a plain append.
const path = require('node:path');
const EV = '/Users/vladislavbuinovski/Projects/Facebook/stylex-swc-plugin.git/bugfix/.scratch/fix_media-query-order-wrapping/evidence';
const { run, runRust } = require(path.join(EV, 'ref.cjs'));

const ALPHABET = [
  '@media not all',
  '@media all',
  '@media (min-width: 100px)',
  '@media (max-width: 99.99px)',
  '@media (min-width: 200px)',
  '@media (min-height: 100px)',
  '@media (min-width: 200px) and (max-width: 100px)',
  '@media (max-width: 99.99px) and (min-height: 100px)',
  '@media screen',
];

const NEST = process.argv[3] === 'nested';

function build(keys) {
  const value = { default: 'black' };
  keys.forEach((k, i) => { value[k] = `c${i}`; });
  const props = NEST ? { color: { default: 'black', ':hover': value } } : { color: value };
  return [
    "import * as stylex from '@stylexjs/stylex';",
    `export const styles = stylex.create({ x: ${JSON.stringify(props)} });`,
    '',
  ].join('\n');
}

const sig = out => out.rules.map(r => `${r[0]}|${r?.[1]?.ltr}`).join('\n');

const hits = [];
function check(keys) {
  const code = build(keys);
  let a, b;
  try { a = sig(run(code)); } catch (e) { a = 'THREW ' + e.message; }
  try { b = sig(runRust(code)); } catch (e) { b = 'THREW ' + e.message; }
  if (a !== b) hits.push({ keys, a, b });
}

const N = Number(process.argv[2] ?? 3);
function* combos(n) {
  if (n === 0) { yield []; return; }
  for (const rest of combos(n - 1)) for (const a of ALPHABET) {
    if (!rest.includes(a)) yield [a, ...rest];
  }
}
let seen = 0;
for (const keys of combos(N)) { seen++; check(keys); }

console.log(`${seen} maps of ${N} keys${NEST ? ' (nested under :hover)' : ''} — disagreements: ${hits.length}`);
for (const h of hits.slice(0, 5)) {
  console.log('\n== ' + JSON.stringify(h.keys));
  console.log('-- babel:\n' + h.a);
  console.log('-- rust:\n' + h.b);
}
