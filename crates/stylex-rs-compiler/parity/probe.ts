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

const sources: Record<string, string> = JSON.parse(process.argv[2] ?? '{}');

for (const [label, source] of Object.entries(sources)) {
  console.log('='.repeat(70));
  console.log(label);
  console.log('-- source:', source.replace(/\n/g, ' ⏎ '));

  report('rust ', runRust(source));
  report('babel', runBabel(source));
}
