import path from 'path';
import stylexPlugin from '@stylexswc/nextjs-plugin';

module.exports = stylexPlugin({
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
    sxPropName: "css",
    env: {
      tokens: {
        layout: {
          fullHeight: '100vh',
        },
        fonts: {
          sansSerif: 'sans-serif',
        },
      },
      wrapper: (value: string) => `${value}`,
    },
  },
})({
  transpilePackages: ['@stylexjs/open-props'],
  // Next.js type-checks builds through the JavaScript TypeScript compiler
  // API, which TypeScript 7 no longer exposes. The CLI path shells out to
  // tsc instead, so it needs no compiler API.
  experimental: {
    useTypeScriptCli: true,
  },
  // Optionally, add any other Next.js config below
});
