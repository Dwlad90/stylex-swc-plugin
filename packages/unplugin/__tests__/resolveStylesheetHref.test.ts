import { describe, expect, test } from 'vitest';

import resolveStylesheetHref from '../src/utils/resolveStylesheetHref';

describe('resolveStylesheetHref', () => {
  test('should return the root-relative path unchanged for the default base', () => {
    expect(resolveStylesheetHref('/', '/stylex.css')).toBe('/stylex.css');
  });

  test('should return the root-relative path unchanged when no base is known', () => {
    expect(resolveStylesheetHref(undefined, '/stylex.css')).toBe('/stylex.css');
  });

  test('should return the root-relative path unchanged for an empty base', () => {
    expect(resolveStylesheetHref('', '/stylex.css')).toBe('/stylex.css');
  });

  test('should prefix a sub-path base', () => {
    expect(resolveStylesheetHref('/app/', '/stylex.css')).toBe('/app/stylex.css');
  });

  test('should prefix a full-URL base', () => {
    expect(resolveStylesheetHref('https://cdn.example.com/app/', '/stylex.css')).toBe(
      'https://cdn.example.com/app/stylex.css'
    );
  });

  test('should not double the slash when the base has no trailing slash', () => {
    expect(resolveStylesheetHref('/app', '/stylex.css')).toBe('/app/stylex.css');
  });

  test('should produce a document-relative href for a relative base', () => {
    expect(resolveStylesheetHref('./', '/stylex.css', '/index.html')).toBe('./stylex.css');
  });

  test('should climb out of a nested document for a relative base', () => {
    expect(resolveStylesheetHref('./', '/stylex.css', '/pages/about.html')).toBe('../stylex.css');
  });

  test('should climb once per nested level for a relative base', () => {
    expect(resolveStylesheetHref('./', '/stylex.css', '/a/b/c.html')).toBe('../../stylex.css');
  });

  test('should fall back to the document itself when no document path is known', () => {
    expect(resolveStylesheetHref('./', '/stylex.css')).toBe('./stylex.css');
  });

  test('should preserve a hashed file name', () => {
    expect(resolveStylesheetHref('https://cdn.example.com/app/', '/stylex.ca990dcb.css')).toBe(
      'https://cdn.example.com/app/stylex.ca990dcb.css'
    );
  });

  test('should preserve a nested assets directory', () => {
    expect(resolveStylesheetHref('/app/', '/assets/stylex.css')).toBe('/app/assets/stylex.css');
  });

  test('should treat a dotted base that is not the relative base as an ordinary prefix', () => {
    expect(resolveStylesheetHref('/../app/', '/stylex.css', '/pages/about.html')).toBe(
      '/../app/stylex.css'
    );
  });

  test('should ignore the document path unless the base is relative', () => {
    expect(resolveStylesheetHref('/app/', '/stylex.css', '/pages/about.html')).toBe(
      '/app/stylex.css'
    );
  });
});
