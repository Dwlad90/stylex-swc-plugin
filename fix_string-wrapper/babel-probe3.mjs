import { transformAsync } from '@babel/core';
import { createRequire } from 'node:module';

const require = createRequire(import.meta.url);
const stylexBabelPlugin = require(
  '/Users/vladislavbuinovski/Projects/Facebook/stylex-swc-plugin.git/bugfix/node_modules/.pnpm/@stylexjs+babel-plugin@0.19.0_supports-color@8.1.1/node_modules/@stylexjs/babel-plugin/lib/index.js',
);

const ROOT =
  '/Users/vladislavbuinovski/Projects/Facebook/stylex-swc-plugin.git/bugfix/crates/stylex-transform/tests/__virtual__/app/';

async function run(name, code, rel) {
  try {
    const r = await transformAsync(code, {
      filename: ROOT + rel,
      babelrc: false,
      configFile: false,
      plugins: [
        stylexBabelPlugin.withOptions({
          dev: false,
          treeshakeCompensation: true,
          unstable_moduleResolution: { type: 'commonJS', rootDir: ROOT },
        }),
      ],
    });
    console.log(`--- ${name}\nCODE: ${r.code}\nMETA: ${JSON.stringify(r.metadata.stylex)}\n`);
  } catch (e) {
    console.log(`--- ${name}\nERR: ${String(e.message).split('\n').slice(0, 3).join(' | ')}\n`);
  }
}

const header = `import * as stylex from '@stylexjs/stylex';
import { colors } from '@design-system/tokens/src/colors.stylex';
`;

await run(
  'theme_override_wrapped_in_string',
  `${header}export const lightTheme = stylex.createTheme(colors, { primary: String('#fff') });`,
  'src/themes/light.stylex.js',
);

await run(
  'theme_override_around_token_reference',
  `${header}export const darkTheme = stylex.createTheme(colors, { primary: String(colors.surface) });`,
  'src/themes/dark.stylex.js',
);

await run(
  'create_with_a_coerced_token_group',
  `${header}export const styles = stylex.create({
  root: { color: String(colors) },
  reference: { color: String(colors.primary) },
});`,
  'src/components/Card.js',
);
