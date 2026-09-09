// One source through both compilers, printed side by side.
//
// The value harness compares a corpus and reports a verdict; this answers the
// question a corpus cannot, which is "what does each compiler do with *this*".
// It lives beside the harness so it resolves the same two compilers from the
// same lockfile, and it is what a divergence is measured with before a row is
// added to the corpus or a ticket is written.
//
// Usage, from `crates/stylex-rs-compiler`:
//
//   node parity/probe.mjs '{"a label": "<module source>"}'
//
// The argument is a JSON object of label to module source. `dist/` has to be
// built first.

import path from 'node:path';
import { pathToFileURL } from 'node:url';

import * as babel from '@babel/core';
import stylexBabelPluginModule from '@stylexjs/babel-plugin';

const packageDir = path.resolve(import.meta.dirname, '..');
const options = { dev: false, unstable_moduleResolution: { type: 'haste', rootDir: packageDir } };
const plugin = stylexBabelPluginModule.default ?? stylexBabelPluginModule;
const { transform } = await import(pathToFileURL(path.join(packageDir, 'dist/index.js')).href);

const filename = path.join(packageDir, 'probe.js');

/** The first three lines of whatever a compiler threw, on one line. */
const refusalOf = error =>
  String(error instanceof Error ? error.message : error)
    .split('\n')
    .slice(0, 3)
    .join(' | ');

const runRust = source => {
  try {
    const out = transform(filename, source, options);

    return { css: JSON.stringify(out.metadata.stylex), code: String(out.code) };
  } catch (error) {
    return { refusal: refusalOf(error) };
  }
};

const runBabel = source => {
  try {
    const out = babel.transformSync(source, {
      filename,
      babelrc: false,
      configFile: false,
      plugins: [[plugin, options]],
    });

    return { css: JSON.stringify(out?.metadata?.stylex), code: String(out?.code) };
  } catch (error) {
    return { refusal: refusalOf(error) };
  }
};

const sources = JSON.parse(process.argv[2] ?? '{}');

for (const [label, source] of Object.entries(sources)) {
  console.log('='.repeat(70));
  console.log(label);
  console.log('-- source:', String(source).replace(/\n/g, ' ⏎ '));

  report('rust ', runRust(String(source)));
  report('babel', runBabel(String(source)));
}

/** One compiler's answer, as two lines or as the sentence it refused with. */
function report(name, result) {
  if (result.refusal !== undefined) {
    console.log(`  ${name} REFUSED: ${result.refusal}`);

    return;
  }

  console.log(`  ${name} css: ${result.css ?? ''}`);
  console.log(`  ${name} out: ${(result.code ?? '').replace(/\s+/g, ' ').slice(0, 300)}`);
}
