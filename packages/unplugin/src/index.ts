import * as path from 'node:path';

import { createUnplugin } from 'unplugin';
import type { UnpluginFactory, UnpluginInstance, UnpluginMessage } from 'unplugin';
import type { BuildOptions } from 'vite';

import getStyleXRules from './utils/getStyleXRules';
import normalizeOptions from './utils/normalizeOptions';
import type { UnpluginStylexRSOptions } from './types';
import stylexRsCompiler, { StyleXMetadata } from '@stylexswc/rs-compiler';

export const unpluginFactory: UnpluginFactory<UnpluginStylexRSOptions | undefined> = (
  options = {}
) => {
  const normalizedOptions = normalizeOptions(options);

  const stylexRules: Record<string, StyleXMetadata['stylex']> = {};

  let viteConfig: { build: BuildOptions | undefined; base: string | undefined } | null = null;

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
        return;
      }

      try {
        const { code, metadata, map } = stylexRsCompiler.transform(
          file,
          inputCode,
          normalizedOptions.rsOptions
        );

        if (metadata.stylex && metadata.stylex.length > 0) {
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
      const collectedCSS = getStyleXRules(stylexRules, normalizedOptions.useCSSLayers);

      if (!collectedCSS) return;

      this.emitFile({
        fileName: normalizedOptions.fileName,
        source: collectedCSS,
        type: 'asset',
      });
    },

    vite: {
      config(config) {
        viteConfig = {
          build: config.build,
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

      transformIndexHtml(html, ctx) {
        const fileName = `${viteConfig?.build?.assetsDir ?? 'assets'}/${normalizedOptions.fileName}`;
        const css = ctx.bundle?.[fileName];

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
  };
};

export const unplugin: UnpluginInstance<UnpluginStylexRSOptions | undefined, boolean> =
  createUnplugin(unpluginFactory);

export * from './types';

export default unplugin;
