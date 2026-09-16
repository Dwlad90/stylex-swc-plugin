// A `stylex.create` call written inside a type assertion is the one shape the
// transform crate refuses and a build compiles. The refusal is there because
// the printer drops the brackets an assertion needs; the shipped pipeline
// strips every type before the StyleX pass, so no build reaches it.
//
// The Rust suite holds the snapshots for every other position. Only the
// boundary can show this one, because only here do the two passes run in
// order. It compiles through the shipped native binding, so rebuild the
// binding before this suite means anything.
import { parseSync } from '@swc/core';
import { expect, test } from 'vitest';

import { transform } from '../dist/index.js';

const FILENAME = '/abs/path/page.tsx';

/** The compiled style object for `display: flex`. */
const COMPILED = 'k1xSpc: "x78zum5"';

const IMPORT = 'import * as stylex from "@stylexjs/stylex";';
const CREATE = 'stylex.create({ root: { display: "flex" } })';

function compile(source: string): string {
  return transform(FILENAME, source, {
    unstable_moduleResolution: { type: 'commonJS' },
  }).code;
}

test('a call inside a type assertion compiles, and the module printed parses', () => {
  const code = compile(`${IMPORT}\nexport const root = (${CREATE} as any).root;\n`);

  expect(code).toContain(COMPILED);
  // The assertion is gone with the rest of the types, so the brackets the
  // printer drops are no longer needed. A module that no parser reads is a
  // defect a `toContain` assertion cannot see.
  expect(code).not.toContain('as any');
  parseSync(code, { syntax: 'typescript', tsx: true });
});

test('a call nothing reads stops the build', () => {
  expect(() => compile(`${IMPORT}\n${CREATE};\n`)).toThrow(
    /create\(\) calls must be bound to a bare variable/
  );
});
