// The child-process helper is the harness that every abort test stands on. A
// fault here reads as a fault in the compiler, so the harness has its own
// guards for the two limits that broke it before: the Windows command line,
// and the parser stack.
import { existsSync } from 'node:fs';

import { describe, expect, test } from 'vitest';

import { runNodeScript } from './nodeScript';

/** The longest command line that Windows accepts. */
const WINDOWS_COMMAND_LINE_LIMIT = 32_767;

describe('runNodeScript', () => {
  test('runs a small script and reports its output', () => {
    const outcome = runNodeScript('process.stdout.write("hello");');

    expect(outcome.error).toBeUndefined();
    expect(outcome.status).toBe(0);
    expect(outcome.signal).toBeNull();
    expect(outcome.stdout).toBe('hello');
  });

  test('runs a script much longer than the Windows command line', () => {
    // A comment of this length cannot pass as an argument on Windows.
    const padding = 'x'.repeat(WINDOWS_COMMAND_LINE_LIMIT * 4);
    const outcome = runNodeScript(`// ${padding}\nprocess.stdout.write("long");`);

    expect(outcome.error).toBeUndefined();
    expect(outcome.status).toBe(0);
    expect(outcome.stdout).toBe('long');
  });

  test('runs a script that holds a deeply nested literal', () => {
    // A literal, not a loop: a parser descends one frame for each level, and
    // this is the shape that ended the child under `node -e`. A `.js` file
    // reaches V8 alone, which holds a much deeper source than the TypeScript
    // stripper does on the small stack that Windows gives the main thread.
    const nested = `${'{ a: '.repeat(1000)}null${' }'.repeat(1000)}`;
    const outcome = runNodeScript(`
      const value = ${nested};
      let depth = 0;
      for (let node = value; node !== null; node = node.a) depth++;
      process.stdout.write(String(depth));
    `);

    expect(outcome.error).toBeUndefined();
    expect(outcome.status).toBe(0);
    expect(outcome.stdout).toBe('1000');
  });

  test('runs a script that builds a shape larger than any parser sees', () => {
    const outcome = runNodeScript(`
      let value = null;
      for (let i = 0; i < 200000; i++) value = { a: value };
      let depth = 0;
      for (let node = value; node !== null; node = node.a) depth++;
      process.stdout.write(String(depth));
    `);

    expect(outcome.error).toBeUndefined();
    expect(outcome.status).toBe(0);
    expect(outcome.stdout).toBe('200000');
  });

  test('reports the exit code of a script that fails', () => {
    const outcome = runNodeScript('process.exit(3);');

    expect(outcome.error).toBeUndefined();
    expect(outcome.status).toBe(3);
    expect(outcome.signal).toBeNull();
  });

  test('reports what a failing script wrote to stderr', () => {
    const outcome = runNodeScript('throw new Error("expected failure");');

    expect(outcome.status).not.toBe(0);
    expect(outcome.stderr).toContain('expected failure');
  });

  test('gives the script the environment variables it asks for', () => {
    const outcome = runNodeScript('process.stdout.write(String(process.env.STYLEX_TEST_VALUE));', {
      env: { ...process.env, STYLEX_TEST_VALUE: 'set' },
    });

    expect(outcome.stdout).toBe('set');
  });

  test('answers empty output for a script that writes nothing', () => {
    const outcome = runNodeScript(';');

    expect(outcome.status).toBe(0);
    expect(outcome.stdout).toBe('');
    expect(outcome.stderr).toBe('');
  });

  test('gives each script a file of its own, and removes it afterwards', () => {
    const reportPath = 'process.stdout.write(__filename);';
    const first = runNodeScript(reportPath);
    const second = runNodeScript(reportPath);

    expect(first.stdout).not.toBe(second.stdout);
    expect(existsSync(first.stdout)).toBe(false);
    expect(existsSync(second.stdout)).toBe(false);
  });

  test('carries an output larger than the default buffer back from the child', () => {
    // The default `maxBuffer` of `spawnSync` is 1 MB, and output above a limit
    // is cut without an error. This asks for more than the default.
    const size = 4 * 1024 * 1024;
    const outcome = runNodeScript(`process.stdout.write("y".repeat(${size}));`);

    expect(outcome.error).toBeUndefined();
    expect(outcome.status).toBe(0);
    expect(outcome.stdout).toHaveLength(size);
  });
});
