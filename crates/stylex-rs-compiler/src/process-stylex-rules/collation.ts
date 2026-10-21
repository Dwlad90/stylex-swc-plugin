/**
 * The guard on the collation that the reference sorts with.
 *
 * The reference sorts with `localeCompare` and no locale, so its order is the
 * collation of the default locale of this process, with the collation data of
 * the ICU that this Node ships. The addon sorts with root collation and with
 * the ICU4X data it was built with (`stylex_stylesheet`'s `collation.rs`).
 * The two agree only when the default locale has no tailoring of its own, and,
 * for text that is not printable ASCII, only when the two ICU data versions
 * are the same.
 */

import { freshRealm, freshReflect } from './built-ins.js';

/** How much of the text the native sort can compare as `localeCompare` does here. */
export type NativeCollation =
  /** Every text. */
  | 'all'
  /** Printable ASCII only: the ICU data versions differ. */
  | 'printable-ascii'
  /** No text: the default locale has a tailoring, or options that are not the defaults. */
  | 'none';

/**
 * The languages whose collation is the root collation, with no tailoring in
 * CLDR. `processStylexRulesGuards.spec.ts` compares each with root collation
 * over a corpus.
 */
export const ROOT_COLLATION_LANGUAGES: readonly string[] = ['en', 'und'];

/** The options of a default `Intl.Collator`, as `resolvedOptions` reports them. */
const DEFAULT_OPTIONS = {
  usage: 'sort',
  sensitivity: 'variant',
  ignorePunctuation: false,
  collation: 'default',
  numeric: false,
  caseFirst: 'false',
} as const;

/** The ICU data the addon was built with. */
export interface CollationDataVersion {
  /** The ICU major version, as in `process.versions.icu` before the dot. */
  readonly icu: string;
  /** The Unicode version, as in `process.versions.unicode`. */
  readonly unicode: string;
  /** The CLDR major version, as in `process.versions.cldr` before the dot. */
  readonly cldr: string;
}

/** The part of a realm that the collation guard reads. */
export interface CollationRealm {
  readonly Intl?: typeof Intl;
}

/**
 * The language and the options of the default collation, or `null` when this
 * runtime has no `Intl` that can tell them. A Node built without ICU has no
 * `Intl`, and an unknown locale tag makes `new Locale(...)` throw.
 */
function defaultCollationOf(
  realm: CollationRealm | undefined
): { language: string; options: Intl.ResolvedCollatorOptions } | null {
  try {
    const realmIntl = (realm ?? freshRealm()).Intl;
    if (realmIntl === undefined) {
      return null;
    }
    const options = new realmIntl.Collator().resolvedOptions();
    return { language: new realmIntl.Locale(options.locale).language, options };
  } catch {
    return null;
  }
}

/**
 * How much text the native sort can compare, for this process. The default
 * locale is read through the fresh realm's `Intl`, because a program can
 * replace `Intl.Collator` without any change to `localeCompare`. A test can
 * pass a realm of its own.
 */
export function nativeCollationOf(
  dataVersion: CollationDataVersion,
  versions: NodeJS.ProcessVersions = process.versions,
  realm?: CollationRealm
): NativeCollation {
  const collation = defaultCollationOf(realm);
  if (collation === null) {
    return 'none';
  }

  // An indexed loop, for the reason the module doc of `built-ins.ts` gives.
  const names = freshReflect().ownKeys(DEFAULT_OPTIONS);
  for (let index = 0; index < names.length; index++) {
    const name = names[index] as keyof typeof DEFAULT_OPTIONS;
    if (collation.options[name] !== DEFAULT_OPTIONS[name]) {
      return 'none';
    }
  }
  if (!ROOT_COLLATION_LANGUAGES.includes(collation.language)) {
    return 'none';
  }

  const sameData =
    versions.icu?.split('.')[0] === dataVersion.icu &&
    versions.unicode === dataVersion.unicode &&
    versions.cldr?.split('.')[0] === dataVersion.cldr;
  return sameData ? 'all' : 'printable-ascii';
}
