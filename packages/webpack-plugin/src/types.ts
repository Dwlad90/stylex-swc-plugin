import type { StyleXOptions } from '@stylexswc/rs-compiler';
import type { LoaderContext } from 'webpack';
import type webpack from 'webpack';
import type { RegisterStyleXRules } from '.';

type AsyncFnParams = Parameters<ReturnType<LoaderContext<unknown>['async']>>;

export type InputCode = AsyncFnParams['1'];
export type SourceMap = AsyncFnParams['2'];

export type CSSTransformer = (
  _css: string,
  _filePath: string | undefined
) => string | Buffer | Promise<string | Buffer>;

export interface StyleXPluginOption {
  /**
   * stylex options passed to stylex babel plugin
   *
   * @see https://stylexjs.com/docs/api/configuration/babel-plugin/
   */
  rsOptions?: Partial<StyleXOptions>;
  /**
   * Specify where stylex will be imported from
   *
   * @default ['stylex', '@stylexjs/stylex']
   */
  stylexImports?: StyleXOptions['importSources'];
  /**
   * Whether to use CSS layers
   *
   * @default false
   */
  useCSSLayers?: boolean;
  /**
   * Next.js Mode
   *
   * @default false
   */
  nextjsMode?: boolean;

  /**
   * Enable other CSS transformation
   *
   * Since @stylexswc/webpack-plugin only inject CSS after all loaders, you can not use postcss-loader.
   * With this you can incovate `postcss()` here.
   */
  transformCss?: CSSTransformer;

  /**
   * Whether to extract CSS
   *
   * @default true
   */
  extractCSS?: boolean;

  /**
   * Loader execution order
   *
   * Determines when the StyleX transformation is applied relative to other webpack loaders:
   * - 'first': StyleX processes source code before any other loaders (recommended)
   * - 'last': StyleX processes after all other loaders have run
   *
   * @default 'first'
   */
  loaderOrder?: 'first' | 'last';
}
export type StyleXWebpackLoaderOptions = {
  stylexImports: StyleXOptions['importSources'];
  rsOptions: Partial<StyleXOptions>;
  nextjsMode: boolean;
  extractCSS?: boolean;
};

export type SupplementedLoaderContext<Options = unknown> = webpack.LoaderContext<Options> & {
  StyleXWebpackContextKey: {
    registerStyleXRules: RegisterStyleXRules;
  };
};

export type SWCPluginRule = {
  class_name: string;
  style: { ltr: string; rtl?: null | string };
  priority: number;
};
