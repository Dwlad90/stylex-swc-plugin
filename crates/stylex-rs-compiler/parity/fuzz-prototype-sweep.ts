/**
 * Prototype sweep: `@stylexswc/rs-compiler` vs `@stylexjs/babel-plugin`, over
 * every method the fold can reach rather than over the ones someone listed.
 *
 * The fold evaluates a method call instead of matching its name against a
 * table, and the argument for that was one sentence: a table is finite by
 * construction, so the method it does not list is the next bug report. A
 * curated corpus cannot demonstrate the claim, because a curated corpus is
 * itself a table — it says what someone thought to measure. This crosses the
 * prototypes and the namespaces the engine owns, read off the language at run
 * time, with both receiver shapes, and asks the reference compiler what each
 * one should have answered.
 *
 * What a row here is *not* is where a reason gets written down. A divergence
 * this sweep finds is accounted for by naming the curated row that argues it —
 * see `lib/prototype-accounts.ts` — so the argument lives in one place and a
 * generated report never becomes a second, unreadable copy of it.
 *
 * It runs on the nightly schedule beside the shorthand sweep. What it guards
 * moves when the *surface* moves: an engine upgrade, a guard widened, a
 * receiver shape added. None of those arrive one commit at a time, and a sweep
 * once a night reports them as surely as one per pull request would. It is
 * listed in the `pr-validation` gate so a failing run fails rather than sitting
 * green beside it.
 *
 * Usage:
 *   pnpm run --filter=@stylexswc/rs-compiler build     # the harness reads dist/
 *   pnpm fuzz:prototypes                               # summary
 *   pnpm fuzz:prototypes --show 60                      # more unexpected rows
 *   pnpm fuzz:prototypes --surface Math                 # one surface; repeatable
 *   pnpm fuzz:prototypes --json parity/results/x.json   # machine-readable
 */

import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseArgs } from 'node:util';

import chalk from 'chalk';

import { createComparer } from './lib/compare.js';
import { subjectBlock } from './lib/compilers.js';
import { loadCorpus } from './lib/corpus.js';
import { countFlag } from './lib/flags.js';
import { answerOf, selectedOrExit, writeJsonReport } from './lib/harness-cli.js';
import { ACCOUNTS, accountOf, unrecorded } from './lib/prototype-accounts.js';
import type { Standing } from './lib/prototype-accounts.js';
import { SURFACES, methodsOf, sweep } from './lib/prototype-surface.js';
import type { Asked, Rejection } from './lib/prototype-surface.js';
import { REFUSAL_FAMILIES, familyOf } from './lib/refusal-families.js';
import { AGREED } from './lib/report.js';
import type { ReportEntry, Verdict } from './lib/types.js';

const parityDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.resolve(parityDir, '..');

const { values: cliOptions } = parseArgs({
  args: process.argv.slice(2).filter(argument => argument !== '--'),
  options: {
    show: { type: 'string', default: '25' },
    json: { type: 'string' },
    surface: { type: 'string', multiple: true },
    help: { type: 'boolean', short: 'h', default: false },
  },
});

if (cliOptions.help) {
  console.log(`
${chalk.bold('StyleX prototype sweep')}

Options:
      --show <n>         unexpected rows to print (default 25)
      --json <path>      write the full report as JSON
      --surface <name>   restrict to one surface; repeatable
  -h, --help             this message

Surfaces: ${SURFACES.map(surface => surface.name).join(', ')}
`);
  process.exit(0);
}

const surfaces = selectedOrExit('--surface', cliOptions.surface, SURFACES, one => one.name);

/** One compared row, with the method and shape that produced it. */
interface Row {
  readonly asked: Asked;
  readonly entry: ReportEntry;
  readonly standing: Standing;
}

/**
 * Where a row stands: agreement, a family, an account, or news.
 *
 * The families are asked first. They are the harness-wide list of divergences
 * this compiler makes on purpose, and a folded value can trip one of them
 * without the fold being what diverged — a fold answering a string with a `;`
 * in it, say. An account is the narrower statement, about the fold's own
 * refusals, so it reads what the families left.
 */
function standingOf(entry: ReportEntry): Standing {
  if (AGREED.has(entry.verdict)) return { kind: 'agreed' };

  const family = familyOf(entry);
  if (family !== undefined) return { kind: 'pinned', family };

  const account = accountOf(entry);

  return account === undefined ? { kind: 'unexpected' } : { kind: 'accounted', account };
}

const comparer = await createComparer({
  packageDir,
  enableFontSizePxToRem: false,
  // Named rather than left to the default, for the reason `lib/compare.ts`
  // gives: a report that does not say which resolution it measured cannot be
  // compared with another one. Nothing here depends on it — every subject
  // declares one longhand — which is why the default is the safe choice.
  styleResolution: 'property-specificity',
});

const generated = sweep(surfaces);
const rows: Row[] = generated.asked.map(asked => {
  const entry = comparer.compare(asked.subject);

  return { asked, entry, standing: standingOf(entry) };
});

const unexpected = rows.filter(row => row.standing.kind === 'unexpected');

const byVerdict = new Map<Verdict, number>();
for (const row of rows) {
  byVerdict.set(row.entry.verdict, (byVerdict.get(row.entry.verdict) ?? 0) + 1);
}

/**
 * `items` gathered under the key each one answers, skipping those with none.
 *
 * One helper rather than the same six-line get-or-insert three times: two of the
 * three groupings below were written out by hand, and the third would have been.
 */
function groupedBy<Key, Item>(
  items: Iterable<Item>,
  keyOf: (item: Item) => Key | undefined
): Map<Key, Item[]> {
  const grouped = new Map<Key, Item[]>();
  for (const item of items) {
    const key = keyOf(item);
    if (key === undefined) continue;
    const found = grouped.get(key);
    if (found === undefined) grouped.set(key, [item]);
    else found.push(item);
  }

  return grouped;
}

const byAccount = groupedBy(rows, row =>
  row.standing.kind === 'accounted' ? row.standing.account : undefined
);

// The link every account rests on, checked against the corpus rather than
// assumed: an account states that a reason is written down somewhere, and a row
// that has been deleted or has lost its note takes the reason with it while the
// count above it goes on reading as accounted for.
const broken = unrecorded(loadCorpus(path.join(parityDir, 'corpus')));

// Named before the numbers, and on stdout rather than only in `--json`: the
// reference plugin is held by the lockfile rather than by an exact range, so it
// moves under a `pnpm update` without anything in this directory changing, and
// a report that does not name it cannot be compared with an older one.
console.log(chalk.bold('\nSubjects'));
console.log(
  subjectBlock(comparer.versions, [
    ['surfaces', surfaces.map(surface => surface.name).join(', ')],
    ['methods read off them', String(surfaces.reduce(counted, 0))],
    ['style resolution', String(comparer.options.styleResolution)],
  ])
);

function counted(total: number, surface: (typeof SURFACES)[number]): number {
  return total + methodsOf(surface).length;
}

console.log(chalk.bold('\nCoverage'));
console.log(`  ${'methods exercised'.padEnd(26)} ${generated.exercised.length}`);
console.log(`  ${'subjects'.padEnd(26)} ${rows.length}`);
console.log(
  `  ${'methods not exercised'.padEnd(26)} ${generated.unexercised.length}   ${chalk.gray(
    '(what stopped each one is listed below)'
  )}`
);

console.log(chalk.bold('\nVerdicts'));
for (const [verdict, count] of [...byVerdict].toSorted((left, right) => right[1] - left[1])) {
  console.log(`  ${verdict.padEnd(26)} ${count}`);
}
console.log(`  ${'total'.padEnd(26)} ${rows.length}`);
console.log(`  ${chalk.bold('unexpected'.padEnd(26))} ${unexpected.length}`);

if (byAccount.size > 0) {
  console.log(chalk.bold('\nAccounted divergences'));
  // Walked in declaration order rather than in the order the surfaces reached
  // them, so two runs over different surface selections print the same list in
  // the same order.
  for (const account of ACCOUNTS) {
    const claimed = byAccount.get(account);
    if (claimed === undefined) continue;
    console.log(
      `  ${account.name.padEnd(44)} ${String(claimed.length).padStart(3)}  ${chalk.gray(account.recordedBy)}`
    );
  }
  console.log(chalk.dim('  reasons: parity/corpus/modules.json, at the ids above'));
}

const pinned = rows.filter(row => row.standing.kind === 'pinned');
if (pinned.length > 0) {
  console.log(chalk.bold('\nPinned refusal families'));
  for (const family of REFUSAL_FAMILIES) {
    const claimed = pinned.filter(
      row => row.standing.kind === 'pinned' && row.standing.family === family
    );
    if (claimed.length === 0) continue;
    console.log(`  ${family.name.padEnd(44)} ${String(claimed.length).padStart(3)}`);
  }
  console.log(chalk.dim('  reasons: parity/lib/refusal-families.ts'));
}

/**
 * The methods the sweep could not ask, grouped by what stopped it.
 *
 * Printed on every run rather than only on a failure, because it is the honest
 * half of the coverage claim: a sweep that quietly dropped a third of a
 * prototype would report the same clean number as one that crossed all of it.
 */
const REJECTIONS = {
  threw: 'every argument list threw',
  unusable: 'answers a value no declaration carries',
  nondeterministic: 'answers differently on two evaluations',
} as const satisfies Record<Rejection, string>;

if (generated.unexercised.length > 0) {
  console.log(chalk.bold('\nMethods not exercised'));
  const byRejection = groupedBy(generated.unexercised, one => one.rejection);
  for (const [rejection, reason] of Object.entries(REJECTIONS) as [Rejection, string][]) {
    const found = byRejection.get(rejection);
    if (found === undefined) continue;
    console.log(`  ${chalk.dim(reason)}`);
    for (const one of found) {
      console.log(`    ${one.surface}.${one.method}  ${chalk.gray(one.detail)}`);
    }
  }
}

const show = countFlag('--show', cliOptions.show, 25, 10_000);
if (unexpected.length > 0 && show > 0) {
  const shown = Math.min(show, unexpected.length);
  console.log(
    chalk.bold(
      `\nUnexpected divergences${shown < unexpected.length ? `, the first ${shown} of ${unexpected.length}` : ''}`
    )
  );
  for (const row of unexpected.slice(0, show)) {
    console.log(
      `\n  ${chalk.yellow(`${row.asked.surface}.${row.asked.method}`)}  ` +
        `${row.asked.shape}  ${chalk.dim(row.entry.verdict)}`
    );
    console.log(`    subject     ${row.asked.subject.label}`);
    console.log(`    javascript  ${row.asked.expression} => ${row.asked.value}`);
    console.log(`    rust        ${answerOf(row.entry.rust)}`);
    console.log(`    babel       ${answerOf(row.entry.babel)}`);
  }
}

if (cliOptions.json != null) {
  const written = writeJsonReport(packageDir, cliOptions.json, {
    subjects: comparer.versions,
    surfaces: surfaces.map(surface => ({
      name: surface.name,
      methods: methodsOf(surface),
    })),
    summary: {
      total: rows.length,
      exercised: generated.exercised.length,
      unexercised: generated.unexercised.length,
      unexpected: unexpected.length,
      byVerdict: Object.fromEntries(byVerdict),
      byAccount: Object.fromEntries(
        [...byAccount].map(([account, claimed]) => [account.name, claimed.length])
      ),
    },
    unexercised: generated.unexercised,
    // Both, and named apart: the unexpected rows are what a reader opens the
    // file for, and the accounted ones are the evidence that the count above
    // them is what it says.
    unexpected: unexpected.map(row => ({ ...row.asked, ...row.entry })),
    accounted: Object.fromEntries(
      [...byAccount].map(([account, claimed]) => [
        account.name,
        { recordedBy: account.recordedBy, rows: claimed.map(row => row.asked.subject.id) },
      ])
    ),
  });
  console.log(chalk.dim(`\nwrote ${written}`));
}

/**
 * Three ways out non-zero, and each is a report that has stopped being read.
 *
 * A divergence nothing accounts for is the one the sweep exists to find. An
 * account whose corpus row no longer carries its reason is the same failure the
 * curated harness's unreached-family check catches, read from the other end. And
 * a run that exercised nothing at all is the failure mode a generated harness is
 * most prone to: the surface changes shape, every candidate stops answering, and
 * a green run reports agreement about no method whatsoever.
 */
if (generated.exercised.length === 0) {
  console.error(
    chalk.red(
      '\nNo method was exercised, so this run measured nothing. That is a failure: the argument\n' +
        'pool in `parity/lib/prototype-surface.ts` no longer answers for any method on the\n' +
        'selected surfaces.'
    )
  );
  process.exitCode = 1;
}

if (broken.length > 0) {
  console.error(chalk.red.bold('\nAccounts whose reason is no longer recorded'));
  for (const one of broken) {
    console.error(`  ${one.account.name}  →  ${one.account.recordedBy}: ${one.problem}`);
  }
  console.error(
    chalk.gray(
      '\nAn account says a curated corpus row carries the argument for a divergence this sweep\n' +
        'expects. Either restore the row in parity/corpus/modules.json, or delete the account in\n' +
        'parity/lib/prototype-accounts.ts and let its rows report as unexpected — which is what\n' +
        'they are once nothing states why they are wanted.'
    )
  );
  process.exitCode = 1;
}

if (unexpected.length > 0) {
  console.log(
    chalk.gray(
      `\n${unexpected.length} divergent row${unexpected.length === 1 ? '' : 's'} nothing accounts for. Each names a method, the receiver\n` +
        'shape it was asked in, and what both compilers answered. A row where this compiler\n' +
        'refused and the reference compiled costs an author a build, so it is either a gap to\n' +
        'close or a refusal to argue for: record it in parity/corpus/modules.json with the\n' +
        'reason, and add the account in parity/lib/prototype-accounts.ts that names that row.\n' +
        'If neither version in the subject block above moved, it was this compiler.'
    )
  );
  process.exitCode = 1;
}

console.log('');
