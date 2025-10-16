import * as path from 'node:path';
import { promises } from 'node:fs';

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

const { writeFile, mkdir } = promises;

const VIRTUAL_CSS_MODULE_ID = 'virtual:stylex.css';
const RESOLVED_VIRTUAL_CSS_MODULE_ID = '\0' + VIRTUAL_CSS_MODULE_ID;
const VIRTUAL_CSS_MARKER_VAR = '--stylex-placeholder';
const VIRTUAL_CSS_MARKER = `:root{${VIRTUAL_CSS_MARKER_VAR}:1}`;

let viteDevServer: ViteDevServer | null = null;
let hasInvalidatedInitialCSS = false;

function replaceFileName(original: string, css: string) {
  if (!original.includes('[hash]')) {
    return original;
  }
  const hash = crypto.createHash('sha256').update(css).digest('hex').slice(0, 8);
  return original.replace(/\[hash\]/g, hash);
}

export const unpluginFactory: UnpluginFactory<UnpluginStylexRSOptions | undefined> = (
  options = {}
) => {
  const normalizedOptions = normalizeOptions(options);

  const stylexRules: StyleXRules = {};

  let viteConfig: UserConfig | null = null;

  let hasCssToExtract = false;
  let cssFileName: string | null = null;

  let wsSend: undefined | ((payload: HotPayload) => void) = undefined;

  return {
    name: 'unplugin-stylex-rs',

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
        return {
          code: inputCode,
        };
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

        // Invalidate virtual CSS module in dev mode (initial load only)
        // For Vue HMR, handleHotUpdate handles the CSS module inclusion
        if (normalizedOptions.useViteCssPipeline && viteDevServer && !hasInvalidatedInitialCSS) {
          const hasStyleXCode = code !== inputCode;

          if (hasStyleXCode) {
            hasInvalidatedInitialCSS = true;

            setTimeout(() => {
              const mod = viteDevServer?.moduleGraph.getModuleById(RESOLVED_VIRTUAL_CSS_MODULE_ID);
              if (mod) {
                viteDevServer?.moduleGraph.invalidateModule(mod);
                viteDevServer?.ws.send({
                  type: 'update',
                  updates: [
                    {
                      type: 'js-update',
                      acceptedPath: RESOLVED_VIRTUAL_CSS_MODULE_ID,
                      path: RESOLVED_VIRTUAL_CSS_MODULE_ID,
                      timestamp: Date.now(),
                    },
                  ],
                });
              }
            }, 50);
          }
        }

        return {
          code,
          map,
          stylexRules,
        };
      } catch (error) {
        console.error('Tansformation error:', error);
        this.error(error as UnpluginMessage);
      }
    },

    buildEnd() {
      const framework = this.getNativeBuildContext?.().framework;
      if (framework === 'esbuild') {
        // will handle the CSS generation in the plugin itself
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

      resolveId(id) {
        if (id === VIRTUAL_CSS_MODULE_ID) {
          return RESOLVED_VIRTUAL_CSS_MODULE_ID;
        }
      },

      async load(id) {
        if (id === RESOLVED_VIRTUAL_CSS_MODULE_ID) {
          // In dev mode, return actual CSS for HMR
          // In build mode, return placeholder that will be replaced in generateBundle
          const isDev = this.meta?.watchMode;

          if (isDev) {
            // Always return current CSS state
            const collectedCSS = getStyleXRules(stylexRules, normalizedOptions.useCSSLayers);

            // If no CSS yet, return a minimal placeholder that won't break
            // HMR will update this when files are transformed
            if (!collectedCSS || collectedCSS.trim().length === 0) {
              return {
                code: '/* StyleX styles will load after transformation */',
                map: null,
              };
            }

            return {
              code: collectedCSS,
              map: null,
            };
          }

          // Build mode: return placeholder
          return {
            code: VIRTUAL_CSS_MARKER,
            map: null,
          };
        }
      },

      generateBundle(_options, bundle) {
        if (!normalizedOptions.useViteCssPipeline) return;

        const collectedCSS = getStyleXRules(stylexRules, normalizedOptions.useCSSLayers);
        if (!collectedCSS) return;

        // Find and update the CSS asset generated from the virtual module
        for (const [fileName, output] of Object.entries(bundle)) {
          // Look for CSS assets that contain our placeholder
          if (output.type === 'asset' && fileName.endsWith('.css')) {
            const source = output.source.toString();
            if (source.includes(VIRTUAL_CSS_MARKER)) {
              // Replace only the placeholder with actual CSS, preserving other CSS
              output.source = source.replace(VIRTUAL_CSS_MARKER, collectedCSS);
              break;
            }
          }
        }
      },

      buildEnd() {
        // Skip emitting CSS file when using Vite's CSS pipeline
        // Vite will handle bundling the virtual CSS module
        if (normalizedOptions.useViteCssPipeline) {
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
        viteDevServer = server;
        hasInvalidatedInitialCSS = false; // Reset on server restart

        server.middlewares.use((req, res, next) => {
          if (cssFileName && req.url?.includes(cssFileName)) {
            const collectedCSS = getStyleXRules(stylexRules, normalizedOptions.useCSSLayers);

            res.setHeader('Content-Type', 'text/css');
            res.end(collectedCSS);
            return;
          }
          next();
        });
      },
      async handleHotUpdate({ file: id, file, server, read, modules }) {
        // For Vue files, include CSS module but don't transform
        // (raw .vue files have <template>, <style> sections that SWC can't parse)
        // The transform hook will update stylexRules when Vue plugin converts it to JS
        if (file.includes('.vue')) {
          if (normalizedOptions.useViteCssPipeline) {
            const virtualMod = server.moduleGraph.getModuleById(RESOLVED_VIRTUAL_CSS_MODULE_ID);
            if (virtualMod) {
              server.moduleGraph.invalidateModule(virtualMod);
              // Return BOTH Vue modules and CSS - Vite will fetch Vue first, triggering
              // our transform hook to update stylexRules before CSS is fetched
              return [...modules, virtualMod];
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

        if (normalizedOptions.useViteCssPipeline) {
          const virtualMod = server.moduleGraph.getModuleById(RESOLVED_VIRTUAL_CSS_MODULE_ID);
          if (virtualMod) {
            server.moduleGraph.invalidateModule(virtualMod);

            return [...modules, virtualMod];
          }
        } else {
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
        // Skip HTML injection when using Vite's CSS pipeline
        // Users should import 'virtual:stylex.css' in their entry file
        if (normalizedOptions.useViteCssPipeline) {
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
        build.onEnd(async ({ outputFiles }) => {
          const fileName = normalizedOptions.fileName;
          const collectedCSS = getStyleXRules(stylexRules, normalizedOptions.useCSSLayers);

          if (!collectedCSS) return;

          const shouldWriteToDisk =
            build.initialOptions.write === undefined || build.initialOptions.write;

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
          if (!hasCssToExtract) return Promise.resolve(resource.htmlResource);

          const htmlResource = resource.htmlResource;

          let htmlContent = Buffer.from(htmlResource.bytes).toString('utf-8');

          htmlContent = `${htmlContent}<link rel="stylesheet" href="/${normalizedOptions.fileName}" />`;

          htmlResource.bytes = [...Buffer.from(htmlContent, 'utf-8')];

          return Promise.resolve(resource.htmlResource);
        },
      },
    },
  };
};

function ensureLeadingSlash(filePath: string) {
  return filePath.startsWith('/') ? filePath : `/${filePath}`;
}

function generateCSSAssets(
  stylexRules: Record<string, [string, { ltr: string; rtl?: null | string }, number][]>,
  normalizedOptions: Required<UnpluginStylexRSOptions>,
  assetsDir?: string
) {
  const collectedCSS = getStyleXRules(stylexRules, normalizedOptions.useCSSLayers);

  const processedFileName = getProcessedFileName(normalizedOptions, collectedCSS || '', assetsDir);

  return { processedFileName, collectedCSS };
}

function hasStyleXCode(normalizedOptions: Required<UnpluginStylexRSOptions>, inputCode: string) {
  return normalizedOptions.rsOptions.importSources?.some(importName =>
    typeof importName === 'string'
      ? inputCode.includes(importName)
      : inputCode.includes(importName.from)
  );
}

function transformStyleXCode(
  file: string,
  inputCode: string,
  normalizedOptions: Required<UnpluginStylexRSOptions>,
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
  normalizedOptions: UnpluginStylexRSOptions,
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
