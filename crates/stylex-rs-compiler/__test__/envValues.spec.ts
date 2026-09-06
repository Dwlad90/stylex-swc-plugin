// `stylex.env` accepts a JavaScript object, and the addon reads that object
// across the NAPI boundary. Two readers do the work: one for a value at the top
// of the object, and one for a value inside an object or an array below it.
//
// The two readers disagreed. The top reader answered a null expression for
// every value it had no rule for. The reader below it called `panic!`, and that
// panic left the option parser, which ran before the compiler installed its
// panic guard. A panic that crosses the boundary aborts the process, so
// `stylex.env = { theme: { fallback: null } }` killed Node with SIGABRT and gave
// JavaScript no error to catch.
//
// Each case runs in a child process, because a test in this process cannot
// report an abort of this process.
import * as path from 'path';

import { describe, expect, test } from 'vitest';

import { runNodeScript } from './nodeScript';

const compilerEntry = path.resolve(__dirname, '../dist/index.js');

/**
 * Compiles one source with one `env` object in a child process.
 *
 * `envLiteral` is JavaScript source rather than a value, because a symbol, a
 * function and a bigint do not survive JSON.
 */
const compileWithEnv = (envLiteral: string, source = 'export const a = 1;') => {
  const script = `
    const { transform } = require(${JSON.stringify(compilerEntry)});
    try {
      const result = transform('page.tsx', ${JSON.stringify(source)}, {
        dev: false,
        unstable_moduleResolution: { type: 'commonJS' },
        env: ${envLiteral},
      });
      process.stdout.write(JSON.stringify({ ok: true, code: result.code }));
    } catch (error) {
      process.stdout.write(JSON.stringify({ ok: false, message: String(error) }));
    }
  `;

  const child = runNodeScript(script);

  return {
    signal: child.signal,
    status: child.status,
    error: child.error,
    stderr: child.stderr,
    result: child.stdout ? (JSON.parse(child.stdout) as { ok: boolean; message?: string }) : null,
  };
};

/** Asserts that the compiler answered JavaScript, and did not end the process. */
const expectNoAbort = (outcome: ReturnType<typeof compileWithEnv>, description: string) => {
  expect(outcome.error, `${description} never started a child process`).toBeUndefined();
  expect(outcome.signal, `${description} killed the process with a signal`).toBeNull();
  expect(outcome.status, `${description} ended the process: ${outcome.stderr}`).toBe(0);
  expect(outcome.result, `${description} printed nothing`).not.toBeNull();
};

// The kinds that have no expression of their own. The top reader answers a null
// expression for each one, so the reader below it must answer the same.
const KINDS_WITHOUT_AN_EXPRESSION = {
  null: 'null',
  undefined: 'undefined',
  symbol: "Symbol('s')",
  bigint: '10n',
  function: '() => 1',
};

describe('a value that env has no expression for', () => {
  for (const [kind, literal] of Object.entries(KINDS_WITHOUT_AN_EXPRESSION)) {
    test(`${kind} at the top of env compiles`, () => {
      const outcome = compileWithEnv(`{ a: ${literal} }`);

      expectNoAbort(outcome, `a top-level ${kind}`);
      expect(outcome.result?.ok).toBe(true);
    });

    test(`${kind} inside an object compiles, as it does at the top`, () => {
      const outcome = compileWithEnv(`{ theme: { fallback: ${literal} } }`);

      expectNoAbort(outcome, `a nested ${kind}`);
      expect(outcome.result?.ok).toBe(true);
    });

    test(`${kind} inside an array compiles, as it does at the top`, () => {
      const outcome = compileWithEnv(`{ theme: [1, ${literal}, 3] }`);

      expectNoAbort(outcome, `a ${kind} in an array`);
      expect(outcome.result?.ok).toBe(true);
    });
  }
});

// A large shape is built by a loop in the child, never written as a literal.
// A parser descends one frame for each level of a nested literal, so a deep
// literal ends the child before the compiler ever reads it. A loop keeps the
// source of the child flat while the shape it makes stays large.
describe('shapes that a reader can descend into too far', () => {
  test('an object nested one thousand levels deep compiles', () => {
    const deep =
      '(() => { let v = null; for (let i = 0; i < 1000; i++) v = { a: v }; return v; })()';
    const outcome = compileWithEnv(`{ theme: ${deep} }`);

    expectNoAbort(outcome, 'a deeply nested object');
    expect(outcome.result?.ok).toBe(true);
  });

  // The reader stops at a stated depth. Below it the process aborts on a stack
  // overflow, which no `catch` in JavaScript can see. A million levels is far
  // past every stack in play, so this compiles only because the reader stops.
  test('an object nested one million levels deep compiles', () => {
    const deeper =
      '(() => { let v = null; for (let i = 0; i < 1000000; i++) v = { a: v }; return v; })()';
    const outcome = compileWithEnv(`{ theme: ${deeper} }`);

    expectNoAbort(outcome, 'an object past every stack');
    expect(outcome.result?.ok).toBe(true);
  });

  test('an array nested fifty thousand levels deep compiles', () => {
    const deep = '(() => { let v = null; for (let i = 0; i < 50000; i++) v = [v]; return v; })()';
    const outcome = compileWithEnv(`{ theme: ${deep} }`);

    expectNoAbort(outcome, 'a deeply nested array');
    expect(outcome.result?.ok).toBe(true);
  });

  test('an object holding ten thousand keys compiles', () => {
    const wide =
      '(() => { const v = {}; for (let i = 0; i < 10000; i++) v["k" + i] = null; return v; })()';
    const outcome = compileWithEnv(`{ theme: ${wide} }`);

    expectNoAbort(outcome, 'a very wide object');
    expect(outcome.result?.ok).toBe(true);
  });

  test('an array holding ten thousand nulls compiles', () => {
    const outcome = compileWithEnv('{ theme: Array(10000).fill(null) }');

    expectNoAbort(outcome, 'a very long array');
    expect(outcome.result?.ok).toBe(true);
  });

  test('a mixed tree of every kind compiles', () => {
    const mixed = `{
      s: 'text', n: 1.5, b: true, nul: null, und: undefined,
      arr: [1, null, 'two', [null, { deep: undefined }]],
      obj: { inner: { deeper: [null, 3n] } },
    }`;
    const outcome = compileWithEnv(`{ theme: ${mixed} }`);

    expectNoAbort(outcome, 'a mixed tree');
    expect(outcome.result?.ok).toBe(true);
  });

  test('an empty object and an empty array compile', () => {
    const outcome = compileWithEnv('{ a: {}, b: [] }');

    expectNoAbort(outcome, 'empty containers');
    expect(outcome.result?.ok).toBe(true);
  });
});

// A cycle has no bottom. A reader that follows it never stops on its own, so
// it runs the stack out and ends the process, which is the same failure that
// a very deep object gives. The depth the reader stops at answers both.
describe('shapes that have no bottom', () => {
  test('an object that holds itself compiles', () => {
    const outcome = compileWithEnv('(() => { const v = {}; v.self = v; return { theme: v }; })()');

    expectNoAbort(outcome, 'an object that holds itself');
    expect(outcome.result?.ok).toBe(true);
  });

  test('an array that holds itself compiles', () => {
    const outcome = compileWithEnv('(() => { const v = []; v.push(v); return { theme: v }; })()');

    expectNoAbort(outcome, 'an array that holds itself');
    expect(outcome.result?.ok).toBe(true);
  });

  test('two objects that hold each other compile', () => {
    const outcome = compileWithEnv(
      '(() => { const a = {}, b = { a }; a.b = b; return { theme: a }; })()'
    );

    expectNoAbort(outcome, 'two objects that hold each other');
    expect(outcome.result?.ok).toBe(true);
  });

  test('a cycle that a long chain leads to compiles', () => {
    const outcome = compileWithEnv(`(() => {
      const head = {};
      let tail = head;
      for (let i = 0; i < 500; i++) { tail.next = {}; tail = tail.next; }
      tail.back = head;
      return { theme: head };
    })()`);

    expectNoAbort(outcome, 'a chain that leads to a cycle');
    expect(outcome.result?.ok).toBe(true);
  });
});
