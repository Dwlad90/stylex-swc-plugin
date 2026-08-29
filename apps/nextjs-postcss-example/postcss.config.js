const fs = require('fs');
const path = require('path');

const projectRoot = __dirname;
const monorepoRoot = path.join(projectRoot, '../../');

function getPackageIncludePaths(packageName, nodeModulePaths) {
  let packagePath = null;

  for (const nodeModulePath of nodeModulePaths) {
    const packageJsonPath = path.resolve(nodeModulePath, packageName, 'package.json');
    if (fs.existsSync(packageJsonPath)) {
      packagePath = path.dirname(packageJsonPath);
      break;
    }
  }
  if (!packagePath) {
    throw new Error(`Could not find package ${packageName}`);
  }

  return [
    path.join(packagePath, '**/*.{js,mjs}'),
    '!' + path.join(packagePath, 'node_modules/**/*.{js,mjs}'),
  ];
}

const includePaths = ['@stylexjs/open-props', '@stylexswc/design-system'].flatMap(packageName =>
  getPackageIncludePaths(packageName, [
    path.join(projectRoot, 'node_modules'),
    path.join(monorepoRoot, 'node_modules'),
  ])
);

module.exports = {
  plugins: {
    '@stylexswc/postcss-plugin': {
      // The same eight extensions that the bundler plugins transform. CSS
      // discovery must not scan less than the transform compiles.
      include: [
        'app/**/*.{js,jsx,mjs,cjs,ts,tsx,mts,cts}',
        'components/**/*.{js,jsx,mjs,cjs,ts,tsx,mts,cts}',
        ...includePaths,
      ],
      useCSSLayers: true,
      rsOptions: {
        aliases: {
          '@/*': [path.join(projectRoot, '*')],
        },
        unstable_moduleResolution: {
          type: 'commonJS',
        },
        dev: process.env.NODE_ENV === 'development',
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
          wrapper: value => `${value}`,
        },
      },
    },
    autoprefixer: {},
  },
};
