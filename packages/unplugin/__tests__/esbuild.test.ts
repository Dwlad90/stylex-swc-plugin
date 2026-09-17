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
    entryNames?: string;
    files?: Record<string, string>;
    metafile?: boolean;
    pluginOptions?: UnpluginStylexRSOptions;
  } = {}
): Promise<{
  cssFiles: BuiltCssFile[];
  metafile: esbuild.Metafile | undefined;
  outDir: string;
  warnings: esbuild.Message[];
}> {
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
    entryNames: options.entryNames,
    entryPoints: [path.join(root, 'main.js')],
    logLevel: 'silent',
    metafile: options.metafile,
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
    metafile: result.metafile,
    outDir,
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

  // The rules are written into the file after esbuild has named and hashed it,
  // so a StyleX-only edit used to change the bytes and keep the name. The names
  // are asserted by hand: reading back every `.css` in the directory would miss
  // a rename entirely.
  describe('stylesheet naming', () => {
    const hashedNames = { entryNames: '[dir]/[name]-[hash]' };
    const otherStyleXSource = stylexSource.replace("color: 'red'", "color: 'rebeccapurple'");

    test('renames the stylesheet after a StyleX-only edit', async () => {
      const before = await buildPlaceholderFixture(hashedNames);
      const after = await buildPlaceholderFixture({
        ...hashedNames,
        files: { 'main.js': otherStyleXSource },
      });

      expect(before.cssFiles[0]?.source).not.toBe(after.cssFiles[0]?.source);
      expect(before.cssFiles[0]?.name).not.toBe(after.cssFiles[0]?.name);
      // Still one stylesheet: a rename must move the file, not copy it.
      expect(after.cssFiles).toHaveLength(1);
      // The hash keeps esbuild's own place and character set, even though the
      // digest is ours: esbuild's `[hash]` cannot be reproduced from contents.
      expect(after.cssFiles[0]?.name).toMatch(/^main-[A-Z0-9]+\.css$/);
    });

    test('gives the same input the same stylesheet name', async () => {
      const first = await buildPlaceholderFixture(hashedNames);
      const second = await buildPlaceholderFixture(hashedNames);

      expect(first.cssFiles.map(file => file.name)).toEqual(second.cssFiles.map(file => file.name));
    });

    // esbuild's default template carries no hash, which is the user opting out
    // of cache busting. Renaming there would only break a hand-written link.
    test('leaves the name alone when the template asks for no hash', async () => {
      const { cssFiles } = await buildPlaceholderFixture();

      expect(cssFiles.map(file => file.name)).toEqual(['main.css']);
    });

    // A stylesheet whose name already holds capitals and dashes is where a
    // careless reading of the template would take part of the name for the
    // hash, or give up and leave the name stale.
    test.each([
      ['a nested output directory', '[dir]/nested/[name]-[hash]', /^nested\/main-[A-Z0-9]+\.css$/],
      ['a hash before the name', '[dir]/[hash]-[name]', /^[A-Z0-9]+-main\.css$/],
    ])('finds the hash in a template with %s', async (_label, entryNames, shape) => {
      const before = await buildPlaceholderFixture({ entryNames });
      const after = await buildPlaceholderFixture({
        entryNames,
        files: { 'main.js': otherStyleXSource },
      });

      expect(after.cssFiles[0]?.name.split(path.sep).join('/')).toMatch(shape);
      expect(before.cssFiles[0]?.name).not.toBe(after.cssFiles[0]?.name);
      // A sub-directory used to hide the stylesheet from the output scan, so
      // the marker never came out.
      expect(after.cssFiles[0]?.source).not.toContain(placeholder);
    });

    test('keeps the metafile in step with the file on disk', async () => {
      const { cssFiles, metafile, outDir } = await buildPlaceholderFixture({
        ...hashedNames,
        metafile: true,
      });

      const cssOutputs = Object.entries(metafile?.outputs ?? {}).filter(([name]) =>
        name.endsWith('.css')
      );

      expect(cssOutputs).toHaveLength(1);

      const [name, output] = cssOutputs[0] ?? [];

      // The metafile is read before the injection, so both the name and the
      // size would otherwise describe a file that no longer exists. Its names
      // are relative to the build's working directory, which the fixture puts
      // one level above the output directory.
      expect(path.resolve(path.dirname(outDir), name ?? '')).toBe(
        path.join(outDir, cssFiles[0]?.name ?? '')
      );
      expect(output?.bytes).toBe(Buffer.byteLength(cssFiles[0]?.source ?? '', 'utf8'));
      // A metafile used to send the injection looking in the wrong directory,
      // which left the marker in the stylesheet.
      expect(cssFiles[0]?.source).toContain('color');
      expect(cssFiles[0]?.source).not.toContain(placeholder);
    });

    test('points the JavaScript output at the renamed stylesheet', async () => {
      const { metafile } = await buildPlaceholderFixture({ ...hashedNames, metafile: true });
      const outputs = Object.entries(metafile?.outputs ?? {});
      const cssNames = outputs.filter(([name]) => name.endsWith('.css')).map(([name]) => name);
      // `cssBundle` names the stylesheet a JavaScript output pulls in. A rename
      // that left it alone would point consumers at a file that is gone.
      const bundled = outputs
        .map(([, output]) => output.cssBundle)
        .filter(name => name !== undefined);

      expect(bundled).not.toHaveLength(0);

      for (const name of bundled) expect(cssNames).toContain(name);
    });
  });
});
