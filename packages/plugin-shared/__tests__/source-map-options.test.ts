// Shared by the webpack/rspack loader and the Turbopack loader, so it is
// tested here directly rather than only through either one.
import { SourceMaps } from '@stylexswc/rs-compiler';
import { afterEach, describe, expect, test } from 'vitest';

import { resolveSourceMapOptions } from '../src/source-map-options';

describe('resolveSourceMapOptions', () => {
  describe('sourceMap', () => {
    test('leaves the compiler default alone when the host says maps are on', () => {
      expect(resolveSourceMapOptions({}, true).sourceMap).toBeUndefined();
    });

    test('disables maps when the host says they are off', () => {
      expect(resolveSourceMapOptions({}, false).sourceMap).toBe(SourceMaps.False);
    });

    test('keeps maps on when the host says nothing', () => {
      // Turbopack's loader context may never define `this.sourceMap`. Treating
      // that as "off" would silently strip the map from every file.
      expect(resolveSourceMapOptions({}, undefined).sourceMap).toBeUndefined();
    });

    test('an explicit rsOptions.sourceMap wins over the host', () => {
      expect(resolveSourceMapOptions({ sourceMap: SourceMaps.True }, false).sourceMap).toBe(
        SourceMaps.True
      );
      expect(resolveSourceMapOptions({ sourceMap: SourceMaps.False }, true).sourceMap).toBe(
        SourceMaps.False
      );
    });
  });

  describe('inlineSourcesContent', () => {
    const { NODE_ENV } = process.env;

    afterEach(() => {
      process.env.NODE_ENV = NODE_ENV;
    });

    test('inlines in a development build', () => {
      expect(resolveSourceMapOptions({ dev: true }, true).inlineSourcesContent).toBe(true);
    });

    test('does not inline in a production build', () => {
      expect(resolveSourceMapOptions({ dev: false }, true).inlineSourcesContent).toBe(false);
    });

    test('falls back to NODE_ENV when dev is unset', () => {
      // The Turbopack wiring passes plugin options through statically and never
      // sets `dev`, so this fallback is the one that runs there.
      process.env.NODE_ENV = 'production';
      expect(resolveSourceMapOptions({}, undefined).inlineSourcesContent).toBe(false);

      process.env.NODE_ENV = 'development';
      expect(resolveSourceMapOptions({}, undefined).inlineSourcesContent).toBe(true);
    });

    test('an explicit value wins over the build mode', () => {
      expect(
        resolveSourceMapOptions({ dev: false, inlineSourcesContent: true }, true)
          .inlineSourcesContent
      ).toBe(true);
      expect(
        resolveSourceMapOptions({ dev: true, inlineSourcesContent: false }, true)
          .inlineSourcesContent
      ).toBe(false);
    });
  });
});
