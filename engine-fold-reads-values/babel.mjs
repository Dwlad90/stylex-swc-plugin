import { transformSync } from '@babel/core';
import styleXPlugin from '@stylexjs/babel-plugin';

const cases = JSON.parse(process.argv[2]);

for (const [name, decls, body] of cases) {
  const src = `
import * as stylex from '@stylexjs/stylex';
${decls}
export const styles = stylex.create({
  base: { ${body} },
});
`;
  try {
    const out = transformSync(src, {
      filename: '/stylex/packages/a.js',
      babelrc: false,
      plugins: [[styleXPlugin, { unstable_moduleResolution: { type: 'haste' }, runtimeInjection: true }]],
    });
    const rules = (out.metadata.stylex || []).map(([cls, r]) => `${cls}:${r.ltr}`).join(' | ');
    console.log(`OK   ${name} :: ${rules}`);
  } catch (e) {
    console.log(`FAIL ${name} :: ${String(e.message).split('\n').filter(Boolean).slice(0,3).join(' / ')}`);
  }
}
