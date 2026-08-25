/**
 * Issue 05 spike: does the engine-backed fold agree with the reference
 * implementation, declaration for declaration?
 *
 * Runs one module through `@stylexjs/babel-plugin` and through this compiler's
 * `dist/`, then compares the emitted class names and declaration text. A
 * spike-only harness: it takes the module path on the command line rather than
 * reading the parity corpus.
 */

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import * as babel from '@babel/core';

import { baseStyleXOptions, loadBabelPlugin, loadRustCompiler } from './lib/compilers.js';

const packageDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');

/** Every `[className, declarationText]` pair a compiler emitted, sorted. */
function rules(metadata: readonly unknown[]): string[] {
  return metadata
    .map(entry => {
      const [className, rule] = entry as [string, { ltr?: string } | undefined];
      return `${className}\t${rule?.ltr ?? ''}`;
    })
    .sort();
}

async function main(): Promise<void> {
  const file = path.resolve(process.argv[2]);
  const code = fs.readFileSync(file, 'utf8');
  const options = baseStyleXOptions(packageDir);

  const { plugin } = loadBabelPlugin();
  const babelResult = await babel.transformAsync(code, {
    filename: file,
    babelrc: false,
    configFile: false,
    plugins: [[plugin, options]],
  });

  const babelRules = rules((babelResult?.metadata as { stylex?: unknown[] })?.stylex ?? []);

  const { transform } = await loadRustCompiler(packageDir);
  const rustRules = rules(transform(file, code, options).metadata.stylex);

  const onlyBabel = babelRules.filter(rule => !rustRules.includes(rule));
  const onlyRust = rustRules.filter(rule => !babelRules.includes(rule));

  console.log(`babel rules: ${babelRules.length}`);
  console.log(`rust  rules: ${rustRules.length}`);
  console.log(`agreed:      ${babelRules.length - onlyBabel.length}`);

  for (const rule of onlyBabel) console.log(`  only babel: ${rule}`);
  for (const rule of onlyRust) console.log(`  only rust : ${rule}`);

  process.exitCode = onlyBabel.length + onlyRust.length === 0 ? 0 : 1;
}

await main();
