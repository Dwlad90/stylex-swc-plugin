import crypto from 'node:crypto';
import * as path from 'node:path';

/**
 * The digest a renamed stylesheet carries when the host cannot be asked to hash
 * it. Eight hex characters, which is the length every host in reach uses.
 */
export function shortContentHash(css: string): string {
  return crypto.createHash('sha256').update(css).digest('hex').slice(0, 8);
}

/** A path in the shape bundlers write, whatever the platform separator is. */
export function toPosixPath(filePath: string): string {
  return filePath.split(path.sep).join('/');
}

/**
 * Whether a host that names its assets from a pattern puts a content hash in
 * them.
 *
 * A pattern with no `[hash]` is the user opting out of cache busting, and a
 * rename there would only make a second file under the same name. A function
 * decides per asset and cannot be read, so it is taken to hash; the caller
 * compares the resulting name and stands down when it did not change.
 */
export function assetNamesCarryHash(pattern: unknown): boolean {
  return typeof pattern === 'string' ? pattern.includes('[hash]') : true;
}

/**
 * The path a Vite build manifest is written to: `true` takes the default, a
 * string names the file, and anything else means no manifest at all.
 */
export function resolveManifestFileName(
  setting: boolean | string | undefined,
  fallback: string
): string | null {
  if (setting === true) return fallback;

  return typeof setting === 'string' ? setting : null;
}

/** The tokens esbuild expands in an output name template. */
const ESBUILD_NAME_TOKEN_RE = /(\[(?:dir|name|hash|ext)\])/;

const REGEXP_SPECIAL_RE = /[.*+?^${}()|[\]\\]/g;

/** What `[dir]` and `[name]` both become: text of any length, matched lazily. */
const WILDCARD = '.*?';

/**
 * The regular-expression source for one template part.
 *
 * `[name]` and `[dir]` are both wildcards. `[name]` is lazy, which is how a name
 * that holds dashes and capitals still gives the trailing run to the hash: the
 * hash class takes no separator, so the match can settle only one way.
 */
function esbuildTokenSource(part: string): string {
  if (part === '[hash]') return '([A-Z0-9]+)';
  if (part === '[dir]' || part === '[name]') return WILDCARD;
  if (part === '[ext]') return '[^./]*';

  return part.replace(REGEXP_SPECIAL_RE, '\\$&');
}

/**
 * Turns an esbuild output name template into a pattern that reads back a name
 * the template produced, capturing where the hash sits.
 *
 * esbuild has no rename API and its `[hash]` cannot be reproduced. The token
 * mixes in the output directory and the template itself, not the contents
 * alone. A renamed stylesheet therefore carries a hash of ours, and the
 * template is read only to find the old one.
 *
 * Two wildcards side by side are collapsed into one. They match the same text
 * either way, but a run of them makes the engine try every way to divide that
 * text between them, and the cost of a name that cannot match then grows with
 * the length of the run. A template such as `[dir]/[name][name]` is nonsense
 * that an author can still write, so the pattern refuses to be slow over it.
 *
 * `null` when the template asks for no hash, which is esbuild's default. The
 * user opted out of cache busting, so there is nothing to keep fresh.
 */
export function esbuildNamePattern(template: string): RegExp | null {
  if (!template.includes('[hash]')) return null;

  const source = template
    // esbuild drops the separator together with the directory when the output
    // sits at the top of the output directory. The pair is read as one unit.
    .replace('[dir]/', '[dir]')
    .split(ESBUILD_NAME_TOKEN_RE)
    // Splitting on a capturing group leaves an empty string between two
    // neighbouring tokens, which would hide the pair from the collapse below.
    .filter(part => part !== '')
    .map(esbuildTokenSource)
    .filter((part, index, parts) => part !== WILDCARD || parts[index - 1] !== WILDCARD)
    .join('');

  // `d` for the hash position. The name is rebuilt around that position rather
  // than rendered again, because `[name]` and `[dir]` are not known here.
  return new RegExp(`^${source}$`, 'd');
}

/**
 * The path a written stylesheet must answer to now.
 *
 * `null` leaves the name as it is, because the name does not read back as one
 * the template produced.
 */
export function renameEsbuildStylesheet(
  pattern: RegExp,
  outDir: string,
  cssFile: string,
  source: string
): string | null {
  const extension = path.extname(cssFile);
  const relative = toPosixPath(path.relative(outDir, cssFile));
  const stem = relative.slice(0, relative.length - extension.length);
  const span = pattern.exec(stem)?.indices?.[1];

  if (!span) return null;

  const [start, end] = span;
  // Upper case, so the segment still looks like the rest of the name. esbuild
  // writes its own hashes in capitals and digits.
  const hash = shortContentHash(source).toUpperCase();

  return path.join(outDir, stem.slice(0, start) + hash + stem.slice(end) + extension);
}

/** One stylesheet the injection wrote, and the name it must answer to now. */
export interface StylesheetRename {
  readonly from: string;
  readonly to: string;
}

/**
 * Replaces every old stylesheet name in one text with its new one.
 *
 * One pass over the text, so a rename whose new name is another rename's old
 * name cannot be rewritten twice, and so the cost does not grow with the number
 * of renames. Returns the text unchanged when it names none of them.
 */
export function applyStylesheetRenames(text: string, renames: readonly StylesheetRename[]): string {
  if (renames.length === 0) return text;

  const replacements = new Map(renames.map(rename => [rename.from, rename.to]));
  // Longest first, so a name that contains a shorter one is matched whole.
  const alternation = [...replacements.keys()]
    .toSorted((a, b) => b.length - a.length)
    .map(name => name.replace(REGEXP_SPECIAL_RE, '\\$&'))
    .join('|');

  return text.replace(new RegExp(alternation, 'g'), match => replacements.get(match) ?? match);
}
