// Ticket 13 -- how parenthesis nesting depth costs, per compiler.
//
//   node nesting-depth.cjs rust 2000
//   node nesting-depth.cjs babel 12
//
// One depth per process, because the reference implementation backtracks and a
// run that never returns should be a timeout rather than a hang in the sweep.
const path = require('node:path');
const { run, runRust } = require(path.join(__dirname, 'ref.cjs'));

const which = process.argv[2];
const levels = Number(process.argv[3]);
const query = `@media ${'('.repeat(levels)}min-width: 100px${')'.repeat(levels)}`;
const props = { color: { default: 'black', [query]: 'red' } };
const code = [
  "import * as stylex from '@stylexjs/stylex';",
  `export const styles = stylex.create({ x: ${JSON.stringify(props)} });`,
  '',
].join('\n');

const started = process.hrtime.bigint();
let outcome;
try {
  outcome = `${(which === 'babel' ? run : runRust)(code).rules.length} rules`;
} catch (error) {
  outcome = `refused: ${String(error?.message ?? error).split('\n')[0].slice(0, 60)}`;
}
const ms = Number(process.hrtime.bigint() - started) / 1e6;

console.log(JSON.stringify({ which, levels, ms: Math.round(ms), outcome }));
