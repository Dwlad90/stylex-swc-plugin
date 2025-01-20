import { normalizeRsOptions } from '@stylexswc/rs-compiler';

import type { UnpluginStylexRSOptions } from '../types';

export default function normalizeOptions(
  options: UnpluginStylexRSOptions
): Required<UnpluginStylexRSOptions> {
  return {
    ...options,
    fileName: options.fileName ?? 'stylex.css',
    useCSSLayers: options.useCSSLayers ?? false,
    pageExtensions: options.pageExtensions ?? ['tsx', 'jsx', 'js', 'ts'],
    rsOptions: normalizeRsOptions(options.rsOptions || {}),
    extractCSS: options.extractCSS ?? true,
  };
}
