import type { StyleXOptions } from '@stylexswc/rs-compiler';

export interface UnpluginStylexRSOptions {
  fileName?: string;
  useCSSLayers?: boolean;
  pageExtensions?: string[];
  rsOptions?: StyleXOptions;
  extractCSS?: boolean;
  /**
   * Enable CSS injection into CSS files via placeholder marker.
   * When enabled, the plugin will look for `@stylex;` marker in your CSS files
   * and replace it with the generated StyleX CSS.
   *
   * Example: Add `@stylex;` to your global.css file, then set this option to true.
   * The plugin will automatically replace the marker with the generated StyleX CSS.
   *
   * This allows StyleX CSS to go through the standard bundler CSS pipeline
   * (PostCSS, css-loader, LightningCSS, etc.) for processing.
   *
   * Works with all bundlers: Vite, Webpack, Rspack, esbuild, Rollup, Farm.
   */
  useCssPlaceholder?: boolean;
}
