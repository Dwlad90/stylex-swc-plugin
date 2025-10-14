import type { StyleXOptions } from '@stylexswc/rs-compiler';

export interface UnpluginStylexRSOptions {
  fileName?: string;
  useCSSLayers?: boolean;
  pageExtensions?: string[];
  rsOptions?: StyleXOptions;
  extractCSS?: boolean;
  /**
   * Use Vite's CSS pipeline for processing StyleX CSS.
   * When enabled, StyleX CSS will be processed through Vite's transforms (LightningCSS, PostCSS, etc.)
   * instead of being directly injected into HTML.
   * @default false
   */
  useViteCssPipeline?: boolean;
}
