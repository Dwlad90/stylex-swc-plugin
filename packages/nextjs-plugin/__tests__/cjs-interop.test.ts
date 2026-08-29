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
    expect(() => exportAsCommonJs(null as unknown as { exports: unknown }, plugin)).not.toThrow();
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
});
