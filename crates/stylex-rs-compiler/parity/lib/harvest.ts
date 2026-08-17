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
  enclosingOpener,
  findCallSites,
  literalsBetween,
  scanRustTestFiles,
  testBlocks,
  type ScannedFile,
} from './rust-source.js';
import type { CorpusEntry } from './types.js';

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

/** How far past the property literal an adjacency guard reads. */
const ADJACENCY_LOOKAHEAD = 40;

export interface HarvestOptions {
  /** Absolute path to the workspace root. */
  workspaceRoot: string;
}

export function harvestCorpus(options: HarvestOptions): CorpusEntry[] {
  const files = scanRustTestFiles(options.workspaceRoot);

  const collected: CorpusEntry[] = [];
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
 */
const PROPERTY_VALUE_CALLS = [
  'normalize_css_property_value',
  'unchanged',
  'same',
  'diverges',
] as const;

/**
 * Shapes 1 and 6 — `normalize_css_property_value("color", "#ff0000", &opts)`,
 * `unchanged("color", "red")` and `same("color", "#ff0000", "#f00")`.
 *
 * The forms where both the property and the value are literals sitting next to
 * each other. Between them these are the bulk of the value-normalization unit
 * tests.
 */
function extractPropertyValueCalls(file: ScannedFile): CorpusEntry[] {
  const entries: CorpusEntry[] = [];

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
function extractRejectionTables(file: ScannedFile): CorpusEntry[] {
  const entries: CorpusEntry[] = [];
  const open = 'rejects(';

  for (const callStart of findCallSites(file.masked, open)) {
    const argsStart = callStart + open.length;
    const [property, ...rest] = literalsBetween(
      file,
      argsStart,
      argsStart + ARGUMENT_WINDOW.rejectionTable
    );
    if (property === undefined) continue;

    // The slice ends at the first `]` after the property, so a literal
    // belonging to a later argument is never read as one of the values.
    const sliceEnd = file.masked.indexOf(']', property.end);
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
function extractCaseTables(file: ScannedFile): CorpusEntry[] {
  const entries: CorpusEntry[] = [];

  for (const block of testBlocks(file.source)) {
    // Located on `masked`, so a call-shaped spelling inside a CSS value literal
    // cannot be read as a call.
    const call = file.masked.indexOf('normalize_css_property_value(', block.start);
    if (call === -1 || call >= block.end) continue;

    const argsStart = call + 'normalize_css_property_value('.length;
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
      if (literal.start === property.start) continue;
      if (!isTableInput(file, literal)) continue;
      entries.push(entry(property.value, literal.value, `${file.relativePath}:${literal.line}`));
    }
  }

  return entries;
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
function extractRuleLiterals(file: ScannedFile): CorpusEntry[] {
  const entries: CorpusEntry[] = [];

  for (const literal of file.literals) {
    const rule = /^\*\s*\{\{?\s*(--[\w-]+|[\w-]+)\s*:\s*([\s\S]*?)\s*;?\s*\}\}?$/.exec(
      literal.value
    );
    if (rule !== null) {
      entries.push(entry(rule[1]!, rule[2]!, `${file.relativePath}:${literal.line}`));
      continue;
    }
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
 */
function extractStyleObjects(file: ScannedFile): CorpusEntry[] {
  const entries: CorpusEntry[] = [];

  for (const literal of file.literals) {
    if (!literal.value.includes('stylex.create(')) continue;

    const declaration =
      /(--[\w-]+|'[^'\n]+'|"[^"\n]+"|[A-Za-z][A-Za-z0-9]*)\s*:\s*('([^'\\\n]|\\.)*'|"([^"\\\n]|\\.)*")/g;
    for (const match of literal.value.matchAll(declaration)) {
      const property = unquote(match[1]!);
      const value = unquote(match[2]!);
      if (!isCssProperty(property)) continue;
      if (value.includes('${')) continue;
      if (value.trim() === '') continue;
      entries.push(entry(property, value, `${file.relativePath}:${literal.line}`));
    }
  }

  return entries;
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
