import { mkdir, mkdtemp, readFile, readdir, rm, writeFile } from 'node:fs/promises';
import path from 'node:path';

import { rspack } from '@rspack/core';
import type { Configuration } from '@rspack/core';
import { afterEach, describe, expect, test } from 'vitest';

import stylexRspack from '../src/rspack';

const roots: string[] = [];
const placeholder = '/* @stylex-placeholder */';

afterEach(async () => {
  await Promise.all(roots.splice(0).map(root => rm(root, { recursive: true, force: true })));
});

// Rspack resolves the runtime for real, so the fixture ships a stub rather than
// depending on the published package.
const runtimeStub = 'export const create = value => value;\n';

function stylexSource(color: string): string {
  return `import './global.css';
import * as stylex from '@stylexjs/stylex';

export const styles = stylex.create({ root: { color: '${color}' } });
`;
}

type BuiltCssFile = { name: string; source: string };

/**
 * Builds a fixture the way a production Rspack app is built, with a
 * content-hashed stylesheet name.
 *
 * Rspack was the one host whose verdict could not be read from its source: its
 * real-content-hash step renames an asset that a plugin changed, but only a real
 * build says whether it runs after this plugin's injection.
 */
async function buildPlaceholderFixture(color: string): Promise<BuiltCssFile[]> {
  const root = await mkdtemp(path.join(process.cwd(), '.stylex-rspack-'));
  roots.push(root);

  const files: Record<string, string> = {
    'global.css': `body { margin: 0; }\n${placeholder}\n`,
    'main.js': stylexSource(color),
    'node_modules/@stylexjs/stylex/index.js': runtimeStub,
    'node_modules/@stylexjs/stylex/package.json': JSON.stringify({
      main: 'index.js',
      name: '@stylexjs/stylex',
      version: '0.0.0',
    }),
  };

  await Promise.all(
    Object.entries(files).map(async ([file, source]) => {
      const filePath = path.join(root, file);

      await mkdir(path.dirname(filePath), { recursive: true });
      await writeFile(filePath, source);
    })
  );

  const config: Configuration = {
    context: root,
    entry: { main: path.join(root, 'main.js') },
    mode: 'production',
    // A plain stylesheet, which is the only shape the marker is supported in.
    // Rspack 2 turns CSS support on through this rule rather than a flag.
    module: { rules: [{ test: /\.css$/, type: 'css/auto' }] },
    // The fixture has no package.json, so Rspack cannot tell that the bare
    // stylesheet import has an effect, and would drop it. A real app declares
    // `sideEffects` and keeps it.
    optimization: { minimize: false, sideEffects: false },
    output: { cssFilename: '[name].[contenthash].css', path: path.join(root, 'dist') },
    plugins: [
      stylexRspack({
        useCssPlaceholder: placeholder,
        rsOptions: { dev: false, unstable_moduleResolution: { type: 'commonJS' } },
      }),
    ],
    target: 'web',
  };

  await new Promise<void>((resolve, reject) => {
    rspack(config, (error, stats) => {
      if (error) return reject(error);
      if (stats?.hasErrors()) return reject(new Error(stats.toString({ errors: true })));

      return resolve();
    });
  });

  const outDir = path.join(root, 'dist');
  const names = (await readdir(outDir)).filter(name => name.endsWith('.css'));

  return Promise.all(
    names.toSorted().map(async name => ({
      name,
      source: await readFile(path.join(outDir, name), 'utf8'),
    }))
  );
}

describe('@stylexswc/unplugin/rspack', () => {
  test('replaces the placeholder marker in the emitted stylesheet', async () => {
    const [stylesheet] = await buildPlaceholderFixture('red');

    expect(stylesheet?.source).toContain('body');
    expect(stylesheet?.source).toContain('color');
    expect(stylesheet?.source).not.toContain(placeholder);
  });

  // The plugin adds no renaming for Rspack. This says why that is enough: the
  // real-content-hash step runs after the injection and renames the asset
  // itself. Were the injection ever to move past it, this test is what notices.
  test('gives the stylesheet a new name after a StyleX-only edit', async () => {
    const [before] = await buildPlaceholderFixture('red');
    const [after] = await buildPlaceholderFixture('rebeccapurple');

    expect(before?.source).not.toBe(after?.source);
    expect(before?.name).not.toBe(after?.name);
  });

  test('gives the same input the same stylesheet name', async () => {
    const [first] = await buildPlaceholderFixture('red');
    const [second] = await buildPlaceholderFixture('red');

    expect(first?.name).toBe(second?.name);
  });
});
