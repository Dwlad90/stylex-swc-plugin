module.exports = {
  plugins: {
    '@stylexswc/postcss-plugin': {
      include: [
        './src/**/*.{js,jsx,ts,tsx}',
        // any other files that should be included
        // this should include NPM dependencies that use StyleX
      ],
      useCSSLayers: true,
      rsOptions: {
        env: {
          tokens: {
            layout: {
              fullHeight: '100vh',
            },
            spacing: {
              gap: '2rem',
            },
          },
          wrapper: (value) => `${value}`,
        },
      },
    },
    autoprefixer: {},
  },
};
