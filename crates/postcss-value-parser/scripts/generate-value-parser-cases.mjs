// Emits the expectation table for the vendored value scanner, read straight out
// of the JavaScript it stands in for rather than written from memory. The Rust
// has to reproduce that JavaScript quirk for quirk, and a quirk nobody
// remembered is exactly the one that changes a class-name hash, so every
// expectation here is a real answer from a real run.
//
// Three answers are recorded per input: the serialised output, a canonical dump
// of the node tree, and -- for every word the tree contains -- how `unit()`
// splits it. Between them they pin parse, stringify and unit-splitting.
//
// Regenerate with:
//
//   pnpm run --filter=@stylexswc/postcss-value-parser generate:value-parser-cases
//
// The inputs come from the parity corpus in `@stylexswc/rs-compiler`, which is
// itself generated from the Rust test sources -- so adding a test that carries a
// CSS value invalidates this fixture, in a different crate. The full chain:
//
//   Rust test sources
//     -> pnpm --filter=@stylexswc/rs-compiler parity:harvest
//          -> crates/stylex-rs-compiler/parity/corpus/harvested.json
//               -> this generator -> src/tests/cases.rs
//
// Row order here is corpus order, so anything that reorders the corpus rewrites
// this file wholesale. `generate:value-parser-cases:check` reports staleness
// without writing, and runs as part of this package's `test` script.

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import parser from 'postcss-value-parser';

// Bound rather than destructured: `stringify` is reached through the parser
// namespace, and pulling it off unbound is the shape the linter flags.
const stringify = parser.stringify.bind(parser);

const here = path.dirname(fileURLToPath(import.meta.url));
const corpusDir = path.resolve(here, '../../stylex-rs-compiler/parity/corpus');

/**
 * The differential harness's corpus: the reported symptoms, a hand-written edge
 * set, and every declaration harvested from the Rust test suites. Reusing it
 * means the scanner is measured against the same values the compiler is.
 */
function corpusValues() {
  const values = [];
  for (const set of ['reported', 'edge', 'harvested']) {
    const file = path.join(corpusDir, `${set}.json`);
    for (const entry of JSON.parse(fs.readFileSync(file, 'utf8')).entries) {
      values.push(entry.value);
    }
  }
  return values;
}

/**
 * Inputs the corpus has no reason to carry, because no author would write them:
 * syntax that is malformed, truncated, or degenerate. The parser is documented
 * as never failing, and this is where that claim is measured.
 */
const MALFORMED = [
  // Nothing at all.
  '',
  ' ',
  '\t\n\r\f',
  // Unclosed and unopened brackets.
  'calc(',
  'calc(1px',
  'calc((1px)',
  'calc(1px))',
  ')',
  '))((',
  '(',
  '((',
  '(((',
  // A parenthesis with no name in front of it, holding something that runs off
  // the end. Both push a span past the end of the input -- the invented closing
  // delimiter extends the buffer the offsets are measured against, and a
  // trailing backslash makes the word scanner overshoot by one.
  "(('",
  '(((\\',
  'translate(1px, ',
  'var(--a',
  'var(--a,',
  'url(',
  'url(a',
  'url( a ',
  'url(a b',
  "url('a",
  'url("a',
  // Unclosed and mismatched quotes.
  '"',
  "'",
  '"a',
  "'a",
  '"a\'',
  '\'a"',
  '""',
  "''",
  '"a" "b"',
  '"a""b"',
  '\'a\'"b"',
  // Escapes at and past the end of the input.
  '\\',
  'a\\',
  '"a\\"',
  "'a\\'",
  '\\\\',
  'a\\\\',
  '"a\\\\"',
  'url(a\\)',
  'url(a\\\\)',
  // Unterminated and degenerate comments.
  '/*',
  '/**',
  '/*a',
  '/*/',
  '/**/',
  '/*/ x */',
  '/*a*/b/*c*/',
  'a/*b*/c',
  '/* nested /* comment */',
  // Separators with nothing to separate.
  ',',
  ',,,',
  ':',
  '/',
  '//',
  '/ /',
  'a,,b',
  'a , , b',
  ' , ',
  'a:b:c',
  // Whitespace in every position it can occupy.
  '  a  ',
  'a  b',
  'calc( 1px )',
  'calc(  )',
  'f( a , b )',
  'f(\n  a\n)',
  // Unicode ranges, real and near-miss.
  'U+26',
  'u+26',
  'U+0-7F',
  'U+4??',
  'U+',
  'U+zz',
  'U+26,U+27',
  'Ux26',
  // Custom properties and their fallbacks.
  'var(--a)',
  'var(--a, 1px)',
  'var(--a, var(--b, 1px))',
  'var(--a,var(--b,var(--c,0)))',
  'var(--not-prefixed)',
  'var(missing-dashes)',
  'var(--a) var(--b)',
  'env(safe-area-inset-top, 0px)',
  // Non-ASCII in every position a token can hold it.
  '"→ Привет 日本語 🙂"',
  'Привет',
  'url(日本語.png)',
  'f(→)',
  '/* 🙂 */',
  '--Привет',
  'a→b',
  '🙂🙂🙂',
  '"\\1F642"',
  'content value',
  // Vendor prefixes and the shapes around them.
  '-webkit-box',
  '-WEBKIT-BOX',
  '-moz-calc(1px + 2px)',
  '-webkit-gradient(linear, left top, left bottom)',
  '--webkit-not-a-prefix',
  '-webkit-image-set(url(a.png) 1x, url(b.png) 2x)',
  'progid:DXImageTransform.Microsoft.gradient(startColorstr=#FF000000)',
  // Importance annotations.
  '1px !important',
  '1px!important',
  '1px  !important',
  '1px ! important',
  '!important',
  'red !IMPORTANT',
  // Numbers the unit splitter has to reason about.
  '0',
  '0px',
  '.5',
  '-.5',
  '+.5',
  '1e3',
  '1e',
  '1e+',
  '1E-3px',
  '-10000px',
  '1.2.3',
  '10PX',
  '10 px',
  // Operators inside and outside calc.
  'calc(-1 * var(--spacing))',
  'calc(100vw * 0.12)',
  'calc(1px+2px)',
  'calc(1px + 2px)',
  'calc(1px/2)',
  'calc(1px / 2)',
  'calc(1px*2)',
  '1px/2',
  '1px / 2',
  'min(1px,2px)',
  'clamp(1px, 2vw, 3px)',
  'calc(calc(1px * 2) / 3)',
  // Colons, which mean different things in different places.
  'a:b',
  'a: b',
  'a :b',
  'url(https://example.com/a.png)',
  'url("https://example.com/a,b.png")',
  'url(data:image/svg+xml;base64,PHN2Zz48L3N2Zz4=)',
  'image-set("a.png" 1x, "b.png" 2x)',
];

/**
 * Every input the JavaScript implementation exercises itself, transcribed from
 * its own test suite. It is the corpus written by the people who know where the
 * awkward cases are, so it is measured here rather than approximated.
 */
const FROM_THE_JAVASCRIPT_TEST_SUITE = [
  // From the scan tests.
  '',
  '\\(',
  '\\)',
  '\\(\\)',
  '\\( \\)',
  '() )wo)rd)',
  '( )',
  '( | )',
  'name()',
  '((()))',
  '( calc(( ) ))word',
  '/',
  ':',
  ',',
  ' , ',
  '( , )',
  ' , : ',
  '""',
  "''",
  "'word\\'word'",
  '"word\\"word"',
  '"word\'word"',
  "'word\"word'",
  '"word',
  '"word\\',
  '"string"',
  'word1"string"word2',
  ' "string" ',
  ' \\"word\\\'\\ \\\t ',
  'bold italic 12px \t /3 \'Open Sans\', Arial, "Helvetica Neue", sans-serif',
  'rgba( 29, 439 , 29 )',
  'url( /gfx/img/bg.jpg )',
  'url( /gfx/img/bg.jpg ',
  'url( "/gfx/img/bg.jpg" hello )',
  'calc(1 + 2)',
  'calc(1 - 2)',
  'calc(1 * 2)',
  'calc(1 / 2)',
  'calc(1*2)',
  'calc(1/2)',
  'calc(((768px - 100vw) / 2) - 15px)',
  '(min-width: 700px) and (orientation: \\$landscape)',
  'url( http://website.com/assets\\)_test )',
  'fn1(fn2(255), fn3(.2)), fn4(fn5(255,.2), fn6)',
  '(0 32 word ',
  '( ( ( ) ',
  '0 32 word ) ',
  'Bond\\ 007',
  'url(foo/bar.jpg), url(http://website.com/img.jpg)',
  'url()',
  'url( )',
  'url(   )',
  'url(\n)',
  'url(\r\n)',
  'url(\n\n\n)',
  'url(\r\n\r\n\r\n)',
  'url(  \n \t  \r\n  )',
  '/*before*/ 1px /*between*/ 1px /*after*/',
  'rgba( 0, 55/55, 0/*,.5*/ )',
  'url( "/demo/bg.png" /*comment*/ )',
  'url( /*comment*/ /demo/bg.png )',
  'url( /demo/bg.png /*comment*/ )',
  '/*comment*/ 1px /* unclosed ',
  'Hawaii \\35 -0',
  'U+26',
  'U+0-7F',
  'U+0-7f',
  'u+26',
  'U+0025-00FF',
  'U+4??',
  'U+0025-00FF, U+4??',
  'U+4??Z',
  'U+',
  'U+Z',
  // From the serialisation tests.
  'bold italic 12px/3 \'Open Sans\', Arial, "Helvetica Neue", sans-serif',
  '" 12,  54, 65 ',
  ' rgba( 12,  54, 65) ',
  ' rgba( 12,  54, 65 ',
  'background-image:linear-gradient(45deg,transparent 25%,hsla(0,0%,100%,.2) 25%,hsla(0,0%,100%,.2) 75%,transparent 75%,transparent 25%,hsla(0,0%,100%,.2) 75%,transparent 75%,transparent),linear-gradient(45deg,transparent 25%,hsla(0,0%,100%,.2))',
  '/!*comment*!/ 1px /!* unclosed ',
  '/!*before*!/ rgb( /!*red component*!/ 12,  54 /!*green component*!/, /!* blue *!/ 65)/!* after *!/ ',
  'url(\t)',
  'url(  \n \t  \n  )',
  // From the unit splitting tests.
  '.23rem',
  '.2.3rem',
  '2.',
  '+2.',
  '-2.',
  '+-2.',
  '.',
  '.rem',
  '1e4px',
  '1em',
  '1e10',
  'e',
  'e1',
  '2rem',
  '2.000rem',
  '+2rem',
  '-2rem',
  '1.1rem',
  '+1.1rem',
  '-1.1rem',
  '1.1e1rem',
  '+1.1e1rem',
  '-1.1e1rem',
  '1.1e+1rem',
  '1.1e-1rem',
  '1.1e1e1rem',
  '1.1e-1e',
  '1.1e--++1e',
  '1.1e--++1rem',
  '100+px',
  '100.0.0px',
  '100e1epx',
  '100e1e1px',
  '+100.1e+1e+1px',
  '-100.1e-1e-1px',
  '.5px',
  '+.5px',
  '-.5px',
  '.5e1px',
  '-.5e1px',
  '+.5e1px',
  '.5e1e1px',
  '.5.5px',
  '1e',
  '1e1',
  '1ee',
  '1e+',
  '1e-',
  '1e+1',
  '1e++1',
  '1e--1',
  '+10',
  '-10',
  '.2px',
  '-.2px',
  '+.2px',
  '.a',
  '+',
  '-',
  '-a',
  '+a',
  '+.a',
  '-.a',
  // From the entry point tests.
  ' rgba( 34 , 45 , 54, .5 ) ',
  'w1 w2 w6 \n f(4) ( ) () \t "s\'t" \'st\\"2\'',
  'fn( ) fn2( fn3())',
];

/**
 * Serialising with a per-node override, and the one shape that needs no
 * override to change what comes out: a node whose kind is rewritten after it
 * was parsed. Every scenario is one the JavaScript exercises itself.
 *
 * The Rust test writes the same override a second time and looks the
 * expectation up by label, so the *behaviour* is spelled twice on purpose and
 * the *answer* still comes from a real run.
 */
const OVERRIDE_SCENARIOS = [
  {
    label: 'function-to-bracket-list',
    input: ' rgba(12,  54, 65 ) ',
    run: nodes => stringify(nodes, bracketFunctions),
  },
  {
    label: 'function-to-bracket-list-one-node',
    input: ' rgba(12,  54, 65 ) ',
    run: nodes => stringify(nodes[1], bracketFunctions),
  },
  {
    label: 'replace-nested-function',
    input: 'calc(1px + var(--bar))',
    run: nodes =>
      stringify(nodes, node =>
        node.type === 'function' && node.value === 'var' ? '10px' : undefined
      ),
  },
  {
    label: 'override-declines-every-node',
    input: ' rgba(12,  54, 65 ) ',
    run: nodes => stringify(nodes, () => undefined),
  },
  {
    label: 'function-retyped-as-word',
    input: ' rgba(12,  54, 65 ) ',
    run: nodes => {
      nodes[1].type = 'word';
      return stringify(nodes);
    },
  },
  // The other side of that coin. `word` is tested before the children are, so a
  // function retyped to one loses them; `unicode-range` is not, so a function
  // retyped to one keeps them and loses its name and parentheses instead.
  {
    label: 'function-retyped-as-unicode-range',
    input: ' rgba(12,  54, 65 ) ',
    run: nodes => {
      nodes[1].type = 'unicode-range';
      return stringify(nodes);
    },
  },
];

/** Collapses a function to `name[arg,arg,arg]`, ignoring everything else. */
function bracketFunctions(node) {
  if (node.type !== 'function') return undefined;
  const args = [node.nodes[0].value, node.nodes[2].value, node.nodes[4].value];
  return `${node.value}[${args.join(',')}]`;
}

/** How deep the stress cases nest, and how long they run. */
const STRESS = [
  ['nested-functions-64', `${'calc('.repeat(64)}1px${')'.repeat(64)}`],
  ['nested-functions-256', `${'calc('.repeat(256)}1px${')'.repeat(256)}`],
  ['unclosed-functions-256', `${'calc('.repeat(256)}1px`],
  ['long-word', 'a'.repeat(10000)],
  ['long-string', `"${'a'.repeat(10000)}"`],
  ['long-comment', `/*${'a'.repeat(10000)}*/`],
  ['many-separators', 'a,'.repeat(2000)],
  ['many-spaces', ' '.repeat(10000)],
  ['many-escapes', '\\a'.repeat(5000)],
  ['many-unclosed-quotes', '"'.repeat(1000)],
];

/**
 * Escapes a string into a quoted literal. Kept deliberately small, because the
 * Rust side reimplements the dump form character for character and a clever
 * escape here is a divergence there.
 *
 * The two callers differ only in how they spell a control character: the dump
 * form is read back by a Rust *function*, which fixes it at four hex digits,
 * and the source form is read back by the Rust *compiler*, whose escape is
 * brace-delimited and variable width.
 */
function quote(text, controlEscape) {
  let out = '"';
  for (const ch of text) {
    const code = ch.codePointAt(0);
    if (ch === '\\') out += '\\\\';
    else if (ch === '"') out += '\\"';
    else if (ch === '\n') out += '\\n';
    else if (ch === '\r') out += '\\r';
    else if (ch === '\t') out += '\\t';
    else if (code < 0x20 || code === 0x7f) out += controlEscape(code);
    else out += ch;
  }
  return `${out}"`;
}

/** The canonical dump's quoted form. Mirrored by `tests/dump.rs`. */
const dumpString = text => quote(text, code => `\\u${code.toString(16).padStart(4, '0')}`);

/** A Rust string literal that parses back to exactly this text. */
const rustLiteral = text => quote(text, code => `\\u{${code.toString(16)}}`);

/**
 * A node tree flattened to one line per node, carrying every field the Rust is
 * required to reproduce: kind, text, source span, surrounding whitespace, quote
 * character and unclosed flag.
 *
 * Source offsets are UTF-16 indices here and byte offsets in the Rust. The two
 * agree for everything the corpus contains and differ only for a token sitting
 * after a non-ASCII character, so a value carrying one has its offsets rebased
 * onto bytes before the comparison — see `rebase` below.
 */
function dumpNodes(nodes, indent, out) {
  for (const node of nodes) {
    let line = `${' '.repeat(indent)}${node.type} ${dumpString(node.value ?? '')} ${node.sourceIndex}..${node.sourceEndIndex}`;
    if (node.before !== undefined) line += ` before=${dumpString(node.before)}`;
    if (node.after !== undefined) line += ` after=${dumpString(node.after)}`;
    if (node.quote !== undefined) line += ` quote=${dumpString(node.quote)}`;
    if (node.unclosed) line += ' unclosed';
    if (Array.isArray(node.nodes)) line += ` nodes=${node.nodes.length}`;
    out.push(line);
    if (Array.isArray(node.nodes)) dumpNodes(node.nodes, indent + 2, out);
  }
}

/**
 * Rewrites every UTF-16 offset in a tree as the byte offset of the same
 * position. The Rust scans bytes -- every character it tests for is ASCII, so
 * token boundaries land in the same places -- and its offsets are therefore
 * byte offsets. Their one consumer compares them against each other, so which
 * unit they are counted in does not matter, but the comparison here has to
 * count in one of them.
 */
function rebase(nodes, toByte) {
  for (const node of nodes) {
    node.sourceIndex = toByte(node.sourceIndex);
    node.sourceEndIndex = toByte(node.sourceEndIndex);
    if (Array.isArray(node.nodes)) rebase(node.nodes, toByte);
  }
}

/** UTF-16 index to byte index, for a string the parser may have extended. */
function byteOffsets(value) {
  const offsets = Array.from({ length: value.length + 1 });
  const encoder = new TextEncoder();
  let bytes = 0;
  let index = 0;
  while (index < value.length) {
    offsets[index] = bytes;
    const point = value.codePointAt(index);
    const width = point > 0xffff ? 2 : 1;
    bytes += encoder.encode(String.fromCodePoint(point)).length;
    for (let step = 1; step < width; step += 1) offsets[index + step] = bytes;
    index += width;
  }
  offsets[value.length] = bytes;
  return index16 => (index16 <= value.length ? offsets[index16] : bytes + (index16 - value.length));
}

/** Every word in a tree, in visit order — the unit splitter's real diet. */
function words(nodes, out) {
  for (const node of nodes) {
    if (node.type === 'word') out.push(node.value);
    if (Array.isArray(node.nodes)) words(node.nodes, out);
  }
  return out;
}

/** `Option<Dimension>` as the Rust spells it. */
function rustDimension(dimension) {
  if (!dimension) return 'None';
  return `Some((${rustLiteral(dimension.number)}, ${rustLiteral(dimension.unit)}))`;
}

const parserCases = [];
const unitInputs = new Set();

// Degenerate unit inputs the corpus cannot produce, because they are not words
// any parse would yield.
for (const input of ['', ' ', '.', '-', '+', 'e', 'e3', '.e3', '-.', '+.', ' 1', '1 ']) {
  unitInputs.add(input);
}

const seen = new Set();
for (const input of [...corpusValues(), ...MALFORMED, ...FROM_THE_JAVASCRIPT_TEST_SUITE]) {
  if (seen.has(input)) continue;
  seen.add(input);

  const ast = parser(input);
  const output = stringify(ast.nodes);
  const lines = [];
  rebase(ast.nodes, byteOffsets(input));
  dumpNodes(ast.nodes, 0, lines);

  parserCases.push({ input, output, ast: lines.join('\n') });
  for (const word of words(parser(input).nodes, [])) unitInputs.add(word);
}

const stressCases = STRESS.map(([label, input]) => ({
  label,
  input,
  output: stringify(parser(input).nodes),
}));

const overrideCases = OVERRIDE_SCENARIOS.map(scenario => ({
  label: scenario.label,
  input: scenario.input,
  output: scenario.run(parser(scenario.input).nodes),
}));

const unitCases = [...unitInputs].map(input => [input, parser.unit(input)]);

const parserRows = parserCases.flatMap(entry => [
  '  ParserCase {',
  `    input: ${rustLiteral(entry.input)},`,
  `    output: ${rustLiteral(entry.output)},`,
  `    ast: ${rustLiteral(entry.ast)},`,
  '  },',
]);

const stressRows = stressCases.flatMap(entry => [
  '  StressCase {',
  `    label: ${rustLiteral(entry.label)},`,
  `    input: ${rustLiteral(entry.input)},`,
  `    output: ${rustLiteral(entry.output)},`,
  '  },',
]);

const overrideRows = overrideCases.flatMap(entry => [
  '  OverrideCase {',
  `    label: ${rustLiteral(entry.label)},`,
  `    input: ${rustLiteral(entry.input)},`,
  `    output: ${rustLiteral(entry.output)},`,
  '  },',
]);

const unitRows = unitCases.map(
  ([input, dimension]) => `  (${rustLiteral(input)}, ${rustDimension(dimension)}),`
);

process.stdout.write(
  [
    '// @generated by scripts/generate-value-parser-cases.mjs -- do not edit by hand.',
    '//',
    '// Every expectation is a literal answer from the JavaScript being stood in for.',
    '// Regenerate after adding an input to the generator, never by eye:',
    '//',
    '//   pnpm run --filter=@stylexswc/postcss-value-parser generate:value-parser-cases',
    '',
    '/// One value, and what the JavaScript does with it.',
    'pub(super) struct ParserCase {',
    '  /// The declaration value, as an author would write it.',
    "  pub input: &'static str,",
    '  /// What serialising the parsed value back out produces. Equal to `input`',
    '  /// for all but the `/*/` comment quirk documented on the module.',
    "  pub output: &'static str,",
    '  /// The node tree, one line per node, nested nodes indented by two.',
    "  pub ast: &'static str,",
    '}',
    '',
    '/// A value large or deep enough to be about resource limits rather than',
    '/// syntax. Only the serialised output is recorded: the tree is thousands of',
    '/// nodes and pinning it would say nothing the smaller cases do not.',
    'pub(super) struct StressCase {',
    '  /// What makes this input extreme, for a failure message that explains',
    '  /// itself without printing ten thousand characters.',
    "  pub label: &'static str,",
    '  /// The value itself.',
    "  pub input: &'static str,",
    '  /// What serialising the parsed value back out produces.',
    "  pub output: &'static str,",
    '}',
    '',
    `/// ${parserCases.length} values: the differential harness's whole corpus, plus`,
    '/// malformed, truncated and degenerate inputs no author would write.',
    'pub(super) const PARSER_CASES: &[ParserCase] = &[',
    ...parserRows,
    '];',
    '',
    `/// ${stressCases.length} values at the sizes where a scanner stops being about CSS.`,
    'pub(super) const STRESS_CASES: &[StressCase] = &[',
    ...stressRows,
    '];',
    '',
    '/// One serialisation that does not spell the tree out plainly — either',
    '/// because an override replaced a node, or because a node was re-kinded',
    '/// after parsing. Paired with the Rust that reproduces it by label.',
    'pub(super) struct OverrideCase {',
    '  /// Names the override the Rust side has to write to match.',
    "  pub label: &'static str,",
    '  /// The value parsed before the override runs.',
    "  pub input: &'static str,",
    '  /// What came out.',
    "  pub output: &'static str,",
    '}',
    '',
    `/// ${overrideCases.length} serialisations that an override or a re-kinded node changed.`,
    'pub(super) const OVERRIDE_CASES: &[OverrideCase] = &[',
    ...overrideRows,
    '];',
    '',
    `/// ${unitCases.length} words paired with their number/unit split, \`None\` standing for a`,
    '/// word that does not start with a number. Every word the cases above parse',
    '/// to, plus splits no parse would ever ask for.',
    'pub(super) const UNIT_CASES: &[(&str, Option<(&str, &str)>)] = &[',
    ...unitRows,
    '];',
    '',
  ].join('\n')
);
