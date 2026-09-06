import { describe, expect, test } from 'vitest';

import { styleObjectsOf } from '../lib/style-object.js';

/**
 * `styleObjectsOf` is the half of the comparison that can see an absent value,
 * so what it must never do is report two different shapes as the same text. Each
 * case here is a pair of module outputs that a rule-text comparison cannot tell
 * apart, plus the inputs that could make it answer nothing at all.
 */
describe('styleObjectsOf', () => {
  test('reads a style object by its $$css marker, and the marker is not a key', () => {
    expect(
      styleObjectsOf('export const styles = { x: { kMwMTN: "x1e2nbdu", $$css: true } };')
    ).toEqual(['{"kMwMTN":class}']);
  });

  test('tells an absent value apart from a missing property', () => {
    const absent = styleObjectsOf('export const styles = { x: { kMwMTN: null, $$css: true } };');
    const missing = styleObjectsOf('export const styles = { x: { $$css: true } };');

    expect(absent).toEqual(['{"kMwMTN":null}']);
    expect(missing).toEqual(['{}']);
    // The distinction the whole extension exists for: both emit no CSS.
    expect(absent).not.toEqual(missing);
  });

  test('records a class name as a placeholder rather than its hash', () => {
    // A hash divergence is already reported by the class-name comparison, so
    // repeating it here would report it twice and hide a key-set divergence
    // behind it.
    const one = styleObjectsOf('export const styles = { x: { kMwMTN: "xaaaaaa", $$css: true } };');
    const other = styleObjectsOf(
      'export const styles = { x: { kMwMTN: "xbbbbbb", $$css: true } };'
    );

    expect(one).toEqual(other);
  });

  test('records a dynamic value as an expression', () => {
    expect(
      styleObjectsOf(
        'export const styles = { x: { kMwMTN: color == null ? null : "x1", $$css: true } };'
      )
    ).toEqual(['{"kMwMTN":expression}']);
  });

  test('keeps key order, because it is what decides precedence within an object', () => {
    expect(
      styleObjectsOf('export const styles = { x: { kWkggS: null, kMwMTN: "x1", $$css: true } };')
    ).not.toEqual(
      styleObjectsOf('export const styles = { x: { kMwMTN: "x1", kWkggS: null, $$css: true } };')
    );
  });

  test('reads a quoted key, which is how a custom property is spelled', () => {
    expect(
      styleObjectsOf('export const styles = { x: { "--x": null, "--✓": "x1", $$css: true } };')
    ).toEqual(['{"--x":null,"--✓":class}']);
  });

  test('reads every style object in a module, in source order', () => {
    expect(
      styleObjectsOf(
        'export const styles = { a: { kMwMTN: null, $$css: true }, b: { kWkggS: "x1", $$css: true } };'
      )
    ).toEqual(['{"kMwMTN":null}', '{"kWkggS":class}']);
  });

  test('reads a style object nested inside other code', () => {
    expect(
      styleObjectsOf(
        'export const C = () => <i className={props({ kMwMTN: null, $$css: true })} />;'
      )
    ).toEqual(['{"kMwMTN":null}']);
  });

  test('ignores an object that carries no marker', () => {
    expect(
      styleObjectsOf('export const options = { dev: false, runtimeInjection: null };')
    ).toEqual([]);
  });

  test('ignores an object whose shape it cannot read, marker or not', () => {
    // A spread or a computed key means this is not a compiled style object, and
    // reporting a partial shape for it would be worse than reporting none.
    expect(styleObjectsOf('const a = { ...b, $$css: true };')).toEqual([]);
    expect(styleObjectsOf('const a = { [k]: null, $$css: true };')).toEqual([]);
    expect(styleObjectsOf('const a = { m() {}, $$css: true };')).toEqual([]);
  });

  test('answers nothing for input it cannot parse', () => {
    // Only reachable if this is handed something other than a compiler's own
    // output. Answering nothing makes that show up as a divergence to look at
    // rather than as a crash mid-corpus.
    expect(styleObjectsOf('export const = {{{')).toEqual([]);
    expect(styleObjectsOf('')).toEqual([]);
  });
});
