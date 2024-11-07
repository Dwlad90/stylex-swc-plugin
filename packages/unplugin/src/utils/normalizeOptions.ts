import type { UnpluginStylexRSOptions } from '../types';

const IS_DEV_ENV = process.env.NODE_ENV === 'development';

export default function normalizeOptions(
  options: UnpluginStylexRSOptions
): Required<UnpluginStylexRSOptions> {
  return {
    fileName: options.fileName ?? 'stylex.css',
    useCSSLayers: options.useCSSLayers ?? false,
    pageExtensions: options.pageExtensions ?? ['tsx', 'jsx', 'js', 'ts'],
    rsOptions: {
      ...options.rsOptions,
      dev: options.rsOptions?.dev ?? IS_DEV_ENV,
      unstable_moduleResolution: options.rsOptions?.unstable_moduleResolution ?? {
        type: 'commonJS',
        rootDir: process.cwd(),
      },
      importSources: options.rsOptions?.importSources ?? ['stylex', '@stylexjs/stylex'],
    },
    ...options,
  };
}
