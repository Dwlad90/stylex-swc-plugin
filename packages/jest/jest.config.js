// The suite exercises the built CommonJS artifact in `dist`, so the only
// transform needed is for the TypeScript test files themselves. `@swc/jest` is
// used rather than `ts-jest` because TypeScript 7 no longer exposes a
// JavaScript compiler API for `ts-jest` to drive.
module.exports = {
  testEnvironment: 'node',
  moduleFileExtensions: ['ts', 'js', 'json', 'node'],
  transform: {
    '^.+\\.(ts|js)$': [
      '@swc/jest',
      {
        jsc: {
          parser: { syntax: 'typescript' },
          target: 'es2022',
        },
        module: { type: 'commonjs' },
      },
    ],
  },
};
