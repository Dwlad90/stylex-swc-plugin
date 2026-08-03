import { mkdir, mkdtemp, rm, writeFile } from 'node:fs/promises';
import path from 'node:path';

import { createServer } from 'vite';
import { afterEach, describe, expect, test, vi } from 'vitest';

import type { UnpluginStylexRSOptions } from '../src/types';
import stylexSwc from '../src/vite';

const roots: string[] = [];
const placeholder = '/* @stylex-placeholder */';

afterEach(async () => {
  await Promise.all(roots.splice(0).map(root => rm(root, { recursive: true, force: true })));
});

async function transformFixture(
  files: Record<string, string>,
  transformCss: (css: string) => string,
  options: UnpluginStylexRSOptions = {}
): Promise<void> {
  const root = await mkdtemp(path.join(process.cwd(), '.stylex-define-consts-'));
  roots.push(root);

  await Promise.all([
    writeFile(path.join(root, 'package.json'), JSON.stringify({ type: 'module' })),
    writeFile(path.join(root, 'stylex.css'), `${placeholder}\n`),
    ...Object.entries(files).map(async ([file, source]) => {
      const filePath = path.join(root, file);
      await mkdir(path.dirname(filePath), { recursive: true });
      await writeFile(filePath, source);
    }),
  ]);

  const server = await createServer({
    root,
    logLevel: 'silent',
    optimizeDeps: { noDiscovery: true },
    plugins: [
      {
        name: 'stylex-runtime-stub',
        resolveId(id) {
          return id === '@stylexjs/stylex' ? '\0stylex-runtime-stub' : null;
        },
        load(id) {
          return id === '\0stylex-runtime-stub' ? 'export const create = value => value;' : null;
        },
      },
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
});
