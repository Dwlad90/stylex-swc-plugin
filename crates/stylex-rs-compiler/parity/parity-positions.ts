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
import { availableParallelism } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseArgs } from 'node:util';

import * as babel from '@babel/core';
import chalk from 'chalk';

import type { StyleXOptions } from '../dist/index.js';
import {
  baseStyleXOptions,
  loadBabelPlugin,
  loadRustCompiler,
  messageOf,
  resolveVersions,
  subjectBlock,
} from './lib/compilers.js';
import { arrayAt, isRecord, stringAt } from './lib/guards.js';
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
 * The fixture path both compilers are handed for one subject.
 *
 * Inside the package, and paired with `haste` resolution, for the reason the
 * value harness gives: neither compiler then needs a real `node_modules` layout
 * beside the fixture, and this compiler's path resolver needs a file that sits
 * under a package it can name.
 *
 * Per subject, where the value harness pins one path for every entry. The reason
 * that harness gives for pinning -- `haste` resolution and the class-name hash
 * both read the filename, so varying it would vary the output for reasons
 * unrelated to the subject -- does not reach this one: what is compared here is
 * a line and a column, read out of each compiler's own message by
 * `rustPosition` and `babelPosition`, and neither looks at the path. Naming the
 * file after the subject is what lets the children overlap; while they shared
 * one path, every one of them was overwriting the file the last was compiling.
 */
function fixtureFor(id: string): string {
  return path.join(packageDir, 'parity/__fixture__', `positions-${id}.js`);
}

/**
 * Handed identically to both compilers: the shared base every harness compiles
 * with, plus `runtimeInjection: false`, since a refusal is reached before
 * anything would be injected.
 */
const STYLEX_OPTIONS: StyleXOptions = {
  ...baseStyleXOptions(packageDir),
  runtimeInjection: false,
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

/**
 * The corpus, narrowed rather than asserted — the trade `lib/corpus.ts` makes
 * for the value sets: an entry missing a field fails here, naming the file,
 * instead of surfacing mid-run as a subject that is not there.
 */
function loadEntries(): PositionEntry[] {
  const raw: unknown = JSON.parse(fs.readFileSync(corpusPath, 'utf8'));
  const entries = arrayAt(raw, 'entries');
  if (stringAt(raw, 'set') === undefined || entries === undefined) {
    throw new Error(
      `Corpus file malformed: ${corpusPath} — expected { set, description, entries }`
    );
  }

  return entries.map((entry, index) => positionEntryFrom(entry, index));
}

function positionEntryFrom(raw: unknown, index: number): PositionEntry {
  const id = stringAt(raw, 'id');
  const label = stringAt(raw, 'label');
  const source = stringAt(raw, 'source');
  const origin = stringAt(raw, 'origin');
  if (id === undefined || label === undefined || source === undefined || origin === undefined) {
    throw new Error(
      `Corpus entry ${index} malformed in ${corpusPath} — expected { id, label, source, origin }.`
    );
  }

  const note = stringAt(raw, 'note');
  const expected = expectedVerdictFrom(raw, id);

  return {
    id,
    label,
    source,
    origin,
    ...(note === undefined ? {} : { note }),
    ...(expected === undefined ? {} : { expected }),
  };
}

/**
 * The pinned verdict, or `undefined` where an entry pins none.
 *
 * Throws on a string that is not a verdict rather than dropping it, for the
 * reason `verdictAt` gives in `lib/guards.ts`: an expectation the loader
 * silently ignored reads as a divergence someone had already looked at, which is
 * the opposite of what the field is for.
 */
function expectedVerdictFrom(raw: unknown, id: string): PositionVerdict | undefined {
  const found = stringAt(raw, 'expected');
  if (found === undefined) return undefined;

  const verdict = POSITION_VERDICTS.find(candidate => candidate === found);
  if (verdict === undefined) {
    throw new Error(
      `Corpus entry ${id} names an unknown expected verdict: ${found} — expected one of ${POSITION_VERDICTS.join(', ')}.`
    );
  }

  return verdict;
}

const POSITION_VERDICTS: readonly PositionVerdict[] = [
  'identical',
  'divergent',
  'no-position',
  'neither-position',
  'not-refused',
];

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

  const fixture = fixtureFor(id);

  fs.mkdirSync(path.dirname(fixture), { recursive: true });
  fs.writeFileSync(fixture, entry.source);

  const outcome: SubjectOutcome = {};

  const { transform } = await loadRustCompiler(packageDir);

  try {
    transform(fixture, entry.source, STYLEX_OPTIONS);
  } catch (error: unknown) {
    outcome.rustMessage = messageOf(error);
  }

  const { plugin } = loadBabelPlugin();

  try {
    babel.transformSync(entry.source, {
      filename: fixture,
      babelrc: false,
      configFile: false,
      parserOpts: { sourceType: 'module', plugins: ['jsx'] },
      plugins: [[plugin, STYLEX_OPTIONS]],
    });
  } catch (error: unknown) {
    outcome.babelMessage = messageOf(error);
  }

  // Written to stdout, which the parent reads; this compiler's code frame is
  // already on stderr, which the parent reads separately.
  process.stdout.write(JSON.stringify(outcome));
}

// ── the parent half: one child per subject, then the report ─────────────────

interface ChildResult {
  outcome: SubjectOutcome;
  stderr: string;
}

/**
 * Every subject, at most `availableParallelism()` children at a time.
 *
 * Results come back in the order the ids were given, not the order the children
 * finished, so the report does not reorder itself run to run.
 */
async function runInChildren(ids: readonly string[]): Promise<ChildResult[]> {
  const results: ChildResult[] = Array.from({ length: ids.length });
  const limit = Math.min(ids.length, availableParallelism());
  let next = 0;

  const worker = async (): Promise<void> => {
    for (let index = next++; index < ids.length; index = next++) {
      results[index] = await runInChild(ids[index]!);
    }
  };

  await Promise.all(Array.from({ length: limit }, worker));

  return results;
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

      resolve({ outcome: outcomeFrom(stdout, id), stderr });
    });
  });
}

/**
 * What the child printed, narrowed. A subject neither compiler refused carries
 * neither message, so both fields being absent is a valid outcome and only the
 * surrounding shape is checked.
 */
function outcomeFrom(stdout: string, id: string): SubjectOutcome {
  const raw: unknown = JSON.parse(stdout);
  if (!isRecord(raw)) {
    throw new Error(`Subject ${id} printed no outcome object: ${stdout}`);
  }

  const rustMessage = stringAt(raw, 'rustMessage');
  const babelMessage = stringAt(raw, 'babelMessage');

  return {
    ...(rustMessage === undefined ? {} : { rustMessage }),
    ...(babelMessage === undefined ? {} : { babelMessage }),
  };
}

async function report(): Promise<void> {
  const filter = cliOptions.filter;
  const entries = loadEntries().filter(entry => filter === undefined || entry.id.includes(filter));

  // An empty selection is a broken filter or a broken corpus, not a pass. The
  // value harness already treats it that way; this one returned cleanly, so a
  // mistyped `--filter` reported position parity over nothing and exited 0.
  if (entries.length === 0) {
    console.error(chalk.red('No position subjects match the given filter.'));
    process.exit(1);
  }

  // Printed before the first child runs, so a failing run is attributable even
  // when it dies partway: the upstream plugin is held by the lockfile rather than
  // by an exact range, so it moves under a `pnpm update` without anything in this
  // directory changing, and the versions are the first thing to read when a run
  // starts failing on a corpus nobody touched.
  const { distEntry } = await loadRustCompiler(packageDir);
  const { pluginEntry } = loadBabelPlugin();
  console.log(
    `${chalk.bold('Subjects')}\n${subjectBlock(resolveVersions(packageDir, distEntry, pluginEntry))}\n`
  );

  // A bounded number of children in flight rather than one at a time.
  //
  // One child per subject is not optional -- this compiler writes its code frame
  // straight to fd 2 and a process cannot redirect its own -- but nothing
  // required them to be serial once each writes its own fixture. Serial, the run
  // was 18 process spawns end to end, each paying for the `tsx` loader, the NAPI
  // addon and `@babel/core` again, which was the whole of its wall clock.
  //
  // Bounded rather than unbounded for the same reason: each child loads both
  // compilers, so the useful limit is cores rather than subjects.
  const children = await runInChildren(entries.map(entry => entry.id));

  const results: PositionReportEntry[] = [];

  for (const [index, entry] of entries.entries()) {
    // Read back in corpus order, whatever order they finished in, so the report
    // a person compares against a previous one is stable.
    const { outcome, stderr } = children[index]!;
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
  // Counted apart from the agreeing total, for the reason `neither-position`
  // exists: a subject where both compilers said nothing is agreement about a
  // hole, and folding it into "point at the same place" would read as a position
  // that was measured.
  const silent = results.filter(result => result.verdict === 'neither-position').length;
  console.log(
    `\n${identical}/${results.length} subjects point at the same place; ` +
      (silent === 0 ? '' : `${silent} point nowhere on either side; `) +
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
  //
  // The advice is printed rather than left implicit because the person who reads
  // this first will not be the person who wrote the corpus.
  if (unexpected > 0) {
    console.log(
      chalk.gray(
        '\nEach ✗ above prints both positions and both first lines. Read them, then either fix\n' +
          "the diagnostic that moved or update the subject's `expected` in corpus/positions.json\n" +
          "-- once you know which of the two moved. If neither compiler's own version changed,\n" +
          'it was this one: the subject block above names both.'
      )
    );
    process.exitCode = 1;
  }
}

const VERDICT_LABELS: Record<PositionVerdict, string> = {
  identical: chalk.green('identical'),
  divergent: chalk.red('divergent'),
  'no-position': chalk.yellow('one side said nothing about where'),
  'neither-position': chalk.gray('neither said anything about where'),
  'not-refused': chalk.magenta('not refused by both'),
};

function firstLine(message: string | undefined): string {
  return message === undefined ? chalk.gray('(compiled)') : (message.split('\n')[0] ?? '');
}

await (cliOptions.subject === undefined ? report() : runSubject(cliOptions.subject));
