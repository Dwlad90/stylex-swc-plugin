import path from 'path';
import stylexPlugin from '@stylexswc/nextjs-plugin';

module.exports = stylexPlugin({
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
      wrapper: (value: string) => `${value}`,
    },
  },
  extractCSS: false,
})({
  transpilePackages: ['@stylexjs/open-props'],
  // Optionally, add any other Next.js config below
});
