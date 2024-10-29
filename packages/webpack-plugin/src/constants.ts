import type { LoaderContext } from 'webpack';

export const PLUGIN_NAME = 'stylex';

export type SupplementedLoaderContext<Options = unknown> = LoaderContext<Options> & {
  // NOTE: Add any additional properties here
};
