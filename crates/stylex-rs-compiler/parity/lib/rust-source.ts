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

/**
 * How far into a source the `@generated` header is looked for.
 *
 * A marker further down than this is a mention in a test value, not a header.
 */
const GENERATED_HEADER_WINDOW = 512;

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
  const scanned: ScannedFile[] = [];

  for (const absolute of collectRustTestFiles(workspaceRoot)) {
    const source = fs.readFileSync(absolute, 'utf8').replaceAll('\r\n', '\n');
    if (isGenerated(source)) continue;

    const literals = scanRustLiterals(source);
    scanned.push({
      relativePath: path.relative(workspaceRoot, absolute).split(path.sep).join('/'),
      source,
      masked: maskLiterals(source, literals),
      literals,
    } satisfies ScannedFile);
  }

  return scanned;
}

/**
 * Whether a source declares itself generated in its header comment.
 *
 * Only the leading comment block counts. A file that spells the marker in a
 * test value further down is still scanned.
 *
 * These are skipped because the chain closes into a loop otherwise. The corpus
 * generates `postcss-value-parser`'s `cases.rs`, and that file spells its
 * inputs as CSS rules. A scan of it feeds the corpus its own output back.
 */
function isGenerated(source: string): boolean {
  for (const line of source.slice(0, GENERATED_HEADER_WINDOW).split('\n')) {
    const text = line.trim();
    if (text === '') continue;
    if (!text.startsWith('//')) return false;
    if (text.includes('@generated')) return true;
  }

  return false;
}

/**
 * Every `.rs` file under `crates/`, `src/` and `benches/` included. A value in
 * a bench or in an inline `mod tests` counts the same as one under `tests/`,
 * and telling them apart would need a Rust parser. Snapshot directories are
 * skipped: they hold generated output, not authored values.
 *
 * The crate names come off the tree, not from a list. A list must be widened by
 * hand, and a list that nobody widens loses values in silence. That happened
 * once, when a crate was split apart. See `parity/README.md`.
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

  walk(path.join(workspaceRoot, 'crates'));

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

/** A callee name, or the start of one: `assert_eq!`, `contains`, `vec!`. */
const CALLEE_NAME = /([A-Za-z_]\w*!?)\s*$/;

/** How far behind a bracket its callee name is looked for. */
const CALLEE_LOOKBEHIND = 64;

/**
 * How far behind an offset the calls enclosing it are looked for.
 *
 * A window rather than the whole statement, because a statement has no bound.
 * A case table is one statement holding hundreds of rows, none of which closes
 * it, so every literal in it would walk back to the `let` — which is the walk
 * repeated once per row, and four times the work for twice the rows.
 *
 * Sized to the widest call in the suites with room to spare: the furthest a
 * callee that disqualifies a literal sits behind it is about a hundred
 * characters. A literal that outruns the window is harvested rather than
 * dropped, which is the safe way round for a guard.
 */
const CALL_LOOKBEHIND = 512;

/**
 * The callees of every call whose argument list encloses `index`, innermost
 * first.
 *
 * This is what says where a literal was *going*, which is the one thing the
 * spelling of a literal cannot say: `"width: limit 64, found 65"` reads as a
 * declaration and is an assertion message, and `", "` reads as a value and is
 * the separator of a `join`. An extractor that sweeps a whole block asks this
 * before it believes a literal.
 *
 * The walk stops at the window above and at a `;` or a brace, whichever comes
 * first. A brace is the coarser of the two bounds: an argument list does cross
 * one, in the body of a closure, so a literal there reports none of the calls
 * outside it. That is deliberate. The suites wrap the call under test in
 * `catch_unwind(AssertUnwindSafe(|| { … }))`, whose literal is the *value* the
 * compiler is given, and reading the assertion around it would drop every one
 * of them. What it costs is a message written inside a closure, which stays.
 */
export function enclosingCallees(masked: string, index: number): string[] {
  const from = Math.max(0, index - CALL_LOOKBEHIND);
  const code = blankOpaque(masked.slice(from, index), opensInLineComment(masked, from));

  const callees: string[] = [];
  let depth = 0;

  for (let i = code.length - 1; i >= 0; i -= 1) {
    const char = code[i];
    const opens = char === '(' || char === '[';
    const closes = char === ')' || char === ']';

    if (!opens && !closes) {
      if (depth === 0 && (char === ';' || char === '{' || char === '}')) break;
      continue;
    }
    if (closes) {
      depth += 1;
      continue;
    }
    // An opener at depth belongs to a call that closed again before the
    // offset, so the offset is not inside it.
    if (depth > 0) {
      depth -= 1;
      continue;
    }

    const callee = CALLEE_NAME.exec(code.slice(Math.max(0, i - CALLEE_LOOKBEHIND), i));
    if (callee !== null) callees.push(callee[1]!);
  }

  return callees;
}

/**
 * Whether the window opens inside a line comment, which the forward scan below
 * cannot see for itself: the `//` that starts one may sit in front of the
 * window.
 *
 * Looked for inside the window's own length, so that one very long line cannot
 * make this the walk it was written to avoid. A `//` further back than that on
 * the same line is not found, and the prose after it reads as code.
 */
function opensInLineComment(masked: string, from: number): boolean {
  const lineStart = masked.lastIndexOf('\n', from) + 1;
  return masked.slice(Math.max(lineStart, from - CALL_LOOKBEHIND), from).includes('//');
}

/**
 * `text` with every comment and character literal blanked, same length.
 *
 * Only string literals are masked, and the rest of what a Rust source writes
 * that is not code carries brackets of its own: `matches('(')` counts a
 * parenthesis that closes nothing, and a `calc(` or a `;` written in prose
 * ends or redirects the walk. Both are the same defect as an unmasked string.
 *
 * The one text this cannot place is a block comment that opened before the
 * window. It is a comment holding a bracket, spanning lines, inside an
 * argument list — which no suite writes.
 */
function blankOpaque(text: string, commented: boolean): string {
  const parts: string[] = [];
  // A comment the window opened inside runs to the end of its line.
  let cursor = commented ? text.indexOf('\n') + 1 || text.length : 0;
  parts.push(' '.repeat(cursor));

  for (let i = cursor; i < text.length; i += 1) {
    const end = opaqueEnd(text, i);
    if (end === -1) continue;
    parts.push(text.slice(cursor, i), ' '.repeat(end - i));
    cursor = end;
    i = end - 1;
  }
  parts.push(text.slice(cursor));

  return parts.join('');
}

/**
 * The offset just past the comment or character literal starting at `i`, or
 * `-1` where the character there starts neither. An unterminated one runs to
 * the end of the text.
 */
function opaqueEnd(text: string, i: number): number {
  const char = text[i];

  if (char === '/' && text[i + 1] === '/') {
    const newline = text.indexOf('\n', i + 2);
    return newline === -1 ? text.length : newline;
  }
  if (char === '/' && text[i + 1] === '*') {
    const close = text.indexOf('*/', i + 2);
    return close === -1 ? text.length : close + 2;
  }
  // A lifetime — `&'static str` — is an apostrophe that opens nothing, so a
  // character literal counts only where a closing quote follows the body.
  if (char !== "'") return -1;
  const body = text[i + 1] === '\\' ? i + 3 : i + 2;
  return text[body] === "'" ? body + 1 : -1;
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
