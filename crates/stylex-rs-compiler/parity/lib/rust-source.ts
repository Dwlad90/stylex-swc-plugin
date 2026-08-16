/**
 * Reading Rust test sources as text.
 *
 * The primitives an extractor needs before it can recognize anything: which
 * files to scan, where the string literals are, which brackets enclose an
 * offset, where a `#[test]` body starts and stops. None of them know what a
 * CSS declaration is — that is `harvest.ts`. Kept apart because they change
 * for different reasons: these change when Rust source is laid out
 * differently, the extractors change when a test is written differently.
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

/** Every scannable Rust test source under the workspace, read and masked. */
export function scanRustTestFiles(workspaceRoot: string): ScannedFile[] {
  return collectRustTestFiles(workspaceRoot).map(absolute => {
    const source = fs.readFileSync(absolute, 'utf8');
    const literals = scanRustLiterals(source);
    return {
      relativePath: path.relative(workspaceRoot, absolute),
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
  return file.literals.filter(literal => literal.start >= start && literal.start < end);
}

/** Offset ranges of every `#[test] fn … { … }` body in a source file. */
export function testBlocks(source: string): { start: number; end: number }[] {
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
