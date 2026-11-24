import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';

import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';

import createBuilder from '../src/builder';
import type { StyleXPluginOption } from '../src/types';

// Counts the transforms and can make the next one fail, so a test can tell a
// retried file from a cached one.
const compiler = vi.hoisted(() => ({ attempts: 0, failNext: false }));

vi.mock('@stylexswc/rs-compiler', async importOriginal => {
  const actual = await importOriginal<typeof import('@stylexswc/rs-compiler')>();

  return {
    ...actual,
    transform: (...args: Parameters<typeof actual.transform>) => {
      compiler.attempts++;
      if (compiler.failNext) {
        compiler.failNext = false;
        throw new Error('transient transform failure');
      }

      return actual.transform(...args);
    },
  };
});

const stylexImport = 'import * as stylex from "@stylexjs/stylex";\n';
const source = (color: string) =>
  `${stylexImport}export const styles = stylex.create({root: {color: "${color}"}});\n`;

describe('PostCSS builder file results', () => {
  let cwd: string;
  let file: string;
  let time: number;

  beforeEach(() => {
    cwd = fs.mkdtempSync(path.join(os.tmpdir(), 'stylex-postcss-rebuild-'));
    file = path.join(cwd, 'component.js');
    // Whole seconds, so each write sets an mtime that no earlier write had.
    time = Math.floor(Date.now() / 1000) * 1000;
    compiler.attempts = 0;
    compiler.failNext = false;
  });

  afterEach(() => {
    vi.restoreAllMocks();
    fs.rmSync(cwd, { recursive: true, force: true });
  });

  function put(contents: string) {
    fs.writeFileSync(file, contents);
    time += 1000;
    fs.utimesSync(file, time / 1000, time / 1000);
  }

  function setup({ isDev = true, excluded = false } = {}) {
    const builder = createBuilder();
    const config: StyleXPluginOption = {
      cwd,
      include: ['*.js'],
      exclude: excluded ? ['component.js'] : [],
      useCSSLayers: false,
      importSources: ['@stylexjs/stylex'],
      isDev,
      rsOptions: { dev: false, runtimeInjection: false },
    };
    builder.configure(config);

    return {
      // Applies a changed configuration, as a new PostCSS config does.
      reconfigure: (change: Partial<StyleXPluginOption>) =>
        builder.configure({ ...config, ...change }),
      build: async (shouldSkipTransformError = false) =>
        builder.build({ shouldSkipTransformError }),
      failOnce: () => {
        compiler.failNext = true;
      },
      attempts: () => compiler.attempts,
    };
  }

  test.each([
    ['last style', `${stylexImport}export const styles = {};\n`],
    ['StyleX import', 'export const styles = {};\n'],
  ])('removes prior CSS when the %s is removed', async (_name, replacement) => {
    const { build } = setup();
    put(source('red'));
    expect(await build()).toContain('color:red');
    put(replacement);
    expect(await build()).toBe('');
  });

  // The titles keep the reference's wording. Here the `exclude` option leaves
  // the same files out.
  test.each([false, true])('Babel-ignored files contribute no CSS (isDev: %s)', async isDev => {
    const { build, attempts } = setup({ isDev, excluded: true });
    put(source('red'));
    expect(await build()).toBe('');
    expect(attempts()).toBe(0);
  });

  test('clears old rules when a changed file becomes Babel-ignored', async () => {
    const { reconfigure, build } = setup();
    put(source('red'));
    expect(await build()).toContain('color:red');
    reconfigure({ exclude: ['component.js'] });
    put(source('blue'));
    expect(await build()).toBe('');
  });

  test('preserves successful rules on a skipped failure and retries unchanged source', async () => {
    const { build, failOnce, attempts } = setup();
    put(source('red'));
    const before = await build();
    expect(before).toContain('color:red');
    put(source('blue'));
    failOnce();
    const warn = vi.spyOn(console, 'warn').mockImplementation(() => {});
    expect(await build(true)).toBe(before);
    expect(warn).toHaveBeenCalledTimes(1);
    expect(attempts()).toBe(2);
    const after = await build(true);
    expect(after).toContain('color:blue');
    expect(after).not.toContain('color:red');
    expect(attempts()).toBe(3);
    expect(warn).toHaveBeenCalledTimes(1);
  });

  test('propagates non-skipped failures and retries on the next build', async () => {
    const { build, failOnce, attempts } = setup();
    put(source('red'));
    await build();
    put(source('blue'));
    failOnce();
    await expect(build(false)).rejects.toThrow('transient transform failure');
    expect(await build(false)).toContain('color:blue');
    expect(attempts()).toBe(3);
  });

  test('still caches successful files and removes deleted files', async () => {
    const { build, attempts } = setup();
    put(source('red'));
    const before = await build();
    expect(before).toContain('color:red');
    expect(await build()).toBe(before);
    expect(attempts()).toBe(1);
    fs.unlinkSync(file);
    expect(await build()).toBe('');
  });
});
