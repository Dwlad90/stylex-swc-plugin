import { NEXTJS_COMPILER_NAMES, isNextJsCompilerName } from './constants';

import type { NextJsCompilerName } from './constants';
import type { Rule as StyleXRule } from '@stylexjs/babel-plugin';

export type StyleXRulesMap = Map<string, readonly StyleXRule[]>;

type NextJsGlobalRegistry = Map<NextJsCompilerName, StyleXRulesMap>;

const REGISTRY_KEY = '__stylexswc_nextjs_global_registry__';

type GlobalWithRegistry = typeof globalThis & {
  [REGISTRY_KEY]?: NextJsGlobalRegistry;
};

/**
 * Next.js App Router runs up to three compilers (client, server, edge-server)
 * in the same process, each with its own plugin instance. Server-only modules
 * (e.g. React Server Components) are never seen by the client compiler, so
 * their rules would be lost without a process-wide registry. Each compiler
 * publishes its rules map here; the client compiler merges them all before
 * emitting the final CSS asset.
 *
 * Requires `experimental.webpackBuildWorker` to be disabled — separate worker
 * processes don't share `globalThis`.
 */
export function publishStyleXRules(
  compilerName: string | undefined,
  rules: StyleXRulesMap
): void {
  if (!isNextJsCompilerName(compilerName)) {
    return;
  }

  const holder = globalThis as GlobalWithRegistry;

  holder[REGISTRY_KEY] ??= new Map();
  holder[REGISTRY_KEY].set(compilerName, rules);
}

export function mergeStyleXRulesInto(
  compilerName: string | undefined,
  target: StyleXRulesMap
): void {
  if (compilerName !== NEXTJS_COMPILER_NAMES.client) {
    return;
  }

  const registry = (globalThis as GlobalWithRegistry)[REGISTRY_KEY];

  registry?.forEach(rulesMap => {
    if (rulesMap === target) {
      return;
    }

    rulesMap.forEach((rules, key) => {
      target.set(key, rules);
    });
  });
}
