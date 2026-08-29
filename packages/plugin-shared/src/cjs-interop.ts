/** The parts of a Node.js module record that the export below writes. */
interface ModuleRecord {
  exports: unknown;
}

/** Tells whether a value is an ES module namespace, which no write can change. */
const isModuleNamespace = (value: unknown): boolean =>
  Object.prototype.toString.call(value) === '[object Module]';

/**
 * Publishes a plugin under CommonJS.
 *
 * A host such as Next.js, webpack or PostCSS reads a plugin with `require`, so
 * `module.exports` must be the plugin itself, not an object that holds it. A
 * bundler or a test runner reads the same file as an ES module. There
 * `module.exports` is read only, and a write to it stops the file from
 * loading. Write only when a write can succeed.
 *
 * Give the caller's own module record to `host`. A module record is not in
 * scope under an ES module, so guard the argument:
 *
 * ```ts
 * exportAsCommonJs(typeof module === 'undefined' ? undefined : module, StyleXPlugin);
 * ```
 *
 * `null` is accepted as well as `undefined`, because a host that builds the
 * module record itself can leave the field empty.
 *
 * @param host - the caller's module record, or nothing under an ES module
 * @param plugin - the value that `require` of this file must give back
 * @param named - further names to publish beside the plugin, such as a loader
 *   path. `default` is always published and needs no entry here. A name the
 *   plugin already holds as a read-only property is passed over: a function
 *   keeps `name` and `length` read only, and a write to one of them fails.
 *   Give `__proto__` no entry, because a plain write to that name changes the
 *   prototype instead of adding a property.
 */
export function exportAsCommonJs(
  host: ModuleRecord | null | undefined,
  plugin: unknown,
  named: Readonly<Record<string, unknown>> = {}
): void {
  if (host == null) {
    return;
  }

  // A namespace reports its properties as writable, but every write to a
  // namespace fails. Test for a namespace first. The descriptor test below
  // cannot catch this case.
  if (isModuleNamespace(host.exports)) {
    return;
  }

  const descriptor = Object.getOwnPropertyDescriptor(host, 'exports');

  if (descriptor?.writable !== true) {
    return;
  }

  host.exports = plugin;

  const published = host.exports as Record<string, unknown>;

  // Keeps `require(...).default` working for callers that read the ES form.
  publish(published, 'default', plugin);

  for (const [name, value] of Object.entries(named)) {
    publish(published, name, value);
  }
}

/**
 * Writes one name on the published plugin. A read-only name is passed over,
 * because a write to one fails and stops the whole file from loading.
 */
function publish(published: Record<string, unknown>, name: string, value: unknown): void {
  const descriptor = Object.getOwnPropertyDescriptor(published, name);

  if (descriptor !== undefined && descriptor.writable !== true) {
    return;
  }

  published[name] = value;
}
