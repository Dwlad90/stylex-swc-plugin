import { normalizeRsOptions } from '@stylexswc/rs-compiler';

import type { UnpluginStylexRSOptions } from '../types';

const DEFAULT_CSS_PLACEHOLDER = '@stylex;';

export type NormalizedOptions = Omit<Required<UnpluginStylexRSOptions>, 'useCssPlaceholder'> & {
  useCssPlaceholder: string | false;
};

export default function normalizeOptions(options: UnpluginStylexRSOptions): NormalizedOptions {
  let useCssPlaceholder: NormalizedOptions['useCssPlaceholder'] = false;

  if (options.useCssPlaceholder === true) {
    useCssPlaceholder = DEFAULT_CSS_PLACEHOLDER;
  } else if (typeof options.useCssPlaceholder === 'string') {
    const useCssPlaceholderTrimmed = options.useCssPlaceholder.trim();

    if (useCssPlaceholderTrimmed) {
      useCssPlaceholder = useCssPlaceholderTrimmed;
    }
  }

  return {
    fileName: options.fileName ?? 'stylex.css',
    useCSSLayers: options.useCSSLayers ?? false,
    pageExtensions: options.pageExtensions ?? ['tsx', 'jsx', 'js', 'ts'],
    rsOptions: normalizeRsOptions(options.rsOptions || {}),
    extractCSS: options.extractCSS ?? true,
    useCssPlaceholder,
  };
}
