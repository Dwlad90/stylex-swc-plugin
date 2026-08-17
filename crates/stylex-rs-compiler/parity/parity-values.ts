/**
 * CSS value parity harness: `@stylexswc/rs-compiler` vs `@stylexjs/babel-plugin`.
 *
 * A StyleX class name is a hash of the canonical declaration text, which makes
 * that text a compatibility contract between the two compilers. This harness
 * runs a corpus of CSS declarations through both and reports, per declaration,
 * whether they agree byte for byte.
 *
 * It is a developer tool, not a test: it lives outside the Rust test suite so
 * `cargo test` never needs a Node toolchain, and it is not wired into CI.
 *
 * Usage:
 *   pnpm parity                              # full corpus, human report
 *   pnpm parity --only-mismatches            # just the divergences
 *   pnpm parity --set reported               # one corpus set; repeatable
 *   pnpm parity --filter calc                # entries whose value contains it
 *   pnpm parity --json parity/results/x.json # machine-readable report
 *   pnpm parity --font-size-px-to-rem        # both compilers with the option on
 */

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseArgs } from 'node:util';

import chalk from 'chalk';

import { createComparer } from './lib/compare.js';
import { loadCorpus } from './lib/corpus.js';
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
      --set <name>              limit to a corpus set (reported|edge|harvested);
                                repeatable
      --filter <substring>      limit to declarations whose value contains it
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
  'acceptance-divergent': chalk.yellow('acceptance divergent'),
};

function isMismatch(verdict: Verdict): boolean {
  return verdict !== 'identical' && verdict !== 'both-reject';
}

function describe(entry: ReportEntry, side: 'rust' | 'babel'): string {
  const outcome = entry[side];
  if (outcome.status === 'error') return chalk.gray(`rejected: ${outcome.message}`);
  return outcome.declarations.map(declaration => `{${declaration}}`).join(' ');
}

async function run(): Promise<void> {
  let corpus = loadCorpus(path.join(parityDir, 'corpus'));

  if (cliOptions.set !== undefined && cliOptions.set.length > 0) {
    const wanted = new Set(cliOptions.set);
    corpus = corpus.filter(entry => wanted.has(entry.set));
  }
  if (cliOptions.filter !== undefined) {
    const needle = cliOptions.filter;
    corpus = corpus.filter(entry => entry.value.includes(needle));
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
    identical: 0,
    'identical-empty': 0,
    divergent: 0,
    'structurally-divergent': 0,
    'both-reject': 0,
    'acceptance-divergent': 0,
  } satisfies Report['summary'];
  for (const entry of entries) summary[entry.verdict]++;

  const shown = cliOptions['only-mismatches']
    ? entries.filter(entry => isMismatch(entry.verdict))
    : entries;

  for (const entry of shown) {
    console.log(
      `${VERDICT_LABELS[entry.verdict]}  ${chalk.bold(entry.property)}: ${JSON.stringify(entry.value)}  ${chalk.gray(`[${entry.set}] ${entry.origin}`)}`
    );
    if (entry.verdict === 'identical' || entry.verdict === 'both-reject') continue;
    console.log(`    rust   ${describe(entry, 'rust')}`);
    console.log(`    babel  ${describe(entry, 'babel')}`);
    if (entry.note !== undefined) console.log(chalk.gray(`    note   ${entry.note}`));
  }

  console.log(
    `\n${chalk.bold('Summary')} over ${summary.total} declarations\n` +
      `  identical              ${summary.identical}\n` +
      `  identical (empty)      ${summary['identical-empty']}   ${chalk.gray('(both emitted nothing; measures nothing)')}\n` +
      `  divergent              ${summary.divergent}   ${chalk.gray('(value normalization)')}\n` +
      `  structurally divergent ${summary['structurally-divergent']}   ${chalk.gray('(different properties emitted; out of scope)')}\n` +
      `  acceptance divergent   ${summary['acceptance-divergent']}   ${chalk.gray('(one compiler rejected)')}\n` +
      `  both reject            ${summary['both-reject']}`
  );

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
}

run().catch((error: unknown) => {
  console.error(chalk.red('Parity harness failed:'), error);
  process.exit(1);
});
