import { describe, expect, test } from 'vitest';

import { exportAsCommonJs } from '../src/cjs-interop';

const plugin = () => 'plugin';

describe('exportAsCommonJs', () => {
  test('publishes the plugin as the whole module', () => {
    const host = { exports: {} };

    exportAsCommonJs(host, plugin);

    expect(host.exports).toBe(plugin);
  });

  test('also publishes the plugin under default', () => {
    const host = { exports: {} };

    exportAsCommonJs(host, plugin);

    expect((host.exports as { default: unknown }).default).toBe(plugin);
  });

  test('does nothing when the module record is absent', () => {
    expect(() => exportAsCommonJs(undefined, plugin)).not.toThrow();
  });

  test('does nothing when the module record is null', () => {
    expect(() => exportAsCommonJs(null, plugin)).not.toThrow();
  });

  // An ES module namespace gives a getter, which a write cannot use.
  test('does nothing when exports has only a getter', () => {
    const host = Object.defineProperty({}, 'exports', {
      get: () => ({}),
      configurable: true,
    }) as { exports: unknown };

    exportAsCommonJs(host, plugin);

    expect(host.exports).not.toBe(plugin);
  });

  test('does nothing when exports is read only', () => {
    const host = Object.defineProperty({}, 'exports', {
      value: {},
      writable: false,
      configurable: true,
    }) as { exports: unknown };

    exportAsCommonJs(host, plugin);

    expect(host.exports).not.toBe(plugin);
  });

  test('does nothing when the module record is frozen', () => {
    const host = Object.freeze({ exports: {} });

    exportAsCommonJs(host, plugin);

    expect(host.exports).not.toBe(plugin);
  });

  test('ignores an exports property that the prototype owns', () => {
    const host = Object.create({ exports: {} }) as { exports: unknown };

    exportAsCommonJs(host, plugin);

    expect(host.exports).not.toBe(plugin);
  });

  test('runs twice without a change in result', () => {
    const host = { exports: {} };

    exportAsCommonJs(host, plugin);
    exportAsCommonJs(host, plugin);

    expect(host.exports).toBe(plugin);
    expect((host.exports as { default: unknown }).default).toBe(plugin);
  });
  // The rspack and webpack plugins publish a loader path beside the plugin.
  test('publishes the further names beside the plugin', () => {
    const host = { exports: {} };

    exportAsCommonJs(host, plugin, { loader: '/loader.js', StyleXPlugin: plugin });

    expect(host.exports).toBe(plugin);
    expect((host.exports as Record<string, unknown>).loader).toBe('/loader.js');
    expect((host.exports as Record<string, unknown>).StyleXPlugin).toBe(plugin);
  });

  test('lets a further name overwrite default', () => {
    const host = { exports: {} };
    const other = () => 'other';

    exportAsCommonJs(host, plugin, { default: other });

    expect((host.exports as Record<string, unknown>).default).toBe(other);
  });

  test('accepts an empty set of further names', () => {
    const host = { exports: {} };

    exportAsCommonJs(host, plugin, {});

    expect(host.exports).toBe(plugin);
  });

  // A namespace reports its properties as writable, yet every write to one
  // fails. The plugins that are read as an ES module land here.
  test('does nothing when exports is an ES module namespace', () => {
    const namespace = Object.defineProperty({}, Symbol.toStringTag, { value: 'Module' });
    const host = { exports: namespace };

    exportAsCommonJs(host, plugin);

    expect(host.exports).toBe(namespace);
  });

  test('writes no further name when the write cannot succeed', () => {
    const host = Object.freeze({ exports: {} });

    exportAsCommonJs(host, plugin, { loader: '/loader.js' });

    expect((host.exports as Record<string, unknown>).loader).toBeUndefined();
  });

  test('carries a large set of further names', () => {
    const host = { exports: {} };
    const named = Object.fromEntries(
      Array.from({ length: 1_000 }, (_unused, index) => [`name${index}`, index])
    );

    exportAsCommonJs(host, plugin, named);

    expect((host.exports as Record<string, unknown>).name999).toBe(999);
  });
  // A function keeps `name` and `length` read only. A plain write to one of
  // them fails and stops the whole file from loading.
  test('passes over a name the plugin holds as read only', () => {
    const host = { exports: {} };

    expect(() => exportAsCommonJs(host, plugin, { name: 'renamed', length: 9 })).not.toThrow();
    expect(host.exports).toBe(plugin);
    expect(plugin.name).toBe('plugin');
  });

  test('writes the other names when one of them is read only', () => {
    const host = { exports: {} };

    exportAsCommonJs(host, plugin, { name: 'renamed', loader: '/loader.js' });

    expect((host.exports as Record<string, unknown>).loader).toBe('/loader.js');
  });

  test('leaves the prototype alone when a name is __proto__', () => {
    const host = { exports: {} };

    exportAsCommonJs(host, plugin, { __proto__: { polluted: true } });

    expect(({} as { polluted?: unknown }).polluted).toBeUndefined();
  });
});
