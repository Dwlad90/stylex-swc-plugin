import { transformAsync } from '@babel/core';
import { createRequire } from 'node:module';
import fs from 'node:fs';
import path from 'node:path';

const require = createRequire(import.meta.url);
const stylexBabelPlugin = require(
  '/Users/vladislavbuinovski/Projects/Facebook/stylex-swc-plugin.git/bugfix/node_modules/.pnpm/@stylexjs+babel-plugin@0.19.0_supports-color@8.1.1/node_modules/@stylexjs/babel-plugin/lib/index.js',
);

const ROOT = '/tmp/stylex-probe';
fs.mkdirSync(ROOT, { recursive: true });
fs.writeFileSync(
  path.join(ROOT, 'tokens.stylex.js'),
  `import * as stylex from '@stylexjs/stylex';\nexport const colors = stylex.defineVars({ primary: 'red' });\n`,
);
fs.writeFileSync(path.join(ROOT, 'package.json'), '{"name":"probe","version":"0.0.0"}');

async function run(name, code, filename = path.join(ROOT, 'repro.stylex.js'), opts = {}) {
  try {
    const r = await transformAsync(code, {
      filename,
      babelrc: false,
      configFile: false,
      plugins: [
        stylexBabelPlugin.withOptions({
          dev: false,
          treeshakeCompensation: true,
          unstable_moduleResolution: { type: 'commonJS', rootDir: ROOT },
          ...opts,
        }),
      ],
    });
    console.log(`--- ${name}\nCODE: ${r.code}\nMETA: ${JSON.stringify(r.metadata.stylex)}\n`);
  } catch (e) {
    console.log(`--- ${name}\nERR: ${String(e.message).split('\n').slice(0, 3).join(' | ')}\n`);
  }
}

const header = `import * as stylex from '@stylexjs/stylex';\n`;

await run(
  'defineVars/zero-args',
  `${header}export const vars = stylex.defineVars({ background: String() });`,
  path.join(ROOT, 'vars.stylex.js'),
);

await run(
  'defineVars/empty-string',
  `${header}export const vars = stylex.defineVars({ background: '' });`,
  path.join(ROOT, 'vars2.stylex.js'),
);

await run(
  'createTheme/string',
  `${header}import { colors } from './tokens.stylex.js';\nexport const t = stylex.createTheme(colors, { primary: String('#fff') });`,
);

await run(
  'create/token-ref',
  `${header}import { colors } from './tokens.stylex.js';\nexport const s = stylex.create({ a: { color: String(colors.primary) } });`,
);

await run(
  'create/token-group',
  `${header}import { colors } from './tokens.stylex.js';\nexport const s = stylex.create({ a: { color: String(colors) } });`,
);

await run(
  'create/env-object',
  `${header}export const s = stylex.create({ a: { color: String(stylex.env) } });`,
  path.join(ROOT, 'repro.stylex.js'),
  { env: { theme: 'dark' } },
);

await run(
  'create/callback',
  `${header}export const s = stylex.create({ a: { color: String(() => 'x') } });`,
);

await run(
  'create/fn-config',
  `${header}export const s = stylex.create({ a: { color: String(stylex.firstThatWorks) } });`,
);

await run(
  'create/neg-zero',
  `${header}export const s = stylex.create({ a: { color: String(-0) } });`,
);

await run(
  'create/small-exp',
  `${header}export const s = stylex.create({ a: { color: String(0.0000001) } });`,
);

await run(
  'create/nested-array',
  `${header}export const s = stylex.create({ a: { color: String([1,[2,3]]) } });`,
);

await run(
  'create/array-of-null',
  `${header}export const s = stylex.create({ a: { color: String([null, undefined, 1]) } });`,
);

await run(
  'create/string-var-string',
  `${header}export const s = stylex.create({ a: { color: 'a'.concat(String(1)) } });`,
);
