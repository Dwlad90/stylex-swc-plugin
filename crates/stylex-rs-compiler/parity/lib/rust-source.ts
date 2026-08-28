/**
 * Reading Rust sources as text.
 *
 * The primitives an extractor needs before it can recognize anything: which
 * files to scan, where the string literals are, which brackets enclose an
 * offset, where a `#[test]` body starts and stops, and what a `phf_set!`
 * declares. None of them know what a CSS declaration is — that is
 * `harvest.ts`. Kept apart because they change for different reasons: these
 * change when Rust source is laid out differently, the extractors change when a
 * test is written differently.
 */

import fs from 'node:fs';
import path from 'node:path';

import { scanRustLiterals, type RustLiteral } from './rust-literals.js';

/** Crates whose test sources are scanned. */
const SCANNED_CRATES = ['stylex-css', 'stylex-transform'] as const;

/** A Rust source file, scanned once and reused by every extractor. */
export interface ScannedFile {
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

/**
 * Every scannable Rust test source under the workspace, read and masked.
 *
 * Two normalizations, both so that the committed corpus is a function of the
 * repository rather than of the checkout. Line endings collapse to `\n`,
 * because a CRLF checkout would otherwise put a `\r` inside every multi-line
 * value and change both the value and the FNV id derived from it. Separators in
 * `relativePath` collapse to `/`, because that path is committed as an entry's
 * `origin`.
 */
export function scanRustTestFiles(workspaceRoot: string): ScannedFile[] {
  return collectRustTestFiles(workspaceRoot).map(absolute => {
    const source = fs.readFileSync(absolute, 'utf8').replaceAll('\r\n', '\n');
    const literals = scanRustLiterals(source);
    return {
      relativePath: path.relative(workspaceRoot, absolute).split(path.sep).join('/'),
      source,
      masked: maskLiterals(source, literals),
      literals,
    } satisfies ScannedFile;
  });
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
 * `source` with each literal blanked out, preserving every offset.
 *
 * Rebuilt by slicing rather than by indexing a character array: the offsets on
 * a `RustLiteral` are UTF-16 indices, and splitting a string into code points
 * would shift every offset past the first astral character — of which the
 * corpus has several, since non-ASCII `content` values are exactly what these
 * tests cover.
 *
 * The result is the same length as `source`, which is the whole contract:
 * every offset the harvester compares against the mask is an offset into the
 * source. Exported so that invariant can be asserted directly.
 */
export function maskLiterals(source: string, literals: RustLiteral[]): string {
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
 * Offset of the innermost `(` or `[` still open at `index`, or `-1` if none.
 * Runs over masked source, so brackets inside a string value are not counted.
 */
export function enclosingOpener(masked: string, index: number): number {
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
 * Byte offsets of every `name(` occurrence that is a call, not a definition.
 *
 * The character before the name must not be one an identifier can contain, or
 * a short name like `same` would match the tail of `is_same` and harvest a
 * declaration from a call that has nothing to do with normalization.
 */
export function findCallSites(source: string, name: string): number[] {
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
export function literalsBetween(file: ScannedFile, start: number, end: number): RustLiteral[] {
  return literalsWithin(file.literals, start, end);
}

/** The same range, over literals a caller scanned rather than a whole file. */
function literalsWithin(literals: RustLiteral[], start: number, end: number): RustLiteral[] {
  return literals.filter(literal => literal.start >= start && literal.start < end);
}

/** Offset ranges of every `#[test] fn … { … }` body in a source file. */
export function testBlocks(source: string): { start: number; end: number }[] {
  const blocks: { start: number; end: number }[] = [];
  const marker = /#\[test\]/g;

  for (const match of source.matchAll(marker)) {
    const open = source.indexOf('{', match.index);
    if (open === -1) continue;
    const close = closingBrace(source, open);
    // An unclosed body runs to the end of the file, which is what the walk
    // answered before it was named.
    blocks.push({ start: open, end: close === -1 ? source.length : close });
  }

  return blocks;
}

/**
 * The string members of the `phf_set!` declared as `name`, or `undefined` where
 * the source declares no such set.
 *
 * Read so that a list the compiler owns can be asserted against a list a
 * harness keeps beside it, rather than the two agreeing today and drifting
 * silently afterwards. It is not a Rust parser and does not need to be: the
 * declaration is found by name, its braces are matched, and the literals inside
 * them are the members — which is the same masked-source, matched-bracket
 * approach every extractor above uses.
 *
 * What it reads is the *declaration*, and only that. A name is mentioned in a
 * `use`, in a comment and at every call site, and any of those followed by
 * somebody else's `phf_set!` would answer with the wrong set — a list that
 * loads, compares and passes while measuring another constant entirely. So an
 * occurrence counts only where Rust declares one: `static NAME:` or `const
 * NAME:`, with `phf_set!` reached before the statement ends.
 */
export function phfSetMembers(source: string, name: string): string[] | undefined {
  const literals = scanRustLiterals(source);
  const masked = maskLiterals(source, literals);

  for (const at of findDeclarations(masked, name)) {
    const macro = masked.indexOf('phf_set!', at);
    const ends = masked.indexOf(';', at);
    if (macro === -1 || (ends !== -1 && ends < macro)) continue;

    const open = masked.indexOf('{', macro);
    const close = open === -1 ? -1 : closingBrace(masked, open);
    if (close === -1) continue;

    return literalsWithin(literals, open + 1, close).map(literal => literal.value);
  }

  return undefined;
}

/**
 * Offsets where `name` is declared as a static or a const, in source order.
 *
 * The keyword in front of it is what tells a declaration from a mention, and
 * the same test settles the other way a text scan answers wrongly: a short name
 * cannot be found inside a longer one, because what precedes it there is the
 * rest of that name rather than `static` or `const`.
 */
function findDeclarations(masked: string, name: string): number[] {
  const found: number[] = [];
  let at = masked.indexOf(name);
  while (at !== -1) {
    // Long enough to hold either keyword, the space after it and one character
    // before — which is what keeps `mystatic` from reading as `static`.
    const before = masked.slice(Math.max(0, at - 16), at);
    if (/(?:^|\W)(?:static|const)\s+$/.test(before)) found.push(at);
    at = masked.indexOf(name, at + name.length);
  }

  return found;
}

/** Offset of the `}` closing the `{` at `open`, or `-1` where none does. */
function closingBrace(masked: string, open: number): number {
  let depth = 0;
  for (let i = open; i < masked.length; i += 1) {
    if (masked[i] === '{') depth += 1;
    else if (masked[i] === '}') {
      depth -= 1;
      if (depth === 0) return i;
    }
  }

  return -1;
}
