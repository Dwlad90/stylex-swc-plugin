/**
 * One source through both compilers, printed side by side.
 *
 * The value harness compares a corpus and reports a verdict. This answers the
 * question a corpus cannot, which is what each compiler does with *this*
 * source. It lives beside the harness so it resolves the same two compilers
 * from the same lockfile, and it is what a divergence is measured with before
 * a row is added to the corpus or a ticket is written.
 *
 * Run it from this package, after `dist/` is built:
 *
 *   pnpm run parity:probe '{"a label": "<module source>"}'
 *
 * The argument is a JSON object of label to module source.
 */

import path from 'node:path';

import * as babel from '@babel/core';

import {
  baseStyleXOptions,
  loadBabelPlugin,
  loadRustCompiler,
  messageOf,
} from './lib/compilers.js';
import { isRecord } from './lib/guards.js';

/** What one compiler answered: the style metadata and the module it printed. */
interface Answer {
  css?: string;
  code?: string;
  refusal?: string;
}

const packageDir = path.resolve(import.meta.dirname, '..');
const filename = path.join(packageDir, 'probe.js');
const options = baseStyleXOptions(packageDir);

const { transform } = await loadRustCompiler(packageDir);
const { plugin } = loadBabelPlugin();

/** The first three lines of whatever a compiler threw, on one line. */
const refusalOf = (error: unknown): string => messageOf(error).split('\n').slice(0, 3).join(' | ');

const runRust = (source: string): Answer => {
  try {
    const out = transform(filename, source, options);

    return { css: JSON.stringify(out.metadata.stylex), code: out.code };
  } catch (error) {
    return { refusal: refusalOf(error) };
  }
};

const runBabel = (source: string): Answer => {
  try {
    const out = babel.transformSync(source, {
      filename,
      babelrc: false,
      configFile: false,
      plugins: [[plugin, options]],
    });

    // The `stylex` key alone, so the two compilers print the same shape: this
    // compiler answers the rule list and Babel answers the whole metadata
    // object that holds it.
    const metadata: unknown = out?.metadata;

    return {
      css: JSON.stringify(isRecord(metadata) ? metadata.stylex : undefined),
      code: out?.code ?? '',
    };
  } catch (error) {
    return { refusal: refusalOf(error) };
  }
};

/** One compiler's answer, as two lines or as the sentence it refused with. */
const report = (name: string, answer: Answer): void => {
  if (answer.refusal !== undefined) {
    console.log(`  ${name} REFUSED: ${answer.refusal}`);

    return;
  }

  console.log(`  ${name} css: ${answer.css ?? ''}`);
  console.log(`  ${name} out: ${(answer.code ?? '').replace(/\s+/g, ' ').slice(0, 300)}`);
};

/**
 * The labelled sources the argument names, in the order it names them.
 *
 * Narrowed rather than cast, and read into a `Map`, which keeps its insertion
 * order. Three shapes are refused because each would make the listing say
 * something the argument does not:
 *
 * - anything but an object, since `Object.entries` over an array or a string
 *   yields indices and would print one line per element or character;
 * - a source that is not a string, since it would reach a compiler as
 *   `undefined` and be reported as that compiler's answer;
 * - an integer-like label, which JavaScript sorts to the front of an object
 *   whatever order it was written in. `{"1": …, "0": …}` is already reordered
 *   by the time the object exists, so the order can only be kept by refusing
 *   the labels that lose it.
 */
const readSources = (argument: string): Map<string, string> => {
  const parsed: unknown = JSON.parse(argument);

  if (!isRecord(parsed)) {
    throw new TypeError('the probe takes a JSON object of label to module source');
  }

  const sources = new Map<string, string>();

  for (const [label, source] of Object.entries(parsed)) {
    if (typeof source !== 'string') {
      throw new TypeError(`the source for "${label}" is not a string`);
    }

    if (String(Number(label)) === label) {
      throw new TypeError(`the label "${label}" reads as an array index, so it would be reordered`);
    }

    sources.set(label, source);
  }

  return sources;
};

const sources = readSources(process.argv[2] ?? '{}');

for (const [label, source] of sources) {
  console.log('='.repeat(70));
  console.log(label);
  console.log('-- source:', source.replace(/\n/g, ' ⏎ '));

  report('rust ', runRust(source));
  report('babel', runBabel(source));
}
