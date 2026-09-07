// A dynamic `stylex.create` entry declared inside a nested scope must stay a
// function after compilation (#1303). The Rust suite holds the snapshots. This
// suite runs the compiled output, because the report was a run-time failure:
// `styles.color is not a function`. A snapshot cannot show that the call
// works, only what it looks like.
//
// The output runs in a child process with the shipped native binding, so the
// assertion is about `dist/*.node` and not about the Rust sources. Rebuild the
// binding before this suite means anything.
import { transformSync } from '@swc/core';
import { describe, expect, test } from 'vitest';

import { transform } from '../dist/index.js';
import { runNodeScript } from './nodeScript';

/** The compiled class name object for `color: var(--x-color)`. */
const COLOR_CLASS = { kMwMTN: 'x14rh7hd', $$css: true } as const;
/** The compiled style object for `display: flex`. */
const BASE_STYLE = { k1xSpc: 'x78zum5', $$css: true } as const;

/**
 * How many entries the many-entry case declares of each kind.
 *
 * The rewrite runs once per dynamic entry. A fault that rewrites only the
 * first entry, or only the entries before a static sibling, passes a case with
 * two entries. The number is only "many"; no threshold is known.
 */
const ENTRY_COUNT = 150;

/** One compiled module and the export that renders a value. */
interface Case {
  readonly source: string;
  /** The path of the render function under `module.exports`. */
  readonly render: string;
}

/** The two scopes the issue reported, each with a static sibling. */
const CASES = {
  namespace: {
    render: 'Demo.render',
    source: `
      import * as stylex from '@stylexjs/stylex';
      export namespace Demo {
        const styles = stylex.create({
          color: (value: string) => ({ color: value }),
          base: { display: 'flex' },
        });
        export function render(value: string) {
          return stylex.props(styles.base, styles.color(value));
        }
      }
    `,
  },
  iife: {
    render: 'render',
    source: `
      import * as stylex from '@stylexjs/stylex';
      export const render = (() => {
        const styles = stylex.create({
          color: (value: string) => ({ color: value }),
          base: { display: 'flex' },
        });
        return (value: string) => stylex.props(styles.base, styles.color(value));
      })();
    `,
  },
} as const satisfies Record<string, Case>;

/** A function body with many dynamic entries, each beside a static sibling. */
function manyEntriesCase(count: number): Case {
  const entries = Array.from({ length: count }, (_, index) =>
    [
      `dynamic${index}: (value: string) => ({ color: value }),`,
      `static${index}: { display: 'flex' },`,
    ].join('\n')
  ).join('\n');
  const calls = Array.from(
    { length: count },
    (_, index) => `stylex.props(styles.static${index}, styles.dynamic${index}(value))`
  ).join(',\n');

  return {
    render: 'render',
    source: `
      import * as stylex from '@stylexjs/stylex';
      export function render(value: string) {
        const styles = stylex.create({
          ${entries}
        });
        return [${calls}];
      }
    `,
  };
}

/** Compiles one source with the shipped binding, as a bundler would. */
function compile(source: string): string {
  const result = transform('MyComponent.tsx', source, {
    dev: false,
    runtimeInjection: false,
    treeshakeCompensation: true,
    unstable_moduleResolution: { type: 'commonJS' },
  });

  return result.code;
}

/**
 * Lowers the compiled TypeScript module to a CommonJS body.
 *
 * The compiled output keeps the TypeScript syntax of the input, and a namespace
 * is not erasable, so the child cannot run it as it is.
 */
function toCommonJs(code: string): string {
  return transformSync(code, {
    filename: 'MyComponent.tsx',
    jsc: { parser: { syntax: 'typescript', tsx: true }, target: 'es2022' },
    module: { type: 'commonjs' },
  }).code;
}

/**
 * Runs the compiled module in a child process and returns what its render
 * function answered for each value.
 *
 * `stylex.props` is a stub that returns its arguments, so the result holds the
 * compiled style objects themselves and the assertions can name them.
 */
function renderInChild({ render, source }: Case, values: readonly string[]): unknown[] {
  const body = toCommonJs(compile(source));
  const script = `
    const stylex = { props: (...styles) => styles };
    const requireForCompiled = (id) => {
      if (id === '@stylexjs/stylex') return stylex;
      throw new Error('The compiled module imported ' + id);
    };
    const module = { exports: {} };
    new Function('require', 'module', 'exports', ${JSON.stringify(body)})(
      requireForCompiled,
      module,
      module.exports
    );
    const results = ${JSON.stringify(values)}.map((value) => module.exports.${render}(value));
    process.stdout.write(JSON.stringify(results));
  `;
  const outcome = runNodeScript(script);

  if (outcome.error) {
    throw new Error(`The child process did not start: ${outcome.error.message}`);
  }

  if (outcome.status !== 0) {
    throw new Error(`The compiled output failed (exit ${outcome.status}):\n${outcome.stderr}`);
  }

  const results: unknown = JSON.parse(outcome.stdout);

  if (!Array.isArray(results)) {
    throw new Error(`The child process answered no array:\n${outcome.stdout}`);
  }

  return results;
}

describe('a dynamic entry in a nested scope', () => {
  test.each(Object.entries(CASES))(
    '%s: the entry is callable and the sibling is intact',
    (_name, testCase) => {
      const [red, blue] = renderInChild(testCase, ['red', 'blue']);

      // Each call gives the static sibling, then the dynamic entry: the class
      // name object and the inline variable that carries the argument.
      expect(red).toStrictEqual([BASE_STYLE, [COLOR_CLASS, { '--x-color': 'red' }]]);
      expect(blue).toStrictEqual([BASE_STYLE, [COLOR_CLASS, { '--x-color': 'blue' }]]);
    }
  );

  test('a function body with many entries answers every call', () => {
    const [results] = renderInChild(manyEntriesCase(ENTRY_COUNT), ['green']);

    expect(results).toStrictEqual(
      Array.from({ length: ENTRY_COUNT }, () => [
        BASE_STYLE,
        [COLOR_CLASS, { '--x-color': 'green' }],
      ])
    );
  });
});
