import crypto from 'node:crypto';
import { promises } from 'node:fs';
import type { IncomingMessage, ServerResponse } from 'node:http';
import * as path from 'node:path';

import { shouldTransformFile, transform as stylexTransform } from '@stylexswc/rs-compiler';
import type { StyleXMetadata, TransformedOptions } from '@stylexswc/rs-compiler';
import { createUnplugin } from 'unplugin';
import type { UnpluginFactory, UnpluginInstance } from 'unplugin';
import type { Connect } from 'vite';
import type { HotPayload, ModuleNode, ViteDevServer } from 'vite';

import type { UnpluginStylexRSOptions } from './types';
import generateHash from './utils/generateHash';
import getStyleXRules from './utils/getStyleXRules';
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

const { writeFile, mkdir } = promises;

const PLUGIN_NAME = 'unplugin-stylex-rs';

/**
 * Stands in for the user's marker between the Vite load hook and the bundle,
 * during builds only.
 *
 * A statement at-rule is the one form that survives CSS minification in place:
 * esbuild and Lightning CSS both drop comments, including legal ones, but must
 * keep `@layer` because it declares layer order. Using it for every marker
 * style also keeps Lightning CSS from ever parsing the default `@stylex;`
 * marker, which it reports as an unknown at-rule.
 */
const CSS_PLACEHOLDER_SENTINEL = '@layer __stylex_placeholder__;';

/**
 * Placeholder mode deliberately skips HTML injection, so a stylesheet emitted
 * on its own would never be linked and the styles would simply be missing at
 * runtime. Failing the build is the only honest outcome.
 */
/**
 * Replaces the first marker occurrence and drops every later one: repeating the
 * whole rule set per marker would only duplicate it. Splitting rather than
 * `String#replace` also keeps `$&`-like sequences in the CSS literal intact.
 */
function replaceFirstMarker(source: string, marker: string, replacement: string): string {
  const [head, ...rest] = source.split(marker);

  if (head === undefined || rest.length === 0) return source;

  return head + replacement + rest.join('');
}

/**
 * Removes every marker occurrence, for the stylesheets that did not receive the
 * rules.
 */
function stripMarkers(source: string, markers: string[]): string {
  return markers.reduce((stripped, marker) => stripped.split(marker).join(''), source);
}

const MISSING_INJECTION_TARGET_ERROR =
  'StyleX: no CSS asset was available to receive the placeholder styles. ' +
  'Make sure the stylesheet holding the marker is imported by the module graph.';

function replaceFileName(original: string, css: string) {
  if (!original.includes('[hash]')) {
    return original;
  }
  const hash = crypto.createHash('sha256').update(css).digest('hex').slice(0, 8);
  return original.replace(/\[hash\]/g, hash);
}

/**
 * Pick a stable CSS asset to inject into.
 * Preference: index.css > style.css > main.css > first .css asset
 */
function pickCssAsset(
  cssAssets: string[],
  chooseFn?: (fileName: string) => boolean
): string | null {
  if (cssAssets.length === 0) return null;

  // If user provided a chooser function, use it first
  if (typeof chooseFn === 'function') {
    const chosen = cssAssets.find(chooseFn);
    if (chosen) return chosen;
  }

  // Prefer well-known CSS filenames
  const preferred =
    cssAssets.find(f => /(^|\/)index\.css$/.test(f)) ||
    cssAssets.find(f => /(^|\/)style\.css$/.test(f)) ||
    cssAssets.find(f => /(^|\/)main\.css$/.test(f));

  return preferred || cssAssets[0] || null;
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
  placeholder: NormalizedOptions['useCssPlaceholder']
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

        const content = await promises.readFile(mod.id, 'utf8');
        if (content.includes(placeholder)) {
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

function hasUnresolvedDefineConstAtRule(css: string): boolean {
  let atRuleStart = true;

  for (let index = 0; index < css.length; index += 1) {
    const character = css[index];
    const nextCharacter = css[index + 1];

    if (character === '/' && nextCharacter === '*') {
      const commentEnd = css.indexOf('*/', index + 2);
      if (commentEnd === -1) return false;
      index = commentEnd + 1;
      continue;
    }

    if (character === '"' || character === "'") {
      const quote = character;
      index += 1;

      for (; index < css.length; index += 1) {
        if (css[index] === '\\') {
          index += 1;
        } else if (css[index] === quote) {
          break;
        }
      }

      atRuleStart = false;
      continue;
    }

    if (character === '{' || character === '}') {
      atRuleStart = true;
      continue;
    }

    if (/\s/.test(character ?? '')) continue;

    if (atRuleStart && css.startsWith('var(--', index)) {
      const closingParenthesis = css.indexOf(')', index + 6);
      if (closingParenthesis === -1) return false;

      if (closingParenthesis > index + 6) {
        let nextToken = closingParenthesis + 1;
        while (/\s/.test(css[nextToken] ?? '')) nextToken += 1;
        if (css[nextToken] === '{') return true;
      }

      index = closingParenthesis;
    }

    atRuleStart = false;
  }

  return false;
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
 * It replaces three `any` parameters, and is stricter than they were: the
 * source produced by `createRawSource` must be the same type `updateAsset` and
 * `emitAsset` accept. Under `any`, handing a webpack `RawSource` to rspack's
 * `updateAsset` type-checked cleanly and would only have failed at runtime.
 */
async function injectStyleXCss<TSource>(
  assets: Record<string, { source(): { toString(): string } }>,
  injectMarker: string,
  collectedCSS: string,
  normalizedOptions: NormalizedOptions,
  updateAsset: (fileName: string, source: TSource) => void,
  createRawSource: (content: string) => TSource
): Promise<void> {
  const cssAssets = Object.keys(assets).filter(f => f.endsWith('.css'));

  // Try to find asset with the marker first
  let injected = false;
  for (const fileName of cssAssets) {
    const asset = assets[fileName];
    if (!asset) continue;
    const source = asset.source().toString();
    if (source.includes(injectMarker)) {
      const finalCSS = await transformStyleXCSS(collectedCSS, fileName, normalizedOptions);
      updateAsset(fileName, createRawSource(replaceFirstMarker(source, injectMarker, finalCSS)));
      injected = true;
      break;
    }
  }

  // Fallback: append to a preferred CSS asset if marker not found
  if (!injected && cssAssets.length > 0) {
    const targetAsset = pickCssAsset(cssAssets);
    if (targetAsset) {
      const asset = assets[targetAsset];
      if (asset) {
        const existing = asset.source().toString();
        const finalCSS = await transformStyleXCSS(collectedCSS, targetAsset, normalizedOptions);
        const newSource = existing ? existing + '\n' + finalCSS : finalCSS;
        updateAsset(targetAsset, createRawSource(newSource));
        injected = true;
      }
    }
  }

  // Nothing else to do when the compilation produced no stylesheet at all: an
  // asset emitted here could not be linked, and unlike the Vite adapter this
  // one has no signal that the marker was ever part of the build.
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
  reportMissingTarget: boolean
): Promise<void> {
  if (!normalizedOptions.useCssPlaceholder) return;

  const collectedCSS = getStyleXRules(stylexRules, transformedOptions);

  const cssAssets = Object.values(bundle).filter(
    (output): output is BundleAssetLike =>
      output.type === 'asset' && output.fileName.endsWith('.css')
  );

  // The sentinel is what the Vite load hook leaves behind; the raw marker still
  // turns up when the stylesheet reached the bundle without passing through it,
  // as it does under plain Rollup.
  const markers = [CSS_PLACEHOLDER_SENTINEL, normalizedOptions.useCssPlaceholder];

  let injected = false;

  // First pass: look for marker-based injection
  for (const asset of cssAssets) {
    let source = asset.source.toString();
    const marker = markers.find(candidate => source.includes(candidate));

    if (!marker) continue;

    if (!injected) {
      // An empty rule set still has to take the sentinel back out, otherwise it
      // ships to the browser.
      const finalCSS = collectedCSS
        ? await transformStyleXCSS(collectedCSS, asset.fileName, normalizedOptions)
        : '';

      source = replaceFirstMarker(source, marker, finalCSS);
      injected = true;
    }

    // Whatever is left over -- a second marker here, or a marker in another
    // stylesheet -- would repeat the rules, so it is only removed.
    asset.source = stripMarkers(source, markers);
  }

  if (injected || !collectedCSS) return;

  // Fallback: if marker not found, append to preferred CSS asset
  if (cssAssets.length > 0) {
    const targetName = pickCssAsset(cssAssets.map(asset => asset.fileName));
    const target = cssAssets.find(asset => asset.fileName === targetName);

    if (target) {
      const existing = target.source.toString();
      const finalCSS = await transformStyleXCSS(collectedCSS, target.fileName, normalizedOptions);

      target.source = existing ? existing + '\n' + finalCSS : finalCSS;
      injected = true;
    }
  }

  // Emitting a standalone stylesheet here used to look like a safety net, but
  // placeholder mode never links it, so it only ever hid missing styles.
  if (!injected && reportMissingTarget) {
    context.error(MISSING_INJECTION_TARGET_ERROR);
  }
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
          normalizedOptions.useCssPlaceholder
        );

        refreshedRulesRevision = coveredRevision;

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

        // Rules that arrived while this refresh was in flight need their own.
        if (viteDevServer === server) scheduleCssRefresh();
      })().catch((error: unknown) => {
        // A transient read or websocket failure must not swallow the refresh:
        // leaving the revision untouched lets the next transform retry.
        console.error('StyleX: failed to refresh placeholder CSS modules', error);
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
  const placeholderGenerateBundle = {
    order: 'post' as const,
    async handler(this: PlaceholderBundleContext, _options: unknown, bundle: OutputBundleLike) {
      await injectPlaceholderIntoBundle(
        this,
        bundle,
        stylexRules,
        normalizedOptions,
        transformedOptions,
        placeholderSeen && !viteIsSsrBuild
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
        // is loaded. Leave a sentinel for generateBundle to replace with the
        // final rules, so they are injected once and at the marker's position.
        // The dev server has no bundle step and keeps inlining what it has.
        if (!viteDevServer) {
          placeholderSeen = true;

          // Every occurrence, so a stray second marker cannot survive into the
          // output: generateBundle fills the first and removes the rest.
          return cssContent
            .split(normalizedOptions.useCssPlaceholder)
            .join(CSS_PLACEHOLDER_SENTINEL);
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

        server.watcher.once('close', () => {
          if (viteDevServer !== server) return;

          viteDevServer = null;
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
              normalizedOptions.useCssPlaceholder
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
            normalizedOptions.useCssPlaceholder
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
        build.onEnd(async ({ outputFiles, metafile }) => {
          const fileName = normalizedOptions.fileName;
          const collectedCSS = getStyleXRules(stylexRules, transformedOptions);

          if (!collectedCSS) return;

          const shouldWriteToDisk =
            build.initialOptions.write === undefined || build.initialOptions.write;

          const outDir =
            build.initialOptions.outdir ||
            (build.initialOptions.outfile ? path.dirname(build.initialOptions.outfile) : null);

          // Handle useCssPlaceholder mode
          if (normalizedOptions.useCssPlaceholder && outDir && shouldWriteToDisk) {
            const injectMarker = normalizedOptions.useCssPlaceholder;

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
                const { readdir } = await import('node:fs/promises');
                const files = await readdir(outDir);
                cssFiles = files.filter(f => f.endsWith('.css')).map(f => path.join(outDir, f));
              } catch {
                // Ignore errors
              }
            }

            // Try to inject into a CSS file with marker
            let injected = false;
            for (const cssFile of cssFiles) {
              try {
                const { readFile } = await import('node:fs/promises');
                const content = await readFile(cssFile, 'utf8');
                if (content.includes(injectMarker)) {
                  const finalCSS = await transformStyleXCSS(
                    collectedCSS,
                    cssFile,
                    normalizedOptions
                  );
                  const newContent = content.replace(injectMarker, () => finalCSS);
                  await writeFile(cssFile, newContent, 'utf8');
                  injected = true;
                  break;
                }
              } catch {
                // Ignore errors
              }
            }

            // Fallback: append to a preferred CSS file
            if (!injected && cssFiles.length > 0) {
              const targetFile = pickCssAsset(cssFiles.map(f => path.basename(f)));
              if (targetFile) {
                const fullPath = cssFiles.find(f => path.basename(f) === targetFile);
                if (fullPath) {
                  try {
                    const { readFile } = await import('node:fs/promises');
                    const existing = await readFile(fullPath, 'utf8');
                    const finalCSS = await transformStyleXCSS(
                      collectedCSS,
                      fullPath,
                      normalizedOptions
                    );
                    const newContent = existing ? existing + '\n' + finalCSS : finalCSS;
                    await writeFile(fullPath, newContent, 'utf8');
                    injected = true;
                  } catch {
                    // Ignore errors
                  }
                }
              }
            }

            // Last resort: emit standalone stylex.css
            if (!injected) {
              const generatedCSSFileName = path.join(outDir, fileName);
              const finalCSS = await transformStyleXCSS(
                collectedCSS,
                generatedCSSFileName,
                normalizedOptions
              );
              await mkdir(path.dirname(generatedCSSFileName), { recursive: true });
              await writeFile(generatedCSSFileName, finalCSS, 'utf8');
            }

            return;
          }

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

            return;
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
            const collectedCSS = getStyleXRules(stylexRules, transformedOptions);
            if (!collectedCSS) return;

            await injectStyleXCss(
              assets,
              injectMarker,
              collectedCSS,
              normalizedOptions,
              (fileName, source) => compilation.updateAsset(fileName, source),
              (content: string) => new compiler.webpack.sources.RawSource(content)
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
            const collectedCSS = getStyleXRules(stylexRules, transformedOptions);
            if (!collectedCSS) return;

            await injectStyleXCss(
              assets,
              injectMarker,
              collectedCSS,
              normalizedOptions,
              (fileName, source) => compilation.updateAsset(fileName, source),
              (content: string) => new compiler.webpack.sources.RawSource(content)
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

  if (normalizedOptions.extractCSS && metadata.stylex && metadata.stylex.length > 0) {
    stylexRules[id] = metadata.stylex;
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
