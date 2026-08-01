import { defineConfig } from 'tsdown';

export default defineConfig({
  entry: ['src/*.ts'],
  format: ['cjs', 'esm'],
  platform: 'node',
  target: 'es2017',
  clean: true,
  dts: true,
  // tsup produced shared chunks between the ten entries; keep that so the
  // published output does not gain ten duplicated copies of the core plugin.
  unbundle: false,
  // Stated explicitly rather than relied on as a default: the CommonJS entries
  // are required directly (`require('@stylexswc/unplugin/vite')`) and must stay
  // callable, which is what tsup's `cjsInterop` was doing via the
  // `scripts/postbuild.ts` rewrite that this replaces.
  cjsDefault: true,
  // tsdown defaults ESM to `.mjs`/`.d.mts`. The published `exports` map points
  // at `.js`/`.d.ts`, so keep tsup's filenames here and change the export
  // contract separately rather than silently repointing every subpath.
  outExtensions: ({ format }) => ({
    js: format === 'es' ? '.js' : '.cjs',
    dts: format === 'es' ? '.d.ts' : '.d.cts',
  }),
});
