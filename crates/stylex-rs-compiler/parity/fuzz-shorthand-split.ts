/**
 * Shorthand-split fuzz: `@stylexswc/rs-compiler` vs `@stylexjs/babel-plugin`.
 *
 * The value-parity harness next door asks about one declaration at a time from
 * a curated corpus. This one asks the same question over a generated corpus,
 * because the defect it exists to find is combinatorial: a shorthand value is
 * cut into parts, and where the cut falls depends on which separator, which
 * spacing and which token shape sit next to each other. Hand-picked probes
 * repeatedly under-reported that surface -- a probe confirms a case, and only
 * an alphabet crossed with itself finds the case nobody thought of.
 *
 * The alphabet is the point of the report. What this harness can claim is the
 * token classes it crossed, never "the splitter is correct" -- so it prints the
 * alphabet beside the count, and a reader compares the two.
 *
 * It is not wired into CI, for the same reason the value harness is not: a
 * divergence is information, and reading it is a person's job.
 *
 * Usage:
 *   pnpm fuzz:shorthand                        # full cross product, summary
 *   pnpm fuzz:shorthand --show 40              # print more divergent rows
 *   pnpm fuzz:shorthand --json out.json        # machine-readable report
 *   pnpm fuzz:shorthand --property padding     # one property; repeatable
 */

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseArgs } from 'node:util';

import chalk from 'chalk';

import { createComparer } from './lib/compare.js';
import { entry } from './lib/declaration.js';
import type { CompilerOutcome, LoadedCorpusEntry, ReportEntry } from './lib/types.js';

const parityDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.resolve(parityDir, '..');

/**
 * The token classes the generated values are built from.
 *
 * Each entry is one class, not one value: the class is what the report claims
 * coverage of, and the sample beside it is only how that class is spelled here.
 * A class is listed once even where several spellings of it exist, because a
 * second spelling of an already-covered class buys a longer run and no new
 * information -- except where the spelling *is* the class, which is why the
 * separators and the spacings are enumerated one by one.
 */
const FRAGMENTS: { label: string; text: string }[] = [
  { label: 'dimension', text: '1px' },
  { label: 'dimension, trailing-zero digits', text: '1.50px' },
  { label: 'dimension, exponent notation', text: '1e2px' },
  { label: 'dimension, escaped unit', text: '1\\70x' },
  { label: 'percentage', text: '50%' },
  { label: 'bare number', text: '1.5' },
  { label: 'signed dimension', text: '-1px' },
  { label: 'keyword', text: 'auto' },
  { label: 'escaped identifier', text: 'A\\42 C' },
  { label: 'hex colour', text: '#007bff' },
  { label: 'quoted string, double', text: '"a"' },
  { label: 'quoted string, single', text: "'a'" },
  { label: 'custom property reference', text: 'var(--x)' },
  { label: 'function, comma-separated arguments', text: 'min(1px,2px)' },
  { label: 'function, spaced comma', text: 'min(1px , 2px)' },
  { label: 'function, spaced operator', text: 'calc(1px + 2px)' },
  { label: 'function, unspaced operator', text: 'calc(1.50px*2)' },
  { label: 'function, spaced slash', text: 'calc(100% / 3)' },
  { label: 'function, unspaced slash', text: 'calc(100%/3)' },
  { label: 'function, adjacent signs', text: 'calc(2px-1px)' },
  { label: 'function, doubled spacing', text: 'calc(1px  +  2px)' },
  { label: 'function, inner padding', text: 'calc( 1px + 2px )' },
  { label: 'nested function', text: 'calc(var(--x) + 1px)' },
  { label: 'unclosed function', text: 'calc(1px' },
  { label: 'unclosed string', text: '"a' },
  { label: 'url token, unquoted', text: 'url(a.png)' },
  { label: 'comment', text: '/*c*/' },
  { label: 'importance annotation', text: '!important' },
  { label: 'bang alone', text: '!' },
  { label: 'unicode range', text: 'U+0-7F' },
  { label: 'non-ascii identifier', text: 'wörld' },
  { label: 'bracket block', text: '[a]' },
  { label: 'stray close paren', text: ')' },
];

/**
 * What goes *between* two fragments.
 *
 * A separator is a class of its own here rather than a fragment, because the
 * defect is about which of these ends a part: `,`, `:` and `/` are structure at
 * the top level and characters inside a function, and the spacings around them
 * are what an author gets back.
 */
const JOINERS: { label: string; text: string }[] = [
  { label: 'single space', text: ' ' },
  { label: 'doubled space', text: '  ' },
  { label: 'newline', text: '\n' },
  { label: 'tab', text: '\t' },
  { label: 'nothing', text: '' },
  { label: 'slash, unspaced', text: '/' },
  { label: 'slash, spaced', text: ' / ' },
  { label: 'comma, unspaced', text: ',' },
  { label: 'comma, spaced', text: ' , ' },
  { label: 'colon, unspaced', text: ':' },
  { label: 'colon, spaced', text: ' : ' },
  { label: 'semicolon', text: ';' },
  { label: 'asterisk', text: '*' },
  { label: 'plus', text: '+' },
];

/**
 * The properties whose expansion cuts a value into parts.
 *
 * Chosen for the arities the splitter is read at -- four parts, two, and the
 * two properties that reduce the part list themselves rather than destructure
 * it -- so that a mis-cut shows up as a wrong side, a wrong axis, and a wrong
 * reduction rather than only ever as the first.
 */
const PROPERTIES = [
  'padding',
  'margin',
  'inset',
  'borderRadius',
  'gap',
  'insetInline',
  'listStyle',
  'containIntrinsicSize',
];

const { values: cliOptions } = parseArgs({
  args: process.argv.slice(2).filter(arg => arg !== '--'),
  options: {
    show: { type: 'string', default: '25' },
    json: { type: 'string' },
    property: { type: 'string', multiple: true },
    help: { type: 'boolean', short: 'h', default: false },
  },
});

if (cliOptions.help) {
  console.log(`
${chalk.bold('StyleX shorthand-split fuzz')}

Options:
      --show <n>          divergent rows to print (default 25)
      --json <path>       write the full report as JSON
      --property <name>   restrict to one property; repeatable
  -h, --help              this message
`);
  process.exit(0);
}

const selected = cliOptions.property;
const properties =
  selected != null && selected.length > 0
    ? PROPERTIES.filter(name => selected.includes(name))
    : PROPERTIES;

/**
 * Every generated value: one fragment alone, and every ordered pair of
 * fragments with every joiner between them.
 *
 * Ordered rather than unordered, and self-pairs included: which side of a
 * separator a token sits on changes how it is scanned, and `1px/1px` asks
 * something `1px 1px` does not.
 */
function values(): string[] {
  const generated: string[] = FRAGMENTS.map(fragment => fragment.text);

  for (const left of FRAGMENTS) {
    for (const right of FRAGMENTS) {
      for (const joiner of JOINERS) {
        generated.push(`${left.text}${joiner.text}${right.text}`);
      }
    }
  }

  return [...new Set(generated)];
}

function corpus(): LoadedCorpusEntry[] {
  const generated = values();
  const entries: LoadedCorpusEntry[] = [];

  for (const property of properties) {
    for (const value of generated) {
      entries.push({
        kind: 'declaration',
        set: 'fuzz-shorthand-split',
        ...entry(property, value, 'parity/fuzz-shorthand-split.ts (generated)'),
      });
    }
  }

  return entries;
}

function describe(outcome: CompilerOutcome): string {
  return outcome.status === 'error'
    ? `refused: ${outcome.sentence}`
    : outcome.declarations.join(' | ');
}

// `legacy-expand-shorthands` is not a variation here, it is the subject: the
// value splitter runs only under that resolution, and under either of the
// other two `padding` reaches the stylesheet whole. A run left on the default
// reports agreement about code neither compiler executed.
const comparer = await createComparer({
  packageDir,
  enableFontSizePxToRem: false,
  styleResolution: 'legacy-expand-shorthands',
});

const entries = corpus();
const results: ReportEntry[] = entries.map(subject => comparer.compare(subject));

const agreed = new Set(['identical', 'identical-empty', 'both-reject']);
const diverged = results.filter(result => !agreed.has(result.verdict));

const byVerdict = new Map<string, number>();
for (const result of results) {
  byVerdict.set(result.verdict, (byVerdict.get(result.verdict) ?? 0) + 1);
}

console.log(chalk.bold('\nAlphabet'));
console.log(
  `  ${FRAGMENTS.length} token classes x ${JOINERS.length} joiners x ${properties.length} properties`
);
console.log(chalk.dim(`  token classes: ${FRAGMENTS.map(f => f.label).join(', ')}`));
console.log(chalk.dim(`  joiners: ${JOINERS.map(j => j.label).join(', ')}`));
console.log(chalk.dim(`  properties: ${properties.join(', ')}`));
console.log(chalk.dim(`  style resolution: legacy-expand-shorthands`));

console.log(chalk.bold('\nVerdicts'));
for (const [verdict, count] of [...byVerdict].toSorted((a, b) => b[1] - a[1])) {
  console.log(`  ${verdict.padEnd(24)} ${count}`);
}
console.log(`  ${'total'.padEnd(24)} ${results.length}`);
console.log(`  ${chalk.bold('divergent (any kind)'.padEnd(24))} ${diverged.length}`);

const show = Number.parseInt(cliOptions.show ?? '25', 10);
if (diverged.length > 0 && show > 0) {
  console.log(chalk.bold(`\nFirst ${Math.min(show, diverged.length)} divergences`));
  for (const result of diverged.slice(0, show)) {
    const label =
      result.kind === 'declaration'
        ? `${result.property}: ${JSON.stringify(result.value)}`
        : result.label;
    console.log(`\n  ${chalk.yellow(label)}  ${chalk.dim(result.verdict)}`);
    console.log(`    rust  ${describe(result.rust)}`);
    console.log(`    babel ${describe(result.babel)}`);
  }
}

if (cliOptions.json != null) {
  const target = path.resolve(process.cwd(), cliOptions.json);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(
    target,
    `${JSON.stringify(
      {
        alphabet: { fragments: FRAGMENTS, joiners: JOINERS, properties },
        subjects: comparer.versions,
        summary: {
          total: results.length,
          divergent: diverged.length,
          byVerdict: Object.fromEntries(byVerdict),
        },
        divergences: diverged,
      },
      null,
      2
    )}\n`
  );
  console.log(chalk.dim(`\nwrote ${target}`));
}

console.log('');
