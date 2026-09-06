import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { expect, test } from 'vitest';

import { runNodeScript } from './nodeScript';

const LEAK_STRING = 'ObjectRef is not unref';

const distEntry = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../dist/index.js');

function runProductionScript(script: string) {
  const result = runNodeScript(script, { env: { ...process.env, NODE_ENV: 'production' } });

  // A child that never starts has no exit code. Name that cause, because an
  // exit code of null on its own reads as a crash of the script.
  if (result.error) {
    throw new Error(`subprocess did not start: ${result.error.message}`);
  }

  if (result.status !== 0) {
    throw new Error(
      `subprocess failed (exit ${result.status}):\nstdout:\n${result.stdout}\nstderr:\n${result.stderr}`
    );
  }
  return result;
}

test('normalizeRsOptions does not emit napi leak warnings across many calls', () => {
  const result = runProductionScript(`
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
  expect(
    result.stderr.includes(LEAK_STRING),
    `napi leak warnings detected in stderr:\n${result.stderr}`
  ).toBe(false);
});

test('transform does not emit napi leak warnings across many calls', () => {
  const result = runProductionScript(`
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
  expect(
    result.stderr.includes(LEAK_STRING),
    `napi leak warnings detected in stderr:\n${result.stderr}`
  ).toBe(false);
});

test('shouldTransformFile does not emit napi leak warnings across many calls', () => {
  const result = runProductionScript(`
    const { shouldTransformFile } = require(${JSON.stringify(distEntry)});
    const include = ['src/**/*.ts', 'packages/*/src/**/*.tsx'];
    const exclude = [/\\.test\\./, /node_modules/];
    for (let i = 0; i < 100; i++) {
      shouldTransformFile('src/foo.ts', include, exclude);
    }
  `);
  expect(
    result.stderr.includes(LEAK_STRING),
    `napi leak warnings detected in stderr:\n${result.stderr}`
  ).toBe(false);
});

test('transform with debugFilePath function returning prefix does not leak', () => {
  const result = runProductionScript(`
    const { transform, normalizeRsOptions } = require(${JSON.stringify(distEntry)});
    const opts = normalizeRsOptions({
      debugFilePath: (p) => 'custom-prefix/' + p,
    });
    for (let i = 0; i < 50; i++) {
      transform('file.ts', 'export const x = 1;', opts);
    }
  `);
  expect(
    result.stderr.includes(LEAK_STRING),
    `napi leak warnings detected in stderr:\n${result.stderr}`
  ).toBe(false);
});

test('transform with env object does not emit napi leak warnings', () => {
  const result = runProductionScript(`
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
  expect(
    result.stderr.includes(LEAK_STRING),
    `napi leak warnings detected in stderr:\n${result.stderr}`
  ).toBe(false);
});

test('transform with stylex code does not emit napi leak warnings', () => {
  const result = runProductionScript(`
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
  expect(
    result.stderr.includes(LEAK_STRING),
    `napi leak warnings detected in stderr:\n${result.stderr}`
  ).toBe(false);
});

test('normalizeRsOptions with various input shapes does not leak', () => {
  const result = runProductionScript(`
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
  expect(
    result.stderr.includes(LEAK_STRING),
    `napi leak warnings detected in stderr:\n${result.stderr}`
  ).toBe(false);
});
