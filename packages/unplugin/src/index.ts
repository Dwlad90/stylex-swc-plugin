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
import type { HotPayload, UserConfig } from 'vite';

type StyleXRules = Record<string, StyleXMetadata['stylex']>;

const { writeFile, mkdir } = promises;

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

      buildEnd() {
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
      async handleHotUpdate({ file: id, file, server, read }) {
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
      },
      transformIndexHtml: (html, ctx) => {
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
