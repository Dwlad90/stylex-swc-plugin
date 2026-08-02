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
  // `./nuxt` exports both a default module and the `ModuleOptions` type, so its
  // declaration cannot use `export =` the way the single-default entries do. It
  // therefore declares a `default` while `cjsDefault` writes `module.exports =`
  // directly, and the two disagree. tsup papered over this by appending
  // `exports.default = module.exports` to every `.cjs`; do the same for the one
  // entry that needs it instead of reinstating that rewrite wholesale.
  footer: ({ format, fileName }) =>
    format === 'cjs' && fileName === 'nuxt.cjs'
      ? { js: '\nmodule.exports.default = module.exports;\n' }
      : {},
  // Both are made blocking on purpose: the exports map has ten subpaths across
  // two module systems, which is far too much surface to keep correct by
  // inspection. tsdown defaults attw to `level: 'warn'`, which reports a
  // types-versus-implementation mismatch -- the footer above dropping off, say
  // -- and then exits 0, so the check has to be raised to `error` to guard
  // anything. Both run against the packed tarball rather than by executing the
  // output, so they hold in the publish job too, where the platform has no
  // `@stylexswc/rs-compiler` native binding to load.
  publint: { strict: true },
  attw: { level: 'error' },
});
