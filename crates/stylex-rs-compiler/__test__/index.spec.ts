// `toStrictEqual`, not `toEqual`: these were AVA's `t.deepEqual`, which treats
// an absent property and one set to `undefined` as different. `toEqual` does
// not, so it would accept a compiler result that grew or lost an
// undefined-valued field. See the note in `normalizeRsOptions.spec.ts`.
import { execFileSync } from 'node:child_process';
import * as path from 'path';

import { expect, test } from 'vitest';

import { transform, normalizeRsOptions } from '../dist/index.js';

const cwd = process.cwd();

/// Resolved once so a child process requires the same binding this suite does.
const compilerEntry = path.resolve(__dirname, '../dist/index.js');

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

// The at-rule priority is deliberately left unrounded; `var_group_priority` in
// `define_vars_utils.rs` records why. Asserted here because the value crosses
// the napi boundary as a double and nothing else in this suite reads it.
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

  expect(priorities, 'var group, default override, then the @media one').toStrictEqual([
    0.1, 0.5, 0.6000000000000001,
  ]);
});

// ── maxEvaluationDepth across the boundary ─────────────────────────

// What only a spec on this side can prove: the number survives serialization and
// reaches the evaluator. The same source compiles under a ceiling that allows it
// and is refused under one that does not, so a value that stopped crossing the
// boundary would show up as the refusal disappearing.
const deepFixture = (depth: number) => {
  let expr = 'MY_CONST';

  for (let i = 0; i < depth; i += 1) {
    expr = `(${expr} + 1)`;
  }

  return `
    import stylex from "@stylexjs/stylex";
    const MY_CONST = 5;
    export const styles = stylex.create({ base: { zIndex: ${expr} } });
  `;
};

const compileAtDepth = (source: string, maxEvaluationDepth?: number) =>
  transform('page.tsx', source, {
    dev: false,
    maxEvaluationDepth,
    unstable_moduleResolution: { type: 'commonJS' },
  });

/// The CSS a compile injected. `$$css` is present for any successful compile, so
/// it says the module compiled and nothing about what the tower folded to --
/// which is the thing a ceiling silently ceasing to apply would change.
const injectedCss = (result: ReturnType<typeof compileAtDepth>) =>
  result.metadata.stylex.map(([, rule]) => rule.ltr).join('');

test('maxEvaluationDepth: a raised ceiling folds what the default refuses', () => {
  const source = deepFixture(100);

  expect(() => compileAtDepth(source)).toThrow(/too deeply nested/);
  // `MY_CONST` is 5 and the fixture adds 1 a hundred times.
  expect(injectedCss(compileAtDepth(source, 320))).toContain('z-index:105');
});

test('maxEvaluationDepth: a lowered ceiling refuses what the default folds', () => {
  const source = deepFixture(10);

  expect(injectedCss(compileAtDepth(source))).toContain('z-index:15');
  expect(() => compileAtDepth(source, 4)).toThrow(
    /At most 4 levels of nested evaluation are supported/
  );
});

// A ceiling the boundary cannot represent is not a ceiling. `napi_get_value_uint32`
// applies `ToUint32` rather than refusing, so a negative number used to arrive as
// ~4.29 billion and remove the guard it was configuring; anything past the 32-bit
// range wrapped instead. Both now read as unset, which is observable as the
// default's refusal still happening.
test('maxEvaluationDepth: a negative ceiling falls back to the default', () => {
  expect(() => compileAtDepth(deepFixture(100), -1)).toThrow(
    /At most 32 levels of nested evaluation are supported/
  );
});

// The other half of the same defect: `ToUint32` wrapped a number past the 32-bit
// range down to something small, so `2 ** 32` used to read as `0` and fall back
// to the default. It is now a legitimate -- if absurd -- depth, clamped to the
// compiler's own limit, so what it must not do is refuse a tower the default
// refuses only because the ceiling wrapped underneath it.
test('maxEvaluationDepth: a ceiling past the 32-bit range is clamped, not wrapped', () => {
  expect(injectedCss(compileAtDepth(deepFixture(100), 2 ** 32))).toContain('z-index:105');
});

// The environment variable the README documents as a process-wide escape hatch,
// exercised where it actually lives. It is read once per process behind a
// `OnceLock`, so it cannot be observed from a test that shares this process --
// which is why the Rust side could only ever assert the precedence rule, and did
// so against a variable name nothing proved was the one being read.
//
// A child per case, because the read is cached for the life of the process.
const compileInChildWithEnv = (source: string, env: Record<string, string>) => {
  const script = `
    const { transform } = require(${JSON.stringify(compilerEntry)});
    const source = ${JSON.stringify(source)};
    try {
      const result = transform('page.tsx', source, {
        dev: false,
        unstable_moduleResolution: { type: 'commonJS' },
      });
      process.stdout.write(JSON.stringify({ ok: true, code: result.code }));
    } catch (error) {
      process.stdout.write(JSON.stringify({ ok: false, message: String(error) }));
    }
  `;

  const printed = execFileSync(process.execPath, ['-e', script], {
    env: { ...process.env, ...env },
    encoding: 'utf8',
  });

  return JSON.parse(printed) as { ok: boolean; code?: string; message?: string };
};

test('STYLEX_MAX_EVALUATION_DEPTH raises the ceiling for the whole process', () => {
  const source = deepFixture(100);

  expect(compileInChildWithEnv(source, {}).message).toMatch(
    /At most 32 levels of nested evaluation are supported/
  );
  expect(compileInChildWithEnv(source, { STYLEX_MAX_EVALUATION_DEPTH: '320' }).ok).toBe(true);
});

// Precedence, across the boundary rather than as a unit rule: a configured
// option beats the environment, so a stray value in a CI environment cannot
// change what a project that configured one compiles to.
test('an explicit maxEvaluationDepth wins over the environment', () => {
  const script = `
    const { transform } = require(${JSON.stringify(compilerEntry)});
    try {
      transform('page.tsx', ${JSON.stringify(deepFixture(100))}, {
        dev: false,
        maxEvaluationDepth: 8,
        unstable_moduleResolution: { type: 'commonJS' },
      });
      process.stdout.write('folded');
    } catch (error) {
      process.stdout.write(String(error));
    }
  `;

  const printed = execFileSync(process.execPath, ['-e', script], {
    env: { ...process.env, STYLEX_MAX_EVALUATION_DEPTH: '320' },
    encoding: 'utf8',
  });

  expect(printed).toMatch(/At most 8 levels of nested evaluation are supported/);
});

// An unusable value is read as unset rather than failing the build, because the
// variable is an escape hatch and one that broke a build when mistyped would be
// a worse one.
test.each([
  ['zero', '0'],
  ['a word', 'nope'],
  ['negative', '-1'],
])('STYLEX_MAX_EVALUATION_DEPTH given %s falls back to the default', (_label, value) => {
  expect(
    compileInChildWithEnv(deepFixture(100), { STYLEX_MAX_EVALUATION_DEPTH: value }).message
  ).toMatch(/At most 32 levels of nested evaluation are supported/);
});

// The default the compiler owns, observed through the boundary rather than read
// from the Rust constant: 29 levels fold and 30 do not.
test("maxEvaluationDepth: the default ceiling is the compiler's own", () => {
  expect(injectedCss(compileAtDepth(deepFixture(29)))).toContain('z-index:34');
  expect(() => compileAtDepth(deepFixture(30))).toThrow(
    /At most 32 levels of nested evaluation are supported/
  );
});
