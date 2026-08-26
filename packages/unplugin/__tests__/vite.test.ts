import { promises } from 'node:fs';
import { mkdir, mkdtemp, readFile, readdir, rm, writeFile } from 'node:fs/promises';
import path from 'node:path';

import { build, createServer } from 'vite';
import type { Plugin, PluginOption } from 'vite';
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
  resolveId(id) {
    return id === '@stylexjs/stylex' ? '\0stylex-runtime-stub' : null;
  },
  load(id) {
    return id === '\0stylex-runtime-stub' ? 'export const create = value => value;' : null;
  },
} satisfies Plugin;

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

// One document pulling in one StyleX module is all either host below needs to
// reach the link injection.
const linkFixtureFiles: Record<string, string> = {
  'index.html': buildFixtureHtml,
  'main.js': buildFixtureSource,
};

// The host is the only thing the two fixtures disagree about, so `dev` is the
// only knob: everything that decides the injected href is shared.
function linkFixturePlugins(dev: boolean): PluginOption[] {
  return [
    stylexRuntimeStub,
    stylexSwc({
      fileName: 'stylex.css',
      rsOptions: { dev, unstable_moduleResolution: { type: 'commonJS' } },
    }),
  ];
}

// Builds for real rather than calling the href helper directly: what only a
// build can settle is which base Vite actually hands the plugin. The per-base
// string cases are covered far more precisely in resolveStylesheetHref.test.ts.
async function buildIndexHtml(base: string, pages: string[] = []): Promise<Record<string, string>> {
  const root = await writeFixtureRoot('.stylex-vite-base-', {
    ...linkFixtureFiles,
    ...Object.fromEntries(pages.map(page => [page, buildFixtureHtml])),
  });

  await build({
    base,
    build: {
      // Spelled out because the assertions read the emitted HTML back off disk.
      write: true,
      rolldownOptions: { input: ['index.html', ...pages].map(page => path.join(root, page)) },
    },
    configFile: false,
    logLevel: 'silent',
    plugins: linkFixturePlugins(false),
    root,
  });

  const built = await Promise.all(
    ['index.html', ...pages].map(async (page): Promise<[string, string]> => [
      page,
      await readFile(path.join(root, 'dist', page), 'utf8'),
    ])
  );

  return Object.fromEntries(built);
}

// The dev server is the one host where the injected href and the path the CSS
// middleware answers on are allowed to differ, and it resolves `base` on its
// own terms, so it needs its own fixture rather than the build one above.
async function transformDevIndexHtml(base: string): Promise<string> {
  const root = await writeFixtureRoot('.stylex-vite-dev-base-', linkFixtureFiles);

  const server = await createServer({
    base,
    configFile: false,
    logLevel: 'silent',
    optimizeDeps: { noDiscovery: true },
    plugins: linkFixturePlugins(true),
    root,
    server: { middlewareMode: true, preTransformRequests: false },
  });

  try {
    // Collects the rules the injection needs; nothing has requested the module
    // yet at the point the document is transformed.
    await server.transformRequest('/main.js');

    return await server.transformIndexHtml('/index.html', buildFixtureHtml);
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

// The reported bug needs one StyleX module to be transformed *after* the
// placeholder CSS has already been loaded. A slow transform makes that ordering
// deterministic; a large module graph produces it on its own.
function delayModuleTransform(suffix: string, ms: number): Plugin {
  return {
    name: 'delay-module-transform',
    enforce: 'pre',
    async transform(code, id) {
      if (!id.endsWith(suffix)) return null;

      await new Promise(resolve => {
        setTimeout(resolve, ms);
      });

      return code;
    },
  };
}

const eagerPlaceholderSource = `import * as stylex from '@stylexjs/stylex';
import './global.css';

export const styles = stylex.create({ eager: { color: 'red' } });

void import('./lazy.js');
`;

const lazyPlaceholderSource = `import * as stylex from '@stylexjs/stylex';

export const styles = stylex.create({ lazy: { backgroundColor: 'blue' } });
`;

// Mirrors the reproduction from issue #1276: an eager rule, a rule behind a
// dynamic import, and a stylesheet carrying the marker.
const placeholderFixtureFiles: Record<string, string> = {
  'index.html': buildFixtureHtml,
  'main.js': eagerPlaceholderSource,
  'lazy.js': lazyPlaceholderSource,
  'global.css': `body { margin: 0; }\n${placeholder}\n.after-marker { color: green; }\n`,
};

type BuiltCssFile = { name: string; source: string; linked: boolean };

// Reads every emitted stylesheet back off disk together with whether the
// document actually links it, which is the distinction the bug turns on.
async function readBuiltCss(outDir: string, html: string): Promise<BuiltCssFile[]> {
  const entries = await readdir(outDir, { recursive: true, withFileTypes: true });
  const cssFiles = entries
    .filter(entry => entry.isFile() && entry.name.endsWith('.css'))
    .map(entry => path.relative(outDir, path.join(entry.parentPath, entry.name)));

  return Promise.all(
    cssFiles.toSorted().map(async name => ({
      name,
      source: await readFile(path.join(outDir, name), 'utf8'),
      linked: html.includes(name.split(path.sep).join('/')),
    }))
  );
}

// Builds the placeholder fixture for real: only a full build exercises the
// ordering between the plugin hooks and the bundler's own CSS asset.
async function buildPlaceholderFixture(
  options: {
    cssCodeSplit?: boolean;
    cssMinify?: boolean | 'esbuild' | 'lightningcss';
    files?: Record<string, string>;
    onWarn?: (warning: { message: string }) => void;
    plugins?: PluginOption[];
    pluginOptions?: UnpluginStylexRSOptions;
  } = {}
): Promise<BuiltCssFile[]> {
  const root = await writeFixtureRoot('.stylex-vite-placeholder-', {
    ...placeholderFixtureFiles,
    ...options.files,
  });

  await build({
    build: {
      cssCodeSplit: options.cssCodeSplit ?? false,
      cssMinify: options.cssMinify,
      outDir: 'dist',
      write: true,
      rolldownOptions: options.onWarn
        ? { onLog: (_level, log) => options.onWarn?.(log) }
        : undefined,
    },
    configFile: false,
    logLevel: 'silent',
    plugins: [
      ...(options.plugins ?? [delayModuleTransform('/lazy.js', 100)]),
      stylexRuntimeStub,
      stylexSwc({
        fileName: 'stylex.[hash].css',
        useCssPlaceholder: placeholder,
        ...options.pluginOptions,
        rsOptions: {
          dev: false,
          unstable_moduleResolution: { type: 'commonJS' },
          ...options.pluginOptions?.rsOptions,
        },
      }),
    ],
    root,
  });

  const outDir = path.join(root, 'dist');
  const html = await readFile(path.join(outDir, 'index.html'), 'utf8');

  return readBuiltCss(outDir, html);
}

// Counting occurrences is what separates "the rules are present" from "the
// rules are present once", which is the difference an appended copy hides.
function countOccurrences(source: string, needle: string): number {
  return source.split(needle).length - 1;
}

// Drops the bundler's stylesheets before the injection runs, which is the one
// way to reach "the marker was in the build but nothing can carry the rules"
// without an exotic host configuration.
const dropCssAssets: Plugin = {
  name: 'drop-css-assets',
  // Also `post`, and registered ahead of the plugin under test, so the
  // stylesheets are gone by the time the injection looks for them.
  generateBundle: {
    order: 'post',
    handler(_options, bundle) {
      for (const fileName of Object.keys(bundle)) {
        if (fileName.endsWith('.css')) Reflect.deleteProperty(bundle, fileName);
      }
    },
  },
};

// Comfortably past the plugin's 50ms debounce, and past the retry that follows
// a refresh whose update could not be sent.
async function settle(): Promise<void> {
  await new Promise(resolve => {
    setTimeout(resolve, 200);
  });
}

// The dev server every placeholder refresh test runs against. Request
// pre-transform is off so each test decides exactly when a module is
// transformed, which is what the refresh behaviour turns on.
async function createPlaceholderDevServer(prefix: string) {
  const root = await writeFixtureRoot(prefix, placeholderFixtureFiles);

  return createServer({
    configFile: false,
    logLevel: 'silent',
    optimizeDeps: { noDiscovery: true },
    plugins: [
      stylexRuntimeStub,
      stylexSwc({
        rsOptions: { dev: true, unstable_moduleResolution: { type: 'commonJS' } },
        useCssPlaceholder: placeholder,
      }),
    ],
    root,
    server: { middlewareMode: true, preTransformRequests: false },
  });
}

// Counts how many times the dev server is told the placeholder stylesheet is
// stale. Without a bundle step that invalidation is the only way rules
// collected after the stylesheet was served can reach the browser.
async function countDevCssInvalidations(): Promise<number> {
  const server = await createPlaceholderDevServer('.stylex-vite-dev-refresh-');
  const invalidate = vi.spyOn(server.moduleGraph, 'invalidateModule');

  try {
    await server.transformRequest('/main.js');
    await server.transformRequest('/global.css');
    await settle();

    const beforeLateModule = invalidate.mock.calls.length;

    // The module behind the dynamic import is transformed only now, long after
    // the stylesheet was served.
    await server.transformRequest('/lazy.js');
    await settle();

    return invalidate.mock.calls.length - beforeLateModule;
  } finally {
    invalidate.mockRestore();
    await server.close();
  }
}

// Drives a dev server whose CSS updates never reach the browser, and reports
// the attempts made before the retries had time to stop and after. The
// stylesheet stays stale until an update lands, so a failure has to be retried;
// a retry that never gives up keeps adding attempts in the second window.
async function measureFailedRefreshRetries(): Promise<{
  attempts: number;
  attemptsAfterSettling: number;
}> {
  const server = await createPlaceholderDevServer('.stylex-vite-dev-retry-');
  const error = vi.spyOn(console, 'error').mockImplementation(() => {});
  const send = vi.spyOn(server.ws, 'send').mockImplementation(() => {
    throw new Error('websocket closed');
  });

  try {
    await server.transformRequest('/main.js');
    await server.transformRequest('/global.css');
    await settle();

    const attempts = send.mock.calls.length;

    // No further transform, so anything counted here is the retry chain alone.
    await settle();
    await settle();

    return { attempts, attemptsAfterSettling: send.mock.calls.length - attempts };
  } finally {
    send.mockRestore();
    error.mockRestore();
    await server.close();
  }
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

  test('should inject the stylesheet link under a sub-path base', async () => {
    const built = await buildIndexHtml('/app/');

    expect(built['index.html']).toContain('href="/app/stylex.css"');
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

  test('injects late dynamic-import rules into the linked stylesheet without code splitting', async () => {
    const cssFiles = await buildPlaceholderFixture();

    const linked = cssFiles.filter(file => file.linked);

    expect(linked).toHaveLength(1);
    expect(linked[0]?.source).toContain('color:red');
    expect(linked[0]?.source).toContain('background-color:');
    expect(cssFiles.filter(file => !file.linked)).toEqual([]);
  });

  test('injects every StyleX rule exactly once', async () => {
    const cssFiles = await buildPlaceholderFixture();
    const linked = cssFiles.find(file => file.linked);

    expect(countOccurrences(linked?.source ?? '', 'color:red')).toBe(1);
    expect(countOccurrences(linked?.source ?? '', 'background-color:')).toBe(1);
  });

  // The rules are spliced in after Vite has already minified the stylesheet, so
  // without minifying them too they would be the only unminified CSS shipped.
  test('minifies the injected rules alongside the rest of the stylesheet', async () => {
    const cssFiles = await buildPlaceholderFixture({ cssMinify: 'esbuild' });
    const source = cssFiles.find(file => file.linked)?.source ?? '';

    expect(source).toContain('color:red');
    // A trailing semicolon before the closing brace is what the unminified
    // rules carry, and the minifier is the only thing that takes it out.
    expect(source).not.toContain('color:red;}');
    expect(source).not.toMatch(/\n\s*\n/);
  });

  test('leaves the injected rules alone when CSS minification is off', async () => {
    const cssFiles = await buildPlaceholderFixture({ cssMinify: false });
    const source = cssFiles.find(file => file.linked)?.source ?? '';

    expect(source).toContain('color:red');
    // Minification is the user's call, so the authored value survives untouched.
    expect(source).toContain('background-color:blue');
  });

  test('keeps StyleX rules at the marker position', async () => {
    const cssFiles = await buildPlaceholderFixture();
    const source = cssFiles.find(file => file.linked)?.source ?? '';

    // The marker sits between the reset and the override, so the injected
    // rules have to land there rather than at the end of the file.
    expect(source.indexOf('margin:0')).toBeLessThan(source.indexOf('color:red'));
    expect(source.indexOf('color:red')).toBeLessThan(source.indexOf('color:green'));
  });

  test('leaves neither marker nor build placeholder behind when no StyleX rules exist', async () => {
    const cssFiles = await buildPlaceholderFixture({
      files: {
        'main.js': "import './global.css';\n\nexport const noStyles = true;\n",
        'lazy.js': 'export const lazy = true;\n',
      },
      plugins: [],
    });

    for (const file of cssFiles) {
      expect(file.source).not.toContain(placeholder);
      expect(file.source).not.toContain('__stylex_build_placeholder__');
    }
  });

  test('fails the build when no CSS asset can carry the placeholder styles', async () => {
    await expect(
      buildPlaceholderFixture({
        plugins: [delayModuleTransform('/lazy.js', 0), dropCssAssets],
      })
    ).rejects.toThrow(/no CSS asset was available/);
  });

  test('stays quiet when an SSR bundle emits no stylesheet of its own', async () => {
    const root = await writeFixtureRoot('.stylex-vite-ssr-', placeholderFixtureFiles);

    await build({
      build: { outDir: 'dist', ssr: 'main.js', write: true },
      configFile: false,
      logLevel: 'silent',
      plugins: [
        stylexRuntimeStub,
        stylexSwc({
          fileName: 'stylex.[hash].css',
          rsOptions: { dev: false, unstable_moduleResolution: { type: 'commonJS' } },
          useCssPlaceholder: placeholder,
        }),
      ],
      root,
    });

    const emitted = await readdir(path.join(root, 'dist'), { recursive: true });

    expect(emitted.filter(name => name.endsWith('.css'))).toEqual([]);
  });

  // The rules accumulate for the whole watch session, so a module whose styles
  // were deleted would otherwise keep serving them until the server restarted.
  test('drops the rules of a module that no longer produces any', async () => {
    const server = await createPlaceholderDevServer('.stylex-vite-dev-prune-');
    const root = server.config.root;

    try {
      await server.transformRequest('/main.js');
      await server.transformRequest('/lazy.js');

      const withRules = await server.transformRequest('/global.css');

      expect(withRules?.code).toContain('background-color:blue');

      // The lazy module keeps its StyleX import, so it is still transformed --
      // it simply has no styles left to contribute.
      await writeFile(
        path.join(root, 'lazy.js'),
        "import * as stylex from '@stylexjs/stylex';\n\nexport const styles = {};\n"
      );
      server.moduleGraph.invalidateAll();
      await server.transformRequest('/lazy.js');

      const afterEdit = await server.transformRequest('/global.css');

      expect(afterEdit?.code).not.toContain('background-color:blue');
    } finally {
      await server.close();
    }
  });

  // Re-reading every stylesheet in the graph on every refresh does not scale
  // with the number of CSS modules, and only an edit can change the answer.
  test('reads a stylesheet once to learn whether it carries the marker', async () => {
    const server = await createPlaceholderDevServer('.stylex-vite-dev-cache-');
    const readFileSpy = vi.spyOn(promises, 'readFile');

    try {
      await server.transformRequest('/main.js');
      await server.transformRequest('/global.css');
      await settle();

      const globalCssReads = () =>
        readFileSpy.mock.calls.filter(
          call => typeof call[0] === 'string' && call[0].endsWith('global.css')
        ).length;
      const before = globalCssReads();

      await server.transformRequest('/lazy.js');
      await settle();

      // The second refresh reuses what the first learned.
      expect(globalCssReads()).toBe(before);
    } finally {
      readFileSpy.mockRestore();
      await server.close();
    }
  });

  test('notices the marker being added to a stylesheet after it was cached', async () => {
    const server = await createPlaceholderDevServer('.stylex-vite-dev-cache-add-');
    const root = server.config.root;
    const extraCss = path.join(root, 'extra.css');

    await writeFile(extraCss, '.extra{outline:0}\n');

    try {
      // Cached as marker-free on the first look.
      await server.transformRequest('/main.js');
      await server.transformRequest('/extra.css');
      await settle();

      await writeFile(extraCss, `.extra{outline:0}\n${placeholder}\n`);
      server.watcher.emit('change', extraCss);

      const invalidate = vi.spyOn(server.moduleGraph, 'invalidateModule');

      await server.transformRequest('/lazy.js');
      await settle();

      const invalidated = invalidate.mock.calls.map(call => call[0]?.id ?? '');

      expect(invalidated.some(id => id.endsWith('extra.css'))).toBe(true);

      invalidate.mockRestore();
    } finally {
      await server.close();
    }
  });

  test('invalidates dev CSS again when a late module adds rules', async () => {
    expect(await countDevCssInvalidations()).toBeGreaterThan(0);
  });

  test('retries the dev CSS refresh when the update cannot be sent', async () => {
    const { attempts } = await measureFailedRefreshRetries();

    expect(attempts).toBeGreaterThan(1);
  });

  test('gives up retrying the dev CSS refresh instead of looping forever', async () => {
    const { attemptsAfterSettling } = await measureFailedRefreshRetries();

    expect(attemptsAfterSettling).toBe(0);
  });

  test('injects once when the marker appears several times', async () => {
    const cssFiles = await buildPlaceholderFixture({
      files: {
        'global.css': `body { margin: 0; }\n${placeholder}\n.after-marker { color: green; }\n${placeholder}\n`,
      },
    });
    const source = cssFiles.find(file => file.linked)?.source ?? '';

    expect(countOccurrences(source, 'color:red')).toBe(1);
    expect(source).not.toContain(placeholder);
    expect(source).not.toContain('__stylex_build_placeholder__');
  });

  test('strips the marker from stylesheets that did not receive the rules', async () => {
    const cssFiles = await buildPlaceholderFixture({
      cssCodeSplit: true,
      files: {
        'lazy.js': `import * as stylex from '@stylexjs/stylex';\nimport './lazy.css';\n\nexport const styles = stylex.create({ lazy: { backgroundColor: 'blue' } });\n`,
        'lazy.css': `.lazy-scope { outline: 0; }\n${placeholder}\n`,
      },
    });

    const withRules = cssFiles.filter(file => file.source.includes('color:red'));

    expect(withRules).toHaveLength(1);

    for (const file of cssFiles) {
      expect(file.source).not.toContain(placeholder);
      expect(file.source).not.toContain('__stylex_build_placeholder__');
    }
  });

  test('injects once when the default `@stylex;` marker is repeated', async () => {
    const cssFiles = await buildPlaceholderFixture({
      files: {
        'global.css': 'body { margin: 0; }\n@stylex;\n.after-marker { color: green; }\n@stylex;\n',
      },
      pluginOptions: { useCssPlaceholder: true },
    });
    const source = cssFiles.find(file => file.linked)?.source ?? '';

    // Unlike a comment marker, this one survives minification, so a leftover
    // would ship as an unknown at-rule.
    expect(source).not.toContain('@stylex;');
    expect(countOccurrences(source, 'color:red')).toBe(1);
  });

  test('keeps an `@import` that follows the marker', async () => {
    const cssFiles = await buildPlaceholderFixture({
      files: {
        'global.css': `${placeholder}\n@import './imported.css';\nbody { margin: 0; }\n`,
        'imported.css': '.imported { outline-style: dashed; }\n',
      },
    });
    const source = cssFiles.find(file => file.linked)?.source ?? '';

    // A `@layer` statement is allowed ahead of `@import`, so replacing the
    // marker in place cannot invalidate one that follows it.
    expect(source).toContain('outline-style:dashed');
    expect(source).toContain('color:red');
  });

  test('downgrades the missing-target failure to a warning on request', async () => {
    const warnings: string[] = [];

    // The setup this option exists for: a plugin that takes the stylesheet out
    // of the bundle, so nothing is left to inject into.
    const cssFiles = await buildPlaceholderFixture({
      onWarn: warning => warnings.push(warning.message),
      pluginOptions: { onMissingCssPlaceholder: 'warn' },
      plugins: [delayModuleTransform('/lazy.js', 0), dropCssAssets],
    });

    expect(cssFiles).toEqual([]);
    expect(warnings).toContainEqual(
      expect.stringContaining('no CSS asset contained the placeholder')
    );
  });

  test('stays silent about a missing target on request', async () => {
    const warnings: string[] = [];

    await buildPlaceholderFixture({
      onWarn: warning => warnings.push(warning.message),
      pluginOptions: { onMissingCssPlaceholder: 'ignore' },
      plugins: [delayModuleTransform('/lazy.js', 0), dropCssAssets],
    });

    expect(warnings).toEqual([]);
  });

  test('should inject a base-prefixed stylesheet link in dev', async () => {
    const html = await transformDevIndexHtml('/app/');

    expect(html).toContain('href="/app/stylex.css"');
  });
});
