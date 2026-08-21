// The seam between type stripping and the StyleX transform, which no Rust test
// can reach: `cargo test` runs the resolver and then the transform, while this
// pipeline runs a type-stripping pass between the two. That pass elides an
// import specifier nothing references as a value, and a dynamic style's
// parameter shadowing an imported name is not a reference — so for a JavaScript
// input the specifier used to be gone before the transform could register it,
// and a module the reference implementation refuses compiled to a runtime value
// instead.
//
// What is asked here is only what the boundary decides: which specifiers
// survive the strip, and what the emitted module carries as a result. That the
// fold itself refuses, and what it refuses beside hostile CSS, is
// `stylex-transform`'s question and is pinned far more cheaply under
// `cargo nextest` in `validation_stylex_create_test::invalid_values`.
//
// Every refusal here is the reference implementation's own sentence, read off
// `@stylexjs/babel-plugin` run over the same source rather than written by eye.
import { describe, expect, test } from 'vitest';

import { transform } from '../dist/index.js';

const JAVASCRIPT_EXTENSIONS = ['js', 'jsx', 'mjs', 'cjs'] as const;
const TYPESCRIPT_EXTENSIONS = ['ts', 'tsx', 'mts', 'cts'] as const;

/** The sentence the reference implementation refuses the fold with. */
const REFUSES_THE_FOLD = '[StyleX] Invalid pseudo or at-rule.';

function compile(code: string, extension: string = 'js'): string {
  return transform(`/abs/path/page.${extension}`, code, {
    unstable_moduleResolution: { type: 'commonJS' },
  }).code;
}

/**
 * The reported module: a dynamic style whose parameter shadows a named StyleX
 * import, with nothing else referencing that import. Only a *named* specifier
 * can reach this state — a default or namespace import of `@stylexjs/stylex` is
 * referenced by the `stylex.create` call itself, so nothing elides it.
 */
function shadowing(name: string, local: string = name): string {
  const specifier = local === name ? name : `${name} as ${local}`;

  return [
    `import { create, ${specifier} } from '@stylexjs/stylex';`,
    '',
    'export const styles = create({',
    `  dyn: (${local}) => ({ height: ${local} }),`,
    '});',
    '',
  ].join('\n');
}

describe('a shadowed StyleX import in a JavaScript module', () => {
  test.each(JAVASCRIPT_EXTENSIONS)('the reported module refuses in a .%s file', extension => {
    expect(() => compile(shadowing('keyframes'), extension)).toThrow(REFUSES_THE_FOLD);
  });

  // `keyframes` folds to `{ fn }` and `when` to the marker object, so the two
  // exercise both materializations the fold has. Their siblings —
  // `firstThatWorks`, `positionTry` — vary the registration and not the
  // elision, and are covered where the fold is.
  test.each(['keyframes', 'when'])('a parameter shadowing %s refuses', name => {
    expect(() => compile(shadowing(name))).toThrow(REFUSES_THE_FOLD);
  });

  // The strip decides usage per local binding, so a renamed specifier is a
  // separate question from the one it was renamed from.
  test('an aliased specifier survives the strip too', () => {
    expect(() => compile(shadowing('keyframes', 'kf'))).toThrow(REFUSES_THE_FOLD);
  });

  test('a non-ASCII local name survives the strip too', () => {
    expect(() => compile(shadowing('keyframes', 'кадры'))).toThrow(REFUSES_THE_FOLD);
  });

  // `defaultMarker` is the one entry the reference implementation registers as
  // a bare function rather than as the wrapper `{ fn }`, so it refuses as an
  // illegal value rather than as a namespace. Both sentences are upstream's,
  // and which one an entry earns is decided by how it is registered.
  test('a parameter shadowing defaultMarker refuses as an illegal value', () => {
    expect(() => compile(shadowing('defaultMarker'))).toThrow(
      '[StyleX] A style value can only contain an array, string or number.'
    );
  });

  test('a lone surrogate refuses at the boundary instead of at the fold', () => {
    // The one shape here that does not read the reference implementation's
    // sentence, and not because of the elision: an unpaired surrogate is not
    // valid UTF-8, so the string never reaches StyleX. Upstream, which never
    // leaves JavaScript, reads it as a condition key and refuses on the fold.
    const source = [
      "import { create, keyframes } from '@stylexjs/stylex';",
      "export const styles = create({ dyn: (keyframes) => ({ '\\uD800': { height: keyframes } }) });",
      '',
    ].join('\n');

    expect(() => compile(source)).toThrow('[StyleX] String value contains invalid UTF-8 encoding.');
  });
});

describe('names the elision must not have swallowed either way', () => {
  // Neither compiler registers `types` for a create call, so nothing folds and
  // the parameter stands. It used to compile here for a different reason — a
  // deopt landing in the same place — and must keep compiling now that the
  // entries beside it refuse.
  test('a parameter shadowing types still compiles to an inline style', () => {
    expect(compile(shadowing('types'))).toContain('--x-height');
  });

  test('the specifier it shadows survives into the emitted module', () => {
    // The visible half of the choice: in JavaScript an unreferenced specifier
    // is not a fiction to remove.
    expect(compile(shadowing('types'))).toContain(
      "import { create, types } from '@stylexjs/stylex'"
    );
  });

  // A parameter shadowing a *theme* import used to compile because the import
  // was elided, so the import lookup had nothing to match. It now compiles
  // because the lookup compares bindings and a parameter is not the import —
  // which is the answer that was always meant. Both halves are asserted, so a
  // regression to the name match cannot hide behind the elision again.
  test.each([
    ['a named theme import', "import { zIndex } from 'zIndex.stylex.js';", 'zIndex'],
    ['a default theme import', "import tokens from 'tokens.stylex.js';", 'tokens'],
  ])('%s shadowed by a dynamic parameter compiles with the import kept', (_label, decl, name) => {
    const code = compile(
      [
        "import * as stylex from '@stylexjs/stylex';",
        decl,
        '',
        'export const styles = stylex.create({',
        `  dyn: (${name}) => ({ color: ${name} }),`,
        '});',
        '',
      ].join('\n')
    );

    expect(code).toContain('--x-color');
    expect(code).toContain('stylex.js');
  });
});

describe('a TypeScript module keeps the elision', () => {
  // A decided divergence, and the reason the two halves answer differently: in
  // TypeScript a specifier with no value reference may name a type, and a type
  // has no module to import at runtime, so removing it is the language's own
  // rule rather than this compiler's choice. The reference implementation never
  // strips before it reads and therefore refuses these. Pinned so the gap is
  // measured rather than assumed, and so closing it later reads as a change.
  test.each(TYPESCRIPT_EXTENSIONS)(
    'the reported module still compiles in a .%s file',
    extension => {
      expect(compile(shadowing('keyframes'), extension)).toContain('--x-height');
    }
  );

  test('an unreferenced specifier is still elided', () => {
    expect(compile("import { unused } from './m';\nexport const x = 1;\n", 'ts')).not.toContain(
      'unused'
    );
  });

  test('an extension no toolchain agrees on is answered as TypeScript', () => {
    // The conservative half: an elision only ever removes, so an unrecognised
    // name is answered the way the pipeline answered it before.
    expect(compile("import { unused } from './m';\nexport const x = 1;\n", 'vue')).not.toContain(
      'unused'
    );
  });
});

describe('what the choice does not change', () => {
  test('a JavaScript module keeps an import nothing references', () => {
    // Babel and esbuild both keep it; eliding it was TypeScript's rule reaching
    // a file that is not TypeScript.
    expect(compile("import { unused } from './m';\nexport const x = 1;\n")).toContain('unused');
  });

  test('a side-effect import is untouched', () => {
    expect(compile("import './m';\nexport const x = 1;\n")).toContain("import './m'");
  });

  // Every *explicitly* type-only form is still removed from a JavaScript
  // module. Only inference — "nothing references this, so it must have been a
  // type" — is what stops, so nothing TypeScript-shaped survives by accident.
  test.each([
    ['an `import type` statement', "import type { T } from './m';\nexport const x = 1;\n", 'T'],
    ['an inline type specifier', "import { type T, v } from './m';\nexport const x = v;\n", 'T'],
    ['a type annotation', 'export const x: number = 1;\n', 'number'],
    ['an interface', 'interface I { a: number }\nexport const x = 1;\n', 'interface'],
    ['a type alias', 'type A = number;\nexport const x = 1;\n', 'type A'],
    ['an `export type`', 'type A = number;\nexport type { A };\nexport const x = 1;\n', 'A'],
    ['an as-expression', 'export const x = (1 as number);\n', 'as number'],
  ])('%s is still stripped from a JavaScript module', (_label, source, removed) => {
    expect(compile(source)).not.toContain(removed);
  });

  test('an enum still becomes its runtime object', () => {
    expect(compile('export enum E { A }\n')).toContain('export var E');
  });

  test('a namespace still becomes its runtime object', () => {
    expect(compile('namespace N { export const a = 1; }\nexport const x = N.a;\n')).toContain(
      'N.a = 1'
    );
  });
});
