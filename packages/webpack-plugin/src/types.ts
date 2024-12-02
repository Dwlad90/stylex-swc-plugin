import type { StyleXOptions } from '@stylexswc/rs-compiler';
import type { LoaderContext } from 'webpack';

type AsyncFnParams = Parameters<ReturnType<LoaderContext<unknown>['async']>>;

export type InputCode = AsyncFnParams['1'];
export type SourceMap = AsyncFnParams['2'];


export type CSSTransformer = (css: string) => string | Buffer | Promise<string | Buffer>;
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
}