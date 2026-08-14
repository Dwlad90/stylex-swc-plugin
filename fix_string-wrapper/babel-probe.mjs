import { transformAsync } from '@babel/core';
import { createRequire } from 'node:module';

const require = createRequire(import.meta.url);
const stylexBabelPlugin = require(
  '/Users/vladislavbuinovski/Projects/Facebook/stylex-swc-plugin.git/bugfix/node_modules/.pnpm/@stylexjs+babel-plugin@0.19.0_supports-color@8.1.1/node_modules/@stylexjs/babel-plugin/lib/index.js',
);

async function run(name, code, filename = '/tmp/x/repro.stylex.js') {
  try {
    const r = await transformAsync(code, {
      filename,
      babelrc: false,
      configFile: false,
      plugins: [
        stylexBabelPlugin.withOptions({
          dev: false,
          treeshakeCompensation: true,
          unstable_moduleResolution: { type: 'commonJS', rootDir: '/tmp/x' },
        }),
      ],
    });
    console.log(`--- ${name}\nCODE: ${r.code}\nMETA: ${JSON.stringify(r.metadata.stylex)}\n`);
  } catch (e) {
    console.log(`--- ${name}\nERR: ${String(e.message).split('\n').slice(0, 3).join(' | ')}\n`);
  }
}

const header = `import * as stylex from '@stylexjs/stylex';\n`;

const createCases = {
  'string-literal': `String('#fff')`,
  'string-number': `String(1)`,
  'string-bool': `String(true)`,
  'string-null': `String(null)`,
  'string-undefined': `String(undefined)`,
  'string-empty-args': `String()`,
  'string-array': `String(['a','b'])`,
  'string-array-hole': `String([1,,2])`,
  'string-object': `String({a:1})`,
  'string-nan': `String(NaN)`,
  'string-infinity': `String(Infinity)`,
  'string-nested': `String(String(1))`,
  'string-surplus': `String(1, 2)`,
  'string-spread': `String(...['a','b'])`,
  'string-bigint': `String(1n)`,
  'string-1e21': `String(1e21)`,
};

for (const [name, expr] of Object.entries(createCases)) {
  await run(
    `create/${name}`,
    `${header}export const s = stylex.create({ a: { color: ${expr} } });`,
  );
}

await run(
  'defineVars/string',
  `${header}export const vars = stylex.defineVars({ background: String('#fff') });`,
  '/tmp/x/vars.stylex.js',
);

await run(
  'keyframes/string',
  `${header}export const k = stylex.keyframes({ from: { color: String('#fff') }, to: { color: String('#000') } });`,
);

await run(
  'computed-key/string',
  `${header}export const s = stylex.create({ a: { [String('color')]: 'red' } });`,
);

await run(
  'nested-value/string',
  `${header}export const s = stylex.create({ a: { color: { default: String('red'), ':hover': String('blue') } } });`,
);

await run(
  'dynamic/string-param',
  `${header}export const s = stylex.create({ a: (c) => ({ color: String(c) }) });`,
);

await run(
  'shadowed/string',
  `${header}const String = () => 'shadowed';\nexport const s = stylex.create({ a: { color: String('#fff') } });`,
);

await run(
  'local-const/string',
  `${header}const c = '#fff';\nexport const s = stylex.create({ a: { color: String(c) } });`,
);
