import stylexBabelPlugin from '@stylexjs/babel-plugin';
import type { Rule } from '@stylexjs/babel-plugin';
import type { TransformedOptions } from '@stylexswc/rs-compiler';

export default function getStyleXRules(
  stylexRules: Record<string, Rule[]>,
  transformedOptions: TransformedOptions
) {
  const rules = Object.values(stylexRules).flat();

  if (!rules.length) {
    return null;
  }
  // Take styles for the modules that were included in the last compilation.
  const allRules = rules.filter((rule): rule is Rule => !!rule);

  const processed = stylexBabelPlugin.processStylexRules(allRules, transformedOptions);
  if (!processed) {
    return null;
  }

  // Guard against malformed selectors which are rejected by Lightning CSS and
  // can intermittently appear in aggregated rule output.
  let sanitized = processed.replace(/(^|\n)\s*\{[^{}]*\}\s*(?=\n|$)/g, '$1');

  // Remove unsupported var()-selector wrapper blocks (e.g. `var(--x){.a{...}}`).
  // These come from unresolved at-rule-like tokens and are invalid CSS selectors.
  sanitized = sanitized.replace(/^\s*var\(--[^)]+\)\{.*\}\s*$/gm, '');

  return sanitized.trim() ? sanitized : null;
}
