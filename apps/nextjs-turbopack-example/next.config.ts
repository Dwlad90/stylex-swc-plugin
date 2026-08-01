import path from 'path';
import stylexPlugin from '@stylexswc/nextjs-plugin/turbopack';

export default stylexPlugin({
  // Add any StyleX options here
  rsOptions: {
    aliases: {
      '@/*': [path.join(__dirname, '*')],
    },
    unstable_moduleResolution: {
      type: 'commonJS',
    },
    runtimeInjection: false,
    treeshakeCompensation: true,
    styleResolution: 'application-order',
    enableDebugClassNames: process.env.NODE_ENV === 'development',
    env: {
      tokens: {
        layout: {
          fullWidth: '100vw',
          fullHeight: '100vh',
        },
        colors: {
          background: 'white',
          text: 'black',
        },
        fonts: {
          sansSerif: 'sans-serif',
        },
      },
    },
  },
  stylexImports: ['stylex', '@stylexjs/stylex'],
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
