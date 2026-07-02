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
      wrapper: (value: string) => `${value}`,
    },
  },
})({
  // Packages that ship untransformed StyleX source; automatically added to
  // the rspack plugin's stylexPackages allowlist
  transpilePackages: ['@stylexjs/open-props', '@stylexswc/design-system'],
  // Optionally, add any other Next.js config below
});
