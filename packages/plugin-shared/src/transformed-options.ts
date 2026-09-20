// This module holds the builder and nothing else, and it imports only a type.
// A plugin can therefore reach it through
// `@stylexswc/plugin-shared/transformed-options` without loading the plugin
// core.

import type { TransformedOptions } from '@stylexswc/rs-compiler';

/**
 * The fields a plugin reads out of its own options to build
 * {@link TransformedOptions}. A caller can hold them on a compiler options
 * object or on its own normalized options, so the source is named by the
 * fields it must carry and not by the type it came from.
 *
 * Every member is already optional, because {@link TransformedOptions} is.
 */
type AssemblerOptionSource = Pick<
  TransformedOptions,
  'enableLTRRTLComments' | 'legacyDisableLayers' | 'useLegacyClassnamesSort'
>;

/**
 * The options the stylesheet assembler reads, taken from the options a plugin
 * was given.
 *
 * Every plugin that assembles a stylesheet needs the same four values, and
 * they arrive by two routes: `useLayers` is a parameter, because a plugin
 * works it out from its own configuration, and the other three are read off
 * {@link AssemblerOptionSource}. One builder keeps a new option from having to
 * be added to each plugin, which is how one of them would be left behind.
 */
export function toTransformedOptions(
  useLayers: TransformedOptions['useLayers'],
  source?: AssemblerOptionSource
): TransformedOptions {
  return {
    useLayers,
    enableLTRRTLComments: source?.enableLTRRTLComments,
    legacyDisableLayers: source?.legacyDisableLayers,
    useLegacyClassnamesSort: source?.useLegacyClassnamesSort,
  };
}
