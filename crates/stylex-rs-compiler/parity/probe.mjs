// Ad-hoc probe: one source through both compilers, printing CSS and refusal.
// Lives beside the parity harness so it resolves the same two compilers.
import path from 'node:path';
import { pathToFileURL } from 'node:url';

import * as babel from '@babel/core';
import stylexBabelPluginModule from '@stylexjs/babel-plugin';

const packageDir = path.resolve(import.meta.dirname, '..');
const options = { dev: false, unstable_moduleResolution: { type: 'haste', rootDir: packageDir } };
const plugin = stylexBabelPluginModule.default ?? stylexBabelPluginModule;
const { transform } = await import(pathToFileURL(path.join(packageDir, 'dist/index.js')).href);

const filename = path.join(packageDir, 'probe.js');

function runRust(source) {
  try {
    const out = transform(filename, source, options);
    return { ok: true, css: out.metadata.stylex, code: out.code };
  } catch (error) {
    return {
      ok: false,
      message: String(error.message ?? error)
        .split('\n')
        .slice(0, 3)
        .join(' | '),
    };
  }
}

function runBabel(source) {
  try {
    const out = babel.transformSync(source, {
      filename,
      babelrc: false,
      configFile: false,
      plugins: [[plugin, options]],
    });
    return { ok: true, css: out.metadata.stylex, code: out.code };
  } catch (error) {
    return {
      ok: false,
      message: String(error.message ?? error)
        .split('\n')
        .slice(0, 3)
        .join(' | '),
    };
  }
}

const sources = JSON.parse(process.argv[2]);

for (const [label, source] of Object.entries(sources)) {
  console.log('='.repeat(70));
  console.log(label);
  console.log('-- source:', source.replace(/\n/g, ' ⏎ '));
  for (const [name, run] of [
    ['rust ', runRust],
    ['babel', runBabel],
  ]) {
    const result = run(source);
    if (!result.ok) {
      console.log(`  ${name} REFUSED: ${result.message}`);
    } else {
      console.log(`  ${name} css: ${JSON.stringify(result.css)}`);
      console.log(`  ${name} out: ${result.code.replace(/\s+/g, ' ').slice(0, 300)}`);
    }
  }
}
