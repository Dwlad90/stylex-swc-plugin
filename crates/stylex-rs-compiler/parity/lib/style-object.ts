/**
 * The style objects a compiled module carries, as comparable text.
 *
 * A compiler's answer about a style value has two halves. One is the CSS, which
 * the rule text and the class name hash already cover. The other is the *style
 * object* — the `{ kMwMTN: "x1e2nbdu", $$css: true }` literal a `stylex.create`
 * call compiles to — and it carries something the CSS cannot: a property whose
 * value is `null`. That is how an absent value is spelled, and it is what makes
 * merging a later style unset an earlier declaration of the same property
 * rather than shadow it. A property that is missing from the object entirely
 * says something different, and emits exactly the same CSS: none.
 *
 * So a corpus that reads only rule text cannot ask what a `null` value does. It
 * reports agreement on every such entry, because both compilers emitted no
 * rules, whether or not they agreed about the key.
 *
 * This reads the shape and nothing else. It is deliberately *not* a comparison
 * of the emitted JavaScript: the two compilers print code differently — which
 * declarations they leave standing, how they wrap an injection — and comparing
 * that would report a divergence on every entry and say nothing about StyleX.
 * What is extracted is only which keys a style object has and, per key, whether
 * it carries a class name or an absence.
 */

import * as babel from '@babel/core';

/** A style object is marked by this property; nothing else in the output is. */
const MARKER = '$$css';

/**
 * The style objects in one compiled module, in source order, each rendered as
 * canonical text.
 *
 * A class name is replaced by a placeholder rather than printed: the names
 * themselves are already compared as `classNames`, and printing them here would
 * report the same hash divergence twice while hiding a key-set divergence
 * behind it. `null` is printed as itself, because that is the thing being
 * measured.
 */
export function styleObjectsOf(code: string): string[] {
  const objects: string[] = [];

  // A parse failure is not an outcome worth failing the run over: it can only
  // mean this function was handed something other than the compiler's own
  // output, and reporting no style objects makes that show up as a divergence
  // to look at rather than as a crash in the middle of a corpus run.
  let ast: babel.types.File | null;
  try {
    ast = babel.parseSync(code, {
      babelrc: false,
      configFile: false,
      parserOpts: { sourceType: 'module', plugins: ['jsx'] },
    });
  } catch {
    return objects;
  }
  if (ast == null) return objects;

  babel.traverse(ast, {
    ObjectExpression(nodePath) {
      const rendered = renderStyleObject(nodePath.node);
      if (rendered !== null) objects.push(rendered);
    },
  });

  return objects;
}

/** One object literal as canonical text, or `null` if it is not a style object. */
function renderStyleObject(node: babel.types.ObjectExpression): string | null {
  const fields: string[] = [];
  let isStyleObject = false;

  for (const property of node.properties) {
    if (property.type !== 'ObjectProperty') return null;

    const key = keyOf(property.key);
    if (key === null) return null;

    if (key === MARKER) {
      isStyleObject = true;
      continue;
    }

    fields.push(`${JSON.stringify(key)}:${valueOf(property.value)}`);
  }

  return isStyleObject ? `{${fields.join(',')}}` : null;
}

function keyOf(key: babel.types.ObjectProperty['key']): string | null {
  if (key.type === 'Identifier') return key.name;
  if (key.type === 'StringLiteral') return key.value;
  return null;
}

/**
 * What a style-object entry carries, at the only granularity that is this
 * harness's business: an absence, a class name, or something neither.
 */
function valueOf(value: babel.types.ObjectProperty['value']): string {
  if (value.type === 'NullLiteral') return 'null';
  if (value.type === 'StringLiteral') return 'class';
  // A dynamic style compiles its value to an expression rather than a literal.
  // Which expression is a fact about the generated JavaScript, so it is
  // recorded as one word.
  return 'expression';
}
