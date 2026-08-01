import path from 'path';

import withStyleXRspack from '@stylexswc/nextjs-plugin/rspack';

module.exports = withStyleXRspack({
  loaderOrder: 'first',
  // Add any StyleX options here
  rsOptions: {
    aliases: {
      '@/*': [path.join(__dirname, '*')],
    },
    unstable_moduleResolution: {
      type: 'commonJS',
    },
    dev: process.env.NODE_ENV === 'development',
    runtimeInjection: false,
    treeshakeCompensation: true,
    styleResolution: 'application-order',
    enableDebugClassNames: process.env.NODE_ENV === 'development',
    sxPropName: 'css',
    env: {
      tokens: {
        layout: {
          fullHeight: '100vh',
        },
        fonts: {
          sansSerif: 'sans-serif',
        },
      },
      wrapper: (value: string) => value,
    },
  },
})({
  // Packages that ship untransformed StyleX source; automatically added to
  // the rspack plugin's stylexPackages allowlist
  transpilePackages: ['@stylexjs/open-props', '@stylexswc/design-system'],
  // Next.js type-checks builds through the JavaScript TypeScript compiler
  // API, which TypeScript 7 no longer exposes. The CLI path shells out to
  // tsc instead, so it needs no compiler API.
  experimental: {
    useTypeScriptCli: true,
  },
  // Optionally, add any other Next.js config below
});
