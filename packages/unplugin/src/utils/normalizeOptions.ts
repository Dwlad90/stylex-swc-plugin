import { normalizeRsOptions } from '@stylexswc/rs-compiler';
import type { TransformedOptions } from '@stylexswc/rs-compiler';

import type { UnpluginStylexRSOptions } from '../types';

const DEFAULT_CSS_PLACEHOLDER = '@stylex;';

export type NormalizedOptions = Omit<
  Required<UnpluginStylexRSOptions>,
  'useCssPlaceholder' | 'useCSSLayers' | 'enableLTRRTLComments' | 'legacyDisableLayers'
> &
  TransformedOptions & {
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

  const normalizedRsOptions = normalizeRsOptions(options.rsOptions || {});

  return {
    fileName: options.fileName ?? 'stylex.css',
    useLayers: options.useCSSLayers,
    pageExtensions: options.pageExtensions ?? ['tsx', 'jsx', 'js', 'ts'],
    rsOptions: normalizedRsOptions,
    extractCSS: options.extractCSS ?? true,
    useCssPlaceholder,
    enableLTRRTLComments: normalizedRsOptions.enableLTRRTLComments,
    legacyDisableLayers: normalizedRsOptions.legacyDisableLayers,
  };
}
