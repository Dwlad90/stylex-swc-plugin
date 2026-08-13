// `toStrictEqual`, not `toEqual`: these were AVA's `t.deepEqual`, which treats
// an absent property and one set to `undefined` as different. `toEqual` does
// not, so it would accept a compiler result that grew or lost an
// undefined-valued field. See the note in `normalizeRsOptions.spec.ts`.
import * as path from 'path';

import { expect, test } from 'vitest';

import { transform, normalizeRsOptions } from '../dist/index.js';

const cwd = process.cwd();

test('sync function from native code', () => {
  const fixture = `
    import stylex from "@stylexjs/stylex";

    export const styles = stylex.create({
      default: {
        backgroundColor: "red",
        color: "blue",
        backgroundPosition: "end",
        float: "start"
      },
    });
  `;

  const result = transform('page.tsx', fixture, {
    dev: false,
    treeshakeCompensation: true,
    unstable_moduleResolution: {
      type: 'commonJS',
    },
  });

  const expected = {
    code: `import stylex from "@stylexjs/stylex";
export const styles = {
    default: {
        kWkggS: "xrkmrrc",
        kMwMTN: "xju2f9n",
        k1YJky: "x1ifmvib",
        kyUFMd: "xrbpyxo",
        $$css: true
    }
};
`,
    metadata: {
      stylex: [
        [
          'xrkmrrc',
          {
            ltr: '.xrkmrrc{background-color:red}',
            rtl: null,
          },
          3000,
        ],
        [
          'xju2f9n',
          {
            ltr: '.xju2f9n{color:blue}',
            rtl: null,
          },
          3000,
        ],
        [
          'x1ifmvib',
          {
            ltr: '.x1ifmvib{background-position:right}',
            rtl: '.x1ifmvib{background-position:left}',
          },
          2000,
        ],
        [
          'xrbpyxo',
          {
            ltr: '.xrbpyxo{float:left}',
            rtl: '.xrbpyxo{float:right}',
          },
          3000,
        ],
      ],
    },
    // Key order matches the serializer's. `sourcesContent` holds the authored
    // text and `mappings` carries columns — both on by default, see
    // `sourceMaps.spec.ts`.
    map: JSON.stringify({
      version: 3,
      sources: ['page.tsx'],
      sourcesContent: [fixture],
      names: [],
      mappings: 'AACI,OAAO,YAAY,mBAAmB;AAEtC,OAAO,MAAM;;;;;;;;EAOV',
    }),
  };

  expect(result).toStrictEqual(expected);
});

// ── transform() include/exclude filtering ────────────────────────────

test('transform: skips file excluded by include pattern', () => {
  const code = 'export const x = 1;';
  const options = normalizeRsOptions({
    include: ['src/**/*.tsx'],
  });

  // File doesn't match include — should return code unmodified
  const result = transform(path.join(cwd, 'lib/file.ts'), code, options);
  expect(result.code).toBe(code);
  expect(result.metadata).toStrictEqual({ stylex: [] });
});

test('transform: skips file matching exclude pattern', () => {
  const code = 'export const x = 1;';
  const options = normalizeRsOptions({
    exclude: [/\.test\./],
  });

  const result = transform(path.join(cwd, 'src/file.test.tsx'), code, options);
  expect(result.code).toBe(code);
  expect(result.metadata).toStrictEqual({ stylex: [] });
});

test('transform: processes file matching include and not matching exclude', () => {
  const code = `
    import stylex from "@stylexjs/stylex";
    export const styles = stylex.create({
      root: { color: "red" },
    });
  `;
  const options = normalizeRsOptions({
    include: ['**/*.tsx'],
    exclude: [/\.test\./],
    treeshakeCompensation: true,
    unstable_moduleResolution: { type: 'commonJS' },
  });

  const result = transform(path.join(cwd, 'src/Button.tsx'), code, options);
  // File is included — native transform should process it
  expect(result.metadata.stylex.length > 0, 'should have stylex metadata').toBeTruthy();
  expect(result.code, 'code should be transformed').not.toBe(code);
});

test('transform: processes file when no include/exclude patterns', () => {
  const code = `
    import stylex from "@stylexjs/stylex";
    export const styles = stylex.create({
      root: { color: "blue" },
    });
  `;
  const options = normalizeRsOptions({
    treeshakeCompensation: true,
    unstable_moduleResolution: { type: 'commonJS' },
  });

  const result = transform('file.tsx', code, options);
  expect(result.metadata.stylex.length > 0, 'should transform when no filters').toBeTruthy();
});

test('transform: returns undefined map when file is filtered out', () => {
  const options = normalizeRsOptions({
    include: ['nonexistent/**'],
  });

  const result = transform('src/file.tsx', 'export const x = 1;', options);
  expect(result.map).toBe(undefined);
});

test('transform: exclude takes precedence over include', () => {
  const code = 'export const x = 1;';
  const options = normalizeRsOptions({
    include: ['src/**/*.tsx'],
    exclude: ['src/internal/**'],
  });

  const result = transform(path.join(cwd, 'src/internal/Secret.tsx'), code, options);
  expect(result.code, 'excluded file should not be transformed').toBe(code);
  expect(result.metadata).toStrictEqual({ stylex: [] });
});

// ── transform() edge cases ──────────────────────────────────────────

test('transform: non-stylex code passes through without metadata', () => {
  const code = `
    import React from 'react';
    export const App = () => <div>Hello</div>;
  `;
  const options = normalizeRsOptions({
    treeshakeCompensation: true,
    unstable_moduleResolution: { type: 'commonJS' },
  });

  const result = transform('app.tsx', code, options);
  expect(result.metadata).toStrictEqual({ stylex: [] });
  expect(result.code.length > 0, 'should still have code output').toBeTruthy();
});

test('transform: empty file returns empty output', () => {
  const options = normalizeRsOptions({
    treeshakeCompensation: true,
    unstable_moduleResolution: { type: 'commonJS' },
  });

  const result = transform('empty.tsx', '', options);
  expect(result.metadata).toStrictEqual({ stylex: [] });
});

test('transform: filtered file returns exact original code', () => {
  const code = '// comment\nexport const x = 1;\n';
  const options = normalizeRsOptions({
    include: ['nope/**'],
  });

  const result = transform('file.tsx', code, options);
  expect(result.code, 'filtered file code must be identical').toBe(code);
  expect(result.map, 'filtered file must have no source map').toBe(undefined);
});

test('transform: result has source map by default', () => {
  const code = `
    import stylex from "@stylexjs/stylex";
    export const s = stylex.create({ r: { color: "red" } });
  `;
  const options = normalizeRsOptions({
    treeshakeCompensation: true,
    unstable_moduleResolution: { type: 'commonJS' },
  });

  const result = transform('page.tsx', code, options);
  expect(result.map, 'should have source map string').toBeTruthy();
  const parsed = JSON.parse(result.map!);
  expect(parsed.version, 'source map v3').toBe(3);
});

test('transform: regex include pattern works', () => {
  const code = `
    import stylex from "@stylexjs/stylex";
    export const s = stylex.create({ r: { color: "green" } });
  `;
  const options = normalizeRsOptions({
    include: [/\.tsx$/],
    treeshakeCompensation: true,
    unstable_moduleResolution: { type: 'commonJS' },
  });

  const result = transform(path.join(cwd, 'src/Comp.tsx'), code, options);
  expect(result.metadata.stylex.length > 0, 'regex include should match .tsx').toBeTruthy();
});

test('transform: regex exclude pattern works', () => {
  const code = 'export const x = 1;';
  const options = normalizeRsOptions({
    exclude: [/\.stories\./],
  });

  const result = transform(path.join(cwd, 'src/Button.stories.tsx'), code, options);
  expect(result.code, '.stories file should be excluded').toBe(code);
  expect(result.metadata).toStrictEqual({ stylex: [] });
});

test('transform: multiple include patterns - match any', () => {
  const code = `
    import stylex from "@stylexjs/stylex";
    export const s = stylex.create({ r: { color: "red" } });
  `;
  const options = normalizeRsOptions({
    include: ['src/**/*.tsx', 'lib/**/*.tsx'],
    treeshakeCompensation: true,
    unstable_moduleResolution: { type: 'commonJS' },
  });

  const result = transform(path.join(cwd, 'lib/Widget.tsx'), code, options);
  expect(result.metadata.stylex.length > 0, 'second include pattern should match').toBeTruthy();
});

test('transform: multiple exclude patterns - match any excludes', () => {
  const code = 'export const x = 1;';
  const options = normalizeRsOptions({
    exclude: [/\.test\./, /\.spec\./],
  });

  const resultTest = transform(path.join(cwd, 'src/file.test.tsx'), code, options);
  const resultSpec = transform(path.join(cwd, 'src/file.spec.tsx'), code, options);
  expect(resultTest.code, '.test file should be excluded').toBe(code);
  expect(resultSpec.code, '.spec file should be excluded').toBe(code);
});

test('transform: preserves whitespace-only code when filtered', () => {
  const code = '   \n\n   \n';
  const options = normalizeRsOptions({
    include: ['nonexistent/**'],
  });

  const result = transform('file.tsx', code, options);
  expect(result.code, 'whitespace-only code should be preserved exactly').toBe(code);
});

test('transform: glob pattern with curly braces', () => {
  const code = 'export const x = 1;';
  const options = normalizeRsOptions({
    include: ['src/**/*.{ts,tsx}'],
  });

  const resultJs = transform(path.join(cwd, 'src/file.js'), code, options);

  // .ts and .tsx should pass include filter (then go to native)
  // .js should NOT match include filter
  expect(resultJs.code, '.js should not match include').toBe(code);
  expect(resultJs.metadata).toStrictEqual({ stylex: [] });
});

// `0.4 + 2 / 10` is `0.6000000000000001`, not `0.6`. Rounding it to `0.6` would
// tie the rule with a var group nested five at-rules deep, whose priority is
// exactly `0.6`; ties are then resolved by rule content rather than by
// priority, which can order an override before the rule it overrides.
//
// Asserted here as well as in Rust because the priority crosses the napi
// boundary as a double, and nothing else in this suite reads it.
test('transform: at-rule priority reaches metadata unrounded', () => {
  const code = `
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({ accent: 'red' });
    export const theme = stylex.createTheme(vars, {
      accent: {
        default: 'blue',
        '@media (min-width: 1024px)': 'green',
      },
    });
  `;

  const result = transform(path.join(cwd, 'theme.stylex.js'), code, {
    dev: false,
    treeshakeCompensation: true,
    unstable_moduleResolution: { type: 'commonJS', rootDir: cwd },
  });

  const priorities = result.metadata.stylex.map(([, , priority]) => priority);

  expect(priorities, 'default override then the @media one').toStrictEqual([
    0.1, 0.5, 0.6000000000000001,
  ]);
});
