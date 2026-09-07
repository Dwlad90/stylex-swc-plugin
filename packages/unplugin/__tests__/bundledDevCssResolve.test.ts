import * as fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';

import { createServer, resolveConfig } from 'vite';
import { afterEach, expect, test, vi } from 'vitest';

import createBundledDevCss from '../src/utils/bundledDevCss';

vi.mock('node:fs/promises', { spy: true });

afterEach(() => vi.restoreAllMocks());

async function fixture() {
  const root = await fs.mkdtemp(path.join(os.tmpdir(), 'stylex-marker-resolve-'));
  const file = path.join(root, 'style.css');
  await fs.writeFile(file, '@stylex;');
  const server = await createServer({
    root,
    configFile: false,
    logLevel: 'silent',
    server: { middlewareMode: true, watch: null },
  });
  const helper = createBundledDevCss('@stylex;', async () => '');
  helper.configure(
    await resolveConfig(
      {
        root,
        configFile: false,
        logLevel: 'silent',
        experimental: { bundledDev: true },
      },
      'serve'
    )
  );
  helper.configureServer(server);
  return {
    file,
    server,
    resolve: () =>
      helper.resolveId.call(
        {
          environment: { config: { isBundled: true } },
          resolve: async () => ({ id: file }),
        },
        './style.css',
        path.join(root, 'main.js')
      ),
    async close() {
      await server.close();
      await fs.rm(root, { recursive: true, force: true });
    },
  };
}

test('shares marker detection across concurrent imports of one stylesheet', async () => {
  const app = await fixture();
  const read = vi.spyOn(fs, 'readFile');
  const watch = vi.spyOn(app.server.watcher, 'add');
  try {
    const results = await Promise.all([app.resolve(), app.resolve(), app.resolve()]);
    expect(results[0]).not.toBeNull();
    expect(results).toEqual([results[0], results[0], results[0]]);
    expect(read.mock.calls.filter(([file]) => file === app.file)).toHaveLength(1);
    expect(watch.mock.calls.filter(([file]) => file === app.file)).toHaveLength(1);
  } finally {
    await app.close();
  }
});

test.each(['change', 'unlink'])('does not cache a marker read invalidated by %s', async event => {
  const app = await fixture();
  const stale = Promise.withResolvers<string>();
  const read = vi.spyOn(fs, 'readFile').mockReturnValueOnce(stale.promise);
  try {
    const first = app.resolve();
    await vi.waitFor(() => expect(read).toHaveBeenCalledWith(app.file, 'utf8'));
    app.server.watcher.emit(event, app.file);
    await fs.writeFile(app.file, 'body { margin: 0; }');
    expect(await app.resolve()).toBeNull();
    stale.resolve('@stylex;');
    expect(await first).toBeNull();
    expect(await app.resolve()).toBeNull();
  } finally {
    stale.resolve('@stylex;');
    await app.close();
  }
});

test('retries marker detection after a failed read', async () => {
  const app = await fixture();
  vi.spyOn(fs, 'readFile').mockRejectedValueOnce(new Error('temporarily unavailable'));
  try {
    expect(await app.resolve()).toBeNull();
    expect(await app.resolve()).not.toBeNull();
  } finally {
    await app.close();
  }
});
