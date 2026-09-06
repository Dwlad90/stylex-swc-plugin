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
 * It runs on the nightly schedule rather than on every pull request. Measured on
 * this repository the curated harness takes ~2.5s and this one ~97s -- roughly
 * forty times as much, because it crosses an alphabet -- and what it is looking
 * for does not arrive one commit at a time: a splitter defect shows up when a
 * value pass or the alphabet itself changes, and a sweep once a night catches
 * that as surely as one per PR would. See the README for where both are wired.
 *
 * Most of the divergent rows are not that information. They are the deliberate
 * refusals `lib/refusal-families.ts` names, reached in bulk because an alphabet
 * crossed with itself produces hundreds of spellings of each one — and a report
 * that printed them undistinguished would put a five-figure number in front of a
 * reader who then has to be told none of it matters. So a row a family accounts
 * for is *pinned*, and the count a reader acts on is the one no family claimed.
 *
 * Pinning is by family and never by row: there is no corpus file to write an
 * expectation on, and hundreds of generated values reach each refusal, so an
 * expectation per row would churn on every alphabet addition. Growing the
 * alphabet therefore costs nothing here unless it reaches a refusal genuinely
 * new — which is what the unexpected count is for.
 *
 * Usage:
 *   pnpm fuzz:shorthand                        # full cross product, summary
 *   pnpm fuzz:shorthand --show 40              # print more unexpected rows
 *   pnpm fuzz:shorthand --json out.json        # machine-readable report
 *   pnpm fuzz:shorthand --property padding     # one property; repeatable
 */

import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseArgs } from 'node:util';

import chalk from 'chalk';

import { createComparer } from './lib/compare.js';
import { subjectBlock } from './lib/compilers.js';
import { entry } from './lib/declaration.js';
import { countFlag } from './lib/flags.js';
import { answerOf, selectedOrExit, writeJsonReport } from './lib/harness-cli.js';
import { REFUSAL_FAMILIES, familyOf, groupByFamily } from './lib/refusal-families.js';
import { AGREED } from './lib/report.js';
import type { LoadedCorpusEntry, ReportEntry } from './lib/types.js';

/**
 * One member of the alphabet: what the report claims coverage of, and the
 * characters it is spelled with here.
 */
interface AlphabetEntry {
  /** The class, as the report names it. */
  readonly label: string;
  /** One spelling of that class, as the generator emits it. */
  readonly text: string;
}

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
 *
 * The test for adding one is whether it can produce a *part shape* no class
 * already here produces -- not whether it looks unusual, and not the count it
 * adds. A class crossed with the rest of the alphabet costs roughly 900
 * subjects per property, so a class that lands on a shape already covered buys
 * a longer run and nothing else.
 *
 * Four candidates were audited and dismissed, and they are recorded because the
 * argument is the useful part of an audit:
 *
 * - *An importance annotation somewhere other than the end.* Already generated:
 *   the pairs are ordered and `!important` is a class, so `!important 1px` puts
 *   the annotation ahead of a part. A value of three parts would let it sit in
 *   the middle, and that is the same part shape -- a part that is exactly the
 *   annotation, with a part after it -- which is what the fold reads.
 * - *A brace as a fragment.* A new part shape, and an unobservable one: a part
 *   that is a `{` or `}` is refused here before any expansion is emitted, so the
 *   row only ever reports the refusal the curated corpus already pins. Its
 *   sibling stayed -- see the semicolon below, which is not refused.
 * - *A unicode range next to a signed number.* Already generated, as `U+0-7F`
 *   joined to `-1px` by the `+` joiner, which is exactly the spelling where the
 *   sign is ambiguous.
 * - *An empty part for a reason other than an unterminated comment.* Audited and
 *   there is one: a *terminated* comment with nothing in it. It is a different
 *   code path from the unterminated one, so it was added rather than dismissed.
 */
const FRAGMENTS: readonly AlphabetEntry[] = [
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
  // Its own class rather than a spelling of the one above: an unterminated
  // comment contributes an *empty* part, and an expansion that folds parts
  // treats an empty one differently from a comment with text in it. A fold bug
  // that only this class reaches went unfound while the alphabet lacked it.
  { label: 'comment, unterminated', text: '/*' },
  // The other way to reach an empty part, and a different code path to it: this
  // comment is closed, so nothing about the rest of the value is swallowed and
  // the empty part has content on both sides of it. The class above can only
  // ever contribute the *last* part.
  { label: 'comment, empty', text: '/**/' },
  { label: 'importance annotation', text: '!important' },
  { label: 'bang alone', text: '!' },
  { label: 'unicode range', text: 'U+0-7F' },
  { label: 'non-ascii identifier', text: 'wörld' },
  { label: 'bracket block', text: '[a]' },
  { label: 'stray close paren', text: ')' },
  // A separator standing as a part of its own rather than between two, which is
  // a shape no joiner can reach: as a joiner a `;` ends the part before it, and
  // here it *is* the part. Neither compiler refuses it -- a `;` alone as a
  // longhand's whole value does not escape the declaration it is in -- so
  // unlike a brace the split stays observable.
  { label: 'separator as a part', text: ';' },
  // More parts than any side-wise expansion reads. Every other class is one
  // part, so a pair is at most two and nothing in the alphabet could reach the
  // fifth side an expansion has to discard, or the sixth a fold has to carry.
  { label: 'five space-separated parts', text: '1px 2px 3px 4px 5px' },
];

/**
 * What goes *between* two fragments.
 *
 * A separator is a class of its own here rather than a fragment, because the
 * defect is about which of these ends a part: `,`, `:` and `/` are structure at
 * the top level and characters inside a function, and the spacings around them
 * are what an author gets back.
 */
const JOINERS: readonly AlphabetEntry[] = [
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
] as const;

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
      --show <n>          unexpected rows to print (default 25)
      --json <path>       write the full report as JSON
      --property <name>   restrict to one property; repeatable
  -h, --help              this message
`);
  process.exit(0);
}

const properties = selectedOrExit('--property', cliOptions.property, PROPERTIES, name => name);

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

/** One alphabet half, as the line the report prints for it. */
function labelled(name: string, entries: readonly AlphabetEntry[]): string {
  // Joined on a separator no label contains. Several labels are themselves
  // comma-separated -- `dimension, trailing-zero digits`, `quoted string,
  // double` -- so a comma-joined list of them read as half again as many classes
  // as the count printed directly above it. The alphabet line is what the report
  // offers as its claim about coverage, so it has to be countable by eye.
  return chalk.dim(`  ${name}: ${entries.map(member => member.label).join(' · ')}`);
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

// `AGREED` rather than a second spelling of it. `lib/report.ts` decides what
// counts as agreement and argues there against exactly this duplicate: a verdict
// added to one list and not the other is either printed here as a divergence it
// is not, or -- the direction that matters -- dropped out of `unexpected` and so
// out of the count this run exits on.
const diverged = results.filter(result => !AGREED.has(result.verdict));

/**
 * The divergent rows, split into the ones a refusal family accounts for and the
 * ones nothing does.
 *
 * A row is asked about its family rather than about its verdict, so a row whose
 * verdict a family also produces is still news when the refusal underneath it is
 * a different one: the two are separated by *why* the compilers disagreed, not
 * by what the disagreement was called.
 */
const pinned = groupByFamily(diverged);
const unexpected = diverged.filter(result => familyOf(result) === undefined);

// A family nothing reached is deliberately *not* checked here, though it is the
// other way an expectation stops measuring and the curated harness does fail on
// it. This sweep is broader in volume but narrower in kind: it generates
// shorthand values for eight properties, so it structurally cannot produce a
// custom-property name, a lone surrogate in a key, nesting past the recursion
// budget, or a key off `Object.prototype`. Measured -- those four families go
// unreached on every run. Checking here would fail the nightly sweep for a
// reason that is a property of the alphabet rather than of the compiler, so the
// check stays with the corpus that can actually reach every family.

const byVerdict = new Map<string, number>();
for (const result of results) {
  byVerdict.set(result.verdict, (byVerdict.get(result.verdict) ?? 0) + 1);
}

// Named before the numbers, and on stdout rather than only in `--json`, because
// a run that fails has to be attributable: the upstream plugin is held by the
// lockfile rather than by an exact range, so it moves under a `pnpm update`
// without anything in this directory changing.
console.log(chalk.bold('\nSubjects'));
console.log(subjectBlock(comparer.versions));

console.log(chalk.bold('\nAlphabet'));
console.log(
  `  ${FRAGMENTS.length} token classes x ${JOINERS.length} joiners x ${properties.length} properties`
);
console.log(labelled('token classes', FRAGMENTS));
console.log(labelled('joiners', JOINERS));
console.log(chalk.dim(`  properties: ${properties.join(', ')}`));
console.log(chalk.dim(`  style resolution: legacy-expand-shorthands`));

console.log(chalk.bold('\nVerdicts'));
for (const [verdict, count] of [...byVerdict].toSorted((a, b) => b[1] - a[1])) {
  console.log(`  ${verdict.padEnd(24)} ${count}`);
}
console.log(`  ${'total'.padEnd(24)} ${results.length}`);
console.log(`  ${'divergent (any kind)'.padEnd(24)} ${diverged.length}`);
console.log(`  ${'  of those, pinned'.padEnd(24)} ${diverged.length - unexpected.length}`);
console.log(`  ${chalk.bold('  unexpected'.padEnd(24))} ${unexpected.length}`);

if (pinned.size > 0) {
  console.log(chalk.bold('\nPinned refusal families'));
  // Walked in the order `lib/refusal-families.ts` declares rather than the
  // order the cross product reaches them in, so two runs over different
  // alphabets print the same list in the same order.
  for (const family of REFUSAL_FAMILIES) {
    const claimed = pinned.get(family);
    if (claimed === undefined) continue;
    console.log(`  ${family.name.padEnd(34)} ${claimed.length}`);
  }
  console.log(chalk.dim('  reasons: parity/lib/refusal-families.ts'));
}

const show = countFlag('--show', cliOptions.show, 25, 10_000);
if (unexpected.length > 0 && show > 0) {
  console.log(chalk.bold(`\nFirst ${Math.min(show, unexpected.length)} unexpected divergences`));
  for (const result of unexpected.slice(0, show)) {
    const label =
      result.kind === 'declaration'
        ? `${result.property}: ${JSON.stringify(result.value)}`
        : result.label;
    console.log(`\n  ${chalk.yellow(label)}  ${chalk.dim(result.verdict)}`);
    console.log(`    rust  ${answerOf(result.rust)}`);
    console.log(`    babel ${answerOf(result.babel)}`);
  }
}

if (cliOptions.json != null) {
  const written = writeJsonReport(packageDir, cliOptions.json, {
    alphabet: { fragments: FRAGMENTS, joiners: JOINERS, properties },
    subjects: comparer.versions,
    summary: {
      total: results.length,
      divergent: diverged.length,
      unexpected: unexpected.length,
      byVerdict: Object.fromEntries(byVerdict),
      byFamily: Object.fromEntries(
        [...pinned].map(([family, claimed]) => [family.name, claimed.length])
      ),
    },
    // Both, and named apart: the unexpected rows are what a reader opens the
    // file for, and the pinned ones are the evidence that the count above
    // them is what it says.
    unexpected,
    pinned: Object.fromEntries([...pinned].map(([family, claimed]) => [family.name, claimed])),
  });
  console.log(chalk.dim(`\nwrote ${written}`));
}

// Non-zero on the one number a reader acts on, for the reason the curated
// harness exits non-zero on a changed expectation: a check that cannot fail is a
// log, and this one runs unattended. Every divergent row belongs to a family as
// it stands, so the count is zero and any other value is news.
if (unexpected.length > 0) {
  console.log(
    chalk.gray(
      `\n${unexpected.length} divergent row${unexpected.length === 1 ? '' : 's'} no refusal family accounts for.\n` +
        'Run `pnpm fuzz:shorthand --show 40` for the rows, or `--json <path>` for all of them\n' +
        'with the pinned ones beside them as evidence. Then either fix the split, or -- if the\n' +
        'divergence is one this compiler makes on purpose -- add a family in\n' +
        'parity/lib/refusal-families.ts, which is where a reason is stated rather than a row\n' +
        'pinned. If neither version in the subject block above moved, it was this compiler.'
    )
  );
  process.exitCode = 1;
}

console.log('');
