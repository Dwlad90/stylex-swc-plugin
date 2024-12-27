import stylexRsCompiler from '@stylexswc/rs-compiler';
import type { Rule } from '@stylexjs/babel-plugin';
import { transform } from 'lightningcss';
import type { TransformOptions } from 'lightningcss';
import type { Plugin, PluginContext, TransformResult, TransformPluginContext } from 'rollup';
import browserslist from 'browserslist';
import { browserslistToTargets } from 'lightningcss';
import stylexBabelPlugin from '@stylexjs/babel-plugin';
import crypto from 'crypto';

import type { StyleXOptions } from '@stylexswc/rs-compiler';

const IS_DEV_ENV = process.env.NODE_ENV === 'development';

function replaceFileName(original: string, css: string) {
  if (!original.includes('[hash]')) {
    return original;
  }
  const hash = crypto.createHash('sha256').update(css).digest('hex').slice(0, 8);
  return original.replace(/\[hash\]/g, hash);
}

export type PluginOptions = {
  rsOptions?: StyleXOptions;
  fileName?: string;
  useCSSLayers?: boolean;
  lightningcssOptions?: Omit<TransformOptions<{}>, 'code' | 'filename' | 'visitor'>;
  extractCSS?: boolean;
};

export default function stylexPlugin({
  rsOptions = {},
  fileName = 'stylex.css',
  useCSSLayers = false,
  lightningcssOptions,
  extractCSS = true,
}: PluginOptions = {}): Plugin {
  const {
    dev = IS_DEV_ENV,
    unstable_moduleResolution = { type: 'commonJS', rootDir: process.cwd() },
    importSources = ['stylex', '@stylexjs/stylex'],
    ...options
  } = rsOptions;

  let stylexRules: Record<string, Rule[]> = {};
  return {
    name: 'rollup-plugin-stylex',
    buildStart() {
      stylexRules = {};
    },
    generateBundle(this: PluginContext) {
      const rules: Array<Rule> = Object.values(stylexRules).flat();
      if (rules.length > 0) {
        const collectedCSS = stylexBabelPlugin.processStylexRules(rules, useCSSLayers);

        // Process the CSS using lightningcss
        const { code } = transform({
          targets: browserslistToTargets(browserslist('>= 1%')),
          ...lightningcssOptions,
          filename: 'stylex.css',
          code: Buffer.from(collectedCSS),
        });

        // Convert the Buffer back to a string
        const processedCSS = code.toString();

        this.emitFile({
          fileName: replaceFileName(fileName, processedCSS),
          source: processedCSS,
          type: 'asset',
        });
      }
    },
    shouldTransformCachedModule({ code: _code, id, meta }) {
      stylexRules[id] = meta.stylex;
      return false;
    },
    async transform(
      this: TransformPluginContext,
      inputCode: string,
      id: string
    ): Promise<null | TransformResult> {
      if (
        !importSources.some(importName =>
          typeof importName === 'string'
            ? inputCode.includes(importName)
            : inputCode.includes(importName.from)
        )
      ) {
        // In rollup, returning null from any plugin phase means
        // "no changes made".
        return null;
      }

      let result = stylexRsCompiler.transform(id, inputCode, {
        ...options,
        dev,
        unstable_moduleResolution,
        importSources,
      });

      if (result == null) {
        console.warn('stylex: transformAsync returned null');
        return { code: inputCode };
      }
      const { code, map, metadata } = result;
      if (code == null) {
        console.warn('stylex: transformAsync returned null code');
        return { code: inputCode };
      }

      if (this.meta.watchMode) {
        const ast = this.parse(code);
        for (const stmt of ast.body) {
          if (stmt.type === 'ImportDeclaration') {
            const resolved = await this.resolve(stmt.source.value?.toString() || '', id);
            if (resolved && !resolved.external) {
              const result = await this.load(resolved);
              if (result && result.meta && 'stylex' in result.meta) {
                stylexRules[resolved.id] = result.meta.stylex;
              }
            }
          }
        }
      }

      if (extractCSS && !dev && metadata.stylex != null && metadata.stylex.length > 0) {
        stylexRules[id] = metadata.stylex;
      }

      return { code, map: map, meta: metadata };
    },
  };
}
