import { mkdir, mkdtemp, readFile, readdir, rm, writeFile } from 'node:fs/promises';
import path from 'node:path';

import * as esbuild from 'esbuild';
import { afterEach, describe, expect, test } from 'vitest';

import stylexEsbuild from '../src/esbuild';
import type { UnpluginStylexRSOptions } from '../src/types';

const roots: string[] = [];
const placeholder = '/* @stylex-placeholder */';

afterEach(async () => {
  await Promise.all(roots.splice(0).map(root => rm(root, { recursive: true, force: true })));
});

const stylexSource = `import * as stylex from '@stylexjs/stylex';
import './global.css';

export const styles = stylex.create({ eager: { color: 'red' } });
`;

// esbuild resolves the runtime for real, so the fixture ships a stub rather than
// depending on the published package.
const runtimeStub = 'export const create = value => value;\n';

async function writeFixtureRoot(files: Record<string, string>): Promise<string> {
  const root = await mkdtemp(path.join(process.cwd(), '.stylex-esbuild-'));
  roots.push(root);

  await Promise.all(
    Object.entries(files).map(async ([file, source]) => {
      const filePath = path.join(root, file);
      await mkdir(path.dirname(filePath), { recursive: true });
      await writeFile(filePath, source);
    })
  );

  return root;
}

type BuiltCssFile = { name: string; source: string };

/**
 * Bundles a fixture through esbuild and reads every emitted stylesheet back off
 * disk, which is the only place the esbuild adapter's injection is observable:
 * it rewrites the files after esbuild has written them.
 */
async function buildPlaceholderFixture(
  options: {
    files?: Record<string, string>;
    pluginOptions?: UnpluginStylexRSOptions;
  } = {}
): Promise<{ cssFiles: BuiltCssFile[]; warnings: esbuild.Message[] }> {
  const root = await writeFixtureRoot({
    'node_modules/@stylexjs/stylex/package.json': JSON.stringify({
      name: '@stylexjs/stylex',
      main: 'index.js',
      version: '0.0.0',
    }),
    'node_modules/@stylexjs/stylex/index.js': runtimeStub,
    'main.js': stylexSource,
    'global.css': `body { margin: 0; }\n${placeholder}\n.after-marker { color: green; }\n`,
    ...options.files,
  });

  const result = await esbuild.build({
    absWorkingDir: root,
    bundle: true,
    entryPoints: [path.join(root, 'main.js')],
    logLevel: 'silent',
    outdir: path.join(root, 'dist'),
    plugins: [
      stylexEsbuild({
        useCssPlaceholder: placeholder,
        ...options.pluginOptions,
        rsOptions: {
          dev: false,
          unstable_moduleResolution: { type: 'commonJS' },
          ...options.pluginOptions?.rsOptions,
        },
      }),
    ],
    write: true,
  });

  const outDir = path.join(root, 'dist');
  const entries = await readdir(outDir, { recursive: true, withFileTypes: true });
  const cssFiles = await Promise.all(
    entries
      .filter(entry => entry.isFile() && entry.name.endsWith('.css'))
      .map(async entry => {
        const filePath = path.join(entry.parentPath, entry.name);

        return { name: path.relative(outDir, filePath), source: await readFile(filePath, 'utf8') };
      })
  );

  return {
    cssFiles: cssFiles.toSorted((a, b) => a.name.localeCompare(b.name)),
    warnings: result.warnings,
  };
}

function countOccurrences(source: string, needle: string): number {
  return source.split(needle).length - 1;
}

describe('@stylexswc/unplugin/esbuild', () => {
  test('replaces the placeholder marker in the emitted stylesheet', async () => {
    const { cssFiles } = await buildPlaceholderFixture();
    const stylesheet = cssFiles.find(file => file.source.includes('color:red'));

    expect(stylesheet).toBeDefined();
    expect(stylesheet?.source).toContain('body');
    expect(stylesheet?.source).toContain('.after-marker');
    expect(stylesheet?.source).not.toContain(placeholder);
    expect(stylesheet?.source).not.toContain('__stylex_build_placeholder__');
  });

  test('keeps the rules at the marker position', async () => {
    const { cssFiles } = await buildPlaceholderFixture();
    const source = cssFiles.find(file => file.source.includes('color:red'))?.source ?? '';

    // The rules belong between the two rules the fixture wrote around the
    // marker, not appended after both.
    expect(source.indexOf('margin')).toBeLessThan(source.indexOf('color:red'));
    expect(source.indexOf('color:red')).toBeLessThan(source.indexOf('.after-marker'));
  });

  test('injects once when the marker appears several times', async () => {
    const { cssFiles } = await buildPlaceholderFixture({
      files: {
        'global.css': `body { margin: 0; }\n${placeholder}\n.after-marker { color: green; }\n${placeholder}\n`,
      },
    });
    const source = cssFiles.find(file => file.source.includes('color:red'))?.source ?? '';

    expect(countOccurrences(source, 'color:red')).toBe(1);
    expect(source).not.toContain(placeholder);
    expect(source).not.toContain('__stylex_build_placeholder__');
  });

  test('leaves no marker behind when the build has no StyleX rules', async () => {
    const { cssFiles } = await buildPlaceholderFixture({
      files: { 'main.js': "import './global.css';\n" },
    });

    expect(cssFiles.length).toBeGreaterThan(0);
    for (const file of cssFiles) {
      expect(file.source).not.toContain(placeholder);
      expect(file.source).not.toContain('__stylex_build_placeholder__');
    }
  });

  test('warns when no stylesheet can carry the rules', async () => {
    const { warnings } = await buildPlaceholderFixture({
      files: { 'main.js': stylexSource.replace("import './global.css';\n", '') },
    });

    expect(warnings.map(warning => warning.text)).toContainEqual(
      expect.stringContaining('no CSS asset contained the placeholder')
    );
  });

  test('stays silent about a missing target on request', async () => {
    const { warnings } = await buildPlaceholderFixture({
      files: { 'main.js': stylexSource.replace("import './global.css';\n", '') },
      pluginOptions: { onMissingCssPlaceholder: 'ignore' },
    });

    expect(warnings).toEqual([]);
  });
});
