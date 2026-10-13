import { processStylexRules } from '@stylexswc/rs-compiler';
import type { StyleXRule as Rule, TransformedOptions } from '@stylexswc/rs-compiler';

export default function getStyleXRules(
  stylexRules: Record<string, Rule[]>,
  transformedOptions: TransformedOptions
): string | null {
  const rules = Object.values(stylexRules).flat();

  if (!rules.length) {
    return null;
  }
  // Take styles for the modules that were included in the last compilation.
  const allRules = rules.filter((rule): rule is Rule => !!rule);

  return processStylexRules(allRules, transformedOptions);
}
