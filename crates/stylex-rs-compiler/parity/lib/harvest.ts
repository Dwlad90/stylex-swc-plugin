/**
 * Harvest CSS declarations out of the Rust test suites.
 *
 * The parity corpus has to cover what the test suites already cover, otherwise
 * a divergence the tests would have caught is invisible to the harness. Rather
 * than transcribing those values by hand — which is how a corpus quietly goes
 * stale — they are extracted from the sources.
 *
 * Seven literal shapes carry a CSS declaration in this repo. Each is handled by
 * an extractor below and each records where it came from, so an unexpected
 * corpus entry can be traced back to the test that motivated it.
 *
 * This module knows only what a CSS declaration looks like in a Rust test.
 * Reading the sources is `rust-source.ts` and identifying an entry is
 * `declaration.ts`; adding a shape should touch only this file.
 */

import { dedupe, entry } from './declaration.js';
import type { RustLiteral } from './rust-literals.js';
import {
  enclosingCallees,
  enclosingOpener,
  findCallSites,
  literalsBetween,
  scanRustTestFiles,
  testBlocks,
  type ScannedFile,
} from './rust-source.js';
import type { DeclarationEntry } from './types.js';

/** Keys that appear in a `stylex.create` object but are not CSS properties. */
const NON_PROPERTY_KEYS = new Set([
  'default',
  'from',
  'to',
  'import',
  'export',
  'const',
  'return',
  'type',
]);

/**
 * How far past a call's opening parenthesis its arguments are looked for.
 *
 * A window rather than real argument parsing, because the alternative is a Rust
 * expression parser. Each is sized to the widest call of its shape in the
 * suites with room to spare, and a call that outgrows one is not harvested
 * rather than mis-harvested — the adjacency guards below are what make that
 * true.
 */
const ARGUMENT_WINDOW = {
  /** `unchanged("color", "red")` and friends: two literals, close together. */
  propertyValueCall: 400,
  /** `rejects("width", &["*(", …], MESSAGE, …)`: a property then a slice. */
  rejectionTable: 800,
  /** The property literal of a `normalize_css_property_value` call. */
  caseTableProperty: 200,
} as const;

/**
 * Callees whose string arguments are not CSS values.
 *
 * An assertion or a panic is handed a *message*, a search is handed a *needle*
 * looked for in output, and a split or a join is handed a *separator*. All
 * three read exactly as a declaration does — `"width: limit 64, found 65"` is
 * an assertion message and `", "` is the separator of a `join` — so the
 * spelling cannot tell them apart. Where the literal was going can.
 *
 * A formatting macro is not listed, and that is the whole of the distinction:
 * `format!` builds both messages and values. The one that is a message is
 * handed to an assertion, so the chain catches it there; the one that is a
 * value — `format!("0px 0px {n}px #000")`, a shadow generated per entry — is
 * handed to the compiler and stays.
 */
const NON_VALUE_CALLEES = new Set([
  'assert!',
  'assert_eq!',
  'assert_ne!',
  'panic!',
  'contains',
  'starts_with',
  'ends_with',
  'matches',
  'join',
  'split',
]);

/** How far past the property literal an adjacency guard reads. */
const ADJACENCY_LOOKAHEAD = 40;

/**
 * What may stand between the property of a rejection table and its slice: the
 * comma, a borrow, and the name of a macro that builds one.
 *
 * Two helpers in the suites are named `rejects`, and they take their arguments
 * in opposite orders: one is `(property, values, ...)` and the other is
 * `(value, property, ...)`. Reading the second as the first pairs a value with
 * a *property* and reports the pair inside out. What tells them apart is the
 * slice, since only the table form has one — so the `[` must be the argument
 * after the property rather than the next one anywhere in the file.
 */
const BEFORE_THE_SLICE = /^\s*,\s*(?:&\s*)?(?:\w+!)?$/;

/** The call whose argument holds the style objects shape 5 reads. */
const CREATE_CALLEE = 'stylex.create';

/** The receiver whose calls are handed style objects. */
const STYLEX_RECEIVER = 'stylex.';

/** The receiver whose call arguments the harvester cannot read. */
const ENV_RECEIVER = 'stylex.env.';

/**
 * Reserved words a parenthesis can follow, which are never callees.
 *
 * `return ({ color: 'red' })` is the arrow body of a dynamic style written the
 * long way, and the parenthesis after the word is the language's rather than a
 * call's. Reading `return` as a receiver would drop the style object inside
 * it.
 */
const RESERVED_BEFORE_PARENTHESIS = new Set([
  'return',
  'yield',
  'await',
  'typeof',
  'void',
  'delete',
  'new',
  'in',
  'of',
  'case',
  'do',
  'else',
]);

const WHITESPACE = /\s/;
const CALLEE_CHARACTER = /[\w$.]/;

/** Half-open offsets of one `stylex.create` argument list. */
interface CallRange {
  start: number;
  end: number;
}

/** The parenthesis depth that says the scan is not inside a call. */
const OUTSIDE_CALL = -1;

export interface HarvestOptions {
  /** Absolute path to the workspace root. */
  workspaceRoot: string;
}

export function harvestCorpus(options: HarvestOptions): DeclarationEntry[] {
  const files = scanRustTestFiles(options.workspaceRoot);

  const collected: DeclarationEntry[] = [];
  for (const file of files) {
    collected.push(...extractPropertyValueCalls(file));
    collected.push(...extractRejectionTables(file));
    collected.push(...extractCaseTables(file));
    collected.push(...extractRuleLiterals(file));
    collected.push(...extractStyleObjects(file));
  }

  return dedupe(collected.filter(candidate => isDeclarationKey(candidate.property)));
}

/**
 * Whether a literal an extractor swept up is a CSS value at all.
 *
 * Shapes 1, 6 and 7 read a literal out of a known argument slot, so they know
 * what they are holding. The two extractors that sweep — a whole `#[test]`
 * block for shape 2, a whole file for shapes 3 and 4 — do not, and every
 * message, needle and separator in that text reads as a declaration to them.
 * Asking who the literal was handed to settles it.
 */
function isValueLiteral(file: ScannedFile, literal: RustLiteral): boolean {
  return !enclosingCallees(file.masked, literal.start).some(callee =>
    NON_VALUE_CALLEES.has(callee)
  );
}

/**
 * Whether a harvested key names a declaration rather than a selector or an
 * at-rule.
 *
 * A `:hover` or `@media` key inside `stylex.create` opens a nested block; the
 * object under it holds the declarations. Feeding one to the comparison as if
 * it were a property asks both compilers what `:hover: <value>` means, and
 * neither answer says anything about how a value is spelled — so a pair like
 * that reports a divergence that no change to normalization could ever close.
 */
function isDeclarationKey(property: string): boolean {
  return !property.startsWith(':') && !property.startsWith('@');
}

/**
 * Calls whose first two arguments are a property literal and a value literal.
 *
 * `normalize_css_property_value` is the direct call form. `unchanged`, `same`
 * and `diverges` build one row of a verdict case table, where the arguments
 * after the value are the expected output and the reference compiler's
 * spelling — deliberately not harvested, since deriving expectations from the
 * reference compiler is the whole point of the harness. `unchanged` has no
 * arguments after the value at all: it is the case whose expectation *is* the
 * input, which is the majority shape and the reason it exists.
 *
 * `refusal_of` and `refuses_with` are the same shape for a value that is
 * refused rather than spelled: property, value, and then the complaint it is
 * expected to earn. The complaint is an expectation like any other and is left
 * where it was written; the pair in front of it is a declaration the suites
 * cover, and a corpus that skipped them would be blind to exactly the
 * degenerate values a refusal test exists to carry.
 */
const PROPERTY_VALUE_CALLS = [
  'normalize_css_property_value',
  'unchanged',
  'same',
  'diverges',
  'refusal_of',
  'refuses_with',
] as const;

/**
 * The calls that are handed a declaration, so their own arguments are never
 * the rows of a case table.
 */
const SUBJECT_CALLS = new Set<string>([...PROPERTY_VALUE_CALLS, 'rejects']);

/**
 * Shapes 1 and 6 — `normalize_css_property_value("color", "#ff0000", &opts)`,
 * `unchanged("color", "red")` and `same("color", "#ff0000", "#f00")`.
 *
 * The forms where both the property and the value are literals sitting next to
 * each other. Between them these are the bulk of the value-normalization unit
 * tests.
 */
function extractPropertyValueCalls(file: ScannedFile): DeclarationEntry[] {
  const entries: DeclarationEntry[] = [];

  for (const name of PROPERTY_VALUE_CALLS) {
    const open = `${name}(`;
    for (const callStart of findCallSites(file.masked, open)) {
      const argsStart = callStart + open.length;
      const args = literalsBetween(
        file,
        argsStart,
        argsStart + ARGUMENT_WINDOW.propertyValueCall
      ).slice(0, 2);
      if (args.length !== 2) continue;
      if (!argumentsAreAdjacent(file, argsStart, args[0]!, args[1]!)) continue;
      entries.push(entry(args[0]!.value, args[1]!.value, `${file.relativePath}:${args[0]!.line}`));
    }
  }

  return entries;
}

/**
 * Whether two literals really are the first two arguments of the call at
 * `argsStart`.
 *
 * Without this, a call whose second argument is an identifier —
 * `normalize_css_property_value("height", value, &opts)`, the shape 2 form —
 * pairs its property with a literal belonging to a *later statement* that
 * happens to fall inside the window. The test is that nothing but whitespace
 * separates the opening parenthesis from the first literal, and nothing but a
 * comma and whitespace separates the two.
 *
 * Read from `source` rather than `masked`, because masking blanks a literal's
 * delimiters along with its body: on `masked` the separators would be there but
 * the literals would not.
 */
function argumentsAreAdjacent(
  file: ScannedFile,
  argsStart: number,
  first: RustLiteral,
  second: RustLiteral
): boolean {
  if (!/^\s*$/.test(file.source.slice(argsStart, first.start))) return false;
  return /^\s*,\s*$/.test(file.source.slice(first.end, second.start));
}

/**
 * Shape 7 — a rejection table: one property, then a slice of the values that
 * property is expected to be rejected for.
 *
 * ```rust
 * rejects("width", &["*(", "/.5 *("], UNCLOSED_FUNCTION, &default_options())
 * ```
 *
 * A rejection has no spelling for a verdict case to compare, so these values
 * never reach shapes 1 or 6 — but the harness still has something to say about
 * them, namely whether the reference compiler rejects them too. Only the
 * literals inside the slice are taken; the diagnostic argument that follows is
 * a message, not a value, which is why it is bound to a constant at the call
 * sites rather than written inline.
 */
function extractRejectionTables(file: ScannedFile): DeclarationEntry[] {
  const entries: DeclarationEntry[] = [];
  const open = 'rejects(';

  for (const callStart of findCallSites(file.masked, open)) {
    const argsStart = callStart + open.length;
    const [property, ...rest] = literalsBetween(
      file,
      argsStart,
      argsStart + ARGUMENT_WINDOW.rejectionTable
    );
    if (property === undefined) continue;

    // The slice is the argument after the property, and it ends at its own
    // `]`, so a literal belonging to a later argument is never read as one of
    // the values.
    const sliceStart = file.masked.indexOf('[', property.end);
    if (sliceStart === -1) continue;
    if (!BEFORE_THE_SLICE.test(file.source.slice(property.end, sliceStart))) continue;
    const sliceEnd = file.masked.indexOf(']', sliceStart);
    if (sliceEnd === -1) continue;

    for (const value of rest) {
      if (value.start > sliceEnd) break;
      entries.push(entry(property.value, value.value, `${file.relativePath}:${value.line}`));
    }
  }

  return entries;
}

/**
 * Shape 2 — a case table looped through a single property.
 *
 * ```rust
 * let cases = [("calc-size(any, 300px)", "calc-size(any,300px)")];
 * for (value, expected) in cases {
 *   assert_eq!(normalize_css_property_value("height", value, &opts), expected);
 * }
 * ```
 *
 * The call above passes an identifier rather than a literal, so shape 1 skips
 * it. Here the enclosing `#[test] fn` block is taken as the unit: the literal
 * property named in the call applies to every input in the block's tables. The
 * *input* is the first element of each tuple, or the whole literal for a flat
 * array of values; expected outputs are deliberately not harvested, since the
 * point of the harness is to derive them from the reference compiler.
 */
function extractCaseTables(file: ScannedFile): DeclarationEntry[] {
  const entries: DeclarationEntry[] = [];

  for (const block of testBlocks(file.masked)) {
    const argsStart = firstSubjectCall(file, block);
    if (argsStart === undefined) continue;

    const property = literalsBetween(
      file,
      argsStart,
      argsStart + ARGUMENT_WINDOW.caseTableProperty
    )[0];
    if (property === undefined) continue;

    // The call already supplied a literal value; shape 1 covered it.
    const between = file.source.slice(argsStart, property.end);
    if (!/^\s*"/.test(between) && !/^\s*r#*"/.test(between)) continue;
    const afterProperty = file.source.slice(property.end, property.end + ADJACENCY_LOOKAHEAD);
    if (/^\s*,\s*(r#*)?"/.test(afterProperty)) continue;

    for (const literal of file.literals) {
      if (literal.start < block.start || literal.end > block.end) continue;
      if (isSubjectArgument(file, literal)) continue;
      if (!isValueLiteral(file, literal)) continue;
      if (!isTableInput(file, literal)) continue;
      entries.push(entry(property.value, literal.value, `${file.relativePath}:${literal.line}`));
    }
  }

  return entries;
}

/**
 * Where the arguments of the block's first subject call start, or `undefined`
 * where the block calls none.
 *
 * Any of the calls that take a declaration names the property a looped table
 * applies to. `refuses_with("color", value, …)` is the shape a table of
 * degenerate values takes, and reading only `normalize_css_property_value`
 * left every one of those values out of the corpus.
 *
 * Located on `masked`, so a call-shaped spelling inside a CSS value literal
 * cannot be read as a call.
 */
function firstSubjectCall(
  file: ScannedFile,
  block: { start: number; end: number }
): number | undefined {
  let earliest: number | undefined;

  for (const name of PROPERTY_VALUE_CALLS) {
    const open = `${name}(`;
    const at = file.masked.indexOf(open, block.start);
    if (at === -1 || at >= block.end) continue;
    if (earliest === undefined || at < earliest) earliest = at + open.length;
  }

  return earliest;
}

/**
 * Whether a literal is an argument of the call under test rather than a row of
 * a table beside it.
 *
 * A block names its property in the call and reads its inputs from a table
 * outside it, so nothing inside the call is an input. The property literal is
 * the one that matters: a block that calls the compiler twice names the
 * property twice, and the second one sits first inside its own parenthesis —
 * which is where a tuple row keeps its input, so the bracket alone cannot tell
 * the two apart. It read as `backgroundImage: backgroundImage`, a property
 * named as its own value.
 */
function isSubjectArgument(file: ScannedFile, literal: RustLiteral): boolean {
  return enclosingCallees(file.masked, literal.start).some(callee => SUBJECT_CALLS.has(callee));
}

/**
 * Whether a literal is a table *input*, decided by the bracket enclosing it.
 *
 * `[...]` is the table itself, so every literal directly inside it is an input:
 * that is the flat `["1px", "2px"]` form. `(...)` is one row, written
 * `("in", "out")`, so only its first literal is an input and the rest are the
 * expected output — which must never be harvested, since deriving expectations
 * from the reference compiler is the whole point of the harness.
 */
function isTableInput(file: ScannedFile, literal: RustLiteral): boolean {
  const opener = enclosingOpener(file.masked, literal.start);
  if (opener === -1) return false;
  if (file.masked[opener] === '[') return true;
  const first = file.literals.find(candidate => candidate.start > opener);
  return first?.start === literal.start;
}

/**
 * Shapes 3 and 4 — a whole CSS rule in one literal.
 *
 * `"* {{ transitionProperty: opacity; }}"` and the minified `"*{color:red}"`
 * both name their property inline, so the rule is read straight off the
 * literal.
 */
function extractRuleLiterals(file: ScannedFile): DeclarationEntry[] {
  const entries: DeclarationEntry[] = [];

  for (const literal of file.literals) {
    const rule = /^\*\s*\{\{?\s*(--[\w-]+|[\w-]+)\s*:\s*([\s\S]*?)\s*;?\s*\}\}?$/.exec(
      literal.value
    );
    // Asked of the few literals that spell a rule rather than of every literal
    // in the file, since reading the shape is much the cheaper question.
    if (rule === null || !isValueLiteral(file, literal)) continue;
    entries.push(entry(rule[1]!, rule[2]!, `${file.relativePath}:${literal.line}`));
  }

  return entries;
}

/**
 * Shape 5 — a JavaScript source embedded in a transform test.
 *
 * The transform tests hold whole modules as raw strings, and their
 * `stylex.create` objects are the richest source of authored values in the
 * repo: they are what a user actually writes. Keys that are selectors, media
 * queries, or JavaScript rather than CSS properties are filtered out, and
 * interpolated values are skipped because they are not literal CSS.
 *
 * Only the argument of a `stylex.create` call is read. A fixture holds more
 * than the call — imports, helper constants, a second module — and an ordinary
 * JavaScript object among them spells `key: 'value'` exactly as a style object
 * does. Reading the whole fixture makes such an object a source of CSS it
 * never was.
 */
function extractStyleObjects(file: ScannedFile): DeclarationEntry[] {
  const entries: DeclarationEntry[] = [];

  for (const literal of file.literals) {
    if (!literal.value.includes(`${CREATE_CALLEE}(`)) continue;
    const { calls, enclosing, preceding, closers } = scanFixture(literal.value);
    if (calls.length === 0) continue;

    const declaration =
      /(--[\w-]+|'[^'\n]+'|"[^"\n]+"|[A-Za-z][A-Za-z0-9]*)\s*:\s*('([^'\\\n]|\\.)*'|"([^"\\\n]|\\.)*")/g;
    let call = 0;
    for (const match of literal.value.matchAll(declaration)) {
      // The calls and the matches are both in source order, so the window
      // moves forward once rather than being searched again for every key.
      while (call < calls.length && calls[call]!.end <= match.index) call += 1;
      if (call === calls.length) break;
      if (match.index < calls[call]!.start) continue;

      const property = unquote(match[1]!);
      const value = unquote(match[2]!);
      if (!isCssProperty(property)) continue;
      if (!isPropertyKey(literal.value, preceding, match.index)) continue;
      if (isCallArgumentKey(literal.value, enclosing, match.index)) continue;
      if (isLookupKey(literal.value, enclosing, closers, match.index)) continue;
      // A JavaScript-style interpolation is skipped because the value it stands
      // for is not in the source; a *Rust* format placeholder is not skipped,
      // and deliberately.
      //
      // A handful of harvested values carry one -- `0px 0px {n}px #000`,
      // `rgb({channel}, 0, 0)`, `url(x{body})`. They read as noise, and it is
      // tempting to filter them, but they are `acceptance-divergent` and
      // pinned by the declaration-terminating token family *on its merits*: a
      // `{` in a value really would close the rule being generated, the guard
      // really does refuse it, and the reference compiler really does emit it.
      // They are degenerate subjects and honest members of that family.
      //
      // Filtering on `{word}` would also drop
      // `url("a;b{c}d: e /* f */")`, which is an authored test value where the
      // braces are literal text. So the noise is left in rather than traded for
      // a filter that removes a real subject.
      if (value.includes('${')) continue;
      if (value.trim() === '') continue;
      entries.push(entry(property, value, `${file.relativePath}:${literal.line}`));
    }
  }

  return entries;
}

/**
 * The offset just past the string, template or comment that starts at `i`, or
 * `-1` where the character there starts none of them.
 *
 * Both scans below step over the same text for the same reason: a brace, a
 * parenthesis or a quote written in a value or a comment is not code, and
 * counting one puts every offset after it out of step. A regex literal is the
 * one form still read as code — telling one from a division needs the grammar,
 * and no fixture writes one.
 */
function skipNonCode(js: string, i: number): number {
  const char = js[i]!;

  if (char === '/' && js[i + 1] === '/') {
    const end = js.indexOf('\n', i + 2);
    return end === -1 ? js.length : end;
  }
  if (char === '/' && js[i + 1] === '*') {
    const end = js.indexOf('*/', i + 2);
    return end === -1 ? js.length : end + 2;
  }
  if (char !== '"' && char !== "'" && char !== '`') return -1;

  for (let at = i + 1; at < js.length; at += 1) {
    if (js[at] === '\\') {
      at += 1;
      continue;
    }
    if (js[at] === char) return at + 1;
  }
  // An unterminated string runs to the end of the fixture.
  return js.length;
}

/**
 * What one pass over a JavaScript fixture tells the key filters.
 *
 * Both answers come from the same walk because both need the same thing: a
 * brace, a parenthesis or a quote written in a value or a comment is not code,
 * and counting one puts every offset after it out of step. Reading the text
 * twice to step over it twice would be the only difference.
 */
interface FixtureScan {
  /**
   * The argument list of every `stylex.create` call, as offsets between the
   * call's `(` and the `)` that closes it.
   *
   * A fixture with two calls gets two ranges, and a call nested in another
   * stays inside the outer one. A call the fixture never closes yields no
   * range. That loses the declarations it holds, which is the safe way round:
   * reading to the end of the text would take in every object after the call,
   * which is what bounding the scan exists to stop.
   */
  calls: CallRange[];
  /**
   * For every offset, the offset of the innermost `{` still open there, or
   * `-1` where none is. Answering by walking backwards from each key would
   * re-read the same text once per key.
   */
  enclosing: Int32Array;
  /**
   * For every offset, the offset of the nearest code character in front of it
   * that is not whitespace, or `-1` where there is none. This is what says
   * whether a pair sits where an object writes a key, and it answers over code
   * so that a comment written between the brace and the key it opens for does
   * not hide the key.
   */
  preceding: Int32Array;
  /** For each `{` the fixture closes, the offset of the `}` that closes it. */
  closers: Map<number, number>;
}

function scanFixture(js: string): FixtureScan {
  const calls: CallRange[] = [];
  const enclosing = new Int32Array(js.length);
  const preceding = new Int32Array(js.length);
  const closers = new Map<number, number>();
  const open: number[] = [];
  let depth = OUTSIDE_CALL;
  let start = 0;
  let last = -1;

  for (let i = 0; i < js.length; i += 1) {
    const skipped = skipNonCode(js, i);
    if (skipped !== -1) {
      enclosing.fill(open.at(-1) ?? -1, i, skipped);
      // A string, a template and a comment all answer with whatever preceded
      // them: the delimiters go with the body, so a value does not become the
      // code character in front of the key after it.
      preceding.fill(last, i, skipped);
      i = skipped - 1;
      continue;
    }

    // Popped before the offset is recorded, so a `}` reads as being inside the
    // block that encloses the one it closes.
    const char = js[i]!;
    if (char === '}') {
      const opened = open.pop();
      if (opened !== undefined) closers.set(opened, i);
    }
    enclosing[i] = open.at(-1) ?? -1;
    preceding[i] = last;
    if (!WHITESPACE.test(char)) last = i;

    if (char === '{') open.push(i);
    else if (char === '(') {
      if (depth !== OUTSIDE_CALL) depth += 1;
      else if (isCreateCallee(js, i)) {
        depth = 0;
        start = i + 1;
      }
    } else if (char === ')' && depth !== OUTSIDE_CALL) {
      if (depth === 0) {
        calls.push({ start, end: i });
        depth = OUTSIDE_CALL;
      } else depth -= 1;
    }
  }

  return { calls, enclosing, preceding, closers };
}

/**
 * Whether the parenthesis at `paren` closes the callee `stylex.create`.
 *
 * The name must start the member chain. A chain that merely ends in it — an
 * `options.stylex.create` — names a different receiver, the same distinction
 * the call-argument guard below draws.
 */
function isCreateCallee(js: string, paren: number): boolean {
  if (!js.endsWith(CREATE_CALLEE, paren)) return false;
  const before = js[paren - CREATE_CALLEE.length - 1];
  return before === undefined || !CALLEE_CHARACTER.test(before);
}

/**
 * Whether the offset `at` is where an object writes a key.
 *
 * A key is preceded by the `{` that opens its object or by the comma after the
 * key before it. Nothing else in JavaScript puts a `name: value` pair there —
 * so a ternary, whose alternative reads the same, is not one:
 *
 * ```js
 * backgroundColor: isDark ? 'black' : 'white',
 * fontFamily: `a${NaN ? 'b' : 'c'}d`,
 * ```
 *
 * `'black': 'white'` and `'b': 'c'` are the branches of a choice, and asking
 * both compilers what `black: white` means says nothing about CSS. The same
 * test settles a numeric key: `1e21: 'a'` offers `e21` as a name, and what
 * precedes it there is the rest of the number rather than a comma.
 */
function isPropertyKey(js: string, preceding: Int32Array, at: number): boolean {
  const char = js[preceding[at] ?? -1];
  return char === '{' || char === ',';
}

/**
 * Whether a key belongs to an object the fixture indexes rather than reads.
 *
 * ```js
 * outline: { true: 'red', false: 'blue' }[String(!!opt?.isPressed)]
 * ```
 *
 * The pairs are the rows of a lookup table and the key is the value being
 * looked up, so `true: red` is a declaration about nothing. What says so is
 * the `[` after the closing brace: a style object is read whole, never
 * subscripted. Reading the shape rather than the spelling is what keeps this
 * from becoming a list of the words a lookup happens to use as keys.
 */
function isLookupKey(
  js: string,
  enclosing: Int32Array,
  closers: Map<number, number>,
  at: number
): boolean {
  const brace = enclosing[at] ?? -1;
  const close = closers.get(brace);
  if (close === undefined) return false;

  let after = close + 1;
  while (after < js.length && WHITESPACE.test(js[after]!)) after += 1;
  return js[after] === '[';
}

/**
 * Whether a key belongs to an object handed straight to a call.
 *
 * ```js
 * color: stylex.env.select({ primary: 'red', secondary: 'blue' }, 'secondary')
 * root: { color: String({ toString: 'notfn' }) }
 * ```
 *
 * `primary` and `secondary` name branches for the function to choose between,
 * and `toString` names the method the argument is meant to be missing. Neither
 * object is a style object, and what says so is the call in front of it.
 *
 * Two callees are handed one and keep their keys. `stylex.positionTry({ top:
 * '0' })` is the API taking a style object by design, and an arrow body —
 * `root: () => ({ color: 'red' })`, whose `({` is a parenthesis with no callee
 * in front of it — is a style object the language merely parenthesizes.
 * Anything else the fixture calls owns its argument, and `stylex.env` is the
 * exception inside the API: what a key means there is decided by the
 * environment function the test supplies, since `colors({ color: 'yellow' })`
 * spells a property and `select({ primary: 'red' })` spells a branch name, and
 * the source cannot tell the two apart.
 *
 * Only the object handed *directly* to a call is skipped. A branch body such
 * as `select({ compact: { padding: '4px' } }, 'compact')` sits one brace
 * deeper and is an ordinary style object, so `padding` is still harvested.
 */
function isCallArgumentKey(js: string, enclosing: Int32Array, at: number): boolean {
  const brace = enclosing[at] ?? -1;
  if (brace === -1) return false;

  // Back over the whitespace between the call's `(` and the `{`. A fixture
  // spreads a long call over several indented lines, so this run has no length
  // worth guessing at.
  let paren = brace;
  while (paren > 0 && WHITESPACE.test(js[paren - 1]!)) paren -= 1;
  if (js[paren - 1] !== '(') return false;

  // Then back over the callee, which is what says whose object this is. The
  // dots come with it, so a chain that merely ends in the API's name — an
  // `options.stylex.positionTry` — reads as the different receiver it is. The
  // whitespace in front of the chain comes off first, since a space between a
  // name and its parenthesis does not stop it being a call.
  let end = paren - 1;
  while (end > 0 && WHITESPACE.test(js[end - 1]!)) end -= 1;
  let start = end;
  while (start > 0 && CALLEE_CHARACTER.test(js[start - 1]!)) start -= 1;

  // A spread writes three dots in front of the call, and a member chain never
  // writes two in a row, so a leading run of them belongs to the spread.
  while (js[start] === '.') start += 1;

  const callee = js.slice(start, end);
  if (callee === '' || RESERVED_BEFORE_PARENTHESIS.has(callee)) return false;
  return !callee.startsWith(STYLEX_RECEIVER) || callee.startsWith(ENV_RECEIVER);
}

function unquote(text: string): string {
  const quote = text[0];
  if (quote !== "'" && quote !== '"') return text;
  return text.slice(1, -1).replaceAll(`\\${quote}`, quote);
}

function isCssProperty(property: string): boolean {
  if (NON_PROPERTY_KEYS.has(property)) return false;
  if (property.startsWith('--')) return true;
  return /^[a-z][A-Za-z0-9]*$/.test(property);
}
