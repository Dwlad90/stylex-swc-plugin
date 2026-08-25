// Ticket 09 -- the three comparisons the sweep is made of. Prints a report.
//
//   node sweep.cjs
const path = require('node:path');
const { run, runRust } = require(path.join(__dirname, 'ref.cjs'));

const module_ = props => [
  "import * as stylex from '@stylexjs/stylex';",
  `export const styles = stylex.create({ x: ${JSON.stringify(props)} });`,
  '',
].join('\n');

function compile(fn, code, options) {
  try {
    const { rules } = fn(code, options);
    return { ok: true, rules: rules.map(r => [r[0], r?.[1]?.ltr ?? '', r?.[1]?.priority]) };
  } catch (error) {
    return { ok: false, message: String(error?.message ?? error).split('\n').filter(Boolean)[0] };
  }
}

function compare(label, props, options) {
  const code = module_(props);
  const babel = compile(run, code, options);
  const rust = compile(runRust, code, options);

  const shape = r => (r.ok ? r.rules.map(x => `${x[0]} ${x[1]}`).join('\n      ') : `REJECTED: ${r.message}`);
  const agree = JSON.stringify(babel.ok ? babel.rules.map(r => r[1]) : babel.ok)
    === JSON.stringify(rust.ok ? rust.rules.map(r => r[1]) : rust.ok);

  console.log(`\n${agree ? 'AGREE   ' : 'DIFFER  '} ${label}`);
  console.log(`    babel ${shape(babel)}`);
  console.log(`    rust  ${shape(rust)}`);
  return agree;
}

let agreements = 0;
let total = 0;
const check = (...args) => { total++; if (compare(...args)) agreements++; };

console.log('== 1. at-rule order, rewritten media keys beside other at-rules');

check('media ladder beside @supports and @container', {
  color: {
    default: 'black',
    '@supports (display: grid)': 'green',
    '@media (min-width: 1440px)': 'c1',
    '@media (min-width: 1200px) and (max-width: 1439px)': 'c2',
    '@container (min-width: 400px)': 'teal',
    '@media (max-width: 479px)': 'c6',
  },
});

check('a rewritten key that sorts before its authored spelling', {
  color: {
    default: 'black',
    '@media (min-width: 200px)': 'a',
    '@media (min-width: 100px)': 'b',
    '@supports (color: red)': 'c',
  },
});

check('rewritten media keys nested under a pseudo-class', {
  color: {
    default: 'black',
    ':hover': {
      default: 'grey',
      '@media (min-width: 200px)': 'a',
      '@media (min-width: 100px)': 'b',
    },
  },
});

check('non-media properties keep their place beside rewritten keys', {
  padding: '10px',
  color: {
    default: 'black',
    '@media (min-width: 200px)': 'a',
    '@media (min-width: 100px)': 'b',
  },
  margin: '2px',
});

console.log('\n== 2. which inputs the invalid-syntax refusal fires on');

const MALFORMED = [
  ['unclosed paren', '@media (min-width: 100px'],
  ['unbalanced closing paren', '@media min-width: 100px)'],
  ['empty condition', '@media ()'],
  ['bare colon', '@media (:)'],
  ['missing value', '@media (min-width:)'],
  ['double and', '@media (min-width: 100px) and and (max-width: 200px)'],
  ['trailing and', '@media (min-width: 100px) and'],
  ['leading and', '@media and (min-width: 100px)'],
  ['unclosed string', '@media (min-width: "100px)'],
  ['unclosed function', '@media (width: calc(100px)'],
  ['nested unclosed', '@media ((min-width: 100px)'],
  ['comma only', '@media ,'],
  ['garbage', '@media ???'],
  ['escaped ident', '@media (min-\\77 idth: 100px)'],
  ['astral plane', '@media (min-width: 100px) and (\u{1D400}: 1)'],
  ['combined operators', '@media (min-width: 100px) and (max-width: 200px) or (color)'],
  ['bare not', '@media not'],
  ['only without type', '@media only'],
  ['negative width', '@media (min-width: -100px)'],
  ['no unit', '@media (min-width: 100)'],
  ['double parens', '@media ((min-width: 100px))'],
  // Four levels, not forty: the reference implementation's parser backtracks
  // exponentially in nesting depth -- 8 levels take 1.2 s, 12 take 20 s, and 16
  // do not finish. That is recorded in the report rather than measured here.
  ['four levels of parens', `@media ${'('.repeat(4)}min-width: 100px${')'.repeat(4)}`],
];

for (const [label, query] of MALFORMED) {
  check(`refusal: ${label}`, { color: { default: 'black', [query]: 'red', '@media (max-width: 50px)': 'blue' } });
}

console.log('\n== 3. the ordering option');

const LADDER = {
  color: {
    default: 'black',
    '@media (min-width: 1440px)': 'c1',
    '@media (min-width: 1200px) and (max-width: 1439px)': 'c2',
    '@media (max-width: 479px)': 'c6',
  },
};

check('default (option unset)', LADDER);
check('explicitly enabled', LADDER, { enableMediaQueryOrder: true });
check('opted out', LADDER, { enableMediaQueryOrder: false });
check('opted out keeps the authored spelling', {
  color: { default: 'black', '@media (max-height:120px) and (min-width: 720px)': 'blue' },
}, { enableMediaQueryOrder: false });

console.log(`\n== ${agreements} of ${total} comparisons agree`);
