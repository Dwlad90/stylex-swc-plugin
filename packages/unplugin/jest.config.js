
export default {
  testPathIgnorePatterns: ['/__fixtures__/'],
  testEnvironment: 'node',
  preset: 'ts-jest',

  moduleFileExtensions: ['ts', 'js', 'json', 'node'],
  transform: {
    '^.+\\.(ts|tsx|mts|js|jsx|mjs|cjs|html)$': [
      'jest-chain-transform',
      {
        transformers: [
          [
            'ts-jest',
            {
              tsconfig: 'tsconfig.json',
              diagnostics: {
                warnOnly: true,
              },
            },
          ]
        ],
      }
    ],
  },
};