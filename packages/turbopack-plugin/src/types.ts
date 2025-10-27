import type { StyleXOptions } from '@stylexswc/rs-compiler';
import type { LoaderContext } from 'webpack';

type AsyncFnParams = Parameters<ReturnType<LoaderContext<unknown>['async']>>;

export type InputCode = AsyncFnParams['1'];
export type SourceMap = AsyncFnParams['2'];

export type StyleXTurbopackLoaderOptions = {
  stylexImports: StyleXOptions['importSources'];
  rsOptions: Partial<StyleXOptions>;
};
