// Ticket 13 -- every combinator shape, through both compilers.
//
//   node grammar-sweep.cjs
//
// A row is the query text each compiler emitted for the same authored key, so a
// disagreement is either an acceptance difference or -- worse -- two different
// rules under two different class names.
const path = require('node:path');
const { run, runRust } = require(path.join(__dirname, 'ref.cjs'));

// Features the range merge cannot read, so the emitted text keeps the shape the
// parse gave it. Width bounds would be intersected into one interval and most
// of these shapes would collapse to the same string, which would make "agree"
// mean far less than it looks.
const A = process.env.MERGEABLE ? '(min-width: 1px)' : '(orientation: portrait)';
const B = process.env.MERGEABLE ? '(min-width: 2px)' : '(monochrome)';
const C = process.env.MERGEABLE ? '(min-width: 3px)' : '(hover: hover)';

const QUERIES = [
  // one operand, wrapped to increasing depth
  A,
  `(${A})`,
  `((${A}))`,
  `((((${A}))))`,
  // pure and, pure or
  `${A} and ${B}`,
  `${A} or ${B}`,
  `${A} and ${B} and ${C}`,
  `${A} or ${B} or ${C}`,
  // mixed at one level, both orders
  `${A} and ${B} or ${C}`,
  `${A} or ${B} and ${C}`,
  // mixed, disambiguated by parentheses
  `(${A} and ${B}) or ${C}`,
  `(${A} or ${B}) and ${C}`,
  `${A} and (${B} or ${C})`,
  `${A} or (${B} and ${C})`,
  // a parenthesized combinator on its own
  `(${A} and ${B})`,
  `(${A} or ${B})`,
  // negation over a combinator
  `not (${A} and ${B})`,
  `not (${A} or ${B})`,
  `(not ${A}) and ${B}`,
  `(not ${A}) or ${B}`,
  // comma segments, each internally consistent
  `${A}, ${B}`,
  `${A} and ${B}, ${C}`,
  `${A} and ${B}, ${C} or ${A}`,
  `${A} or ${B}, ${C} and ${A}`,
  // a media type beside a condition
  `screen and ${A}`,
  `screen and ${A} and ${B}`,
  `screen, ${A} or ${B}`,
  `only screen and ${A}`,
  `not screen and ${A}`,
  // shapes the mixing refusal could plausibly over-refuse
  `screen and ${A} or ${B}`,
  `not ${A} or ${B}`,
  `not ${A} and ${B}`,
  `${A} and (${B} and ${C})`,
  `${A} or (${B} or ${C})`,
  `(${A} and ${B}) and ${C}`,
  `(${A} or ${B}) or ${C}`,
  `((${A} and ${B}) or ${C}) and ${A}`,
  `${A} and ${B} and ${C} and ${A}`,
  `${A} or ${B} or ${C} or ${A}`,
  `${A}, ${B} and ${C}, ${A} or ${B}`,
  `(not ${A}) and (not ${B})`,
  `(not ${A}) or (not ${B})`,
  `not (${A} and (${B} or ${C}))`,
  // a bare `not` is the whole condition, in either position
  `not ${A}`,
  `${B} or not ${A}`,
  `${B} and not ${A}`,
  `not ${A} or not ${B}`,
  // the one position a bare `not` is a query, and one operand past it
  `screen and not ${A}`,
  `not screen and not ${A}`,
  `screen and ${A} and not ${B}`,
];

function compile(fn, query) {
  const props = { color: { default: 'black', [`@media ${query}`]: 'red', '@media (max-width: 50px)': 'blue' } };
  const code = `import * as stylex from '@stylexjs/stylex';\nexport const styles = stylex.create({ x: ${JSON.stringify(props)} });\n`;
  try {
    const first = fn(code).rules
      .map(r => (r?.[1]?.ltr ?? '').match(/@media[^{]*/g))
      .filter(Boolean)[0];
    return (first?.[0] ?? '').trim();
  } catch {
    return 'REFUSED';
  }
}

const rows = QUERIES.map(query => {
  const babel = compile(run, query);
  const rust = compile(runRust, query);
  let verdict;
  if (babel === rust) verdict = 'agree';
  else if (babel === 'REFUSED') verdict = 'we accept, upstream refuses';
  else if (rust === 'REFUSED') verdict = 'we refuse, upstream accepts';
  else verdict = 'BOTH ACCEPT, DIFFERENT TEXT';
  return { query, babel, rust, verdict };
});

for (const row of rows) {
  console.log(`\n${row.verdict === 'agree' ? 'agree' : row.verdict.toUpperCase()}  @media ${row.query}`);
  if (row.verdict !== 'agree') {
    console.log(`    upstream  ${row.babel}`);
    console.log(`    here      ${row.rust}`);
  }
}

const counts = rows.reduce((acc, r) => ({ ...acc, [r.verdict]: (acc[r.verdict] ?? 0) + 1 }), {});
console.log(`\n${rows.length} shapes (${process.env.MERGEABLE ? 'mergeable widths' : 'features the merge cannot read'}):`, JSON.stringify(counts, null, 0));
