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
 */

import fs from 'node:fs';
import path from 'node:path';

import { scanRustLiterals, type RustLiteral } from './rust-literals.js';
import { SEPARATOR } from './separator.js';
import type { CorpusEntry } from './types.js';

/**
 * Property used for declarations harvested value-first, with no property in
 * sight — the arguments to the property-agnostic whitespace helpers. It is the
 * property those tests themselves use as their neutral probe, so the choice is
 * inherited rather than invented.
 */
const PROPERTY_AGNOSTIC_PROPERTY = 'height';

/** Crates whose test sources are scanned. */
const SCANNED_CRATES = ['stylex-css', 'stylex-transform'] as const;

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

/** A Rust source file, scanned once and reused by every extractor. */
interface ScannedFile {
  /** Path relative to the workspace root, for the `origin` field. */
  relativePath: string;
  source: string;
  /**
   * `source` with every string literal blanked out, same length. Bracket
   * matching runs over this: a value like `"calc(a"` carries brackets of its
   * own, and counting those as code puts the scan permanently out of step.
   */
  masked: string;
  literals: RustLiteral[];
}

export interface HarvestOptions {
  /** Absolute path to the workspace root. */
  workspaceRoot: string;
}

export function harvestCorpus(options: HarvestOptions): CorpusEntry[] {
  const files = collectRustTestFiles(options.workspaceRoot).map(absolute => {
    const source = fs.readFileSync(absolute, 'utf8');
    const literals = scanRustLiterals(source);
    return {
      relativePath: path.relative(options.workspaceRoot, absolute),
      source,
      masked: maskLiterals(source, literals),
      literals,
    } satisfies ScannedFile;
  });

  const collected: CorpusEntry[] = [];
  for (const file of files) {
    collected.push(...extractPropertyValueCalls(file));
    collected.push(...extractRejectionTables(file));
    collected.push(...extractCaseTables(file));
    collected.push(...extractRuleLiterals(file));
    collected.push(...extractStyleObjects(file));
  }

  return dedupe(collected);
}

/**
 * Every `.rs` file under a scanned crate that plausibly holds tests. Snapshot
 * directories are skipped: they hold generated output, not authored values.
 */
function collectRustTestFiles(workspaceRoot: string): string[] {
  const found: string[] = [];

  const walk = (dir: string): void => {
    let dirents: fs.Dirent[];
    try {
      dirents = fs.readdirSync(dir, { withFileTypes: true });
    } catch {
      return;
    }
    for (const dirent of dirents) {
      const absolute = path.join(dir, dirent.name);
      if (dirent.isDirectory()) {
        if (dirent.name === 'target' || dirent.name === '__swc_snapshots__') continue;
        walk(absolute);
        continue;
      }
      if (!dirent.name.endsWith('.rs')) continue;
      found.push(absolute);
    }
  };

  for (const crate of SCANNED_CRATES) {
    walk(path.join(workspaceRoot, 'crates', crate));
  }

  return found.toSorted();
}

/**
 * Calls whose first two arguments are a property literal and a value literal.
 *
 * `normalize_css_property_value` is the direct call form. `same` and `diverges`
 * build one row of a verdict case table, where the arguments after the value
 * are the expected output and the reference compiler's spelling — deliberately
 * not harvested, since deriving expectations from the reference compiler is the
 * whole point of the harness.
 */
const PROPERTY_VALUE_CALLS = ['normalize_css_property_value', 'same', 'diverges'] as const;

/**
 * Shapes 1 and 6 — `normalize_css_property_value("color", "#ff0000", &opts)`
 * and `same("color", "#ff0000", "#f00")`.
 *
 * The forms where both the property and the value are literals sitting next to
 * each other. Between them these are the bulk of the value-normalization unit
 * tests.
 */
function extractPropertyValueCalls(file: ScannedFile): CorpusEntry[] {
  const entries: CorpusEntry[] = [];

  for (const name of PROPERTY_VALUE_CALLS) {
    const open = `${name}(`;
    for (const callStart of findCallSites(file.source, open)) {
      const argsStart = callStart + open.length;
      const args = literalsBetween(file, argsStart, argsStart + 400).slice(0, 2);
      if (args.length !== 2) continue;
      entries.push(entry(args[0]!.value, args[1]!.value, `${file.relativePath}:${args[0]!.line}`));
    }
  }

  return entries;
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

  for (const callStart of findCallSites(file.source, open)) {
    const argsStart = callStart + open.length;
    const [property, ...rest] = literalsBetween(file, argsStart, argsStart + 800);
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
    const call = file.source.indexOf('normalize_css_property_value(', block.start);
    if (call === -1 || call >= block.end) continue;

    const argsStart = call + 'normalize_css_property_value('.length;
    const property = literalsBetween(file, argsStart, argsStart + 200)[0];
    if (property === undefined) continue;

    // The call already supplied a literal value; shape 1 covered it.
    const between = file.source.slice(argsStart, property.end);
    if (!/^\s*"/.test(between) && !/^\s*r#*"/.test(between)) continue;
    const afterProperty = file.source.slice(property.end, property.end + 40);
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
 * Offset of the innermost `(` or `[` still open at `index`, or `-1` if none.
 * Runs over masked source, so brackets inside a string value are not counted.
 */
function enclosingOpener(masked: string, index: number): number {
  let depth = 0;
  for (let i = index - 1; i >= 0; i--) {
    const char = masked[i];
    if (char === ')' || char === ']') depth++;
    else if (char === '(' || char === '[') {
      if (depth === 0) return i;
      depth--;
    }
  }
  return -1;
}

/**
 * `source` with each literal blanked out, preserving every offset.
 *
 * Rebuilt by slicing rather than by indexing a character array: the offsets on
 * a `RustLiteral` are UTF-16 indices, and splitting a string into code points
 * would shift every offset past the first astral character — of which the
 * corpus has several, since non-ASCII `content` values are exactly what these
 * tests cover.
 */
function maskLiterals(source: string, literals: RustLiteral[]): string {
  const parts: string[] = [];
  let cursor = 0;
  // Literals arrive in source order and never overlap, so one pass suffices.
  for (const literal of literals) {
    parts.push(source.slice(cursor, literal.start), ' '.repeat(literal.end - literal.start));
    cursor = literal.end;
  }
  parts.push(source.slice(cursor));
  return parts.join('');
}

/**
 * Shapes 3 and 4 — a whole CSS rule in one literal.
 *
 * `"* {{ transitionProperty: opacity; }}"` is what the normalizing-visitor
 * tests feed the CSS parser; `"*{color:red}"` is the minified form the
 * whitespace-repair tests operate on. Both name their property inline.
 *
 * A literal that is a bare value with no rule around it — the argument to the
 * property-agnostic whitespace helpers — is taken as a declaration on the
 * neutral probe property.
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

    if (!isPropertyAgnosticHelperArgument(file.source, literal)) continue;
    if (literal.value.trim() === '') continue;
    entries.push(
      entry(PROPERTY_AGNOSTIC_PROPERTY, literal.value, `${file.relativePath}:${literal.line}`)
    );
  }

  return entries;
}

const PROPERTY_AGNOSTIC_HELPERS = ['normalize_spacing(', 'extract_css_value('] as const;

function isPropertyAgnosticHelperArgument(source: string, literal: RustLiteral): boolean {
  const before = source.slice(Math.max(0, literal.start - 80), literal.start);
  return PROPERTY_AGNOSTIC_HELPERS.some(helper => {
    const at = before.lastIndexOf(helper);
    if (at === -1) return false;
    return before.slice(at + helper.length).trim() === '';
  });
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

/**
 * Byte offsets of every `name(` occurrence that is a call, not a definition.
 *
 * The character before the name must not be one an identifier can contain, or
 * a short name like `same` would match the tail of `is_same` and harvest a
 * declaration from a call that has nothing to do with normalization.
 */
function findCallSites(source: string, name: string): number[] {
  const sites: number[] = [];
  let at = source.indexOf(name);
  while (at !== -1) {
    const before = source.slice(Math.max(0, at - 8), at);
    const isDefinition = before.endsWith('fn ');
    const continuesAnIdentifier = at > 0 && /[\w]/.test(source[at - 1] ?? '');
    if (!isDefinition && !continuesAnIdentifier) sites.push(at);
    at = source.indexOf(name, at + name.length);
  }
  return sites;
}

/** Literals whose opening delimiter falls inside `[start, end)`. */
function literalsBetween(file: ScannedFile, start: number, end: number): RustLiteral[] {
  return file.literals.filter(literal => literal.start >= start && literal.start < end);
}

/** Offset ranges of every `#[test] fn … { … }` body in a source file. */
function testBlocks(source: string): { start: number; end: number }[] {
  const blocks: { start: number; end: number }[] = [];
  const marker = /#\[test\]/g;

  for (const match of source.matchAll(marker)) {
    const open = source.indexOf('{', match.index);
    if (open === -1) continue;
    let depth = 0;
    let i = open;
    for (; i < source.length; i++) {
      if (source[i] === '{') depth++;
      else if (source[i] === '}') {
        depth--;
        if (depth === 0) break;
      }
    }
    blocks.push({ start: open, end: i });
  }

  return blocks;
}

/**
 * Collapse duplicates by declaration, keeping the first origin seen. Values
 * repeat heavily across suites and a corpus entry costs two compiler runs.
 */
function dedupe(entries: CorpusEntry[]): CorpusEntry[] {
  const byId = new Map<string, CorpusEntry>();
  for (const candidate of entries) {
    if (!byId.has(candidate.id)) byId.set(candidate.id, candidate);
  }
  return [...byId.values()].toSorted((a, b) =>
    a.property === b.property
      ? a.value.localeCompare(b.value)
      : a.property.localeCompare(b.property)
  );
}

export function entry(property: string, value: string, origin: string): CorpusEntry {
  return { id: entryId(property, value), property, value, origin };
}

/**
 * The identity of a declaration, for deduplication and hashing.
 *
 * The separator is a NUL because it is the one character a CSS property name
 * and a CSS value cannot contain, so no pair of distinct declarations can
 * collide onto one key.
 */
export function declarationKey(property: string, value: string): string {
  return `${property}${SEPARATOR}${value}`;
}

/**
 * Identify an entry by its declaration rather than its position, so that
 * re-harvesting after a test file moves does not renumber the whole corpus and
 * bury the real diff.
 */
export function entryId(property: string, value: string): string {
  let hash = 0x811c9dc5;
  const text = declarationKey(property, value);
  for (let i = 0; i < text.length; i++) {
    hash ^= text.codePointAt(i) ?? 0;
    hash = Math.imul(hash, 0x01000193) >>> 0;
  }
  return hash.toString(16).padStart(8, '0');
}
