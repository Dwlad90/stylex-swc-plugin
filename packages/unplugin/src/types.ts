import type { StyleXOptions } from '@stylexswc/rs-compiler';

export interface UnpluginStylexRSOptions {
  fileName?: string;
  useCSSLayers?: boolean;
  pageExtensions?: string[];
  rsOptions?: StyleXOptions;
  extractCSS?: boolean;
  /**
   * Enable CSS injection into CSS files via placeholder marker.
   *
   * - When set to `true`, the plugin will look for the default `@stylex;` marker
   * - When set to a string, the plugin will use that string as the marker
   *
   * Example with default marker:
   * ```css
   * @stylex;
   * ```
   * ```js
   * { useCssPlaceholder: true }
   * ```
   *
   * Example with custom marker:
   * ```css
   * /* INJECT_STYLEX_HERE *\/
   * ```
   * ```js
   * { useCssPlaceholder: '/* INJECT_STYLEX_HERE *\/' }
   * ```
   *
   * This allows StyleX CSS to go through the standard bundler CSS pipeline
   * (PostCSS, css-loader, LightningCSS, etc.) for processing.
   *
   * Works with all bundlers: Vite, Webpack, Rspack, esbuild, Rollup, Farm.
   */
  useCssPlaceholder?: boolean | string;
}
