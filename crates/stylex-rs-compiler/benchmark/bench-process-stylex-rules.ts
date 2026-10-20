/**
 * What `processStylexRules` costs, with its gate, against the reference.
 *
 * The export runs a JavaScript gate before the native sheet
 * (`src/process-stylex-rules/fast-path-gate.ts`): the gate reads every rule
 * once and packs it for the addon. This bench measures three subjects on the
 * rules of a large build: the reference from `@stylexjs/babel-plugin`, the
 * export as a consumer calls it, and the gate alone. It prints the median of
 * each, and the share of the export's time that the gate takes.
 *
 * Each subject is checked once before it is measured: the export must write
 * the reference's sheet.
 *
 * Usage:
 *   pnpm bench:process-stylex-rules
 */

import { createRequire } from 'node:module';

import { Bench } from 'tinybench';

import { processStylexRules, type StyleXRule } from '../dist/index.js';
import { nativeInputOf } from '../dist/process-stylex-rules/fast-path-gate.js';

const require = createRequire(import.meta.url);
const { processStylexRules: referenceProcessStylexRules } = require('@stylexjs/babel-plugin') as {
  processStylexRules: typeof processStylexRules;
};

/** Rules per build: a small application, a large one, and a very large one. */
const SIZES = [1_000, 10_000, 50_000];

const PROPERTIES = ['color', 'margin-top', 'padding-left', 'display', 'width'];
const PRIORITIES = [3000, 3500, 4000, 1000];

/** `size` rules as a build emits them: a fifth with an RTL text, a quarter in a media query. */
function rulesOf(size: number): StyleXRule[] {
  return Array.from({ length: size }, (_, index): StyleXRule => {
    const className = `x${index.toString(36)}`;
    const property = PROPERTIES[index % PROPERTIES.length];
    const declaration = `.${className}{${property}:${index % 13}px}`;
    const ltr =
      index % 4 === 0 ? `@media (min-width: ${(index % 7) * 100}px){${declaration}}` : declaration;
    const rtl = index % 5 === 0 ? ltr.replace('px', 'em') : null;

    return [className, { ltr, rtl }, PRIORITIES[index % PRIORITIES.length] ?? 3000];
  });
}

const bench = new Bench({ time: 1_000, warmup: true });

for (const size of SIZES) {
  const rules = rulesOf(size);
  if (processStylexRules(rules, true) !== referenceProcessStylexRules(rules, true)) {
    throw new Error(`the export and the reference write different sheets for ${size} rules`);
  }
  if (nativeInputOf(rules, true, 'all') === null) {
    throw new Error(`the gate leaves ${size} rules to the reference`);
  }

  bench
    .add(`reference ${size}`, () => referenceProcessStylexRules(rules, true))
    .add(`export ${size}`, () => processStylexRules(rules, true))
    .add(`gate ${size}`, () => nativeInputOf(rules, true, 'all'));
}

await bench.run();

const medianOf = (name: string): number => {
  const result = bench.getTask(name)?.result;
  if (result?.state !== 'completed') {
    throw new Error(`no result for ${name}`);
  }
  return result.latency.p50;
};

for (const size of SIZES) {
  const reference = medianOf(`reference ${size}`);
  const exported = medianOf(`export ${size}`);
  const gate = medianOf(`gate ${size}`);

  console.log(
    `${String(size).padStart(6)} rules: reference ${reference.toFixed(2)} ms, ` +
      `export ${exported.toFixed(2)} ms (${(reference / exported).toFixed(1)}x faster), ` +
      `gate ${gate.toFixed(2)} ms (${((gate / exported) * 100).toFixed(0)}% of the export)`
  );
}
