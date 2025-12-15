import * as path from 'node:path';
import { promises } from 'node:fs';
import type { IncomingMessage, ServerResponse } from 'node:http';
import type { Connect } from 'vite';

import { createUnplugin } from 'unplugin';
import type { UnpluginFactory, UnpluginInstance, UnpluginMessage } from 'unplugin';

import getStyleXRules from './utils/getStyleXRules';
import normalizeOptions from './utils/normalizeOptions';
import type { UnpluginStylexRSOptions } from './types';
import stylexRsCompiler, { shouldTransformFile } from '@stylexswc/rs-compiler';
import generateHash from './utils/generateHash';
import crypto from 'crypto';

import type { StyleXMetadata } from '@stylexswc/rs-compiler';
import type { HotPayload, UserConfig, ViteDevServer } from 'vite';

type StyleXRules = Record<string, StyleXMetadata['stylex']>;

// Use the normalized options type from utils
import type { NormalizedOptions as NormalizedOptionsType } from './utils/normalizeOptions';
type NormalizedOptions = NormalizedOptionsType;

const { writeFile, mkdir } = promises;

const PLUGIN_NAME = 'unplugin-stylex-rs';

// Track Vite dev server for CSS invalidation
let viteDevServer: ViteDevServer | null = null;
let hasInvalidatedInitialCSS = false;

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
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
): Promise<any[]> {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const cssModules: any[] = [];

  // Skip if placeholder is not a string
  if (!placeholder || typeof placeholder !== 'string') {
    return cssModules;
  }

  const allCssModules = Array.from(server.moduleGraph.urlToModuleMap.values()).filter(
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (mod: any) => mod.id && mod.id.endsWith('.css')
  );

  // Check each CSS module for the placeholder
  // Note: We must read the original source file, not the transformed result,
  // because the transformed result already has the placeholder replaced
  await Promise.all(
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    allCssModules.map(async (mod: any) => {
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

/**
 * Injects StyleX CSS into CSS assets for webpack/rspack bundlers.
 * Shared logic to avoid code duplication between webpack and rspack hooks.
 */
function injectStyleXCss(
  assets: Record<string, { source(): { toString(): string } }>,
  injectMarker: string,
  collectedCSS: string,
  fallbackFileName: string,
  /* eslint-disable @typescript-eslint/no-explicit-any */
  updateAsset: (fileName: string, source: any) => void,
  emitAsset: (fileName: string, source: any) => void,
  createRawSource: (content: string) => any
  /* eslint-enable @typescript-eslint/no-explicit-any */
): void {
  const cssAssets = Object.keys(assets).filter(f => f.endsWith('.css'));

  // Try to find asset with the marker first
  let injected = false;
  for (const fileName of cssAssets) {
    const asset = assets[fileName];
    if (!asset) continue;
    const source = asset.source().toString();
    if (source.includes(injectMarker)) {
      const newSource = source.replace(injectMarker, collectedCSS);
      updateAsset(fileName, createRawSource(newSource));
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
        const newSource = existing ? existing + '\n' + collectedCSS : collectedCSS;
        updateAsset(targetAsset, createRawSource(newSource));
        injected = true;
      }
    }
  }

  // Last resort: emit standalone stylex.css
  if (!injected) {
    emitAsset(fallbackFileName, createRawSource(collectedCSS));
  }
}

export const unpluginFactory: UnpluginFactory<UnpluginStylexRSOptions | undefined> = (
  options = {}
) => {
  const normalizedOptions = normalizeOptions(options);

  // Mutable state for each compilation - reset in buildStart
  const stylexRules: StyleXRules = {};

  let viteConfig: UserConfig | null = null;

  let hasCssToExtract = false;
  let cssFileName: string | null = null;

  let wsSend: undefined | ((payload: HotPayload) => void) = undefined;

  return {
    name: PLUGIN_NAME,

    buildStart() {
      // stylexRules accumulates during watch mode for proper HMR
      hasCssToExtract = false;
      // Reset initial CSS invalidation flag for better lifecycle management
      hasInvalidatedInitialCSS = false;
    },

    transformInclude(id) {
      const pageExtensions = normalizedOptions.pageExtensions;

      const extensionName = path.extname(id);

      // Specific for Vite support
      const questionSignIndex = extensionName.indexOf('?');

      let cleanedExtensionName =
        questionSignIndex > -1 ? extensionName.slice(0, questionSignIndex) : extensionName;

      if (cleanedExtensionName.startsWith('.')) {
        cleanedExtensionName = cleanedExtensionName.slice(1);
      }

      if (!pageExtensions.includes(cleanedExtensionName)) {
        return false;
      }

      // Add path filtering using Rust function
      return shouldTransformFile(
        id,
        normalizedOptions.rsOptions?.include,
        normalizedOptions.rsOptions?.exclude
      );
    },

    async transform(inputCode, id) {
      if (!hasStyleXCode(normalizedOptions, inputCode)) {
        return null;
      }

      const dir = path.dirname(id);
      const basename = path.basename(id);
      const file = path.join(dir, basename.split('?')[0] || basename);

      try {
        const { code, map } = transformStyleXCode(
          file,
          inputCode,
          normalizedOptions,
          stylexRules,
          id
        );

        // Invalidate CSS modules in dev mode (initial load only)
        // For subsequent HMR, handleHotUpdate handles the CSS module inclusion
        if (normalizedOptions.useCssPlaceholder && viteDevServer && !hasInvalidatedInitialCSS) {
          // Set flag immediately to prevent concurrent invalidations
          hasInvalidatedInitialCSS = true;
          const wasCodeTransformed = code !== inputCode;

          if (wasCodeTransformed) {
            setTimeout(async () => {
              // Find all CSS modules that actually contain the placeholder
              const cssModules = await invalidateAndCollectCssModules(
                viteDevServer!,
                normalizedOptions.useCssPlaceholder
              );

              // Send update to trigger HMR
              if (cssModules.length > 0) {
                viteDevServer!.ws.send({
                  type: 'update',
                  updates: cssModules.map(mod => ({
                    type: 'css-update' as const,
                    acceptedPath: mod.url,
                    path: mod.url,
                    timestamp: Date.now(),
                  })),
                });
              }
            }, 50);
          }
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
        console.error('Transformation error:', error);
        this.error(error as UnpluginMessage);
      }
    },

    buildEnd() {
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

      const { processedFileName, collectedCSS } = generateCSSAssets(stylexRules, normalizedOptions);

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

    vite: {
      config(config) {
        viteConfig = {
          build: config.build as UserConfig['build'],
          base: config.base,
        };
      },

      configResolved(config) {
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

        // Get collected StyleX CSS
        const collectedCSS = getStyleXRules(stylexRules, normalizedOptions.useCSSLayers);
        // Check if dev server is running (more reliable than watchMode)
        const isDevMode = !!viteDevServer;

        // Determine replacement CSS based on mode and whether CSS exists
        let replacementCSS: string;
        if (!collectedCSS?.trim()) {
          // No CSS yet: use a comment that indicates the mode
          replacementCSS = isDevMode
            ? '/* StyleX styles will load after transformation */'
            : '/* No StyleX styles */';
        } else {
          replacementCSS = collectedCSS;
        }

        return cssContent.replace(normalizedOptions.useCssPlaceholder, replacementCSS);
      },

      generateBundle(_options, bundle) {
        if (!normalizedOptions.useCssPlaceholder) return;

        const collectedCSS = getStyleXRules(stylexRules, normalizedOptions.useCSSLayers);
        if (!collectedCSS) return;

        // Collect all CSS assets
        const cssAssets: Array<{ fileName: string; output: (typeof bundle)[string] }> = [];
        for (const [fileName, output] of Object.entries(bundle)) {
          if (output.type === 'asset' && fileName.endsWith('.css')) {
            cssAssets.push({ fileName, output });
          }
        }

        let injected = false;

        // First pass: look for marker-based injection
        for (const { output } of cssAssets) {
          if (output.type !== 'asset') continue;
          const source = output.source.toString();

          // Handle useCssPlaceholder (custom marker in real CSS file)
          if (
            normalizedOptions.useCssPlaceholder &&
            source.includes(normalizedOptions.useCssPlaceholder)
          ) {
            output.source = source.replace(normalizedOptions.useCssPlaceholder, collectedCSS);
            injected = true;
            break;
          }
        }

        // Fallback: if marker not found, append to preferred CSS asset
        if (!injected && cssAssets.length > 0) {
          const targetName = pickCssAsset(cssAssets.map(a => a.fileName));
          const target = cssAssets.find(a => a.fileName === targetName);
          if (target && target.output.type === 'asset') {
            const existing = target.output.source.toString();
            target.output.source = existing ? existing + '\n' + collectedCSS : collectedCSS;
            injected = true;
          }
        }

        // Last resort: emit standalone stylex.css if no CSS assets found
        if (!injected) {
          this.emitFile({
            type: 'asset',
            fileName: normalizedOptions.fileName,
            source: collectedCSS,
          });
        }
      },

      buildEnd() {
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

        const { processedFileName, collectedCSS } = generateCSSAssets(
          stylexRules,
          normalizedOptions,
          viteConfig?.build?.assetsDir
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
        viteDevServer = server as unknown as ViteDevServer;
        hasInvalidatedInitialCSS = false;

        server.middlewares.use(
          (req: IncomingMessage, res: ServerResponse, next: Connect.NextFunction) => {
            if (cssFileName && req.url?.includes(cssFileName)) {
              const collectedCSS = getStyleXRules(stylexRules, normalizedOptions.useCSSLayers);

              res.setHeader('Content-Type', 'text/css');
              res.end(collectedCSS);
              return;
            }
            next();
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
              server as unknown as ViteDevServer,
              normalizedOptions.useCssPlaceholder
            );

            if (cssModules.length > 0) {
              // Return BOTH Vue modules and CSS - Vite will fetch Vue first, triggering
              // our transform hook to update stylexRules before CSS is fetched
              return [...modules, ...cssModules];
            }
          }

          return;
        }

        const inputCode = await read();

        if (!hasStyleXCode(normalizedOptions, inputCode)) {
          return;
        }

        transformStyleXCode(file, inputCode, normalizedOptions, stylexRules, id);

        const { processedFileName, collectedCSS } = generateCSSAssets(
          stylexRules,
          normalizedOptions,
          viteConfig?.build?.assetsDir
        );

        if (!collectedCSS) return;

        if (normalizedOptions.useCssPlaceholder) {
          // Find CSS modules that contain the placeholder
          const cssModules = await invalidateAndCollectCssModules(
            server as unknown as ViteDevServer,
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
      },
      transformIndexHtml: (html, ctx) => {
        // Skip HTML injection when using useCssPlaceholder
        // CSS is injected into the specified CSS file
        if (normalizedOptions.useCssPlaceholder) {
          return html;
        }

        const isDev = !!ctx.server;

        const { processedFileName } = generateCSSAssets(
          stylexRules,
          normalizedOptions,
          viteConfig?.build?.assetsDir
        );

        if (!processedFileName) {
          return html;
        }

        const normalizedFileName = ensureLeadingSlash(processedFileName);

        if (isDev) {
          wsSend ||= ctx.server?.ws.send.bind(ctx.server.ws);
          cssFileName ||= normalizedFileName;
        }

        return [
          {
            tag: 'link',
            attrs: {
              rel: 'stylesheet',
              href: normalizedFileName,
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
          const collectedCSS = getStyleXRules(stylexRules, normalizedOptions.useCSSLayers);

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
                  const newContent = content.replace(injectMarker, collectedCSS);
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
                const fullPath = cssFiles.find(f => f.endsWith(targetFile));
                if (fullPath) {
                  try {
                    const { readFile } = await import('node:fs/promises');
                    const existing = await readFile(fullPath, 'utf8');
                    const newContent = existing ? existing + '\n' + collectedCSS : collectedCSS;
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
              await mkdir(path.dirname(generatedCSSFileName), { recursive: true });
              await writeFile(generatedCSSFileName, collectedCSS, 'utf8');
            }

            return;
          }

          // Default behavior: emit standalone CSS file
          if (shouldWriteToDisk) {
            const generatedCSSFileName = path.join(process.cwd(), fileName);
            await mkdir(path.dirname(generatedCSSFileName), {
              recursive: true,
            });
            await writeFile(generatedCSSFileName, collectedCSS, 'utf8');

            return;
          }

          if (outputFiles !== undefined) {
            outputFiles.push({
              path: '<stdout>',
              contents: new TextEncoder().encode(collectedCSS),
              hash: generateHash(collectedCSS),
              get text() {
                return collectedCSS;
              },
            });
          }
        });
      },
    },
    farm: {
      transformHtml: {
        executor(resource) {
          // Skip HTML injection when using useCssPlaceholder
          if (normalizedOptions.useCssPlaceholder) {
            return Promise.resolve(resource.htmlResource);
          }

          if (!hasCssToExtract) return Promise.resolve(resource.htmlResource);

          const htmlResource = resource.htmlResource;

          let htmlContent = Buffer.from(htmlResource.bytes).toString('utf-8');

          htmlContent = `${htmlContent}<link rel="stylesheet" href="/${normalizedOptions.fileName}" />`;

          htmlResource.bytes = [...Buffer.from(htmlContent, 'utf-8')];

          return Promise.resolve(resource.htmlResource);
        },
      },
      // Farm uses Rollup-like bundle hooks, so generateBundle handles CSS injection
      // The useCssPlaceholder logic is handled in the shared generateBundle (via Vite hooks)
    },
    rspack(compiler) {
      if (!normalizedOptions.useCssPlaceholder) return;

      const injectMarker = normalizedOptions.useCssPlaceholder;

      // Use processAssets hook to replace the CSS marker with actual StyleX content
      // This runs after all CSS is processed through loaders (PostCSS, etc.)
      compiler.hooks.thisCompilation.tap(PLUGIN_NAME, compilation => {
        compilation.hooks.processAssets.tap(
          {
            name: PLUGIN_NAME,
            stage: compiler.webpack.Compilation.PROCESS_ASSETS_STAGE_OPTIMIZE_SIZE,
          },
          assets => {
            const collectedCSS = getStyleXRules(stylexRules, normalizedOptions.useCSSLayers);
            if (!collectedCSS) return;

            injectStyleXCss(
              assets,
              injectMarker,
              collectedCSS,
              normalizedOptions.fileName,
              (fileName, source) => compilation.updateAsset(fileName, source),
              (fileName, source) => compilation.emitAsset(fileName, source),
              content => new compiler.webpack.sources.RawSource(content)
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
        compilation.hooks.processAssets.tap(
          {
            name: PLUGIN_NAME,
            stage: compiler.webpack.Compilation.PROCESS_ASSETS_STAGE_OPTIMIZE_SIZE,
          },
          assets => {
            const collectedCSS = getStyleXRules(stylexRules, normalizedOptions.useCSSLayers);
            if (!collectedCSS) return;

            injectStyleXCss(
              assets,
              injectMarker,
              collectedCSS,
              normalizedOptions.fileName,
              (fileName, source) => compilation.updateAsset(fileName, source),
              (fileName, source) => compilation.emitAsset(fileName, source),
              content => new compiler.webpack.sources.RawSource(content)
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

function generateCSSAssets(
  stylexRules: Record<string, [string, { ltr: string; rtl?: null | string }, number][]>,
  normalizedOptions: NormalizedOptions,
  assetsDir?: string
) {
  const collectedCSS = getStyleXRules(stylexRules, normalizedOptions.useCSSLayers);

  const processedFileName = getProcessedFileName(normalizedOptions, collectedCSS || '', assetsDir);

  return { processedFileName, collectedCSS };
}

function hasStyleXCode(normalizedOptions: NormalizedOptions, inputCode: string) {
  return normalizedOptions.rsOptions.importSources?.some((importName: string | { from: string }) =>
    typeof importName === 'string'
      ? inputCode.includes(importName)
      : inputCode.includes(importName.from)
  );
}

function transformStyleXCode(
  file: string,
  inputCode: string,
  normalizedOptions: NormalizedOptions,
  stylexRules: StyleXRules,
  id: string
) {
  const rsOptions = { ...normalizedOptions.rsOptions };

  rsOptions.include = undefined;
  rsOptions.exclude = undefined;

  const result = stylexRsCompiler.transform(file, inputCode, rsOptions);

  const { metadata } = result;

  if (normalizedOptions.extractCSS && metadata.stylex && metadata.stylex.length > 0) {
    stylexRules[id] = metadata.stylex;
  }

  return result;
}

function getProcessedFileName(
  normalizedOptions: NormalizedOptions,
  collectedCSS?: string,
  assetsDir?: string
) {
  if (!normalizedOptions.fileName) return null;

  const computedFileName = assetsDir
    ? path.posix.join(assetsDir, normalizedOptions.fileName)
    : normalizedOptions.fileName;

  return replaceFileName(computedFileName, collectedCSS || '');
}

export const unplugin: UnpluginInstance<UnpluginStylexRSOptions | undefined, boolean> =
  createUnplugin(unpluginFactory);

export * from './types';

export default unplugin;
