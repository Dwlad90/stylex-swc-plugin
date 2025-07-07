import * as path from 'node:path';
import { promises } from 'node:fs';

import { createUnplugin } from 'unplugin';
import type { UnpluginFactory, UnpluginInstance, UnpluginMessage } from 'unplugin';

import getStyleXRules from './utils/getStyleXRules';
import normalizeOptions from './utils/normalizeOptions';
import type { UnpluginStylexRSOptions } from './types';
import stylexRsCompiler from '@stylexswc/rs-compiler';
import generateHash from './utils/generateHash';

import type { StyleXMetadata } from '@stylexswc/rs-compiler';
import { UserConfig } from 'vite';

const { writeFile, mkdir } = promises;

export const unpluginFactory: UnpluginFactory<UnpluginStylexRSOptions | undefined> = (
  options = {}
) => {
  const normalizedOptions = normalizeOptions(options);

  const stylexRules: Record<string, StyleXMetadata['stylex']> = {};

  let viteConfig: UserConfig | null = null;

  let hasCssToExtract = false;
  let cssFileName: string | null = null;

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

      return pageExtensions.includes(cleanedExtensionName);
    },

    async transform(inputCode, id) {
      const dir = path.dirname(id);
      const basename = path.basename(id);
      const file = path.join(dir, basename.includes('?') ? basename.split('?')[0] : basename);

      if (
        !normalizedOptions.rsOptions.importSources?.some(importName =>
          typeof importName === 'string'
            ? inputCode.includes(importName)
            : inputCode.includes(importName.from)
        )
      ) {
        return {
          code: inputCode,
        };
      }

      try {
        const { code, metadata, map } = stylexRsCompiler.transform(
          file,
          inputCode,
          normalizedOptions.rsOptions
        );

        if (normalizedOptions.extractCSS && metadata.stylex && metadata.stylex.length > 0) {
          stylexRules[id] = metadata.stylex;
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
      const collectedCSS = getStyleXRules(stylexRules, normalizedOptions.useCSSLayers);

      if (!collectedCSS) return;

      hasCssToExtract = true;

      this.emitFile({
        fileName: normalizedOptions.fileName,
        source: collectedCSS,
        type: 'asset',
      });
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
        const fileName = `${viteConfig?.build?.assetsDir ?? 'assets'}/${normalizedOptions.fileName}`;
        const collectedCSS = getStyleXRules(stylexRules, normalizedOptions.useCSSLayers);

        if (!collectedCSS) return;

        this.emitFile({
          fileName,
          source: collectedCSS,
          type: 'asset',
        });
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
      transformIndexHtml(html, ctx) {
        const isDev = !!ctx.server;

        const fileName = `${viteConfig?.build?.assetsDir ?? 'assets'}/${normalizedOptions.fileName}`;

        if (isDev) {
          cssFileName = fileName;
        }

        const css = ctx.bundle?.[fileName] || cssFileName;

        if (!css) {
          return html;
        }

        const publicPath = path.posix.join(viteConfig?.base ?? '/', fileName.replace(/\\/g, '/'));

        return [
          {
            tag: 'link',
            attrs: {
              rel: 'stylesheet',
              href: publicPath,
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

export const unplugin: UnpluginInstance<UnpluginStylexRSOptions | undefined, boolean> =
  createUnplugin(unpluginFactory);

export * from './types';

export default unplugin;
