/**
 * CSS value parity harness: `@stylexswc/rs-compiler` vs `@stylexjs/babel-plugin`.
 *
 * A StyleX class name is a hash of the canonical declaration text, which makes
 * that text a compatibility contract between the two compilers. This harness
 * runs a corpus of CSS declarations through both and reports, per declaration,
 * whether they agree byte for byte.
 *
 * A few entries carry a whole module rather than a declaration, for questions
 * a declaration cannot ask — see `ModuleEntry` in `lib/types.ts`.
 *
 * It lives outside the Rust test suite so `cargo test` never needs a Node
 * toolchain, and it is not wired into CI. Reading a verdict is still a person's
 * job — a divergence is information, not a failure — with one exception: an
 * entry whose recorded `expected` verdict no longer holds exits non-zero, so a
 * pinned divergence cannot change unnoticed by whoever runs this.
 *
 * Usage:
 *   pnpm parity                              # full corpus, human report
 *   pnpm parity --only-mismatches            # just the divergences
 *   pnpm parity --set reported               # one corpus set; repeatable
 *   pnpm parity --filter calc                # entries whose subject contains it
 *   pnpm parity --json parity/results/x.json # machine-readable report
 *   pnpm parity --font-size-px-to-rem        # both compilers with the option on
 */

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseArgs } from 'node:util';

import chalk from 'chalk';

import { createComparer, styleObjectsAgree } from './lib/compare.js';
import { loadCorpus } from './lib/corpus.js';
import { subjectLabel, subjectText } from './lib/subject.js';
import type { Report, ReportEntry, Verdict } from './lib/types.js';

const parityDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.resolve(parityDir, '..');
const workspaceRoot = path.resolve(packageDir, '../..');

const { values: cliOptions } = parseArgs({
  args: process.argv.slice(2).filter(arg => arg !== '--'),
  options: {
    'only-mismatches': { type: 'boolean', default: false },
    set: { type: 'string', multiple: true },
    filter: { type: 'string' },
    json: { type: 'string' },
    'font-size-px-to-rem': { type: 'boolean', default: false },
    help: { type: 'boolean', short: 'h', default: false },
  },
});

if (cliOptions.help) {
  console.log(`
${chalk.bold('StyleX CSS value parity harness')}

Options:
      --only-mismatches         report only divergent declarations
      --set <name>              limit to a corpus set
                                (reported|modules|edge|harvested); repeatable
      --filter <substring>      limit to entries whose subject text contains it
      --json <path>             also write the full machine-readable report
      --font-size-px-to-rem     enable the font-size conversion in both compilers
  -h, --help                    show this help
`);
  process.exit(0);
}

const VERDICT_LABELS: Record<Verdict, string> = {
  identical: chalk.green('identical'),
  'identical-empty': chalk.yellow('identical (nothing emitted)'),
  divergent: chalk.red('divergent'),
  'structurally-divergent': chalk.magenta('structurally divergent'),
  'both-reject': chalk.gray('both reject'),
  'both-reject-divergent': chalk.cyan('both reject (diverged)'),
  'acceptance-divergent': chalk.yellow('acceptance divergent'),
};

/**
 * The verdicts where the two compilers agreed, whatever they agreed about.
 *
 * One set rather than two spellings of the same list: `--only-mismatches`
 * filters on it and the per-entry printer skips its side-by-side detail on it,
 * and a verdict added to one place but not the other is either hidden from the
 * filter or printed as two empty lines.
 *
 * `identical-empty` belongs here. Both compilers accepted and emitted nothing,
 * which is agreement — that it measures nothing is a fact about the corpus
 * rather than about parity, and the summary reports it on its own line where a
 * count is the useful form. Listing it as a mismatch would overload the word
 * for the one verdict that is not a disagreement.
 *
 * `both-reject-divergent` does not. Two refusals worded differently are the
 * only thing that separates it from `both-reject`, and that difference is the
 * whole of what an author whose build stopped is handed — so it is a mismatch
 * to chase, and its two sentences are what the entry prints.
 */
const AGREED: ReadonlySet<Verdict> = new Set<Verdict>([
  'identical',
  'identical-empty',
  'both-reject',
]);

function isMismatch(verdict: Verdict): boolean {
  return !AGREED.has(verdict);
}

/**
 * How an entry's verdict stands against the one it recorded as expected.
 *
 * `expected` where it still holds, `changed` where it does not, and `unset` for
 * the overwhelming majority carrying no expectation at all. A changed verdict
 * is the loud case in both directions: a divergence that has gone away is a
 * corpus row that has stopped measuring what it was written for, exactly as a
 * new one is a regression.
 */
function expectation(entry: ReportEntry): 'expected' | 'changed' | 'unset' {
  if (entry.expected === undefined) return 'unset';

  return entry.verdict === entry.expected ? 'expected' : 'changed';
}

function describe(entry: ReportEntry, side: 'rust' | 'babel'): string {
  const outcome = entry[side];
  // The normalized sentence rather than the raw message: the raw one carries a
  // code frame on one side and a repaired rule on the other, which is many
  // lines of where-it-happened around the one line that says what the compiler
  // objected to. The raw message is in `--json` for whoever needs it.
  if (outcome.status === 'error') return chalk.gray(`rejected: ${outcome.sentence}`);
  const declarations = outcome.declarations.map(declaration => `{${declaration}}`).join(' ');
  // The style objects are printed only when they are what differ. On a value
  // divergence they are noise, and on a divergence that shows in the CSS the
  // declarations above already say it.
  if (styleObjectsAgree(entry.rust, entry.babel)) return declarations;
  const objects = chalk.gray(`style objects: ${outcome.styleObjects.join(' ')}`);
  return declarations === '' ? objects : `${declarations}   ${objects}`;
}

async function run(): Promise<void> {
  let corpus = loadCorpus(path.join(parityDir, 'corpus'));

  if (cliOptions.set !== undefined && cliOptions.set.length > 0) {
    const wanted = new Set(cliOptions.set);
    corpus = corpus.filter(entry => wanted.has(entry.set));
  }
  if (cliOptions.filter !== undefined) {
    const needle = cliOptions.filter;
    corpus = corpus.filter(entry => subjectText(entry).includes(needle));
  }
  if (corpus.length === 0) {
    console.error(chalk.red('No corpus entries match the given filters.'));
    process.exit(1);
  }

  const comparer = await createComparer({
    packageDir,
    enableFontSizePxToRem: cliOptions['font-size-px-to-rem'],
  });

  console.log(
    `${chalk.bold('Subjects')}\n` +
      `  @stylexswc/rs-compiler   v${comparer.versions.rust.version}\n` +
      `  @stylexjs/babel-plugin   v${comparer.versions.babel.version}\n` +
      `  @babel/core              v${comparer.versions.babelCore}\n` +
      `  options                  ${JSON.stringify(comparer.options)}\n`
  );

  const entries = corpus.map(entry => comparer.compare(entry));

  const summary = {
    total: entries.length,
    expected: 0,
    changed: 0,
    identical: 0,
    'identical-empty': 0,
    divergent: 0,
    'structurally-divergent': 0,
    'both-reject': 0,
    'both-reject-divergent': 0,
    'acceptance-divergent': 0,
  } satisfies Report['summary'];
  for (const entry of entries) {
    summary[entry.verdict]++;
    const stance = expectation(entry);
    if (stance !== 'unset') summary[stance]++;
  }

  const changed = entries.filter(entry => expectation(entry) === 'changed');

  // A mismatch an entry recorded as expected is not one to chase, so
  // `--only-mismatches` leaves it out — a changed verdict is shown whatever it
  // reads, because that is the entry someone has to look at.
  const shown = cliOptions['only-mismatches']
    ? entries.filter(
        entry =>
          expectation(entry) === 'changed' ||
          (isMismatch(entry.verdict) && expectation(entry) === 'unset')
      )
    : entries;

  for (const entry of shown) {
    const stance = expectation(entry);
    const stanceLabel =
      stance === 'expected'
        ? chalk.gray(' (expected)')
        : stance === 'changed'
          ? chalk.red(` (expected ${entry.expected})`)
          : '';
    console.log(
      `${VERDICT_LABELS[entry.verdict]}${stanceLabel}  ${chalk.bold(subjectLabel(entry))}  ${chalk.gray(`[${entry.set}] ${entry.origin}`)}`
    );
    if (AGREED.has(entry.verdict)) continue;
    console.log(`    rust   ${describe(entry, 'rust')}`);
    console.log(`    babel  ${describe(entry, 'babel')}`);
    if (entry.note !== undefined) console.log(chalk.gray(`    note   ${entry.note}`));
  }

  console.log(
    `\n${chalk.bold('Summary')} over ${summary.total} subjects\n` +
      `  identical              ${summary.identical}\n` +
      `  identical (empty)      ${summary['identical-empty']}   ${chalk.gray('(both emitted nothing; measures nothing)')}\n` +
      `  divergent              ${summary.divergent}   ${chalk.gray('(value normalization)')}\n` +
      `  structurally divergent ${summary['structurally-divergent']}   ${chalk.gray('(different properties emitted; out of scope)')}\n` +
      `  acceptance divergent   ${summary['acceptance-divergent']}   ${chalk.gray('(one compiler rejected)')}\n` +
      `  both reject            ${summary['both-reject']}\n` +
      `  both reject (diverged) ${summary['both-reject-divergent']}   ${chalk.gray('(both refused, for reasons worded differently)')}\n` +
      `  expected               ${summary.expected}   ${chalk.gray('(divergences already looked at)')}\n` +
      `  changed                ${summary.changed}   ${chalk.gray('(no longer the recorded verdict)')}`
  );

  if (changed.length > 0) {
    console.log(
      `\n${chalk.red.bold('Verdicts that changed')}  ${chalk.gray('— each entry recorded a different one')}`
    );
    for (const entry of changed) {
      console.log(
        `  ${chalk.bold(subjectLabel(entry))}  expected ${entry.expected}, read ${entry.verdict}`
      );
    }
    console.log(
      chalk.gray(
        '\nUpdate the entry — or its `expected` — in the corpus once you know which of the two moved.'
      )
    );
  }

  if (cliOptions.json !== undefined) {
    const report: Report = {
      generatedAt: new Date().toISOString(),
      subjects: comparer.versions,
      options: comparer.options,
      summary,
      entries,
    };
    // Resolved against the package rather than the shell's working directory,
    // which is what `pnpm run --filter` leaves it as: the same command run from
    // the repo root and from this package would otherwise write to two
    // different places, while the line below reports the path relative to the
    // workspace either way.
    const outputPath = path.resolve(packageDir, cliOptions.json);
    fs.mkdirSync(path.dirname(outputPath), { recursive: true });
    fs.writeFileSync(outputPath, `${JSON.stringify(report, null, 2)}\n`, 'utf8');
    console.log(chalk.green(`\nReport written to ${path.relative(workspaceRoot, outputPath)}`));
  }

  // Set after the report is written rather than by returning early: a changed
  // verdict is exactly when someone wants the machine-readable output, and an
  // exit code that skipped writing it would hide the evidence for the failure it
  // is reporting.
  if (changed.length > 0) process.exitCode = 1;
}

run().catch((error: unknown) => {
  console.error(chalk.red('Parity harness failed:'), error);
  process.exit(1);
});
