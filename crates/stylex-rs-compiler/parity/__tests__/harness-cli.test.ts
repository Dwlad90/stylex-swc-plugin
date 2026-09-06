import fs from 'node:fs';
import path from 'node:path';

import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';

import { answerOf, selectedOrExit, writeJsonReport } from '../lib/harness-cli.js';
import { ACCEPTED, accepted, refused, temporaryDir } from './support.js';

/**
 * The three things every generated harness does around its comparison, none of
 * which had a test — while the file's own header says `answerOf` exists because
 * a refusal was being joined wrong.
 *
 * `process.exit` is replaced with a throw rather than counted: the real one ends
 * the worker, and the functions under test are written so that every path after
 * the refusal is unreachable.
 */

class Exited extends Error {
  constructor(readonly code: number | undefined) {
    super(`exited ${code}`);
  }
}

let complaints: string[] = [];

beforeEach(() => {
  complaints = [];
  vi.spyOn(process, 'exit').mockImplementation(code => {
    throw new Exited(typeof code === 'number' ? code : undefined);
  });
  vi.spyOn(console, 'error').mockImplementation(line => {
    complaints.push(String(line));
  });
});

afterEach(() => {
  vi.restoreAllMocks();
});

/** The code an exit carried, or a failure naming what happened instead. */
function exitCodeOf(run: () => unknown): number | undefined {
  try {
    run();
  } catch (thrown) {
    if (thrown instanceof Exited) return thrown.code;
    throw thrown;
  }

  throw new Error('the call returned rather than exiting');
}

interface Surface {
  readonly name: string;
}

const SURFACES: readonly Surface[] = [{ name: 'Math' }, { name: 'String' }, { name: 'Array' }];
const nameOf = (surface: Surface): string => surface.name;

/** A throwaway package root, for the resolution `writeJsonReport` performs. */
const packageDirOf = (): string => temporaryDir('parity-report-');

describe('selecting members', () => {
  test('no selection runs every member', () => {
    expect(selectedOrExit('--surface', undefined, SURFACES, nameOf)).toStrictEqual([...SURFACES]);
  });

  /** A copy, so a harness that sorts its selection cannot reorder the table. */
  test('the answer to no selection is a copy of the table', () => {
    expect(selectedOrExit('--surface', undefined, SURFACES, nameOf)).not.toBe(SURFACES);
  });

  test('an empty selection runs every member', () => {
    expect(selectedOrExit('--surface', [], SURFACES, nameOf)).toStrictEqual([...SURFACES]);
  });

  test('a selection runs the members it names, in table order', () => {
    expect(selectedOrExit('--surface', ['Array', 'Math'], SURFACES, nameOf)).toStrictEqual([
      { name: 'Math' },
      { name: 'Array' },
    ]);
  });

  test('a name repeated selects its member once', () => {
    expect(selectedOrExit('--surface', ['Math', 'Math'], SURFACES, nameOf)).toStrictEqual([
      { name: 'Math' },
    ]);
  });

  test('every name unknown exits non-zero and lists what is known', () => {
    expect(exitCodeOf(() => selectedOrExit('--surface', ['Strnig'], SURFACES, nameOf))).toBe(1);
    expect(complaints.join('\n')).toContain('Math, String, Array');
  });

  /**
   * The failure the ticket names: filtering by inclusion succeeds on a partial
   * match, so a typo alongside a correct name ran one surface and exited zero —
   * a mistyped sweep reading as a pass in a harness whose whole claim is how
   * much it covered.
   */
  test('one unknown name alongside a known one exits non-zero', () => {
    expect(
      exitCodeOf(() => selectedOrExit('--surface', ['Math', 'Strnig'], SURFACES, nameOf))
    ).toBe(1);
  });

  /**
   * The two halves are asserted apart because they say opposite things about the
   * same name: `Math` belongs in the list of what is known and nowhere in the
   * list of what was not found, and a reader handed all four names back cannot
   * see which one they mistyped.
   */
  test('the complaint names only the unknown part of the selection', () => {
    exitCodeOf(() => selectedOrExit('--surface', ['Math', 'Strnig', 'Objcet'], SURFACES, nameOf));
    const [refusal, known] = complaints.join('\n').split('\n');

    expect(refusal).toContain('--surface');
    expect(refusal).toContain('Strnig');
    expect(refusal).toContain('Objcet');
    expect(refusal).not.toContain('Math');
    expect(known).toContain('Math');
  });

  test('an unknown name differing only in case is still unknown', () => {
    expect(exitCodeOf(() => selectedOrExit('--surface', ['math'], SURFACES, nameOf))).toBe(1);
  });

  test('a name that is a prefix of a known one is unknown', () => {
    expect(exitCodeOf(() => selectedOrExit('--surface', ['Mat'], SURFACES, nameOf))).toBe(1);
  });

  test('an empty table refuses any selection', () => {
    expect(exitCodeOf(() => selectedOrExit('--surface', ['Math'], [], nameOf))).toBe(1);
  });

  test('an empty table with no selection answers nothing', () => {
    expect(selectedOrExit('--surface', undefined, [], nameOf)).toStrictEqual([]);
  });

  test('a selection of plain names works without a projection', () => {
    expect(
      selectedOrExit('--property', ['margin'], ['margin', 'padding'], name => name)
    ).toStrictEqual(['margin']);
  });
});

describe('writing a report', () => {
  /**
   * `pnpm run --filter` leaves the shell's working directory at the repo root,
   * so a relative target resolved against the cwd writes a different file
   * depending on where the same command was typed — and these are the reports CI
   * archives. The second assertion is what says the cwd was not consulted: a
   * target nothing else names, looked for where the cwd would have put it.
   */
  test('a relative target resolves against the package, not the working directory', () => {
    const packageDir = packageDirOf();
    const target = 'results/relative-target-check.json';

    const written = writeJsonReport(packageDir, target, { ok: true });

    expect(written).toBe(path.join(packageDir, target));
    expect(fs.existsSync(path.resolve(process.cwd(), target))).toBe(false);
  });

  test('a nested target has its directories created', () => {
    const packageDir = packageDirOf();

    const written = writeJsonReport(packageDir, 'results/deep/deeper/report.json', { ok: true });

    expect(fs.existsSync(written)).toBe(true);
  });

  test('the file ends with a newline', () => {
    const packageDir = packageDirOf();

    const written = writeJsonReport(packageDir, 'report.json', { ok: true });

    expect(fs.readFileSync(written, 'utf8').endsWith('\n')).toBe(true);
  });

  test('the report is indented JSON, and reads back as what was handed over', () => {
    const packageDir = packageDirOf();
    const report = { summary: { total: 2 }, rows: ['a', 'b'] };

    const written = writeJsonReport(packageDir, 'report.json', report);
    const text = fs.readFileSync(written, 'utf8');

    expect(JSON.parse(text)).toStrictEqual(report);
    expect(text).toContain('\n  "summary"');
  });

  test('an absolute target is written where it says', () => {
    const packageDir = packageDirOf();
    const target = path.join(packageDirOf(), 'elsewhere/report.json');

    expect(writeJsonReport(packageDir, target, { ok: true })).toBe(target);
  });

  test('writing twice replaces the file rather than appending to it', () => {
    const packageDir = packageDirOf();

    writeJsonReport(packageDir, 'report.json', { run: 1 });
    const written = writeJsonReport(packageDir, 'report.json', { run: 2 });

    expect(JSON.parse(fs.readFileSync(written, 'utf8'))).toStrictEqual({ run: 2 });
  });

  test('a large report survives the round trip', () => {
    const packageDir = packageDirOf();
    const report = { rows: Array.from({ length: 20_000 }, (_, index) => ({ id: `row-${index}` })) };

    const written = writeJsonReport(packageDir, 'results/large.json', report);

    expect(JSON.parse(fs.readFileSync(written, 'utf8'))).toStrictEqual(report);
  });
});

describe('one compiler answer as a cell', () => {
  test('an acceptance joins its declarations with a pipe', () => {
    expect(answerOf(accepted(['color:red', 'margin-top:1px']))).toBe('color:red | margin-top:1px');
  });

  test('an acceptance emitting nothing is an empty cell', () => {
    expect(answerOf(accepted([]))).toBe('');
  });

  test('an acceptance emitting one declaration carries no separator', () => {
    expect(answerOf(ACCEPTED)).toBe('color:red');
  });

  /**
   * The bug the shared helper was extracted for: several diagnostics are two
   * lines in both compilers and the second is the one that names the rule, so a
   * cell showing only the first says a call could not be folded without saying
   * what declined it.
   */
  test('a two-line refusal keeps both lines, joined', () => {
    expect(answerOf(refused('Could not fold this call\nOnly pure functions may be called'))).toBe(
      'refused: Could not fold this call / Only pure functions may be called'
    );
  });

  test('a refusal of more than two lines keeps every one', () => {
    expect(answerOf(refused('one\ntwo\nthree'))).toBe('refused: one / two / three');
  });

  test('surrounding whitespace is trimmed before the lines are joined', () => {
    expect(answerOf(refused('\n  one\ntwo  \n\n'))).toBe('refused: one / two');
  });

  test('a blank line inside a refusal is kept as an empty segment', () => {
    expect(answerOf(refused('one\n\ntwo'))).toBe('refused: one /  / two');
  });

  test('a one-line refusal carries no separator', () => {
    expect(answerOf(refused('Unclosed string'))).toBe('refused: Unclosed string');
  });

  /**
   * Both compilers write `\n` today, so this is about what a cell must never
   * carry rather than about a diagnostic anyone has seen: a control character
   * inside a report cell reads as corruption in every tool that opens it.
   */
  test('a carriage return is a line ending rather than content', () => {
    expect(answerOf(refused('one\r\ntwo'))).toBe('refused: one / two');
  });
});
