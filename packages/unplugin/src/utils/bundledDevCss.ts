import { readFile } from 'node:fs/promises';
import path from 'node:path';

import type { Connect, ResolvedConfig, ViteDevServer } from 'vite';

import { replaceFirstMarker } from './cssPlaceholder';

// The runtime id ends in `.js` so that no CSS pipeline, Vite's own included,
// claims the module by its extension.
const RUNTIME_ID_PREFIX = '\0stylex:stylesheet:';
const RUNTIME_ID_SUFFIX = '.js';
// The query that tells the served stylesheet apart from any other request
// for the same path, and the hot event that asks the browser to refetch it.
const STYLESHEET_QUERY = 'stylex-css';
const REFRESH_EVENT = 'stylex:css-refresh';

// A CSS module exports its class map; turning one into the runtime would
// leave that import undefined.
const CSS_MODULE_RE = /\.module\.css$/;

interface ResolveContext {
  /** Vite's per-environment context; absent on hosts without environments. */
  environment?: { config: { isBundled: boolean } };
  resolve: (
    id: string,
    importer?: string,
    options?: { skipSelf: boolean }
  ) => Promise<{ id: string } | null>;
}

interface ResolvedRuntime {
  id: string;
  moduleSideEffects: true;
}

export interface BundledDevCss {
  /** Whether the current Vite config is a bundled dev server with a marker. */
  readonly enabled: boolean;
  configure: (config: ResolvedConfig) => void;
  configureServer: (server: ViteDevServer) => void;
  resolveId: (
    this: ResolveContext,
    id: string,
    importer: string | undefined
  ) => Promise<ResolvedRuntime | null>;
  load: (id: string) => string | null;
  /** Asks every connected browser to refetch the stylesheet. */
  refresh: () => void;
}

// What Vite's `normalizePath` does: the watcher reports native paths, while
// resolver ids and the CSS pipeline's dependency list use forward slashes.
function normalizePath(file: string): string {
  return path.sep === '\\' ? file.replaceAll('\\', '/') : file;
}

// The browser side. Runs once per page: it adds one `<link>` where the
// stylesheet import sits in module order, so the cascade keeps the position
// the import gave it, and swaps that link for a fresh one on every refresh
// event. A swap only takes effect once the new sheet has loaded, and only if
// no newer swap started meanwhile, so a slow fetch cannot roll back a fast one.
function runtimeModule(href: string): string {
  return `
const href = ${JSON.stringify(href)};
const hot = import.meta.hot;
let link = hot?.data.link;
let sequence = 0;
if (!link) {
  link = document.createElement('link');
  link.rel = 'stylesheet';
  link.href = href;
  document.head.appendChild(link);
}
function refresh() {
  const current = ++sequence;
  const next = link.cloneNode();
  next.href = href + '&v=' + current + '-' + Date.now();
  next.onload = () => {
    if (current !== sequence) {
      next.remove();
      return;
    }
    link.remove();
    link = next;
  };
  next.onerror = () => next.remove();
  link.after(next);
}
// Without a hot context the link stays as a plain stylesheet.
if (hot) {
  hot.on(${JSON.stringify(REFRESH_EVENT)}, refresh);
  hot.accept();
  hot.dispose(() => {
    sequence += 1;
    hot.data.link = link;
    hot.off(${JSON.stringify(REFRESH_EVENT)}, refresh);
  });
  hot.prune(() => link.remove());
}
`;
}

/**
 * Serves the placeholder stylesheet under Vite's bundled dev server, where
 * imported CSS is compiled into JavaScript and never requested on its own.
 * The import of a stylesheet that carries the marker resolves to a runtime
 * module that links the stylesheet from the dev server instead, a middleware
 * renders it from the file and the rules on every request, and one hot event
 * asks the browser to refetch it. The README's `useCssPlaceholder` section
 * has the rationale and the limits.
 *
 * Only the bundled client environment is redirected; any other environment
 * that imports the stylesheet keeps the regular path. `renderRules` produces
 * the transformed rules to put at the marker; Vite's CSS pipeline then runs
 * over the whole stylesheet, without a url resolver, so `url()` references
 * are left as written.
 */
export default function createBundledDevCss(
  marker: string | false,
  renderRules: (file: string) => Promise<string>
): BundledDevCss {
  let config: ResolvedConfig | null = null;
  let server: ViteDevServer | null = null;
  let enabled = false;

  // Whether a stylesheet carries the marker, by absolute path. Both answers
  // are kept and an edit drops the entry, so a file is read once per version.
  const carriesMarker = new Map<string, boolean>();
  // Served marker files by absolute path, with the href each is served under,
  // and the reverse lookup the middleware needs.
  const hrefByFile = new Map<string, string>();
  const fileByPathname = new Map<string, string>();
  // Files the stylesheets `@import`, as Vite's CSS pipeline reports them.
  const imported = new Set<string>();

  function refresh(): void {
    server?.ws.send({ type: 'custom', event: REFRESH_EVENT });
  }

  // The dev server watcher does not cover the project root under bundled
  // serve, Rolldown watches the graph instead, so every file whose contents
  // matter here is added by hand. Rolldown never sees them: the stylesheet
  // is not in its graph, which is the point.
  function watchImported(file: string): void {
    if (imported.has(file)) return;
    imported.add(file);
    server?.watcher.add(file);
  }

  // A deleted file loses what was learned from its contents. Its href stays:
  // the bundle keeps linking it, and a file written back under the same
  // path is served again as if nothing happened.
  function forget(file: string): void {
    carriesMarker.delete(file);
    imported.delete(file);
  }

  function hrefFor(root: string, base: string, file: string): string {
    const relative = normalizePath(path.relative(root, file));
    // Outside the root the conventional `/@fs/` form is kept as is, so the
    // href reads like any other Vite URL for the same file.
    const served = relative.startsWith('../') ? `@fs/${normalizePath(file)}` : relative;
    // Per segment, so `#` and `?` in a file name cannot cut the href short;
    // the `@fs/` prefix itself is kept literal.
    return (
      base +
      served
        .split('/')
        .map((segment, index) =>
          index === 0 && segment === '@fs' ? segment : encodeURIComponent(segment)
        )
        .join('/')
    );
  }

  function serveStylesheet(
    req: Parameters<Connect.NextHandleFunction>[0],
    res: Parameters<Connect.NextHandleFunction>[1],
    next: Connect.NextFunction
  ): void {
    // The base is still on the URL here: this runs ahead of Vite's own base
    // middleware, and the hrefs were built with the base for that reason.
    const url = new URL(req.url ?? '/', 'http://localhost');
    // A parent router in middleware mode strips its mount path before this
    // runs, so an href is also matched by its tail.
    const file =
      fileByPathname.get(url.pathname) ??
      [...fileByPathname].find(([href]) => href.endsWith(url.pathname))?.[1];
    if (!file || !config || !marker || !url.searchParams.has(STYLESHEET_QUERY)) {
      next();
      return;
    }

    const resolvedConfig = config;
    const resolvedMarker = marker;

    // Connect does not forward async rejections, hence the explicit catch.
    void (async () => {
      // Vite is imported on demand: this module is bundled into the chunk
      // every host entry shares, and a static import would make webpack,
      // Rspack and esbuild users load Vite (ESM-only since Vite 8) at plugin
      // load. The module loader caches the namespace after the first call.
      const [{ preprocessCSS, send }, source, rules] = await Promise.all([
        import('vite'),
        readFile(file, 'utf8'),
        renderRules(file),
      ]);
      const stylesheet = replaceFirstMarker(source, resolvedMarker, rules);
      const result = await preprocessCSS(stylesheet, file, resolvedConfig);
      for (const dependency of result.deps ?? []) watchImported(normalizePath(dependency));

      // Vite's own responder: content type, ETag with `304` on a match, and
      // `HEAD` handling.
      send(req, res, result.code, 'css', { cacheControl: 'no-store' });
      // `next` is Connect's error handler here, not a completion callback:
      // forwarding the rejection to it is the documented middleware contract.
      // oxlint-disable-next-line promise/no-callback-in-promise
    })().catch(next);
  }

  return {
    get enabled() {
      return enabled;
    },

    configure(resolved) {
      config = resolved;
      // The client environment's own flag rather than the experimental
      // option that seeds it: `environments.client.isBundled` can turn
      // bundled serve on or off independently. Optional all the way down,
      // since `environments` only exists from Vite 6.
      enabled =
        !!marker &&
        resolved.command === 'serve' &&
        resolved.environments?.client?.isBundled === true;
    },

    configureServer(devServer) {
      if (!enabled) return;
      server = devServer;

      // A restarted server brings a fresh watcher, so everything learned on
      // the previous one is registered again.
      const known = [...carriesMarker.keys(), ...imported];
      if (known.length > 0) devServer.watcher.add(known);

      devServer.watcher.on('change', changed => {
        const file = normalizePath(changed);
        // An edit is the one thing that can change whether a file carries
        // the marker, so the cached answer goes; a served file is refetched.
        carriesMarker.delete(file);
        if (hrefByFile.has(file) || imported.has(file)) refresh();
      });
      devServer.watcher.on('unlink', changed => forget(normalizePath(changed)));
      devServer.watcher.on('add', added => {
        if (hrefByFile.has(normalizePath(added))) refresh();
      });
      devServer.middlewares.use(serveStylesheet);
    },

    // Redirects the import of a stylesheet that carries the marker to the
    // runtime module. Stylesheets reached through `@import` stay with Vite:
    // they end up inlined into the importing stylesheet either way. So does a
    // stylesheet used as an entry, which has no module to link it from.
    async resolveId(id, importer) {
      if (
        !enabled ||
        !marker ||
        !config ||
        !importer ||
        !id.endsWith('.css') ||
        CSS_MODULE_RE.test(id) ||
        importer.endsWith('.css') ||
        !this.environment?.config.isBundled
      ) {
        return null;
      }

      const resolved = await this.resolve(id, importer, { skipSelf: true });
      if (!resolved || !path.isAbsolute(resolved.id)) return null;

      const file = normalizePath(resolved.id);
      let holdsMarker = carriesMarker.get(file);
      if (holdsMarker === undefined) {
        try {
          holdsMarker = (await readFile(file, 'utf8')).includes(marker);
        } catch {
          return null;
        }
        carriesMarker.set(file, holdsMarker);
        // Watched either way: only an edit can change the answer.
        server?.watcher.add(file);
      }
      if (!holdsMarker) return null;

      if (!hrefByFile.has(file)) {
        const href = hrefFor(config.root, config.base, file);
        hrefByFile.set(file, href);
        fileByPathname.set(href, file);
      }

      return { id: RUNTIME_ID_PREFIX + file + RUNTIME_ID_SUFFIX, moduleSideEffects: true };
    },

    load(id) {
      if (!id.startsWith(RUNTIME_ID_PREFIX) || !id.endsWith(RUNTIME_ID_SUFFIX)) return null;

      const file = id.slice(RUNTIME_ID_PREFIX.length, -RUNTIME_ID_SUFFIX.length);
      const href = hrefByFile.get(file);
      if (!href) return null;

      return runtimeModule(`${href}?${STYLESHEET_QUERY}`);
    },

    refresh,
  };
}
