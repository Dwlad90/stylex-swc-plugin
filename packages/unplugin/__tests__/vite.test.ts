import { mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import path from 'node:path';

import { build, createServer } from 'vite';
import { afterEach, describe, expect, test, vi } from 'vitest';

import type { UnpluginStylexRSOptions } from '../src/types';
import stylexSwc from '../src/vite';

const roots: string[] = [];
const placeholder = '/* @stylex-placeholder */';

afterEach(async () => {
  await Promise.all(roots.splice(0).map(root => rm(root, { recursive: true, force: true })));
});

// Stands in for the runtime so the fixtures never need the real package
// installed; the compiler only cares that the import resolves.
const stylexRuntimeStub = {
  name: 'stylex-runtime-stub',
  resolveId(id: string) {
    return id === '@stylexjs/stylex' ? '\0stylex-runtime-stub' : null;
  },
  load(id: string) {
    return id === '\0stylex-runtime-stub' ? 'export const create = value => value;' : null;
  },
};

// Each fixture gets its own root so the temp directories cannot collide, and
// every root is registered for the afterEach cleanup.
async function writeFixtureRoot(prefix: string, files: Record<string, string>): Promise<string> {
  const root = await mkdtemp(path.join(process.cwd(), prefix));
  roots.push(root);

  await Promise.all([
    writeFile(path.join(root, 'package.json'), JSON.stringify({ type: 'module' })),
    ...Object.entries(files).map(async ([file, source]) => {
      const filePath = path.join(root, file);
      await mkdir(path.dirname(filePath), { recursive: true });
      await writeFile(filePath, source);
    }),
  ]);

  return root;
}

async function transformFixture(
  files: Record<string, string>,
  transformCss: (css: string) => string,
  options: UnpluginStylexRSOptions = {}
): Promise<void> {
  const root = await writeFixtureRoot('.stylex-define-consts-', {
    'stylex.css': `${placeholder}\n`,
    ...files,
  });

  const server = await createServer({
    root,
    logLevel: 'silent',
    optimizeDeps: { noDiscovery: true },
    plugins: [
      stylexRuntimeStub,
      stylexSwc({
        ...options,
        useCssPlaceholder: placeholder,
        rsOptions: {
          dev: true,
          unstable_moduleResolution: { type: 'commonJS' },
          ...options.rsOptions,
        },
        transformCss,
      }),
    ],
    server: { middlewareMode: true, preTransformRequests: false },
  });

  try {
    await server.transformRequest('/styles.ts');
    await server.transformRequest('/stylex.css');
  } finally {
    await server.close();
  }
}

const buildFixtureSource = `import * as stylex from '@stylexjs/stylex';

export const styles = stylex.create({
  root: { color: 'red' },
});
`;

const buildFixtureHtml =
  '<!doctype html><html><head></head><body><script type="module" src="/main.js"></script></body></html>\n';

// Builds for real rather than calling the href helper directly: what only a
// build can settle is which base Vite actually hands the plugin. The per-base
// string cases are covered far more precisely in resolveStylesheetHref.test.ts.
async function buildIndexHtml(base: string, pages: string[] = []): Promise<Record<string, string>> {
  const root = await writeFixtureRoot('.stylex-vite-base-', {
    'index.html': buildFixtureHtml,
    'main.js': buildFixtureSource,
    ...Object.fromEntries(pages.map(page => [page, buildFixtureHtml])),
  });

  await build({
    base,
    build: {
      write: true,
      rolldownOptions: { input: ['index.html', ...pages].map(page => path.join(root, page)) },
    },
    logLevel: 'silent',
    plugins: [
      stylexRuntimeStub,
      stylexSwc({
        fileName: 'stylex.css',
        rsOptions: { dev: false, unstable_moduleResolution: { type: 'commonJS' } },
      }),
    ],
    root,
  });

  const built = await Promise.all(
    ['index.html', ...pages].map(async page => [
      page,
      await readFile(path.join(root, 'dist', page), 'utf8'),
    ])
  );

  return Object.fromEntries(built);
}

const containerConsts = `import * as stylex from '@stylexjs/stylex';

export const Container = stylex.defineConsts({
  query: '@container example (max-width: 600px)',
});
`;

const containerStyles = `import * as stylex from '@stylexjs/stylex';
import { Container } from './tokens.stylex';

export const styles = stylex.create({
  root: {
    display: {
      default: 'flex',
      [Container.query]: 'none',
    },
  },
});
`;

function rejectUnresolvedAtRules(css: string): string {
  if (/var\(--[^)]+\)\s*\{/.test(css)) throw new Error('Invalid unresolved at-rule');
  return css;
}

describe('Vite', () => {
  test('resolves imported defineConsts at-rules before transforming placeholder CSS', async () => {
    const transformCss = vi.fn<(css: string) => string>(rejectUnresolvedAtRules);

    await transformFixture(
      { 'styles.ts': containerStyles, 'tokens.stylex.ts': containerConsts },
      transformCss
    );

    expect(transformCss).toHaveBeenCalledTimes(1);
    expect(transformCss.mock.calls[0]?.[0]).toContain('@container example (max-width: 600px)');
  });

  test('resolves defineConsts from independent dependency branches', async () => {
    const transformCss = vi.fn<(css: string) => string>(rejectUnresolvedAtRules);

    await transformFixture(
      {
        'styles.ts': `import * as stylex from '@stylexjs/stylex';
import { Container } from './container.stylex';
import { Media } from './media.stylex';

export const styles = stylex.create({
  root: {
    color: {
      default: 'red',
      [Container.query]: 'blue',
      [Media.query]: 'green',
    },
  },
});
`,
        'container.stylex.ts': containerConsts,
        'media.stylex.ts': `import * as stylex from '@stylexjs/stylex';

export const Media = stylex.defineConsts({
  query: '@media (min-width: 800px)',
});
`,
      },
      transformCss
    );

    expect(transformCss).toHaveBeenCalledTimes(1);
    expect(transformCss.mock.calls[0]?.[0]).toContain('@container example (max-width: 600px)');
    expect(transformCss.mock.calls[0]?.[0]).toContain('@media (min-width: 800px)');
  });

  test('handles cycles while transforming pending dependencies', async () => {
    const transformCss = vi.fn<(css: string) => string>(rejectUnresolvedAtRules);

    await transformFixture(
      {
        'styles.ts': containerStyles,
        'tokens.stylex.ts': `import './cycle';
${containerConsts}`,
        'cycle.ts': `import './tokens.stylex';
export const cycle = true;
`,
      },
      transformCss
    );

    expect(transformCss).toHaveBeenCalledTimes(1);
    expect(transformCss.mock.calls[0]?.[0]).toContain('@container example (max-width: 600px)');
  });

  test('resolves imported defineConsts inside CSS layers', async () => {
    const transformCss = vi.fn<(css: string) => string>(rejectUnresolvedAtRules);

    await transformFixture(
      { 'styles.ts': containerStyles, 'tokens.stylex.ts': containerConsts },
      transformCss,
      { useCSSLayers: true }
    );

    expect(transformCss).toHaveBeenCalledTimes(1);
    expect(transformCss.mock.calls[0]?.[0]).toContain('@layer');
    expect(transformCss.mock.calls[0]?.[0]).toContain('@container example (max-width: 600px)');
  });

  test('does not treat CSS variable declarations as unresolved at-rules', async () => {
    const transformCss = vi.fn<(css: string) => string>(rejectUnresolvedAtRules);

    await transformFixture(
      {
        'styles.ts': `import * as stylex from '@stylexjs/stylex';

export const styles = stylex.create({
  root: {
    color: 'var(--brand-color)',
  },
});
`,
      },
      transformCss
    );

    expect(transformCss).toHaveBeenCalledTimes(1);
    expect(transformCss.mock.calls[0]?.[0]).toContain('color:var(--brand-color)');
  });

  test('does not treat CSS strings as unresolved at-rules', async () => {
    const transformCss = vi.fn<(css: string) => string>(css => css);

    await transformFixture(
      {
        'styles.ts': `import * as stylex from '@stylexjs/stylex';

export const styles = stylex.create({
  root: {
    content: '"} var(--brand-color) {"',
  },
});
`,
      },
      transformCss
    );

    expect(transformCss).toHaveBeenCalledTimes(1);
  });

  test('defers CSS transformation when defineConsts metadata remains unavailable', async () => {
    const transformCss = vi.fn<(css: string) => string>(rejectUnresolvedAtRules);

    await transformFixture(
      { 'styles.ts': containerStyles, 'tokens.stylex.ts': containerConsts },
      transformCss,
      { rsOptions: { exclude: [/tokens\.stylex\.ts$/] } }
    );

    expect(transformCss).not.toHaveBeenCalled();
  });

  test('should inject a root-relative stylesheet link for the default base', async () => {
    const built = await buildIndexHtml('/');

    expect(built['index.html']).toContain('href="/stylex.css"');
  });

  test('should inject the stylesheet link under a full-URL base', async () => {
    const built = await buildIndexHtml('https://cdn.example.com/app/');

    expect(built['index.html']).toContain('href="https://cdn.example.com/app/stylex.css"');
  });

  test('should resolve the stylesheet link against a relative base per document', async () => {
    const built = await buildIndexHtml('./', ['pages/about.html']);

    expect(built['index.html']).toContain('href="./stylex.css"');
    expect(built['pages/about.html']).toContain('href="../stylex.css"');
  });
});
