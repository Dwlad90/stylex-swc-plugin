/**
 * Resolves the href for the injected stylesheet link against the host's base,
 * so the stylesheet is served from wherever the rest of the bundle is served
 * from: a sub-path, a CDN origin, or the document itself for a relative base.
 *
 * The root-relative path stays the fallback, which is what keeps a nested route
 * resolving the stylesheet against the origin rather than against its own
 * directory.
 *
 * @param base The host's resolved base. Vite normalizes this to a trailing
 *   slash, but an unset or unnormalized value is tolerated.
 * @param rootRelativeFileName The emitted asset path, with a leading slash.
 * @param documentPath The path of the HTML document the link is injected into,
 *   needed only for a relative base, where the href resolves against the
 *   document rather than the origin.
 */
export default function resolveStylesheetHref(
  base: string | undefined,
  rootRelativeFileName: string,
  documentPath?: string
): string {
  if (!base || base === '/') {
    return rootRelativeFileName;
  }

  // A relative base makes the whole output origin-agnostic, so the href has to
  // climb back to the output root the same way the host's own asset URLs do.
  // Vite normalizes every relative base to exactly `./` when it resolves the
  // config, so anything else starting with a dot is an ordinary prefix.
  if (base === RELATIVE_BASE) {
    return `${climbToOutputRoot(documentPath)}${rootRelativeFileName.slice(1)}`;
  }

  return `${base.replace(/\/$/, '')}${rootRelativeFileName}`;
}

const RELATIVE_BASE = './';

// The document path is a URL path, not a file system path: hosts normalize the
// separator to `/` before it gets here, so this stays correct on Windows.
function climbToOutputRoot(documentPath: string | undefined): string {
  const depth = documentPath ? documentPath.split('/').filter(Boolean).length - 1 : 0;

  return depth > 0 ? '../'.repeat(depth) : RELATIVE_BASE;
}
