import type { StyleXOptions } from '@stylexswc/rs-compiler';

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
  useCSSLayers?: boolean;

  /** Glob patterns or RegExp to exclude files */
  exclude?: StyleXOptions['exclude'];
  /** Glob patterns or RegExp to include files */
  include?: StyleXOptions['include'];
  cwd?: string;
  isDev?: boolean;
}

export interface TransformOptions {
  shouldSkipTransformError?: boolean;
  isDev?: boolean;
}
