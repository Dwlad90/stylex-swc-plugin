import nativeBinding from '../../dist/transform';
import type { StyleXMetadata } from '../../dist/transform';
import { builtInsAreIntact } from './built-ins.js';
import { type NativeCollation, nativeCollationOf } from './collation.js';
import { nativeInputOf } from './fast-path-gate.js';
import { createReferenceProcessStylexRules } from './reference.js';

/** One emitted rule, a row of `StyleXMetadata['stylex']`: the reference's `Rule`. */
export type StyleXRule = StyleXMetadata['stylex'][number];

/**
 * The `useLayers` object form: layers around the priority layers, and a prefix
 * for their names. One shape for `processStylexRules` and for the plugins'
 * `useCSSLayers` option (`UseLayersType` in `src/index.ts`).
 */
export interface ProcessStylexRulesLayerOptions {
  readonly before?: ReadonlyArray<string>;
  readonly after?: ReadonlyArray<string>;
  readonly prefix?: string;
}

/**
 * The options of `processStylexRules`, as `@stylexjs/babel-plugin` spells
 * them. A superset of the type that package publishes in its `index.d.ts`
 * (0.19.1): `useLayers` also takes `null` (read as absent, as the runtime
 * reads it), and `useLegacyClassnamesSort` is declared, which the reference's
 * Flow source reads but its `.d.ts` omits.
 */
export interface ProcessStylexRulesConfig {
  readonly useLayers?: boolean | ProcessStylexRulesLayerOptions | null;
  readonly enableLTRRTLComments?: boolean;
  readonly legacyDisableLayers?: boolean;
  readonly useLegacyClassnamesSort?: boolean;
}

/** Upstream's `processStylexRules`, with the addon's media query parser. */
const referenceProcessStylexRules = createReferenceProcessStylexRules(
  nativeBinding.mediaQueryWidthBound
);

/** Read on the first call: the default locale and ICU of a process are fixed. */
let collation: NativeCollation | undefined;

/**
 * The stylesheet for the rules a build's `transform` calls emitted, as
 * `@stylexjs/babel-plugin`'s `processStylexRules` writes it, byte for byte,
 * for every input: constants resolved and substituted, the rules sorted and
 * grouped by priority, each group a `@layer` or carrying the specificity
 * polyfill, RTL twins under `html[dir='rtl']` and its negation or between
 * `@ltr`/`@rtl` comments. An input on which the reference throws throws the
 * reference's error.
 *
 * The addon writes the sheet when it can prove that its sheet is the
 * reference's: the arguments are plain values in the shapes a build emits
 * (`fast-path-gate.ts`), the built-ins the reference calls are intact
 * (`built-ins.ts`), and the default collation is the one the addon sorts with
 * (`collation.ts`). In every other case, and wherever the reference throws,
 * the vendored copy of the reference runs (`reference.ts`).
 *
 * `rules` is the concatenation of every file's `metadata.stylex`, in the order
 * the files were compiled; `config` is the reference's, a boolean standing for
 * `useLayers` or the options object.
 *
 * The signature is a superset of the one `@stylexjs/babel-plugin` 0.19.1
 * publishes: `rules` is `readonly` where the reference says `Array<Rule>`,
 * `config` also takes `null` (read as absent, as the runtime reads it), and
 * the tuple type is named `StyleXRule` here, `Rule` there. Every call that
 * typechecks against the reference typechecks here.
 */
export function processStylexRules(
  rules: readonly StyleXRule[],
  config?: boolean | ProcessStylexRulesConfig | null
): string {
  if (builtInsAreIntact()) {
    collation ??= nativeCollationOf(nativeBinding.collationDataVersion());
    const input = nativeInputOf(rules, config, collation);

    if (input !== null) {
      const sheet = nativeBinding.processStylexRulesNative(
        input.texts,
        input.lengths,
        input.kinds,
        input.priorities,
        input.config
      );
      // `null` where the reference throws, or where the sheet holds text that
      // Rust cannot hold.
      if (sheet != null) {
        return sheet;
      }
    }
  }

  return referenceProcessStylexRules(rules, config);
}
