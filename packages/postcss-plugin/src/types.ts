import type { StyleXOptions, UseLayersType } from '@stylexswc/rs-compiler';

export interface StyleXPluginOption {
  /**
   * stylex options passed to stylex babel plugin
   *
   * @see https://stylexjs.com/docs/api/configuration/babel-plugin/
   */
  rsOptions?: Partial<Omit<StyleXOptions, 'exclude' | 'include'>>;
  /**
   * Whether to use CSS layers
   *
   * @default false
   */
  useCSSLayers?: UseLayersType;

  /** Glob patterns or RegExp to exclude files */
  exclude?: StyleXOptions['exclude'];
  /** Glob patterns or RegExp to include files */
  include?: StyleXOptions['include'];
  cwd?: string;
  isDev?: boolean;

  /**
   * Override import sources at the PostCSS plugin level.
   * When provided, takes precedence over rsOptions.importSources.
   * When omitted, falls back to rsOptions.importSources, then built-in defaults.
   */
  importSources?: Array<string | { from: string; as?: string }>;
}

export interface TransformOptions {
  shouldSkipTransformError?: boolean;
  isDev?: boolean;
}
