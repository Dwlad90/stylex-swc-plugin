/**
 * Refusal *position* parity: `@stylexswc/rs-compiler` vs `@stylexjs/babel-plugin`.
 *
 * The value harness beside this one compares what each compiler *says* about a
 * refused input, with the text saying where it happened deliberately stripped —
 * so it cannot see a diagnostic that names the wrong line. That is the gap this
 * fills, one corpus set of refusals whose position is the whole question.
 *
 * Each subject runs in a child process, because the two positions arrive in
 * different channels: upstream throws a `@babel/code-frame` excerpt inside the
 * message, while this compiler writes its frame to **stderr** and throws the
 * sentence alone. Node cannot redirect its own file descriptor 2, and a native
 * write goes straight to it, so capturing that frame means being the parent of
 * the process that produced it. One subject per child, so no marker is needed to
 * tell two subjects' frames apart.
 *
 * Usage:
 *   pnpm parity:positions                       # the whole set
 *   pnpm parity:positions --filter import       # subjects whose id contains it
 *   pnpm parity:positions --json parity/results/positions.json
 */

import { spawn } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { parseArgs } from 'node:util';

import * as babel from '@babel/core';
import stylexBabelPluginModule from '@stylexjs/babel-plugin';
import chalk from 'chalk';

import type { StyleXOptions } from '../dist/index.js';
import { isRecord } from './lib/guards.js';
import {
  babelPosition,
  formatPosition,
  positionVerdict,
  rustPosition,
  type PositionVerdict,
  type ReportedPosition,
} from './lib/position.js';

const parityDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.resolve(parityDir, '..');
const corpusPath = path.join(parityDir, 'corpus/positions.json');

/**
 * The fixture path both compilers are handed.
 *
 * Inside the package, and paired with `haste` resolution, for the reason the
 * value harness gives: neither compiler then needs a real `node_modules` layout
 * beside the fixture, and this compiler's path resolver needs a file that sits
 * under a package it can name.
 */
const FIXTURE = path.join(packageDir, 'parity/__fixture__/positions.js');

/**
 * Handed identically to both compilers, and the same shape the value harness
 * uses: `haste` resolution so neither needs a `node_modules` layout beside the
 * fixture, and `dev: false` so no debug annotation reads a path.
 */
const STYLEX_OPTIONS: StyleXOptions = {
  dev: false,
  runtimeInjection: false,
  unstable_moduleResolution: { type: 'haste', rootDir: packageDir },
};

interface PositionEntry {
  id: string;
  label: string;
  source: string;
  origin: string;
  note?: string;
  /** The verdict this subject is known to read, where it is not agreement. */
  expected?: PositionVerdict;
}

interface SubjectOutcome {
  rustMessage?: string;
  babelMessage?: string;
}

interface PositionReportEntry extends PositionEntry {
  verdict: PositionVerdict;
  rust?: ReportedPosition;
  babel?: ReportedPosition;
  rustMessage?: string;
  babelMessage?: string;
}

// ── the child half: one subject, both compilers ─────────────────────────────

const { values: cliOptions } = parseArgs({
  args: process.argv.slice(2).filter(argument => argument !== '--'),
  options: {
    subject: { type: 'string' },
    filter: { type: 'string' },
    json: { type: 'string' },
    help: { type: 'boolean', short: 'h', default: false },
  },
});

if (cliOptions.help) {
  console.log(`
${chalk.bold('StyleX refusal position parity')}

Options:
      --filter <substring>      limit to subjects whose id contains it
      --json <path>             also write the machine-readable report
  -h, --help                    show this help
`);
  process.exit(0);
}

function loadEntries(): PositionEntry[] {
  const raw: unknown = JSON.parse(fs.readFileSync(corpusPath, 'utf8'));
  if (typeof raw !== 'object' || raw === null || !Array.isArray((raw as PositionSet).entries)) {
    throw new Error(
      `Corpus file malformed: ${corpusPath} — expected { set, description, entries }`
    );
  }

  return (raw as PositionSet).entries;
}

interface PositionSet {
  set: string;
  description: string;
  entries: PositionEntry[];
}

/** The message a thrown value carries, however it was thrown. */
function messageOf(error: unknown): string {
  if (error instanceof Error) return error.message;

  return typeof error === 'string' ? error : JSON.stringify(error);
}

/**
 * Runs one subject through both compilers and prints what each threw.
 *
 * The fixture is written to disk because this compiler locates a refusal in the
 * file it names, reading it back from there rather than from the string it was
 * handed.
 */
async function runSubject(id: string): Promise<void> {
  const entry = loadEntries().find(candidate => candidate.id === id);
  if (entry === undefined) throw new Error(`No such subject: ${id}`);

  fs.mkdirSync(path.dirname(FIXTURE), { recursive: true });
  fs.writeFileSync(FIXTURE, entry.source);

  const outcome: SubjectOutcome = {};

  const distEntry = path.join(packageDir, 'dist/index.js');
  const loaded: unknown = await import(pathToFileURL(distEntry).href);
  const transform = isRecord(loaded) ? loaded.transform : undefined;
  if (typeof transform !== 'function') {
    throw new Error(
      `${distEntry} does not export a transform function — run \`pnpm build\` in this package first.`
    );
  }

  try {
    (transform as RustTransform)(FIXTURE, entry.source, STYLEX_OPTIONS);
  } catch (error: unknown) {
    outcome.rustMessage = messageOf(error);
  }

  // Published both as a default export and as the module object itself,
  // depending on how the consumer resolves it; either is accepted, as in the
  // value harness.
  const pluginModule: unknown = stylexBabelPluginModule;
  const plugin = (isRecord(pluginModule) ? pluginModule.default : undefined) ?? pluginModule;
  if (typeof plugin !== 'function') {
    throw new Error('@stylexjs/babel-plugin did not export a Babel plugin function');
  }

  try {
    babel.transformSync(entry.source, {
      filename: FIXTURE,
      babelrc: false,
      configFile: false,
      parserOpts: { sourceType: 'module', plugins: ['jsx'] },
      plugins: [[plugin as babel.PluginTarget, STYLEX_OPTIONS]],
    });
  } catch (error: unknown) {
    outcome.babelMessage = messageOf(error);
  }

  // Written to stdout, which the parent reads; this compiler's code frame is
  // already on stderr, which the parent reads separately.
  process.stdout.write(JSON.stringify(outcome));
}

type RustTransform = (
  filename: string,
  code: string,
  options: StyleXOptions
) => { metadata: { stylex: unknown[] }; code: string };

// ── the parent half: one child per subject, then the report ─────────────────

interface ChildResult {
  outcome: SubjectOutcome;
  stderr: string;
}

function runInChild(id: string): Promise<ChildResult> {
  return new Promise((resolve, reject) => {
    const child = spawn(
      process.execPath,
      ['--import', 'tsx/esm', fileURLToPath(import.meta.url), '--subject', id],
      { cwd: packageDir, stdio: ['ignore', 'pipe', 'pipe'] }
    );

    let stdout = '';
    let stderr = '';
    child.stdout.setEncoding('utf8');
    child.stderr.setEncoding('utf8');
    child.stdout.on('data', chunk => (stdout += chunk));
    child.stderr.on('data', chunk => (stderr += chunk));

    child.on('error', reject);
    child.on('close', code => {
      if (code !== 0 && stdout === '') {
        reject(new Error(`Subject ${id} exited with ${String(code)}:\n${stderr}`));
        return;
      }

      resolve({ outcome: JSON.parse(stdout) as SubjectOutcome, stderr });
    });
  });
}

async function report(): Promise<void> {
  const filter = cliOptions.filter;
  const entries = loadEntries().filter(entry => filter === undefined || entry.id.includes(filter));

  if (entries.length === 0) {
    console.log(chalk.yellow('No subjects matched.'));
    return;
  }

  const results: PositionReportEntry[] = [];

  for (const entry of entries) {
    const { outcome, stderr } = await runInChild(entry.id);
    const rust = rustPosition(stderr);
    const upstream =
      outcome.babelMessage === undefined ? undefined : babelPosition(outcome.babelMessage);
    const bothRefused = outcome.rustMessage !== undefined && outcome.babelMessage !== undefined;

    results.push({
      ...entry,
      verdict: positionVerdict(rust, upstream, bothRefused),
      ...(rust === undefined ? {} : { rust }),
      ...(upstream === undefined ? {} : { babel: upstream }),
      ...outcome,
    });
  }

  let unexpected = 0;

  for (const result of results) {
    const expected = result.expected ?? 'identical';
    const agrees = result.verdict === expected;
    if (!agrees) unexpected += 1;

    const mark = agrees ? chalk.green('✓') : chalk.red('✗');
    console.log(`${mark} ${chalk.bold(result.label)} ${chalk.gray(`(${result.id})`)}`);
    console.log(
      `    verdict ${VERDICT_LABELS[result.verdict]}` +
        (result.expected === undefined ? '' : chalk.gray(`, expected ${result.expected}`))
    );
    console.log(
      `    here ${chalk.cyan(formatPosition(result.rust))}` +
        `   upstream ${chalk.cyan(formatPosition(result.babel))}`
    );
    if (!agrees) {
      console.log(`    ${chalk.gray('here')}     ${firstLine(result.rustMessage)}`);
      console.log(`    ${chalk.gray('upstream')} ${firstLine(result.babelMessage)}`);
    }
  }

  const identical = results.filter(result => result.verdict === 'identical').length;
  console.log(
    `\n${identical}/${results.length} subjects point at the same place; ` +
      `${unexpected} unexpected verdict${unexpected === 1 ? '' : 's'}.`
  );

  if (cliOptions.json !== undefined) {
    const jsonPath = path.resolve(packageDir, cliOptions.json);
    fs.mkdirSync(path.dirname(jsonPath), { recursive: true });
    fs.writeFileSync(jsonPath, `${JSON.stringify(results, null, 2)}\n`);
    console.log(chalk.gray(`Report written to ${jsonPath}`));
  }

  // Non-zero for the same reason the value harness is: an expectation that no
  // longer holds. A position that silently starts diverging is exactly what this
  // set exists to catch, and one that silently starts agreeing is a pinned
  // divergence nobody unpinned.
  if (unexpected > 0) process.exitCode = 1;
}

const VERDICT_LABELS: Record<PositionVerdict, string> = {
  identical: chalk.green('identical'),
  divergent: chalk.red('divergent'),
  'no-position': chalk.yellow('one side said nothing about where'),
  'not-refused': chalk.magenta('not refused by both'),
};

function firstLine(message: string | undefined): string {
  return message === undefined ? chalk.gray('(compiled)') : (message.split('\n')[0] ?? '');
}

await (cliOptions.subject === undefined ? report() : runSubject(cliOptions.subject));
