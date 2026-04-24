import test from 'ava';
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const LEAK_STRING = 'ObjectRef is not unref';

const distEntry = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../dist/index.js');

function runNodeScript(script: string) {
  const result = spawnSync(process.execPath, ['-e', script], {
    encoding: 'utf8',
    env: { ...process.env, NODE_ENV: 'production' },
  });
  if (result.status !== 0) {
    throw new Error(
      `subprocess failed (exit ${result.status}):\nstdout:\n${result.stdout}\nstderr:\n${result.stderr}`
    );
  }
  return result;
}

test('normalizeRsOptions does not emit napi leak warnings across many calls', t => {
  const result = runNodeScript(`
    const { normalizeRsOptions } = require(${JSON.stringify(distEntry)});
    for (let i = 0; i < 100; i++) {
      normalizeRsOptions({
        include: ['src/**/*.ts', 'packages/*/src/**/*.tsx'],
        exclude: [/\\.test\\./, /node_modules/],
        swcPlugins: [['@swc/plugin-example', { foo: 'bar' }]],
        debugFilePath: (p) => p,
      });
    }
  `);
  t.false(
    result.stderr.includes(LEAK_STRING),
    `napi leak warnings detected in stderr:\n${result.stderr}`
  );
});

test('transform does not emit napi leak warnings across many calls', t => {
  const result = runNodeScript(`
    const { transform, normalizeRsOptions } = require(${JSON.stringify(distEntry)});
    const opts = normalizeRsOptions({
      include: ['**/*.ts'],
      exclude: [/\\.test\\./],
      debugFilePath: (p) => p,
    });
    for (let i = 0; i < 50; i++) {
      transform('file.ts', 'export const x = 1;', opts);
    }
  `);
  t.false(
    result.stderr.includes(LEAK_STRING),
    `napi leak warnings detected in stderr:\n${result.stderr}`
  );
});

test('shouldTransformFile does not emit napi leak warnings across many calls', t => {
  const result = runNodeScript(`
    const { shouldTransformFile } = require(${JSON.stringify(distEntry)});
    const include = ['src/**/*.ts', 'packages/*/src/**/*.tsx'];
    const exclude = [/\\.test\\./, /node_modules/];
    for (let i = 0; i < 100; i++) {
      shouldTransformFile('src/foo.ts', include, exclude);
    }
  `);
  t.false(
    result.stderr.includes(LEAK_STRING),
    `napi leak warnings detected in stderr:\n${result.stderr}`
  );
});

test('transform with debugFilePath function returning prefix does not leak', t => {
  const result = runNodeScript(`
    const { transform, normalizeRsOptions } = require(${JSON.stringify(distEntry)});
    const opts = normalizeRsOptions({
      debugFilePath: (p) => 'custom-prefix/' + p,
    });
    for (let i = 0; i < 50; i++) {
      transform('file.ts', 'export const x = 1;', opts);
    }
  `);
  t.false(
    result.stderr.includes(LEAK_STRING),
    `napi leak warnings detected in stderr:\n${result.stderr}`
  );
});

test('transform with env object does not emit napi leak warnings', t => {
  const result = runNodeScript(`
    const { transform, normalizeRsOptions } = require(${JSON.stringify(distEntry)});
    const opts = normalizeRsOptions({});
    // Transform with env passed to native
    for (let i = 0; i < 50; i++) {
      try {
        transform('file.ts', 'export const x = 1;', { ...opts, env: { APP_NAME: 'test' } });
      } catch (e) {
        // env parsing errors are ok, we're testing for leaks not correctness
      }
    }
  `);
  t.false(
    result.stderr.includes(LEAK_STRING),
    `napi leak warnings detected in stderr:\n${result.stderr}`
  );
});

test('transform with stylex code does not emit napi leak warnings', t => {
  const result = runNodeScript(`
    const { transform, normalizeRsOptions } = require(${JSON.stringify(distEntry)});
    const opts = normalizeRsOptions({
      treeshakeCompensation: true,
      unstable_moduleResolution: { type: 'commonJS' },
    });
    const code = \`
      import stylex from "@stylexjs/stylex";
      export const styles = stylex.create({ root: { color: "red" } });
    \`;
    for (let i = 0; i < 50; i++) {
      transform('page.tsx', code, opts);
    }
  `);
  t.false(
    result.stderr.includes(LEAK_STRING),
    `napi leak warnings detected in stderr:\n${result.stderr}`
  );
});

test('normalizeRsOptions with various input shapes does not leak', t => {
  const result = runNodeScript(`
    const { normalizeRsOptions } = require(${JSON.stringify(distEntry)});
    for (let i = 0; i < 100; i++) {
      normalizeRsOptions({});
      normalizeRsOptions({ dev: true, test: true });
      normalizeRsOptions({ importSources: ['@scope/pkg'] });
      normalizeRsOptions({
        include: ['src/**'],
        exclude: [/test/],
        swcPlugins: [['@swc/plugin', {}]],
      });
    }
  `);
  t.false(
    result.stderr.includes(LEAK_STRING),
    `napi leak warnings detected in stderr:\n${result.stderr}`
  );
});
