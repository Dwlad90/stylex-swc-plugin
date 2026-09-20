import type { TransformedOptions } from '@stylexswc/rs-compiler';

/**
 * The fields a plugin reads out of its own options to build
 * {@link TransformedOptions}. A caller can hold them on a compiler options
 * object or on its own normalized options, so the source is named by the
 * fields it must carry and not by the type it came from.
 */
type AssemblerOptionSource = Pick<
  TransformedOptions,
  'enableLTRRTLComments' | 'legacyDisableLayers' | 'useLegacyClassnamesSort'
>;

/**
 * The options the stylesheet assembler reads, taken from the options a plugin
 * was given.
 *
 * Every plugin that assembles a stylesheet needs the same four values. One
 * builder keeps a new option from having to be added to each of them, which is
 * how one of them would be left behind.
 *
 * This module holds nothing but the builder, and imports only a type, so a
 * plugin can reach it through `@stylexswc/plugin-shared/transformed-options`
 * without loading the plugin core.
 */
export function toTransformedOptions(
  useLayers: TransformedOptions['useLayers'],
  source?: Partial<AssemblerOptionSource>
): TransformedOptions {
  return {
    useLayers,
    enableLTRRTLComments: source?.enableLTRRTLComments,
    legacyDisableLayers: source?.legacyDisableLayers,
    useLegacyClassnamesSort: source?.useLegacyClassnamesSort,
  };
}
