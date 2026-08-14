import { transformAsync } from '@babel/core';
import stylexBabelPlugin from '/Users/vladislavbuinovski/Projects/Facebook/stylex-swc-plugin.git/bugfix/node_modules/.pnpm/@stylexjs+babel-plugin@0.19.0_supports-color@8.1.1/node_modules/@stylexjs/babel-plugin/lib/index.js';

const V = {
  'String("#fff")': `String('#fff')`,
  'String(1)': `String(1)`,
  'String(true)': `String(true)`,
  'String(null)': `String(null)`,
  'String(undefined)': `String(undefined)`,
  'String()': `String()`,
  'String([1,2])': `String([1,2])`,
  'String({})': `String({})`,
  'String({a:1})': `String({a:1})`,
  'Number("10")': `Number('10')`,
  'Number("")': `Number('')`,
  'Number()': `Number()`,
  'Number(null)': `Number(null)`,
  'Number(undefined)': `Number(undefined)`,
  'Number([])': `Number([])`,
  'Number({})': `Number({})`,
  'Number("10px")': `Number('10px')`,
  'Array(3)': `Array(3)`,
  'Array(1,2)': `Array(1,2)`,
  'Object({a:1})': `Object({a:1})`,
  'Object(null)': `Object(null)`,
  'Object("str")': `Object('str')`,
  'Object(5)': `Object(5)`,
  'Math("x")': `Math('x')`,
  'String(...arr)': `String(...['a','b'])`,
  'String(1,2)': `String(1,2)`,
  'String(String(1))': `String(String(1))`,
};

for (const [name, expr] of Object.entries(V)) {
  const code = `import * as stylex from '@stylexjs/stylex';
export const s = stylex.create({ a: { color: ${expr} } });`;
  try {
    const r = await transformAsync(code, {
      filename: '/tmp/x/repro.stylex.js', babelrc: false, configFile: false,
      plugins: [stylexBabelPlugin.withOptions({ dev: false, treeshakeCompensation: true,
        unstable_moduleResolution: { type: 'commonJS', rootDir: '/tmp/x' } })],
    });
    console.log(`${name.padEnd(20)} OK   ${JSON.stringify(r.metadata.stylex?.map(m => m[1]?.ltr))}`);
  } catch (e) {
    console.log(`${name.padEnd(20)} ERR  ${String(e.message).split('\n')[0].slice(0,110)}`);
  }
}
