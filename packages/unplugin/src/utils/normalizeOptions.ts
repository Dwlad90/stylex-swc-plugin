import { INCLUDE_EXTENSIONS } from '@stylexswc/plugin-shared/constants';
import { normalizeRsOptions } from '@stylexswc/rs-compiler';
import type { TransformedOptions } from '@stylexswc/rs-compiler';

import type { UnpluginStylexRSOptions } from '../types';

const DEFAULT_CSS_PLACEHOLDER = '@stylex;';
export const identityTransformCss: NonNullable<UnpluginStylexRSOptions['transformCss']> = css =>
  css;

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
    // The shared list holds every extension that the compiler transforms. A
    // shorter default lets StyleX in the extensions it leaves out reach the
    // browser uncompiled.
    pageExtensions: options.pageExtensions ?? [...INCLUDE_EXTENSIONS],
    rsOptions: normalizedRsOptions,
    extractCSS: options.extractCSS ?? true,
    transformCss: options.transformCss ?? identityTransformCss,
    useCssPlaceholder,
    onMissingCssPlaceholder: options.onMissingCssPlaceholder ?? 'error',
    enableLTRRTLComments: normalizedRsOptions.enableLTRRTLComments,
    legacyDisableLayers: normalizedRsOptions.legacyDisableLayers,
  };
}
