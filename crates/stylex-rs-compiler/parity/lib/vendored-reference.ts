/**
 * The vendored copy of the reference `processStylexRules`.
 *
 * The addon answers `processStylexRules` natively when it can prove that its
 * sheet is the reference's sheet, and runs the reference itself in every other
 * case. That reference is not imported from `@stylexjs/babel-plugin` at run
 * time: the plugin is a development dependency only, and it pulls in Babel.
 * Instead, the functions are copied out of the published bundle, as they are,
 * into `src/process-stylex-rules/reference.ts`.
 *
 * The copy is generated, never edited by hand. One function is replaced:
 * `mediaQueryWidthSortKey` parses a media query with a parser that the
 * plugin bundles and does not publish, so the copy calls the addon's own
 * parser through a hook instead. The replaced source is kept in the copy as a
 * comment, so that a change to it upstream changes the copy too.
 *
 * `vendoredReference.spec.ts` renders the copy again from the installed plugin
 * and fails when the two differ, so that a new plugin version cannot go
 * unnoticed.
 */

import fs from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';

import * as babel from '@babel/core';

const require = createRequire(import.meta.url);

/** Where the copy lives, from the package directory. */
export const VENDORED_REFERENCE_FILE = 'src/process-stylex-rules/reference.ts';

/**
 * The top-level declarations the copy holds, in no particular order: the
 * copy keeps the order of the bundle.
 */
const COPIED_DECLARATIONS = new Set([
  'LOGICAL_FLOAT_START_VAR',
  'LOGICAL_FLOAT_END_VAR',
  'getLogicalFloatVars',
  'atRulePreludes',
  'widthSortKeyForChain',
  'declaredPropertyName',
  'compareWidthSortKeys',
  'processStylexRules',
  'addAncestorSelector',
  'addSpecificityLevel',
]);

/**
 * The declarations the hook replaces. `collectWidthBounds` reads the parsed
 * query, so it goes with the parser.
 */
const REPLACED_DECLARATIONS = new Set(['collectWidthBounds', 'mediaQueryWidthSortKey']);

/** What the copy puts in place of `mediaQueryWidthSortKey`. */
const WIDTH_SORT_KEY_HOOK = `function mediaQueryWidthSortKey(prelude) {
  return mediaQueryWidthBound(prelude);
}`;

/**
 * The globals the copy can read. `src/process-stylex-rules/built-ins.ts`
 * guards each of them; the sync test holds the two lists together.
 */
export const REFERENCE_GLOBALS: readonly string[] = [
  'Array',
  'Error',
  'Map',
  'Math',
  'Object',
  'Set',
  'String',
  'undefined',
];

const MIT_NOTICE = `/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */`;

/** The name a top-level declaration declares, if it declares one name. */
function declaredName(statement: babel.types.Statement): string | undefined {
  if (statement.type === 'FunctionDeclaration') {
    return statement.id?.name;
  }
  if (statement.type === 'VariableDeclaration' && statement.declarations.length === 1) {
    const [declarator] = statement.declarations;
    return declarator?.id.type === 'Identifier' ? declarator.id.name : undefined;
  }
  return undefined;
}

/** The source text of a node, as the bundle spells it. */
function sourceOf(source: string, node: babel.types.Node): string {
  if (node.start == null || node.end == null) {
    throw new Error('the bundle was parsed without source positions');
  }
  return source.slice(node.start, node.end);
}

/** Every global the code reads, by name. */
function globalsOf(code: string): string[] {
  const ast = babel.parseSync(code, { babelrc: false, configFile: false, sourceType: 'script' });
  if (ast == null) {
    throw new Error('the copy does not parse');
  }

  let globals: string[] = [];
  babel.traverse(ast, {
    Program(programPath) {
      globals = Object.keys(programPath.scope.globals);
    },
  });
  return globals;
}

/** The installed plugin's version and the path of its bundle. */
function installedPlugin(): { version: string; bundle: string } {
  const manifestPath = require.resolve('@stylexjs/babel-plugin/package.json');
  const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8')) as { version: string };

  return {
    version: manifest.version,
    bundle: path.join(path.dirname(manifestPath), 'lib/index.js'),
  };
}

/** The copy, rendered from the installed plugin. */
export function renderVendoredReference(): string {
  const { version, bundle } = installedPlugin();
  const source = fs.readFileSync(bundle, 'utf8');
  const ast = babel.parseSync(source, { babelrc: false, configFile: false, sourceType: 'script' });
  if (ast == null) {
    throw new Error(`cannot parse ${bundle}`);
  }

  const copied: string[] = [];
  const replaced: string[] = [];
  const found = new Set<string>();

  for (const statement of ast.program.body) {
    const name = declaredName(statement);
    if (name === undefined) {
      continue;
    }
    if (COPIED_DECLARATIONS.has(name)) {
      copied.push(sourceOf(source, statement));
      found.add(name);
    } else if (REPLACED_DECLARATIONS.has(name)) {
      replaced.push(sourceOf(source, statement));
      found.add(name);
      if (name === 'mediaQueryWidthSortKey') {
        copied.push(WIDTH_SORT_KEY_HOOK);
      }
    }
  }

  const missing = [...COPIED_DECLARATIONS, ...REPLACED_DECLARATIONS].filter(
    name => !found.has(name)
  );
  if (missing.length > 0) {
    throw new Error(`the bundle of ${version} has no top-level ${missing.join(', ')}`);
  }

  const body = copied.join('\n');
  const unknown = globalsOf(body).filter(
    name => name !== 'mediaQueryWidthBound' && !REFERENCE_GLOBALS.includes(name)
  );
  if (unknown.length > 0) {
    throw new Error(
      `the copied functions read ${unknown.join(', ')}, which the copy does not declare. ` +
        'Copy the declaration too, or add the global to REFERENCE_GLOBALS and guard it.'
    );
  }

  const replacedComment = replaced
    .join('\n')
    .split('\n')
    .map(line => `// ${line}`.trimEnd())
    .join('\n');

  return `// @ts-nocheck
// Generated by \`pnpm vendor:reference\` from @stylexjs/babel-plugin ${version}
// (lib/index.js). Do not edit by hand.
//
// The functions below are upstream's, as the published bundle holds them. The
// one change: \`mediaQueryWidthSortKey\` calls the addon's media query parser
// through \`mediaQueryWidthBound\`, because the bundle's parser is not
// published. The two upstream declarations it replaces follow, so that a change
// to them upstream changes this file:
//
${replacedComment}

${MIT_NOTICE}

/** The reference \`processStylexRules\`, with the addon's media query parser. */
export function createReferenceProcessStylexRules(
  mediaQueryWidthBound: (prelude: string) => { kind: string; value: number } | null
): (rules: unknown, config?: unknown) => string {
${body}
return processStylexRules;
}
`;
}
