import { once } from 'node:events';
import { mkdtemp, rm, writeFile } from 'node:fs/promises';
import { createServer as createHttpServer } from 'node:http';
import path from 'node:path';

import { createLogger, createServer } from 'vite';
import type { Connect } from 'vite';
import { afterEach, expect, test, vi } from 'vitest';

import createBundledDevCss from '../src/utils/bundledDevCss';

const marker = '/* @stylex-placeholder */';
const cleanups: (() => Promise<void>)[] = [];
const errorHandler: Connect.ErrorHandleFunction = (error: unknown, _req, res, _next) => {
  res.statusCode = 500;
  res.end(error instanceof Error ? error.message : 'Unknown stylesheet error');
};

afterEach(async () => {
  for (const cleanup of cleanups.splice(0).toReversed()) await cleanup();
});

async function fixture(
  files: Record<string, string>,
  renderRules: (file: string) => Promise<string> = async () => '.generated{color:red}',
  base = '/'
) {
  const root = await mkdtemp(path.join(process.cwd(), '.stylex-bundled-http-'));
  cleanups.push(() => rm(root, { recursive: true, force: true }));
  await Promise.all(
    Object.entries(files).map(([file, source]) => writeFile(path.join(root, file), source))
  );
  const css = createBundledDevCss(marker, renderRules);
  const logger = createLogger('silent');
  const info = vi.spyOn(logger, 'info');
  const server = await createServer({
    root,
    base,
    configFile: false,
    logLevel: 'silent',
    customLogger: logger,
    experimental: { bundledDev: true },
    server: { middlewareMode: true },
    plugins: [
      {
        name: 'bundled-css-fixture',
        configResolved: config => css.configure(config),
        configureServer(devServer) {
          css.configureServer(devServer);
          devServer.middlewares.use((_req, res) => {
            res.statusCode = 418;
            res.end('fixture fallthrough');
          });
          devServer.middlewares.use(errorHandler);
        },
      },
    ],
  });
  cleanups.push(() => server.close());
  const http = createHttpServer(server.middlewares);
  http.listen(0, '127.0.0.1');
  await once(http, 'listening');
  cleanups.push(
    () =>
      new Promise<void>((resolve, reject) => {
        http.close(error => (error ? reject(error) : resolve()));
        http.closeAllConnections();
      })
  );
  const address = http.address();
  if (!address || typeof address === 'string') throw new Error('Missing HTTP fixture address');
  const origin = `http://127.0.0.1:${address.port}`;

  async function runtime(file: string): Promise<string> {
    const resolved = await css.resolveId.call(
      {
        environment: { config: { isBundled: true } },
        resolve: async id => ({ id: path.join(root, id) }),
      },
      file,
      path.join(root, 'main.js')
    );
    if (!resolved) throw new Error(`Stylesheet was not redirected: ${file}`);
    const code = css.load(resolved.id);
    if (!code) throw new Error(`Runtime was not loaded: ${file}`);
    return code;
  }

  async function href(file: string): Promise<string> {
    const code = await runtime(file);
    const match = /const href = (.+);/.exec(code);
    if (!match) throw new Error('Runtime has no stylesheet href');
    const value: unknown = JSON.parse(match[1]!);
    if (typeof value !== 'string') throw new Error('Stylesheet href is not a string');
    return value;
  }

  return { root, css, server, origin, runtime, href, info };
}

test('replaces imported dependencies when the served stylesheet changes', async () => {
  const { root, server, origin, href } = await fixture({
    'global.css': `@import './old.css';\n${marker}`,
    'old.css': '.old{color:blue}',
    'new.css': '.new{color:green}',
  });
  const url = origin + (await href('global.css'));
  expect(await (await fetch(url)).text()).toContain('.old');
  await writeFile(path.join(root, 'global.css'), `@import './new.css';\n${marker}`);
  expect(await (await fetch(url)).text()).toContain('.new');

  const send = vi.spyOn(server.ws, 'send');
  server.watcher.emit('change', path.join(root, 'old.css'));
  expect(send).not.toHaveBeenCalled();
  server.watcher.emit('change', path.join(root, 'new.css'));
  expect(send).toHaveBeenCalledWith({ type: 'custom', event: 'stylex:css-refresh' });
});

test('loads a runtime that links the served stylesheet and subscribes to refreshes', async () => {
  const { runtime } = await fixture({ 'global.css': marker });
  const code = await runtime('global.css');

  expect(code).toContain('const href = "/global.css?stylex-css"');
  expect(code).toContain("document.createElement('link')");
  expect(code).toContain("link.rel = 'stylesheet'");
  expect(code).toContain('hot.on("stylex:css-refresh", refresh)');
  expect(code).toContain('hot.prune(');
});

test('bundles the stylesheet runtime instead of the authored marker stylesheet', async () => {
  const root = await mkdtemp(path.join(process.cwd(), '.stylex-bundled-import-'));
  cleanups.push(() => rm(root, { recursive: true, force: true }));
  await Promise.all([
    writeFile(
      path.join(root, 'index.html'),
      '<html><body><script type="module" src="/main.js"></script></body></html>'
    ),
    writeFile(path.join(root, 'main.js'), "import './global.css';\nconsole.log('fixture ready');"),
    writeFile(path.join(root, 'global.css'), `.authored-only{color:purple}\n${marker}`),
  ]);
  const css = createBundledDevCss(marker, async () => '.generated{color:red}');
  const server = await createServer({
    root,
    configFile: false,
    logLevel: 'silent',
    experimental: { bundledDev: true },
    server: { host: '127.0.0.1', port: 0 },
    plugins: [
      {
        name: 'bundled-css-import-fixture',
        configResolved: config => css.configure(config),
        configureServer: devServer => css.configureServer(devServer),
        resolveId: {
          order: 'pre',
          handler(id, importer) {
            return css.resolveId.call(this, id, importer);
          },
        },
        load: id => css.load(id),
      },
    ],
  });
  cleanups.push(() => server.close());
  await server.listen();
  const address = server.httpServer?.address();
  if (!address || typeof address === 'string') throw new Error('Missing bundled server address');
  const origin = `http://127.0.0.1:${address.port}`;
  let html = '';
  await vi.waitFor(
    async () => {
      html = await (await fetch(origin)).text();
      expect(html).not.toContain('__vite_is_fallback_page__');
      expect(html).toContain('<script');
    },
    { timeout: 5000 }
  );
  const scripts = [...html.matchAll(/<script[^>]+src="([^"]+)"/g)].map(match => match[1]!);
  expect(scripts.length).toBeGreaterThan(0);
  const bundle = (
    await Promise.all(scripts.map(async src => (await fetch(new URL(src, origin))).text()))
  ).join('\n');

  expect(bundle).toContain('/global.css?stylex-css');
  expect(bundle).toContain('stylex:css-refresh');
  expect(bundle).not.toContain('.authored-only');
  expect(bundle).not.toContain(marker);
  const stylesheet = await (await fetch(`${origin}/global.css?stylex-css`)).text();
  expect(stylesheet).toContain('.authored-only');
  expect(stylesheet).toContain('.generated{color:red}');
});

test('serves CSS with no-store, ETag, HEAD and conditional GET support', async () => {
  const { origin, href } = await fixture({ 'global.css': `.before{margin:0}\n${marker}` });
  const url = origin + (await href('global.css'));
  const response = await fetch(url);

  expect(response.status).toBe(200);
  expect(response.headers.get('content-type')).toContain('text/css');
  expect(response.headers.get('cache-control')).toBe('no-store');
  expect(await response.text()).toContain('.generated{color:red}');
  const etag = response.headers.get('etag');
  expect(etag).toBeTruthy();

  const head = await fetch(url, { method: 'HEAD' });
  expect(head.status).toBe(200);
  expect(head.headers.get('etag')).toBe(etag);
  expect(await head.text()).toBe('');

  const unchanged = await fetch(url, { headers: { 'If-None-Match': etag! } });
  expect(unchanged.status).toBe(304);
  expect(await unchanged.text()).toBe('');
});

test('serves encoded stylesheet paths under a base and after a parent router strips it', async () => {
  const { origin, href } = await fixture({ 'global #1.css': marker }, undefined, '/app/');
  const stylesheet = await href('global #1.css');

  expect(stylesheet).toBe('/app/global%20%231.css?stylex-css');
  expect(await (await fetch(origin + stylesheet)).text()).toContain('.generated{color:red}');
  // Connect parent routers pass a URL with the mount prefix already removed.
  expect(await (await fetch(origin + stylesheet.slice('/app'.length))).text()).toContain(
    '.generated{color:red}'
  );
});

test('passes requests without the stylesheet query or a known path to the next middleware', async () => {
  const { origin, href } = await fixture({ 'global.css': marker });
  await href('global.css');

  for (const pathname of ['/global.css', '/missing.css?stylex-css']) {
    const response = await fetch(origin + pathname);
    expect(response.status).toBe(418);
    expect(await response.text()).toBe('fixture fallthrough');
  }
});

test('forwards asynchronous rendering errors to Connect error middleware', async () => {
  const { origin, href } = await fixture({ 'global.css': marker }, async () => {
    throw new Error('Rule rendering failed');
  });
  const response = await fetch(origin + (await href('global.css')));

  expect(response.status).toBe(500);
  expect(await response.text()).toBe('Rule rendering failed');
});

test('rereads the stylesheet and current rules on every request without watcher invalidation', async () => {
  let rules = '.generated{color:red}';
  const { root, origin, href } = await fixture(
    { 'global.css': `.old{margin:0}\n${marker}` },
    async () => rules
  );
  const url = origin + (await href('global.css'));
  const original = await (await fetch(url)).text();
  expect(original).toContain('.old');
  expect(original).toContain('color:red');

  rules = '.generated{color:blue}';
  await writeFile(path.join(root, 'global.css'), `.new{margin:1px}\n${marker}`);
  const changed = await (await fetch(url)).text();
  expect(changed).toContain('.new');
  expect(changed).toContain('color:blue');
  expect(changed).not.toContain('.old');
  expect(changed).not.toContain('color:red');
});

test('rewatches served stylesheets on restart after their marker cache was invalidated', async () => {
  const { root, server, href } = await fixture({ 'global.css': marker });
  await href('global.css');
  server.watcher.emit('change', path.join(root, 'global.css'));

  await server.restart();

  await vi.waitFor(() => {
    expect(server.watcher.getWatched()[root]).toContain('global.css');
  });
});

test('keeps a shared import active until no served stylesheet imports it', async () => {
  const { root, server, origin, href } = await fixture({
    'first.css': `@import './shared.css';\n${marker}`,
    'second.css': `@import './shared.css';\n${marker}`,
    'shared.css': '.shared{color:blue}',
  });
  const first = origin + (await href('first.css'));
  const second = origin + (await href('second.css'));
  await (await fetch(first)).text();
  await (await fetch(second)).text();
  await writeFile(path.join(root, 'first.css'), marker);
  await (await fetch(first)).text();

  const send = vi.spyOn(server.ws, 'send');
  server.watcher.emit('change', path.join(root, 'shared.css'));
  expect(send).toHaveBeenCalledWith({ type: 'custom', event: 'stylex:css-refresh' });

  await writeFile(path.join(root, 'second.css'), marker);
  await (await fetch(second)).text();
  send.mockClear();
  server.watcher.emit('change', path.join(root, 'shared.css'));
  expect(send).not.toHaveBeenCalled();
});

test('refreshes when an active imported file is deleted and recreated', async () => {
  const { root, server, origin, href } = await fixture({
    'global.css': `@import './shared.css';\n${marker}`,
    'shared.css': '.shared{color:blue}',
  });
  await (await fetch(origin + (await href('global.css')))).text();

  const send = vi.spyOn(server.ws, 'send');
  for (const event of ['unlink', 'add']) {
    send.mockClear();
    server.watcher.emit(event, path.join(root, 'shared.css'));
    expect(send).toHaveBeenCalledWith({ type: 'custom', event: 'stylex:css-refresh' });
  }
});

test('renders concurrent requests independently without restoring stale import dependencies', async () => {
  const started = Promise.withResolvers<undefined>();
  const release = Promise.withResolvers<string>();
  let firstRequest = true;
  const { root, server, origin, href } = await fixture(
    {
      'global.css': `@import './old.css';\n${marker}`,
      'old.css': '.old{color:blue}',
      'new.css': '.new{color:green}',
    },
    async () => {
      if (!firstRequest) return '.generated{color:green}';
      firstRequest = false;
      started.resolve(undefined);
      return release.promise;
    }
  );
  const url = origin + (await href('global.css'));
  const earlier = fetch(url);

  try {
    await started.promise;
    await writeFile(path.join(root, 'global.css'), `@import './new.css';\n${marker}`);
    // The earlier request remains blocked; a fresh request must not share its render.
    const latest = await fetch(url, { signal: AbortSignal.timeout(2000) });
    const body = await latest.text();
    expect(body).toContain('.new');
    expect(body).toContain('.generated{color:green}');
  } finally {
    release.resolve('.generated{color:red}');
  }
  expect(await (await earlier).text()).toContain('.generated{color:red}');

  const send = vi.spyOn(server.ws, 'send');
  server.watcher.emit('change', path.join(root, 'old.css'));
  expect(send).not.toHaveBeenCalled();
  server.watcher.emit('change', path.join(root, 'new.css'));
  expect(send).toHaveBeenCalledWith({ type: 'custom', event: 'stylex:css-refresh' });
});

test('logs bundled mode and each newly served stylesheet once', async () => {
  const { info, href } = await fixture({ 'first.css': marker, 'second.css': marker });
  expect(info).toHaveBeenCalledWith(expect.stringContaining('bundled dev server'));
  info.mockClear();

  await href('first.css');
  await href('first.css');
  await href('second.css');

  expect(info).toHaveBeenCalledTimes(2);
  expect(info).toHaveBeenCalledWith(expect.stringContaining('serving /first.css'));
  expect(info).toHaveBeenCalledWith(expect.stringContaining('serving /second.css'));
});
