// The fallback of `processStylexRules` is upstream's own code, copied out of
// the published plugin into `src/process-stylex-rules/reference.ts`. These
// tests keep the copy honest: it is what the generator writes from the
// installed plugin, and with the addon's media query parser in place of the
// bundled one it writes the plugin's sheets.
import * as fs from 'node:fs';
import * as path from 'node:path';

import { describe, expect, test } from 'vitest';

import type { ProcessStylexRulesConfig, StyleXRule } from '../dist/index.js';
import { createReferenceProcessStylexRules } from '../dist/process-stylex-rules/reference.js';
import { mediaQueryWidthBound } from '../dist/transform.js';
import { loadBabelPlugin } from '../parity/lib/compilers.js';
import { PROCESS_STYLEX_RULES_CASES, stylesheetFixturesDir } from '../parity/lib/fixtures.js';
import { withLfEndings } from '../parity/lib/text.js';
import {
  renderVendoredReference,
  VENDORED_REFERENCE_FILE,
} from '../parity/lib/vendored-reference.js';

interface RecordedCase {
  name: string;
  calls: Array<{
    rules: StyleXRule[];
    config: boolean | ProcessStylexRulesConfig | null;
    output: string;
  }>;
}

const packageDir = path.resolve(__dirname, '..');
const reference = createReferenceProcessStylexRules(mediaQueryWidthBound);
const { processStylexRules: babelProcessStylexRules } = loadBabelPlugin();
const fixture = JSON.parse(
  fs.readFileSync(path.join(stylesheetFixturesDir(packageDir), PROCESS_STYLEX_RULES_CASES), 'utf8')
) as { cases: RecordedCase[] };

/** The rules of one `@media` rule per prelude, all for one property. */
function rulesFor(preludes: readonly string[]): StyleXRule[] {
  return preludes.map((prelude, index) => [
    `x${index}`,
    { ltr: `${prelude}{.x${index}{width:${index}px}}`, rtl: null },
    3200,
  ]);
}

describe('the vendored reference', () => {
  test('is what the generator writes from the installed plugin', () => {
    const committed = withLfEndings(
      fs.readFileSync(path.join(packageDir, VENDORED_REFERENCE_FILE), 'utf8')
    );

    // Run `pnpm vendor:reference` when this fails after a plugin bump.
    expect(committed).toBe(renderVendoredReference());
  });

  for (const recorded of fixture.cases) {
    test(`writes the recorded sheet: ${recorded.name}`, () => {
      for (const call of recorded.calls) {
        expect(reference(call.rules, call.config)).toBe(call.output);
      }
    });
  }

  test('sorts by the bound the plugin reads from each media query', () => {
    const preludes = [
      '@media (min-width: 768px)',
      '@media (max-width: 1024px)',
      '@media (min-width: 0)',
      '@media (max-width: 40em)',
      '@media (min-width: 500px) and (min-width: 900px)',
      '@media (min-width: 500px) and (not (max-width: 1000px))',
      '@media not all and (min-width: 300px)',
      '@media (MIN-WIDTH: 300PX)',
      '@media (min-width: 300px) or (max-width: 100px)',
      '@media (orientation: landscape)',
      // A lone surrogate reaches the addon's parser as U+FFFD; both preludes
      // must have no bound, as they have none for the plugin's parser.
      '@media (min-width: \uD800px)',
      '@media (min-width: �px)',
      '@media (min-width: 10px) \uD800',
      '@media �',
    ];
    const rules = rulesFor(preludes);

    expect(reference(rules)).toBe(babelProcessStylexRules(rules));
    expect(reference([...rules].reverse())).toBe(babelProcessStylexRules([...rules].reverse()));
  });

  test('reads a media query the parser rejects as one with no bound', () => {
    expect(mediaQueryWidthBound('@media (min-width: 768px)')).toEqual({ kind: 'min', value: 768 });
    expect(mediaQueryWidthBound('@media (max-width: 0)')).toEqual({ kind: 'max', value: 0 });
    expect(mediaQueryWidthBound('@media (min-width:')).toBeNull();
    expect(mediaQueryWidthBound('')).toBeNull();
  });
});
