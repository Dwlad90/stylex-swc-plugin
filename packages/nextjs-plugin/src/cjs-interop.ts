/** The parts of a Node.js module record that the export below writes. */
type ModuleRecord = { exports: unknown };

/**
 * Publishes a config plugin under CommonJS.
 *
 * Next.js reads a config plugin with `require`, so `module.exports` must be the
 * plugin function itself, not an object that holds it. A bundler or a test
 * runner reads the same file as an ES module. There `module.exports` is read
 * only, and a write to it stops the file from loading. Write only when a write
 * can succeed.
 *
 * Give the caller's own module record to `host`. A module record is not in
 * scope under an ES module, so guard the argument:
 *
 * ```ts
 * exportAsCommonJs(typeof module === 'undefined' ? undefined : module, withStyleX);
 * ```
 */
export function exportAsCommonJs(host: ModuleRecord | undefined, plugin: unknown): void {
  if (host === undefined || host === null) {
    return;
  }

  const descriptor = Object.getOwnPropertyDescriptor(host, 'exports');

  if (descriptor?.writable !== true) {
    return;
  }

  host.exports = plugin;

  // Keeps `require(...).default` working for callers that read the ES form.
  (host.exports as { default: unknown }).default = plugin;
}
