import stylexBabelPlugin from '@stylexjs/babel-plugin';
import type { Rule } from '@stylexjs/babel-plugin';

export default function getStyleXRules(stylexRules: Record<string, Rule[]>, useCSSLayers: boolean) {
  const rules = Object.values(stylexRules).flat();

  if (!rules.length) {
    return null;
  }
  // Take styles for the modules that were included in the last compilation.
  const allRules = rules.filter((rule): rule is Rule => !!rule);

  return stylexBabelPlugin.processStylexRules(allRules, useCSSLayers);
}
