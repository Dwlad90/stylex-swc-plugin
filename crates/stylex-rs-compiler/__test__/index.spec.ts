import test from 'ava';

import { transform, normalizeRsOptions } from '../dist/index.js';
import * as path from 'path';

const cwd = process.cwd();

test('sync function from native code', t => {
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
    map: '{"version":3,"sources":["page.tsx"],"names":[],"mappings":"AACI;AAEA;;;;;;;;EAOG"}',
  };

  t.deepEqual(result, expected);
});

// ── transform() include/exclude filtering ────────────────────────────

test('transform: skips file excluded by include pattern', t => {
  const code = 'export const x = 1;';
  const options = normalizeRsOptions({
    include: ['src/**/*.tsx'],
  });

  // File doesn't match include — should return code unmodified
  const result = transform(path.join(cwd, 'lib/file.ts'), code, options);
  t.is(result.code, code);
  t.deepEqual(result.metadata, { stylex: [] });
});

test('transform: skips file matching exclude pattern', t => {
  const code = 'export const x = 1;';
  const options = normalizeRsOptions({
    exclude: [/\.test\./],
  });

  const result = transform(path.join(cwd, 'src/file.test.tsx'), code, options);
  t.is(result.code, code);
  t.deepEqual(result.metadata, { stylex: [] });
});

test('transform: processes file matching include and not matching exclude', t => {
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
  t.truthy(result.metadata.stylex.length > 0, 'should have stylex metadata');
  t.not(result.code, code, 'code should be transformed');
});

test('transform: processes file when no include/exclude patterns', t => {
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
  t.truthy(result.metadata.stylex.length > 0, 'should transform when no filters');
});

test('transform: returns undefined map when file is filtered out', t => {
  const options = normalizeRsOptions({
    include: ['nonexistent/**'],
  });

  const result = transform('src/file.tsx', 'export const x = 1;', options);
  t.is(result.map, undefined);
});

test('transform: exclude takes precedence over include', t => {
  const code = 'export const x = 1;';
  const options = normalizeRsOptions({
    include: ['src/**/*.tsx'],
    exclude: ['src/internal/**'],
  });

  const result = transform(path.join(cwd, 'src/internal/Secret.tsx'), code, options);
  t.is(result.code, code, 'excluded file should not be transformed');
  t.deepEqual(result.metadata, { stylex: [] });
});

// ── transform() edge cases ──────────────────────────────────────────

test('transform: non-stylex code passes through without metadata', t => {
  const code = `
    import React from 'react';
    export const App = () => <div>Hello</div>;
  `;
  const options = normalizeRsOptions({
    treeshakeCompensation: true,
    unstable_moduleResolution: { type: 'commonJS' },
  });

  const result = transform('app.tsx', code, options);
  t.deepEqual(result.metadata, { stylex: [] });
  t.truthy(result.code.length > 0, 'should still have code output');
});

test('transform: empty file returns empty output', t => {
  const options = normalizeRsOptions({
    treeshakeCompensation: true,
    unstable_moduleResolution: { type: 'commonJS' },
  });

  const result = transform('empty.tsx', '', options);
  t.deepEqual(result.metadata, { stylex: [] });
});

test('transform: filtered file returns exact original code', t => {
  const code = '// comment\nexport const x = 1;\n';
  const options = normalizeRsOptions({
    include: ['nope/**'],
  });

  const result = transform('file.tsx', code, options);
  t.is(result.code, code, 'filtered file code must be identical');
  t.is(result.map, undefined, 'filtered file must have no source map');
});

test('transform: result has source map by default', t => {
  const code = `
    import stylex from "@stylexjs/stylex";
    export const s = stylex.create({ r: { color: "red" } });
  `;
  const options = normalizeRsOptions({
    treeshakeCompensation: true,
    unstable_moduleResolution: { type: 'commonJS' },
  });

  const result = transform('page.tsx', code, options);
  t.truthy(result.map, 'should have source map string');
  const parsed = JSON.parse(result.map!);
  t.is(parsed.version, 3, 'source map v3');
});

test('transform: regex include pattern works', t => {
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
  t.truthy(result.metadata.stylex.length > 0, 'regex include should match .tsx');
});

test('transform: regex exclude pattern works', t => {
  const code = 'export const x = 1;';
  const options = normalizeRsOptions({
    exclude: [/\.stories\./],
  });

  const result = transform(path.join(cwd, 'src/Button.stories.tsx'), code, options);
  t.is(result.code, code, '.stories file should be excluded');
  t.deepEqual(result.metadata, { stylex: [] });
});

test('transform: multiple include patterns - match any', t => {
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
  t.truthy(result.metadata.stylex.length > 0, 'second include pattern should match');
});

test('transform: multiple exclude patterns - match any excludes', t => {
  const code = 'export const x = 1;';
  const options = normalizeRsOptions({
    exclude: [/\.test\./, /\.spec\./],
  });

  const resultTest = transform(path.join(cwd, 'src/file.test.tsx'), code, options);
  const resultSpec = transform(path.join(cwd, 'src/file.spec.tsx'), code, options);
  t.is(resultTest.code, code, '.test file should be excluded');
  t.is(resultSpec.code, code, '.spec file should be excluded');
});

test('transform: preserves whitespace-only code when filtered', t => {
  const code = '   \n\n   \n';
  const options = normalizeRsOptions({
    include: ['nonexistent/**'],
  });

  const result = transform('file.tsx', code, options);
  t.is(result.code, code, 'whitespace-only code should be preserved exactly');
});

test('transform: glob pattern with curly braces', t => {
  const code = 'export const x = 1;';
  const options = normalizeRsOptions({
    include: ['src/**/*.{ts,tsx}'],
  });

  const resultJs = transform(path.join(cwd, 'src/file.js'), code, options);

  // .ts and .tsx should pass include filter (then go to native)
  // .js should NOT match include filter
  t.is(resultJs.code, code, '.js should not match include');
  t.deepEqual(resultJs.metadata, { stylex: [] });
});
