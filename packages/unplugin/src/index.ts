import crypto from 'node:crypto';
import { promises } from 'node:fs';
import type { IncomingMessage, ServerResponse } from 'node:http';
import * as path from 'node:path';

import { shouldTransformFile, transform as stylexTransform } from '@stylexswc/rs-compiler';
import type { StyleXMetadata, TransformedOptions } from '@stylexswc/rs-compiler';
import type { OnEndResult } from 'esbuild';
import { createUnplugin } from 'unplugin';
import type { UnpluginFactory, UnpluginInstance } from 'unplugin';
import type { Connect } from 'vite';
import type { HotPayload, ModuleNode, ViteDevServer } from 'vite';

import type { UnpluginStylexRSOptions } from './types';
import injectIntoCssTargets, {
  BUILD_CSS_PLACEHOLDER,
  replaceFirstMarker,
  toBuildPlaceholder,
} from './utils/cssPlaceholder';
import type { CssInjectionTarget, FinalizeCss } from './utils/cssPlaceholder';
import generateHash from './utils/generateHash';
import getStyleXRules from './utils/getStyleXRules';
import hasUnresolvedDefineConstAtRule from './utils/hasUnresolvedDefineConstAtRule';
import normalizeOptions, { identityTransformCss } from './utils/normalizeOptions';
import resolveStylesheetHref from './utils/resolveStylesheetHref';

type StyleXRules = Record<string, StyleXMetadata['stylex']>;

function hasValidExtension(filePath: string, pageExtensions: string[]): boolean {
  const extensionName = path.extname(filePath);
  const questionSignIndex = extensionName.indexOf('?');

  let cleanedExtensionName =
    questionSignIndex > -1 ? extensionName.slice(0, questionSignIndex) : extensionName;

  if (cleanedExtensionName.startsWith('.')) {
    cleanedExtensionName = cleanedExtensionName.slice(1);
  }

  return pageExtensions.includes(cleanedExtensionName);
}

// Use the normalized options type from utils
import type { NormalizedOptions as NormalizedOptionsType } from './utils/normalizeOptions';
type NormalizedOptions = NormalizedOptionsType;

function shouldTransformStyleXFile(id: string, normalizedOptions: NormalizedOptions): boolean {
  return (
    hasValidExtension(id, normalizedOptions.pageExtensions) &&
    shouldTransformFile(
      id,
      normalizedOptions.rsOptions.include,
      normalizedOptions.rsOptions.exclude
    )
  );
}

const { writeFile, mkdir, readFile, readdir } = promises;

const PLUGIN_NAME = 'unplugin-stylex-rs';

/**
 * How many times in a row a dev CSS refresh may fail before the plugin stops
 * retrying it. Enough to ride out a reconnecting websocket, small enough that a
 * socket which never recovers stops logging.
 */
const MAX_CSS_REFRESH_FAILURES = 3;

/**
 * Whether the host minifies CSS, taken from Vite's `build.cssMinify`. Undefined
 * outside Vite, which is what keeps plain Rollup unchanged.
 */
type CssMinifier = boolean | string | undefined;

/**
 * Minifies the rules the injection splices in.
 *
 * The host's CSS plugin has already minified the stylesheet by the time the
 * bundle hook runs, so rules added there would otherwise be the only unminified
 * CSS in the output. PostCSS cannot be recovered this late -- it runs per
 * module, long before the full rule set is known -- which is why the placeholder
 * docs promise the pipeline for the stylesheet, not for the rules.
 *
 * Always esbuild, even where the host chose Lightning CSS: this only has to
 * shrink the rules, both minifiers leave StyleX's atomic longhand declarations
 * alone, and esbuild is already here for every host worth minifying for.
 */
async function minifyInjectedCss(css: string, minifier: CssMinifier): Promise<string> {
  if (!minifier || !css.trim()) return css;

  try {
    const { transform } = await import('esbuild');

    return (await transform(css, { loader: 'css', minify: true })).code;
  } catch (error) {
    // Shipping the rules unminified beats failing a build over the minifier.
    console.warn('StyleX: could not minify the injected placeholder CSS', error);

    return css;
  }
}

/**
 * Placeholder mode deliberately skips HTML injection, so a stylesheet emitted
 * on its own would never be linked and the styles would simply be missing at
 * runtime. Saying so is the only honest outcome.
 */
const MISSING_INJECTION_TARGET_ERROR =
  'StyleX: no CSS asset was available to receive the placeholder styles. ' +
  'Make sure the stylesheet holding the marker is imported by the module graph. ' +
  "Set `onMissingCssPlaceholder` to 'warn' or 'ignore' if that is expected.";

/**
 * Only the Vite adapter learns from its load hook that the marker really is
 * part of the build. Everywhere else a missing target is indistinguishable from
 * a build that legitimately has no CSS, so it is reported rather than fatal --
 * silence would leave the styles missing with nothing to explain it.
 */
const MISSING_INJECTION_TARGET_WARNING =
  'StyleX: no CSS asset contained the placeholder marker, so no styles were ' +
  'injected. The stylesheet holding the marker may be missing from the build. ' +
  "Set `onMissingCssPlaceholder` to 'ignore' if that is expected.";

function replaceFileName(original: string, css: string) {
  if (!original.includes('[hash]')) {
    return original;
  }
  const hash = crypto.createHash('sha256').update(css).digest('hex').slice(0, 8);
  return original.replace(/\[hash\]/g, hash);
}

/**
 * Helper function to invalidate and collect CSS modules containing the placeholder.
 * Used to avoid code duplication in HMR handling.
 * @param server - Vite dev server instance
 * @param placeholder - CSS placeholder string to search for
 * @returns Array of CSS modules that contain the placeholder
 */
async function invalidateAndCollectCssModules(
  server: ViteDevServer,
  placeholder: NormalizedOptions['useCssPlaceholder'],
  carriesMarker: Map<string, boolean>
): Promise<ModuleNode[]> {
  const cssModules: ModuleNode[] = [];

  // Skip if placeholder is not a string
  if (!placeholder || typeof placeholder !== 'string') {
    return cssModules;
  }

  // `mod.id` is `string | null`, so the guard both filters non-CSS modules and
  // narrows the type for the read below.
  const allCssModules = Array.from(server.moduleGraph.urlToModuleMap.values()).filter(
    mod => mod.id?.endsWith('.css') ?? false
  );

  // Check each CSS module for the placeholder
  // Note: We must read the original source file, not the transformed result,
  // because the transformed result already has the placeholder replaced
  await Promise.all(
    allCssModules.map(async mod => {
      try {
        // Skip modules without a valid id
        if (!mod.id) return;

        // Whether a stylesheet holds the marker only changes when the file
        // does, and the watcher already reports that, so this is read once
        // rather than on every refresh.
        let holdsMarker = carriesMarker.get(mod.id);

        if (holdsMarker === undefined) {
          holdsMarker = (await promises.readFile(mod.id, 'utf8')).includes(placeholder);
          carriesMarker.set(mod.id, holdsMarker);
        }

        if (holdsMarker) {
          server.moduleGraph.invalidateModule(mod);
          cssModules.push(mod);
        }
      } catch (e) {
        // Log read errors for debugging HMR issues
        console.debug(`[stylex-unplugin] Failed to read CSS file "${mod.id}":`, e);
      }
    })
  );

  return cssModules;
}

async function transformStyleXDependencies(
  server: ViteDevServer,
  stylexRules: StyleXRules,
  normalizedOptions: NormalizedOptions
): Promise<void> {
  const visited = new Set<string>();

  const transformDependencies = async (module: ModuleNode): Promise<void> => {
    await Promise.all(
      Array.from(module.importedModules, async importedModule => {
        const moduleId = importedModule.id ?? importedModule.url;

        if (visited.has(moduleId)) return;
        visited.add(moduleId);

        if (!shouldTransformStyleXFile(moduleId, normalizedOptions)) return;

        await server.transformRequest(importedModule.url);
        await transformDependencies(importedModule);
      })
    );
  };

  await Promise.all(
    Object.keys(stylexRules).map(async id => {
      const module = server.moduleGraph.getModuleById(id);

      if (!module) return;
      visited.add(id);
      await transformDependencies(module);
    })
  );
}

/**
 * Injects StyleX CSS into CSS assets for webpack/rspack bundlers.
 * Shared logic to avoid code duplication between webpack and rspack hooks.
 *
 * `TSource` is the bundler's asset-source type. It is a type parameter rather
 * than a concrete type because webpack and rspack each declare their own
 * incompatible `Source`, and this function is called once per bundler; naming
 * either one here would force a cast at the other call site.
 *
 * It replaces the `any` parameters it started with, and is stricter than they
 * were: the source produced by `createRawSource` must be the same type
 * `updateAsset` accepts. Under `any`, handing a webpack `RawSource` to rspack's
 * `updateAsset` type-checked cleanly and would only have failed at runtime.
 */
async function injectStyleXCss<TSource>(
  assets: Record<string, { source(): { toString(): string } }>,
  injectMarker: string,
  collectedCSS: string | null,
  normalizedOptions: NormalizedOptions,
  updateAsset: (fileName: string, source: TSource) => void,
  createRawSource: (content: string) => TSource,
  reportMissingTarget: (message: string) => void
): Promise<void> {
  const targets = Object.keys(assets)
    .filter(fileName => fileName.endsWith('.css'))
    .flatMap<CssInjectionTarget>(fileName => {
      const asset = assets[fileName];

      if (!asset) return [];

      return [
        {
          name: fileName,
          read: () => asset.source().toString(),
          write: source => updateAsset(fileName, createRawSource(source)),
        },
      ];
    });

  const handled = await injectIntoCssTargets(targets, [injectMarker], collectedCSS, (css, name) =>
    transformStyleXCSS(css, name, normalizedOptions)
  );

  // An asset emitted here could not be linked, and unlike the Vite adapter this
  // one has no signal that the marker was ever part of the build, so the styles
  // going missing is reported rather than fatal.
  if (!handled && normalizedOptions.onMissingCssPlaceholder !== 'ignore') {
    reportMissingTarget(MISSING_INJECTION_TARGET_WARNING);
  }
}

/**
 * Shape of a Rollup-style output bundle, described structurally rather than
 * imported: Vite and Rollup do not share a plugin type, and the injection only
 * ever touches these members.
 */
type BundleAssetLike = { type: string; fileName: string; source: string | Uint8Array };
type BundleOutputLike = { type: string; fileName: string; source?: string | Uint8Array };
type OutputBundleLike = Record<string, BundleOutputLike>;
type PlaceholderBundleContext = {
  error(message: string): never;
  warn(message: string): void;
};

/**
 * Replaces the CSS placeholder marker in a Rollup-style bundle with the
 * collected StyleX rules. Shared by the Vite and Rollup adapters; webpack and
 * rspack have their own asset pipeline and use `injectStyleXCss` instead.
 */
async function injectPlaceholderIntoBundle(
  context: PlaceholderBundleContext,
  bundle: OutputBundleLike,
  stylexRules: StyleXRules,
  normalizedOptions: NormalizedOptions,
  transformedOptions: TransformedOptions,
  canProveMarkerWasBuilt: boolean,
  finalizeCss: FinalizeCss
): Promise<void> {
  if (!normalizedOptions.useCssPlaceholder) return;

  const collectedCSS = getStyleXRules(stylexRules, transformedOptions);

  // The source is tested, not assumed: the narrowing hands back a type where it
  // is required, so a host reporting a stylesheet without one would otherwise
  // fail on the read below rather than here.
  const targets = Object.values(bundle)
    .filter(
      (output): output is BundleAssetLike =>
        output.type === 'asset' && output.fileName.endsWith('.css') && output.source != null
    )
    .map<CssInjectionTarget>(asset => ({
      name: asset.fileName,
      read: () => asset.source.toString(),
      write: source => {
        asset.source = source;
      },
    }));

  // The build placeholder is what the load hook leaves behind; the raw marker
  // still turns up when the stylesheet reached the bundle without passing
  // through that hook, as it does under plain Rollup.
  const markers = [BUILD_CSS_PLACEHOLDER, normalizedOptions.useCssPlaceholder];

  const handled = await injectIntoCssTargets(targets, markers, collectedCSS, finalizeCss);

  // Emitting a standalone stylesheet here used to look like a safety net, but
  // placeholder mode never links it, so it only ever hid missing styles.
  if (handled || normalizedOptions.onMissingCssPlaceholder === 'ignore') return;

  // Failing is only fair where the marker is known to have been in the build:
  // elsewhere a missing stylesheet looks the same as a build with no CSS.
  if (normalizedOptions.onMissingCssPlaceholder === 'error' && canProveMarkerWasBuilt) {
    context.error(MISSING_INJECTION_TARGET_ERROR);
  }

  context.warn(MISSING_INJECTION_TARGET_WARNING);
}

export const unpluginFactory: UnpluginFactory<UnpluginStylexRSOptions | undefined> = (
  options = {},
  meta
) => {
  const normalizedOptions = normalizeOptions(options);

  if (normalizedOptions.useCssPlaceholder && meta.framework === 'farm') {
    // Farm maps a fixed hook list and never receives `generateBundle`, so the
    // marker would simply stay in the stylesheet.
    console.warn(
      'StyleX: `useCssPlaceholder` is not supported under Farm yet; the marker will not be replaced.'
    );
  }

  const transformedOptions: TransformedOptions = {
    useLayers: normalizedOptions.useLayers,
    enableLTRRTLComments: normalizedOptions.enableLTRRTLComments,
    legacyDisableLayers: normalizedOptions.legacyDisableLayers,
  };

  // Mutable state for each compilation - reset in buildStart
  const stylexRules: StyleXRules = {};

  // Not one config snapshot: each is read from the hook that reports the value
  // we actually want, which is a different hook for each. Both stay undefined
  // for hosts other than Vite.
  let viteUserAssetsDir: string | undefined;
  let viteResolvedBase: string | undefined;
  let viteCssMinify: CssMinifier;

  let hasCssToExtract = false;
  let cssFileName: string | null = null;

  let wsSend: undefined | ((payload: HotPayload) => void) = undefined;

  // Scoped to this plugin instance so multiple compilers/builds sharing the
  // same process (e.g. Next.js client/server, or several Vite dev servers)
  // don't clobber each other's dev-server reference or invalidation flag.
  let viteDevServer: ViteDevServer | null = null;
  // Counts the transforms that contributed StyleX rules. A refresh is owed
  // whenever this moves past the revision the last one covered, which is what
  // makes late modules -- anything behind a dynamic import -- reach the browser.
  let rulesRevision = 0;
  let refreshedRulesRevision = 0;
  let cssRefreshTimer: ReturnType<typeof setTimeout> | null = null;
  // A refresh whose update never reached the browser leaves the stylesheet
  // stale, and no further transform is guaranteed to come along and re-arm it,
  // so the failure has to retry itself. Counted so a permanently broken socket
  // cannot turn into an endless retry loop.
  let cssRefreshFailures = 0;
  // Which stylesheets hold the marker, keyed by module id. Cleared per file by
  // the watcher below, since only an edit can change the answer.
  const cssMarkerCache = new Map<string, boolean>();

  // Debounced so a burst of transforms costs one refresh, and re-armable so the
  // next burst gets its own. `viteDevServer` is re-read inside the callback
  // rather than captured: it is mutable and the server can be torn down during
  // the wait.
  function scheduleCssRefresh(): void {
    if (cssRefreshTimer || rulesRevision === refreshedRulesRevision) return;

    cssRefreshTimer = setTimeout(() => {
      cssRefreshTimer = null;

      const server = viteDevServer;
      if (!server) return;

      const coveredRevision = rulesRevision;

      // `setTimeout` expects a void-returning callback, so the async work is
      // wrapped rather than handed to it directly. The `catch` is the part that
      // matters: nothing awaits this, so without it a rejection would reach
      // `unhandledRejection` and take the dev server down over a failed CSS
      // refresh. `void` alone would silence the lint without handling anything.
      void (async () => {
        // Find all CSS modules that actually contain the placeholder
        const cssModules = await invalidateAndCollectCssModules(
          server,
          normalizedOptions.useCssPlaceholder,
          cssMarkerCache
        );

        // Send update to trigger HMR
        if (cssModules.length > 0) {
          server.ws.send({
            type: 'update',
            updates: cssModules.map(mod => ({
              type: 'css-update' as const,
              acceptedPath: mod.url,
              path: mod.url,
              timestamp: Date.now(),
            })),
          });
        }

        // Only now is the revision genuinely covered. Recording it before the
        // send would call a refresh done that the browser never heard about.
        refreshedRulesRevision = coveredRevision;
        cssRefreshFailures = 0;

        // Rules that arrived while this refresh was in flight need their own.
        if (viteDevServer === server) scheduleCssRefresh();
      })().catch((error: unknown) => {
        console.error('StyleX: failed to refresh placeholder CSS modules', error);

        // The revision is untouched, so the refresh is still owed. Re-arm it
        // here because nothing else will, and stop once the budget is spent so
        // a socket that never recovers cannot spin.
        cssRefreshFailures += 1;

        if (cssRefreshFailures < MAX_CSS_REFRESH_FAILURES && viteDevServer === server) {
          scheduleCssRefresh();
        }
      });
    }, 50);
  }

  // Only the Vite load hook can tell that the marker really is part of this
  // build, which is what separates a broken configuration from a build that
  // legitimately produces no CSS, such as an SSR bundle.
  let placeholderSeen = false;
  let viteIsSsrBuild = false;

  // One hook object, registered by both Rollup-style hosts below.
  //
  // `post` is load-bearing: in the default order the hook runs before Vite's
  // own CSS plugin emits the combined stylesheet, so a build with
  // `cssCodeSplit: false` saw no CSS asset to inject into and fell through to a
  // standalone file that nothing links.
  // The rules are spliced in after the host's CSS plugin has already minified
  // the stylesheet, so they arrive unminified unless they are minified here.
  // `viteCssMinify` stays unset outside Vite, which leaves plain Rollup, with no
  // CSS pipeline of its own, exactly as it was.
  const finalizePlaceholderCss: FinalizeCss = async (css, targetName) =>
    minifyInjectedCss(await transformStyleXCSS(css, targetName, normalizedOptions), viteCssMinify);

  const placeholderGenerateBundle = {
    order: 'post' as const,
    async handler(this: PlaceholderBundleContext, _options: unknown, bundle: OutputBundleLike) {
      await injectPlaceholderIntoBundle(
        this,
        bundle,
        stylexRules,
        normalizedOptions,
        transformedOptions,
        placeholderSeen && !viteIsSsrBuild,
        finalizePlaceholderCss
      );
    },
  };

  return {
    name: PLUGIN_NAME,

    buildStart() {
      // stylexRules accumulates during watch mode for proper HMR
      hasCssToExtract = false;
      placeholderSeen = false;
    },

    transformInclude(id) {
      return shouldTransformStyleXFile(id, normalizedOptions);
    },

    async transform(inputCode, id) {
      if (!hasStyleXCode(normalizedOptions, inputCode)) {
        return null;
      }

      const dir = path.dirname(id);
      const basename = path.basename(id);
      const file = path.join(dir, basename.split('?')[0] || basename);

      try {
        // Only Rollup-compatible hosts (Rollup, Vite) expose the combined
        // source map of previous plugins; other bundlers fall back to
        // locating positions in the source text. The compiler only reads
        // the input map for debug source-map annotations (`debug` +
        // `enableDebugDataProp`) — plain map chaining is handled by the
        // host — and fetching it unconditionally is expensive: with no
        // previous maps Rollup synthesizes a hi-res map of the whole
        // module, which then gets stringified, re-parsed and cloned per
        // module.
        let inputSourceMap: string | undefined;

        const needsInputSourceMap =
          (normalizedOptions.rsOptions.debug ?? normalizedOptions.rsOptions.dev) === true &&
          normalizedOptions.rsOptions.enableDebugDataProp !== false;

        if (needsInputSourceMap && hasCombinedSourcemap(this)) {
          try {
            const combinedMap = this.getCombinedSourcemap();

            if (combinedMap?.mappings) {
              inputSourceMap = JSON.stringify(combinedMap);
            }
          } catch {
            // No usable source map for this module.
          }
        }

        const { code, map } = transformStyleXCode(
          file,
          inputCode,
          normalizedOptions,
          stylexRules,
          id,
          inputSourceMap
        );

        // Refresh the placeholder CSS in dev whenever a module actually
        // contributed rules. Comparing the code is what tells the two apart:
        // an untouched module adds nothing to refresh for. HMR for later edits
        // is still handled by handleHotUpdate.
        if (normalizedOptions.useCssPlaceholder && viteDevServer && code !== inputCode) {
          // Bumped synchronously, before any await, so concurrent transforms
          // cannot lose each other's contribution.
          rulesRevision += 1;
          // New rules deserve a fresh set of attempts, so the retry budget
          // counts only the failures that happened with no new work in between.
          cssRefreshFailures = 0;
          scheduleCssRefresh();
        }

        if (typeof wsSend === 'function' && cssFileName) {
          wsSend({
            type: 'update',
            updates: [
              {
                acceptedPath: cssFileName,
                path: cssFileName,
                timestamp: Date.now(),
                type: 'css-update',
              },
            ],
          });
        }

        return {
          code,
          map,
          stylexRules,
        };
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        const enhancedMessage = `StyleX transformation error in ${file}:\n  ${message}`;
        console.error(enhancedMessage, error);
        this.error(new Error(enhancedMessage, { cause: error }));
        // `this.error` throws, but its signature is not `never`, so the
        // function is otherwise seen as falling through without a value.
        return null;
      }
    },

    async buildEnd() {
      const framework = this.getNativeBuildContext?.().framework;
      if (framework === 'esbuild') {
        // will handle the CSS generation in the plugin itself
        return;
      }

      // Skip emitting separate CSS file when using useCssPlaceholder
      // The CSS will be injected into the specified CSS file via framework-specific hooks
      if (normalizedOptions.useCssPlaceholder) {
        return;
      }

      const { processedFileName, collectedCSS } = await generateCSSAssets(
        stylexRules,
        normalizedOptions,
        transformedOptions
      );

      if (!collectedCSS) return;

      hasCssToExtract = true;

      if (processedFileName) {
        this.emitFile({
          fileName: processedFileName,
          source: collectedCSS,
          type: 'asset',
        });
      }
    },

    // Rollup needs its own registration: the hook used to live under `vite`
    // only, which left plain Rollup builds with no StyleX CSS at all. Webpack
    // and Rspack map a fixed hook list and ignore this one, so their own
    // `processAssets` injection cannot run twice.
    rollup: {
      generateBundle: placeholderGenerateBundle,
    },

    vite: {
      config(config) {
        // Deliberately the *user* `assetsDir`: leaving it unset keeps the
        // stylesheet at the output root, whereas the resolved config always
        // reports Vite's `assets` default and would relocate it. The cost is
        // that an `assetsDir` a later plugin's `config` hook returns is merged
        // in after this runs, so it is invisible here.
        viteUserAssetsDir = config.build?.assetsDir;
      },

      configResolved(config) {
        // An SSR bundle emits no stylesheet of its own, so a missing injection
        // target there is expected rather than a misconfiguration.
        viteIsSsrBuild = !!config.build.ssr;

        // The injection runs after Vite has minified the stylesheet, so it has
        // to apply the same minifier to the rules it splices in.
        viteCssMinify = config.build.cssMinify;

        // `base`, unlike `assetsDir`, is wanted in its resolved form: the user
        // value may be unset or missing the trailing slash Vite normalizes in.
        viteResolvedBase = config.base;

        config.optimizeDeps.exclude = config.optimizeDeps.exclude || [];
        config.optimizeDeps.exclude.push('@stylexjs/open-props');
      },

      // Load CSS files to replace placeholder before Vite's CSS processing
      async load(id) {
        // Only handle CSS files with useCssPlaceholder
        if (!normalizedOptions.useCssPlaceholder) return null;
        if (!id.endsWith('.css')) return null;

        // Read the CSS file
        let cssContent: string;
        try {
          cssContent = await promises.readFile(id, 'utf-8');
        } catch {
          return null;
        }

        // Check if it contains the placeholder
        if (!cssContent.includes(normalizedOptions.useCssPlaceholder)) return null;

        // In a build the rule set is still incomplete here: modules reached
        // through a dynamic import are transformed long after this stylesheet
        // is loaded. Leave the build placeholder for generateBundle to fill with
        // the final rules, so they are injected once and at the marker's position.
        // The dev server has no bundle step and keeps inlining what it has.
        if (!viteDevServer) {
          placeholderSeen = true;

          // Every occurrence, so a stray second marker cannot survive into the
          // output: generateBundle fills the first and removes the rest.
          return toBuildPlaceholder(cssContent, normalizedOptions.useCssPlaceholder);
        }

        // Get collected StyleX CSS
        let collectedCSS = getStyleXRules(stylexRules, transformedOptions);

        if (collectedCSS && hasUnresolvedDefineConstAtRule(collectedCSS)) {
          // Static imports can be registered but not transformed when Vite's request
          // pre-transform is disabled, leaving defineConsts metadata unavailable.
          await transformStyleXDependencies(viteDevServer, stylexRules, normalizedOptions);
          collectedCSS = getStyleXRules(stylexRules, transformedOptions);
        }

        // Determine replacement CSS based on whether usable CSS exists yet
        let replacementCSS: string;
        if (!collectedCSS?.trim() || hasUnresolvedDefineConstAtRule(collectedCSS)) {
          replacementCSS = '/* StyleX styles will load after transformation */';
        } else {
          replacementCSS = await transformStyleXCSS(collectedCSS, id, normalizedOptions);
        }

        return replaceFirstMarker(cssContent, normalizedOptions.useCssPlaceholder, replacementCSS);
      },

      generateBundle: placeholderGenerateBundle,

      async buildEnd() {
        // Skip emitting CSS file when using useCssPlaceholder
        // CSS will be injected into the specified file via generateBundle
        if (normalizedOptions.useCssPlaceholder) {
          return;
        }

        // Skip emitting files in dev/serve mode
        const isDev = this.meta?.watchMode;
        if (isDev) {
          return;
        }

        const { processedFileName, collectedCSS } = await generateCSSAssets(
          stylexRules,
          normalizedOptions,
          transformedOptions,
          viteUserAssetsDir
        );

        if (!collectedCSS) return;

        if (processedFileName) {
          this.emitFile({
            fileName: processedFileName,
            source: collectedCSS,
            type: 'asset',
          });
        }
      },
      configureServer(server) {
        if (cssRefreshTimer) {
          clearTimeout(cssRefreshTimer);
          cssRefreshTimer = null;
        }

        viteDevServer = server;
        refreshedRulesRevision = rulesRevision;
        cssRefreshFailures = 0;
        cssMarkerCache.clear();

        // Editing a stylesheet is the only thing that can add or remove its
        // marker, so the cached answer is dropped for exactly that file.
        const forgetMarker = (file: string) => cssMarkerCache.delete(file);

        server.watcher.on('change', forgetMarker);
        server.watcher.on('unlink', forgetMarker);
        server.watcher.on('add', forgetMarker);

        server.watcher.once('close', () => {
          if (viteDevServer !== server) return;

          viteDevServer = null;
          cssMarkerCache.clear();
          if (cssRefreshTimer) {
            clearTimeout(cssRefreshTimer);
            cssRefreshTimer = null;
          }
        });

        server.middlewares.use(
          (req: IncomingMessage, res: ServerResponse, next: Connect.NextFunction) => {
            const requestedCssFileName = cssFileName;
            // A substring match on purpose: `cssFileName` is base-less, while
            // this middleware runs ahead of Vite's base middleware, so a
            // non-root base is still attached to `req.url` here. The HMR
            // timestamp query is likewise only tolerated by matching loosely.
            if (!requestedCssFileName || !req.url?.includes(requestedCssFileName)) {
              next();
              return;
            }

            // Connect does not forward async rejections, hence the explicit catch
            void (async () => {
              const collectedCSS = getStyleXRules(stylexRules, transformedOptions);
              const finalCSS = collectedCSS
                ? await transformStyleXCSS(collectedCSS, requestedCssFileName, normalizedOptions)
                : collectedCSS;

              res.setHeader('Content-Type', 'text/css');
              res.end(finalCSS);
              // `next` is Connect's error handler, not a completion callback.
              // Forwarding the rejection to it is the documented middleware
              // contract, which is what this rule is written to discourage.
              // oxlint-disable-next-line promise/no-callback-in-promise
            })().catch(next);
          }
        );
      },
      async handleHotUpdate({ file: id, file, server, read, modules }) {
        // For Vue files, include CSS module but don't transform
        // (raw .vue files have <template>, <style> sections that SWC can't parse)
        // The transform hook will update stylexRules when Vue plugin converts it to JS
        if (file.endsWith('.vue')) {
          if (normalizedOptions.useCssPlaceholder) {
            // Find CSS modules that contain the placeholder
            const cssModules = await invalidateAndCollectCssModules(
              server,
              normalizedOptions.useCssPlaceholder,
              cssMarkerCache
            );

            if (cssModules.length > 0) {
              // Return BOTH Vue modules and CSS - Vite will fetch Vue first, triggering
              // our transform hook to update stylexRules before CSS is fetched
              return [...modules, ...cssModules];
            }
          }

          return undefined;
        }

        // Skip files that wouldn't pass transformInclude (e.g. package.json, .css, images)
        // to avoid parsing non-JS/TS files with SWC
        if (!hasValidExtension(file, normalizedOptions.pageExtensions)) {
          return undefined;
        }

        const inputCode = await read();

        if (!hasStyleXCode(normalizedOptions, inputCode)) {
          return undefined;
        }

        transformStyleXCode(file, inputCode, normalizedOptions, stylexRules, id);

        const { processedFileName, collectedCSS } = await generateCSSAssets(
          stylexRules,
          normalizedOptions,
          transformedOptions,
          viteUserAssetsDir
        );

        if (!collectedCSS) return undefined;

        if (normalizedOptions.useCssPlaceholder) {
          // Find CSS modules that contain the placeholder
          const cssModules = await invalidateAndCollectCssModules(
            server,
            normalizedOptions.useCssPlaceholder,
            cssMarkerCache
          );

          if (cssModules.length > 0) {
            // Return both the changed modules and CSS modules
            // Vite will handle HMR for both
            return [...modules, ...cssModules];
          }
        } else {
          // Original behavior for non-placeholder mode
          if (processedFileName) {
            const normalizedFileName = ensureLeadingSlash(processedFileName);

            server.ws.send({
              type: 'update',
              updates: [
                {
                  acceptedPath: normalizedFileName,
                  path: normalizedFileName,
                  timestamp: Date.now(),
                  type: 'css-update',
                },
              ],
            });
          }
        }

        // Undefined tells Vite to apply its default HMR handling; the array
        // returns above are the only cases that override it.
        return undefined;
      },
      transformIndexHtml: async (html, ctx) => {
        // Skip HTML injection when using useCssPlaceholder
        // CSS is injected into the specified CSS file
        if (normalizedOptions.useCssPlaceholder) {
          return html;
        }

        const isDev = !!ctx.server;

        const { processedFileName } = await generateCSSAssets(
          stylexRules,
          normalizedOptions,
          transformedOptions,
          viteUserAssetsDir
        );

        if (!processedFileName) {
          return html;
        }

        const normalizedFileName = ensureLeadingSlash(processedFileName);

        if (isDev) {
          wsSend ||= ctx.server?.ws.send.bind(ctx.server.ws);
          // Deliberately the base-less path, unlike the href below. It is sent
          // as an HMR path, which is base-less, and matched against request
          // URLs, which still carry the base: the middleware is registered from
          // `configureServer`, so it runs ahead of Vite's own base middleware.
          // Hence the substring match there rather than an exact one.
          cssFileName ||= normalizedFileName;
        }

        return [
          {
            tag: 'link',
            attrs: {
              rel: 'stylesheet',
              href: resolveStylesheetHref(viteResolvedBase, normalizedFileName, ctx.path),
            },
            injectTo: 'head',
          },
        ];
      },
    },
    esbuild: {
      setup(build) {
        // esbuild drops CSS comments, so a comment marker never reaches the
        // output and the injection could only ever append the rules at the end.
        // Rewriting it to a statement at-rule here, the way the Vite load hook
        // does, is what keeps the marker's position.
        if (normalizedOptions.useCssPlaceholder) {
          const marker = normalizedOptions.useCssPlaceholder;

          build.onLoad({ filter: /\.css$/ }, async ({ path: cssPath }) => {
            let contents: string;

            try {
              contents = await readFile(cssPath, 'utf8');
            } catch {
              return null;
            }

            if (!contents.includes(marker)) return null;

            return {
              // Every occurrence, so a stray second marker cannot survive into
              // the output: the bundle step fills the first and removes the rest.
              contents: toBuildPlaceholder(contents, marker),
              loader: 'css' as const,
              resolveDir: path.dirname(cssPath),
            };
          });
        }

        build.onEnd(async ({ outputFiles, metafile }): Promise<OnEndResult> => {
          const fileName = normalizedOptions.fileName;
          const collectedCSS = getStyleXRules(stylexRules, transformedOptions);

          // A build with no rules still has to take the marker back out, so the
          // placeholder branch below runs either way.
          if (!collectedCSS && !normalizedOptions.useCssPlaceholder) return {};

          const shouldWriteToDisk =
            build.initialOptions.write === undefined || build.initialOptions.write;

          const outDir =
            build.initialOptions.outdir ||
            (build.initialOptions.outfile ? path.dirname(build.initialOptions.outfile) : null);

          // Handle useCssPlaceholder mode
          if (normalizedOptions.useCssPlaceholder && outDir && shouldWriteToDisk) {
            // Find CSS files in output
            let cssFiles: string[] = [];

            // Try to get CSS files from metafile
            if (metafile?.outputs) {
              cssFiles = Object.keys(metafile.outputs)
                .filter(f => f.endsWith('.css'))
                .map(f => (path.isAbsolute(f) ? f : path.join(process.cwd(), f)));
            }

            // Fallback: scan outDir for CSS files
            if (cssFiles.length === 0) {
              try {
                const files = await readdir(outDir);
                cssFiles = files.filter(f => f.endsWith('.css')).map(f => path.join(outDir, f));
              } catch {
                // Ignore errors
              }
            }

            // Read up front so a stylesheet that cannot be read is left out
            // entirely rather than being picked as the fallback and overwritten
            // with only the rules.
            const targets: CssInjectionTarget[] = [];

            for (const cssFile of cssFiles) {
              try {
                const existing = await readFile(cssFile, 'utf8');

                targets.push({
                  name: cssFile,
                  read: () => existing,
                  write: source => writeFile(cssFile, source, 'utf8'),
                });
              } catch {
                // Ignore errors
              }
            }

            const handled = await injectIntoCssTargets(
              targets,
              [BUILD_CSS_PLACEHOLDER, normalizedOptions.useCssPlaceholder],
              collectedCSS,
              (css, name) => transformStyleXCSS(css, name, normalizedOptions)
            );

            // A standalone file written here would never be linked, since
            // placeholder mode skips HTML injection.
            if (!handled && normalizedOptions.onMissingCssPlaceholder !== 'ignore') {
              return { warnings: [{ text: MISSING_INJECTION_TARGET_WARNING }] };
            }

            return {};
          }

          // Past the placeholder branch there is nothing to emit without rules.
          if (!collectedCSS) return {};

          // Default behavior: emit standalone CSS file
          if (shouldWriteToDisk) {
            const generatedCSSFileName = path.join(process.cwd(), fileName);
            const finalCSS = await transformStyleXCSS(
              collectedCSS,
              generatedCSSFileName,
              normalizedOptions
            );
            await mkdir(path.dirname(generatedCSSFileName), {
              recursive: true,
            });
            await writeFile(generatedCSSFileName, finalCSS, 'utf8');

            return {};
          }

          if (outputFiles !== undefined) {
            const finalCSS = await transformStyleXCSS(collectedCSS, fileName, normalizedOptions);
            outputFiles.push({
              path: '<stdout>',
              contents: new TextEncoder().encode(finalCSS),
              hash: generateHash(finalCSS),
              get text() {
                return finalCSS;
              },
            });
          }

          return {};
        });
      },
    },
    farm: {
      transformHtml: {
        async executor(resource) {
          // Skip HTML injection when using useCssPlaceholder
          if (normalizedOptions.useCssPlaceholder) {
            return resource.htmlResource;
          }

          if (!hasCssToExtract) return resource.htmlResource;

          const htmlResource = resource.htmlResource;

          let htmlContent = Buffer.from(htmlResource.bytes).toString('utf-8');

          htmlContent = `${htmlContent}<link rel="stylesheet" href="/${normalizedOptions.fileName}" />`;

          htmlResource.bytes = [...Buffer.from(htmlContent, 'utf-8')];

          return resource.htmlResource;
        },
      },
    },
    rspack(compiler) {
      if (!normalizedOptions.useCssPlaceholder) return;

      const injectMarker = normalizedOptions.useCssPlaceholder;

      // Use processAssets hook to replace the CSS marker with actual StyleX content
      // This runs after all CSS is processed through loaders (PostCSS, etc.)
      compiler.hooks.thisCompilation.tap(PLUGIN_NAME, compilation => {
        compilation.hooks.processAssets.tapPromise(
          {
            name: PLUGIN_NAME,
            stage: compiler.webpack.Compilation.PROCESS_ASSETS_STAGE_OPTIMIZE_SIZE,
          },
          async assets => {
            // Called even with no rules: the marker still has to come out.
            await injectStyleXCss(
              assets,
              injectMarker,
              getStyleXRules(stylexRules, transformedOptions),
              normalizedOptions,
              (fileName, source) => compilation.updateAsset(fileName, source),
              (content: string) => new compiler.webpack.sources.RawSource(content),
              message => compilation.warnings.push(new compiler.webpack.WebpackError(message))
            );
          }
        );
      });
    },
    webpack(compiler) {
      if (!normalizedOptions.useCssPlaceholder) return;

      const injectMarker = normalizedOptions.useCssPlaceholder;

      // Use processAssets hook to replace the CSS marker with actual StyleX content
      // This runs after all CSS is processed through loaders (PostCSS, etc.)
      compiler.hooks.thisCompilation.tap(PLUGIN_NAME, compilation => {
        compilation.hooks.processAssets.tapPromise(
          {
            name: PLUGIN_NAME,
            stage: compiler.webpack.Compilation.PROCESS_ASSETS_STAGE_OPTIMIZE_SIZE,
          },
          async assets => {
            // Called even with no rules: the marker still has to come out.
            await injectStyleXCss(
              assets,
              injectMarker,
              getStyleXRules(stylexRules, transformedOptions),
              normalizedOptions,
              (fileName, source) => compilation.updateAsset(fileName, source),
              (content: string) => new compiler.webpack.sources.RawSource(content),
              message => compilation.warnings.push(new compiler.webpack.WebpackError(message))
            );
          }
        );
      });
    },
  };
};

function ensureLeadingSlash(filePath: string) {
  return filePath.startsWith('/') ? filePath : `/${filePath}`;
}

async function generateCSSAssets(
  stylexRules: Record<string, [string, { ltr: string; rtl?: null | string }, number][]>,
  normalizedOptions: NormalizedOptions,
  transformedOptions: TransformedOptions,
  assetsDir?: string
) {
  const collectedCSS = getStyleXRules(stylexRules, transformedOptions);
  // The callback receives the un-hashed template path: the hashed name depends on
  // the transform output, so it cannot exist before the transform runs
  const finalCSS = collectedCSS
    ? await transformStyleXCSS(
        collectedCSS,
        getCssFilePathTemplate(normalizedOptions, assetsDir) ?? undefined,
        normalizedOptions
      )
    : collectedCSS;

  const processedFileName = getProcessedFileName(normalizedOptions, finalCSS || '', assetsDir);

  return { processedFileName, collectedCSS: finalCSS };
}

// Memoized per transformCss callback and file path while the input CSS is unchanged,
// because the same collected CSS is requested by several hooks per build and by
// every dev-server request for the CSS file
const transformCssCache = new WeakMap<
  NormalizedOptions['transformCss'],
  Map<string, { css: string; result: string }>
>();

async function transformStyleXCSS(
  css: string,
  filePath: string | undefined,
  normalizedOptions: NormalizedOptions
): Promise<string> {
  const { transformCss } = normalizedOptions;

  if (transformCss === identityTransformCss) {
    return css;
  }

  let cache = transformCssCache.get(transformCss);
  if (!cache) {
    cache = new Map();
    transformCssCache.set(transformCss, cache);
  }

  const cacheKey = filePath ?? '';
  const cached = cache.get(cacheKey);
  if (cached && cached.css === css) {
    return cached.result;
  }

  const result = (await transformCss(css, filePath)).toString();
  cache.set(cacheKey, { css, result });

  return result;
}

function hasStyleXCode(normalizedOptions: NormalizedOptions, inputCode: string) {
  return normalizedOptions.rsOptions.importSources?.some((importName: string | { from: string }) =>
    typeof importName === 'string'
      ? inputCode.includes(importName)
      : inputCode.includes(importName.from)
  );
}

/**
 * Narrows a transform-hook context to Rollup-compatible hosts that expose
 * the combined source map of previous plugins.
 */
function hasCombinedSourcemap(
  context: object
): context is { getCombinedSourcemap: () => { mappings?: string } } {
  return (
    'getCombinedSourcemap' in context &&
    typeof (context as { getCombinedSourcemap?: unknown }).getCombinedSourcemap === 'function'
  );
}

function transformStyleXCode(
  file: string,
  inputCode: string,
  normalizedOptions: NormalizedOptions,
  stylexRules: StyleXRules,
  id: string,
  inputSourceMap?: string
) {
  const rsOptions = { ...normalizedOptions.rsOptions };

  rsOptions.include = undefined;
  rsOptions.exclude = undefined;

  // Forward the combined map of previous plugins so debug source-map
  // annotations and the emitted map resolve to the original authored file.
  if (inputSourceMap !== undefined && rsOptions.inputSourceMap === undefined) {
    rsOptions.inputSourceMap = inputSourceMap;
  }

  const result = stylexTransform(file, inputCode, rsOptions);

  const { metadata } = result;

  if (normalizedOptions.extractCSS) {
    if (metadata.stylex && metadata.stylex.length > 0) {
      stylexRules[id] = metadata.stylex;
    } else {
      // The rules accumulate across a watch session, so a module that no longer
      // produces any has to drop its old ones rather than keep serving them.
      Reflect.deleteProperty(stylexRules, id);
    }
  }

  return result;
}

function getCssFilePathTemplate(normalizedOptions: NormalizedOptions, assetsDir?: string) {
  if (!normalizedOptions.fileName) return null;

  return assetsDir
    ? path.posix.join(assetsDir, normalizedOptions.fileName)
    : normalizedOptions.fileName;
}

function getProcessedFileName(
  normalizedOptions: NormalizedOptions,
  collectedCSS?: string,
  assetsDir?: string
) {
  const template = getCssFilePathTemplate(normalizedOptions, assetsDir);

  return template ? replaceFileName(template, collectedCSS || '') : null;
}

export const unplugin: UnpluginInstance<UnpluginStylexRSOptions | undefined> =
  createUnplugin(unpluginFactory);

export * from './types';

export default unplugin;
