import test from 'ava';
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

// Regression guard for napi 3.8.x `UnknownRef` leak warnings:
//   "ObjectRef is not unref, it considered as a memory leak"
// This fires once per element on every Rust→JS round-trip of UnknownRef
// fields. The fix (see src/lib.rs + src/structs/mod.rs) must keep stderr
// clean of that string — otherwise user builds are noisy.

const LEAK_STRING = 'ObjectRef is not unref';

const distEntry = path.resolve(fileURLToPath(import.meta.url), '../../dist/index.js');

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
