import type { StyleXOptions } from '@stylexswc/rs-compiler';

export interface StyleXPluginOption {
  /**
   * stylex options passed to stylex babel plugin
   *
   * @see https://stylexjs.com/docs/api/configuration/babel-plugin/
   */
  rsOptions?: Partial<StyleXOptions>;
  /**
   * Whether to use CSS layers
   *
   * @default false
   */
  useCSSLayers?: boolean;

  exclude?: string[];
  include?: string[];
  cwd?: string;
  isDev?: boolean;
}

export interface TransformOptions {
  shouldSkipTransformError?: boolean;
  isDev?: boolean;
}
